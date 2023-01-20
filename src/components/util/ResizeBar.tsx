/**
 * @file ResizeBar.tsx
 * @description ResizeBar component for the Tauri app. Allows the user to resize the sidebar.
 * @author
 * @version 0.0.1
 *
 */

import React from "react";

import "../../styles/Sidebar.scss";

// create a new resizebar component
function ResizeBar(): JSX.Element {
  // it should contain a logo component and a explorer component
  return (
    <div className="resizebar">
      <div className="resizebar-handle"></div>
    </div>
  );
}

export default ResizeBar;
