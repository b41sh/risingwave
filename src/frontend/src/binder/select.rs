// Copyright 2022 Singularity Data
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt::Debug;
use std::iter;

use itertools::Itertools;
use regex::Regex;
use risingwave_common::error::{ErrorCode, Result};
use risingwave_common::types::DataType;
use risingwave_sqlparser::ast::{Expr, Ident, Select, SelectItem};

use super::bind_context::{Clause, ColumnBinding};
use super::UNNAMED_COLUMN;
use crate::binder::{Binder, Relation};
use crate::catalog::{is_row_id_column_name, ROWID_PREFIX};
use crate::expr::{Expr as _, ExprImpl, InputRef};

#[derive(Debug)]
pub struct BoundSelect {
    pub distinct: bool,
    pub select_items: Vec<ExprImpl>,
    pub aliases: Vec<Option<String>>,
    pub from: Option<Relation>,
    pub where_clause: Option<ExprImpl>,
    pub group_by: Vec<ExprImpl>,
}

impl BoundSelect {
    /// The names returned by this [`BoundSelect`].
    pub fn names(&self) -> Vec<String> {
        self.aliases
            .iter()
            .cloned()
            .map(|alias| alias.unwrap_or_else(|| UNNAMED_COLUMN.to_string()))
            .collect()
    }

    /// The types returned by this [`BoundSelect`].
    pub fn data_types(&self) -> Vec<DataType> {
        self.select_items
            .iter()
            .map(|item| item.return_type())
            .collect()
    }

    pub fn is_correlated(&self) -> bool {
        self.select_items
            .iter()
            .chain(self.group_by.iter())
            .chain(self.where_clause.iter())
            .any(|expr| expr.has_correlated_input_ref())
    }
}

impl Binder {
    pub(super) fn bind_select(&mut self, select: Select) -> Result<BoundSelect> {
        // Bind FROM clause.
        let from = self.bind_vec_table_with_joins(select.from)?;

        // Bind WHERE clause.
        self.context.clause = Some(Clause::Where);
        let selection = select
            .selection
            .map(|expr| self.bind_expr(expr))
            .transpose()?;
        self.context.clause = None;

        if let Some(selection) = &selection {
            let return_type = selection.return_type();
            if return_type != DataType::Boolean {
                return Err(ErrorCode::InternalError(format!(
                    "argument of WHERE must be boolean, not type {:?}",
                    return_type
                ))
                .into());
            }
        }

        // Bind GROUP BY clause.
        let group_by = select
            .group_by
            .into_iter()
            .map(|expr| self.bind_expr(expr))
            .try_collect()?;

        // Bind SELECT clause.
        let (select_items, aliases) = self.bind_project(select.projection)?;

        Ok(BoundSelect {
            distinct: select.distinct,
            select_items,
            aliases,
            from,
            where_clause: selection,
            group_by,
        })
    }

    pub fn bind_project(
        &mut self,
        select_items: Vec<SelectItem>,
    ) -> Result<(Vec<ExprImpl>, Vec<Option<String>>)> {
        let mut select_list = vec![];
        let mut aliases = vec![];
        for item in select_items {
            match item {
                SelectItem::UnnamedExpr(expr) => {
                    let (select_expr, alias) = match &expr.clone() {
                        Expr::Identifier(ident) => {
                            (self.bind_expr(expr)?, Some(ident.value.clone()))
                        }
                        Expr::CompoundIdentifier(idents) => (
                            self.bind_expr(expr)?,
                            idents.last().map(|ident| ident.value.clone()),
                        ),
                        Expr::FieldIdentifier(field_expr, idents) => {
                            self.bind_single_field_column(*field_expr.clone(), idents)?
                        }
                        _ => (self.bind_expr(expr)?, None),
                    };
                    select_list.push(select_expr);
                    aliases.push(alias);
                }
                SelectItem::ExprWithAlias { expr, alias } => {
                    if is_row_id_column_name(&alias.value) {
                        return Err(ErrorCode::InternalError(format!(
                            "column name prefixed with {:?} are reserved word.",
                            ROWID_PREFIX
                        ))
                        .into());
                    }

                    let expr = self.bind_expr(expr)?;
                    select_list.push(expr);
                    aliases.push(Some(alias.value));
                }
                SelectItem::QualifiedWildcard(obj_name) => {
                    let table_name = &obj_name.0.last().unwrap().value;
                    let (begin, end) = self.context.range_of.get(table_name).ok_or_else(|| {
                        ErrorCode::ItemNotFound(format!("relation \"{}\"", table_name))
                    })?;
                    let (exprs, names) = Self::bind_columns(&self.context.columns[*begin..*end])?;
                    select_list.extend(exprs);
                    aliases.extend(names);
                }
                SelectItem::ExprQualifiedWildcard(expr, idents) => {
                    let (exprs, names) = self.bind_wildcard_field_column(expr, &idents.0)?;
                    select_list.extend(exprs);
                    aliases.extend(names);
                }
                SelectItem::Wildcard => {
                    let (exprs, names) = Self::bind_visible_columns(&self.context.columns[..])?;
                    select_list.extend(exprs);
                    aliases.extend(names);
                }
            }
        }
        Ok((select_list, aliases))
    }

    /// Bind wildcard field column.
    pub fn bind_wildcard_field_column(
        &mut self,
        expr: Expr,
        ids: &[Ident],
    ) -> Result<(Vec<ExprImpl>, Vec<Option<String>>)> {
        let idents = self.extract_table_and_field_name(expr, ids)?;
        let (begin, end, column_name) = match &idents[..] {
            [table, column] => {
                // Only struct type column can use wildcard selection.
                let bind = self
                    .context
                    .get_column_with_table_name(&column.value, &table.value)?;
                if !bind.data_type.is_struct() {
                    return Err(ErrorCode::BindError(format!(
                        "type {:?} is not composite",
                        bind.data_type
                    ))
                    .into());
                }
                let (begin, end) = self.context.range_of.get(&table.value).ok_or_else(|| {
                    ErrorCode::ItemNotFound(format!("relation \"{}\"", table.value))
                })?;
                (*begin, *end, column.value.clone())
            }
            [column] => {
                // Only struct type column can use wildcard selection.
                let bind = self.context.get_column(&column.value)?;
                if !bind.data_type.is_struct() {
                    return Err(ErrorCode::BindError(format!(
                        "type {:?} is not composite",
                        bind.data_type
                    ))
                    .into());
                }
                // If not given table name, use whole index in columns.
                (0, self.context.columns.len(), column.value.clone())
            }
            _ => {
                return Err(ErrorCode::InternalError(format!(
                    "The number of idents is not match: {:?}",
                    idents
                ))
                .into());
            }
        };

        // Regular expression is '^name+' which match column_name is start with this.
        let column_re = Regex::new(&("^".to_owned() + &column_name + "+")).unwrap();
        let mut columns: Vec<ColumnBinding> = vec![];
        for col in &self.context.columns[begin..end] {
            if column_re.is_match(&col.column_name) {
                columns.push(col.clone());
            }
        }

        Self::bind_columns(&columns[..])
    }

    /// Bind single field column.
    pub fn bind_single_field_column(
        &mut self,
        expr: Expr,
        ids: &[Ident],
    ) -> Result<(ExprImpl, Option<String>)> {
        let idents = self.extract_table_and_field_name(expr, ids)?;
        let column = match &idents[..] {
            [table, column] => self
                .context
                .get_column_with_table_name(&column.value, &table.value)?,
            [column] => self.context.get_column(&column.value)?,
            _ => {
                return Err(ErrorCode::InternalError(format!(
                    "The number of idents is not match: {:?}",
                    idents
                ))
                .into())
            }
        };
        Ok((
            InputRef::new(column.index, column.data_type.clone()).into(),
            Some(column.column_name.clone()),
        ))
    }

    /// This function will accept three expr type: CompoundIdentifier,Identifier,Cast(Todo)
    /// We will extract the column name and concat it with idents in ids to form field name.
    /// Will return table name and field name or just field name.
    pub fn extract_table_and_field_name(
        &mut self,
        expr: Expr,
        ids: &[Ident],
    ) -> Result<Vec<Ident>> {
        match expr.clone() {
            // For CompoundIdentifier, we will use first ident as table name and second ident as
            // column name.
            Expr::CompoundIdentifier(idents) => {
                let (table_name, column): (&String, &String) = match &idents[..] {
                    [table, column] => (&table.value, &column.value),
                    _ => {
                        return Err(ErrorCode::InternalError(format!(
                            "Too many idents: {:?}",
                            idents
                        ))
                        .into());
                    }
                };
                let fields = iter::once(column.clone())
                    .chain(ids.iter().map(|id| id.value.clone()))
                    .collect_vec();
                Ok(vec![
                    Ident::new(table_name),
                    Ident::new(Self::concat_field_idents(fields)),
                ])
            }
            // For Identifier, we will first use the ident as column name and judge if field name is
            // exist. If not we will use the ident as table name.
            // The reason is that in pgsql, for table name v3 have a column name v3 which
            // have a field name v3. Select (v3).v3 from v3 will return the field value instead
            // of column value.
            Expr::Identifier(ident) => {
                let fields: Vec<String> = iter::once(ident.value.clone())
                    .chain(ids.iter().map(|id| id.value.clone()))
                    .collect_vec();
                let field_name = Self::concat_field_idents(fields.clone());
                if self.context.judge_column_is_exist(&field_name) {
                    Ok(vec![Ident::new(field_name)])
                } else {
                    let field_name = Self::concat_field_idents(fields[1..].to_vec());
                    Ok(vec![ident, Ident::new(field_name)])
                }
            }
            Expr::Cast { .. } => {
                todo!()
            }
            _ => Err(ErrorCode::NotImplementedError(format!("{:?}", expr)).into()),
        }
    }

    /// Use . to concat value.
    pub fn concat_field_idents(idents: Vec<String>) -> String {
        if idents.is_empty() {
            return "".to_string();
        }
        let mut column_name = idents[0].clone();
        for id in &idents[1..] {
            column_name = column_name + "." + id;
        }
        column_name
    }

    pub fn bind_columns(columns: &[ColumnBinding]) -> Result<(Vec<ExprImpl>, Vec<Option<String>>)> {
        let bound_columns = columns
            .iter()
            .map(|column| {
                (
                    InputRef::new(column.index, column.data_type.clone()).into(),
                    Some(column.column_name.clone()),
                )
            })
            .unzip();
        Ok(bound_columns)
    }

    pub fn bind_visible_columns(
        columns: &[ColumnBinding],
    ) -> Result<(Vec<ExprImpl>, Vec<Option<String>>)> {
        let bound_columns = columns
            .iter()
            .filter_map(|column| {
                if !column.is_hidden {
                    Some((
                        InputRef::new(column.index, column.data_type.clone()).into(),
                        Some(column.column_name.clone()),
                    ))
                } else {
                    None
                }
            })
            .unzip();
        Ok(bound_columns)
    }
}
