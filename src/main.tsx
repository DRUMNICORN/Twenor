import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./style.css";

// ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
//   <React.StrictMode>
//     <App />
//   </React.StrictMode>
// ); // for some reason this causes an error and run everything twice
// thats because: 
// - StrictMode is a tool for highlighting potential problems in an application. Like Fragment, StrictMode does not render any visible UI. It activates additional checks and warnings for its descendants.

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <App />
);
