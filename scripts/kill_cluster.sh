#!/bin/bash

echo "Killing all compute-node processes"
pgrep -lf compute-node | awk '{print $1}' | xargs kill 2>/dev/null
pgrep -lf metadata-node | awk '{print $1}' | xargs kill 2>/dev/null
pgrep -lf pgserver | awk '{print $1}' | xargs kill 2>/dev/null
sleep 1

SCRIPT_PATH="$(cd "$(dirname "$0")" >/dev/null 2>&1 && pwd)"
cd "$SCRIPT_PATH/.." || exit 1
rm -rf log/
echo "Cleaned up $(pwd)/log"
