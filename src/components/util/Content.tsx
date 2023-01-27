/**
 * @file Content.tsx
 * @description Content component for the Tauri app. Displays the track library and the track editor.
 * @author
 * @version 0.0.1
 * @date 2021-06-01
 *
 */

import React from "react";

import "../../styles/Content.scss";

type ContentProps = {
  children: React.ReactNode;
};

function Content(props: ContentProps): JSX.Element {
  // it should contain a logo component and a explorer component
  return (
    <div className="content">
      <div className="content-wrapper">{props.children}</div>
    </div>
  );
}

export default Content;
