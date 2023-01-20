/**
 * @file SelectRecordbox.tsx
 * @description Display input field with fileselector for xml library and a button next to it to open a file dialog.
 * @author cherob
 * @version 0.0.1
 * @date 2021-06-01
 */

import React from "react";

import { useAsync } from "react-async";

import { invoke } from "@tauri-apps/api/tauri";

import "../../styles/FileSelector.scss";
import FileSelector from "../util/FileSelector";

// create a new file selector component

function SelectRecordbox(): JSX.Element {
  // it should a input field with fileselector for xml library and a button next to it to select a file
  // have a little text explaining what the app does with recordbox, that you can export your library to xml and import it here
  // and a button to inintiliaze the library

  const fetch_recordbox_library = async (): Promise<string> => {
    const path = await invoke("get_recordbox_library");
    return path as string;
  };

  return (
    <div className="select-recordbox">
      <div className="select-recordbox-wrapper">
        <h2>Select your Recordbox library</h2>
        <p>
          In order to use this app, you need to export your Recordbox library to an XML file. You can do this by opening
          Recordbox, go to File -&gt; Export Library and select the XML format. Then select the file below.
        </p>
        <FileSelector
          label="Recordbox Library"
          accept=".xml"
          onChange={() => {}}
          defaultValue={useAsync(fetch_recordbox_library).data || ""}
        />
      </div>
    </div>
  );
}

export default SelectRecordbox;
