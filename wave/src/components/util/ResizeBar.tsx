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

type ResizeBarProps = {
  onResizerBarHold: (e: React.MouseEvent<HTMLDivElement, MouseEvent>) => void;
  onResizerBarStopHold: (e: React.MouseEvent<HTMLDivElement, MouseEvent>) => void;
};

function ResizeBar(props: ResizeBarProps): JSX.Element {
  // it should contain a logo component and a explorer component
  return (
    <div
      className="resizebar"
      onMouseDown={props.onResizerBarHold}
      onMouseUp={props.onResizerBarStopHold}
    ></div>
  );
}

export default ResizeBar;
