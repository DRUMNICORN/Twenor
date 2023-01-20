/**
 * @file Content.tsx
 * @description Content component for the Tauri app. Displays the track library and the track editor.
 * @author
 * @version 0.0.1
 * @date 2021-06-01
 *
 */

import React from "react";

import "../styles/Content.scss";
import SelectRecordbox from "./pages/SelectRecordbox";
import Welcome from "./pages/Welcome";
import FileSelector from "./util/FileSelector";

function Content(): JSX.Element {
  // it should contain a logo component and a explorer component
  return (
    <div className="content">
      <div className="content-wrapper">
        <Welcome />
        <SelectRecordbox />
      </div>
    </div>
  );
}

export default Content;
