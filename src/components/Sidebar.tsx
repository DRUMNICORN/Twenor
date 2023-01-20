/**
 * @file Sidebar.tsx
 * @description Sidebar component for the Tauri app.
 * @author
 * @version 0.0.1
 * @date 2021-06-01
 *
 */

import React from "react";

import ResizeBar from "./util/ResizeBar";

import "../styles/Sidebar.scss";

// the parent element passes the children to the sidebar component

function Sidebar(props: { children: React.ReactNode }): JSX.Element {
  // it should contain a logo component and a explorer component
  return (
    <div className="sidebar">
      <div className="content">{props.children}</div>
      <div className="resizebar"></div>
    </div>
  );
}

export default Sidebar;
