/**
 * @file Sidebar.tsx
 * @description Sidebar component for the Tauri app.
 * @author
 * @version 0.0.1
 * @date 2021-06-01
 *
 */

import React from "react";

import Logo from "./Sidebar/Logo";
import Explorer from "./Sidebar/Explorer";

import "../styles/Sidebar.scss";

// create a new sidebar component
function Sidebar(): JSX.Element {
  // it should contain a logo component and a explorer component
  return (
    <div className="sidebar">
      <Logo />
      <Explorer />
    </div>
  );
}

export default Sidebar;
