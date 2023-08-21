(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[695],{69850:function(e,t,r){(window.__NEXT_P=window.__NEXT_P||[]).push(["/cluster",function(){return r(97050)}])},14993:function(e,t,r){"use strict";r.d(t,{OL:function(){return o},X8:function(){return s},xv:function(){return a}});var n=r(77617),i=r(79897);async function s(){let e=await i.Z.get("/api/metrics/cluster");return e}async function a(){let e=(await i.Z.get("/api/clusters/1")).map(n.cX.fromJSON);return e}async function o(){let e=(await i.Z.get("/api/clusters/2")).map(n.cX.fromJSON);return e}},97050:function(e,t,r){"use strict";r.r(t),r.d(t,{default:function(){return _}});var n=r(85893),i=r(40639),s=r(52189),a=r(66479),o=r(66678),l=r.n(o),c=r(31351),u=r.n(c),d=r(89734),m=r.n(d),h=r(9008),x=r.n(h),p=r(67294),f=r(9253),j=r(83323),w=r(3023),g=r(75358),y=r(86108),b=r(96756),v=r(14993);function C(e){var t,r;let{workerNodeType:s,workerNode:a}=e;return(0,n.jsx)(p.Fragment,{children:(0,n.jsxs)(i.gC,{alignItems:"start",spacing:1,children:[(0,n.jsxs)(i.Ug,{children:[(0,n.jsx)(i.xu,{w:3,h:3,flex:"none",bgColor:"green.600",rounded:"full"}),(0,n.jsxs)(i.xv,{fontWeight:"medium",fontSize:"xl",textColor:"black",children:[s," #",a.id]})]}),(0,n.jsx)(i.xv,{textColor:"gray.500",m:0,children:"Running"}),(0,n.jsxs)(i.xv,{textColor:"gray.500",m:0,children:[null===(t=a.host)||void 0===t?void 0:t.host,":",null===(r=a.host)||void 0===r?void 0:r.port]})]})})}function k(e){let{job:t,instance:r,metrics:a,isCpuMetrics:o}=e,c=(0,p.useCallback)(()=>{let e=[];if(0===a.length)return[];let t=a.at(-1).timestamp;for(let r of u()(l()(a))){for(;t-r.timestamp>0;)t-=60,e.push({timestamp:t,value:0});e.push(r),t-=60}for(;e.length<60;)e.push({timestamp:t,value:0}),t-=60;return u()(e)},[a]);return(0,n.jsx)(p.Fragment,{children:(0,n.jsxs)(i.gC,{alignItems:"start",spacing:1,children:[(0,n.jsxs)(i.xv,{textColor:"gray.500",mx:3,children:[(0,n.jsx)("b",{children:t})," ",r]}),(0,n.jsx)(f.h,{width:"100%",height:100,children:(0,n.jsxs)(j.T,{data:c(),children:[(0,n.jsx)(w.K,{dataKey:"timestamp",type:"number",domain:["dataMin","dataMax"],hide:!0}),o&&(0,n.jsx)(g.B,{type:"number",domain:[0,1],hide:!0}),(0,n.jsx)(y.u,{isAnimationActive:!1,type:"linear",dataKey:"value",strokeWidth:1,stroke:s.rS.colors.teal["500"],fill:s.rS.colors.teal["100"]})]})})]})})}function _(){let[e,t]=(0,p.useState)([]),[r,s]=(0,p.useState)([]),[o,l]=(0,p.useState)(),c=(0,a.pm)();(0,p.useEffect)(()=>((async function(){try{t(await (0,v.xv)()),s(await (0,v.OL)())}catch(e){c({title:"Error Occurred",description:e.toString(),status:"error",duration:5e3,isClosable:!0}),console.error(e)}})(),()=>{}),[c]),(0,p.useEffect)(()=>((async function(){for(;;)try{let e=await (0,v.X8)();e.cpuData=m()(e.cpuData,e=>e.metric.instance),e.memoryData=m()(e.memoryData,e=>e.metric.instance),l(e),await new Promise(e=>setTimeout(e,5e3))}catch(e){c({title:"Error Occurred",description:e.toString(),status:"error",duration:5e3,isClosable:!0}),console.error(e);break}})(),()=>{}),[c]);let u=(0,n.jsxs)(i.xu,{p:3,children:[(0,n.jsx)(b.Z,{children:"Cluster Overview"}),(0,n.jsxs)(i.rj,{my:3,templateColumns:"repeat(3, 1fr)",gap:6,width:"full",children:[e.map(e=>(0,n.jsx)(i.P4,{w:"full",rounded:"xl",bg:"white",shadow:"md",borderWidth:1,p:6,children:(0,n.jsx)(C,{workerNodeType:"Frontend",workerNode:e})},e.id)),r.map(e=>(0,n.jsx)(i.P4,{w:"full",rounded:"xl",bg:"white",shadow:"md",borderWidth:1,p:6,children:(0,n.jsx)(C,{workerNodeType:"Compute",workerNode:e})},e.id))]}),(0,n.jsx)(b.Z,{children:"CPU Usage"}),(0,n.jsx)(i.MI,{my:3,columns:3,spacing:6,width:"full",children:o&&o.cpuData.map(e=>(0,n.jsx)(i.P4,{w:"full",rounded:"xl",bg:"white",shadow:"md",borderWidth:1,children:(0,n.jsx)(k,{job:e.metric.job,instance:e.metric.instance,metrics:e.sample,isCpuMetrics:!0})},e.metric.instance))}),(0,n.jsx)(b.Z,{children:"Memory Usage"}),(0,n.jsx)(i.MI,{my:3,columns:3,spacing:6,width:"full",children:o&&o.memoryData.map(e=>(0,n.jsx)(i.P4,{w:"full",rounded:"xl",bg:"white",shadow:"md",borderWidth:1,children:(0,n.jsx)(k,{job:e.metric.job,instance:e.metric.instance,metrics:e.sample,isCpuMetrics:!1})},e.metric.instance))})]});return(0,n.jsxs)(p.Fragment,{children:[(0,n.jsx)(x(),{children:(0,n.jsx)("title",{children:"Cluster Overview"})}),u]})}}},function(e){e.O(0,[184,340,133,575,774,888,179],function(){return e(e.s=69850)}),_N_E=e.O()}]);