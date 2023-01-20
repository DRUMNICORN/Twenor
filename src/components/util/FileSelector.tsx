/**
 * @file FileSelector.tsx
 * @description Display input field with fileselector for xml library and a button next to it to open a file dialog.
 * @author roggen
 * @version 0.0.1
 * @date 2021-06-01
 */

import { invoke } from "@tauri-apps/api";
import React from "react";
// import Api from "../../Api";

import "../../styles/FileSelector.scss";

type FileSelectorProps = {
  label: string;
  accept: string;
  defaultValue?: string;
  onChange: (event: React.ChangeEvent<HTMLInputElement>) => void;
};

type FileSelectorState = {
  path: string;
};

function onChange(event: React.ChangeEvent<HTMLInputElement>): void {
  const path = event.target.value;
  // should open a file dialog from input field
  console.log(path);
}

function FileSelector(props: FileSelectorProps, state: FileSelectorState): JSX.Element {
  let path = state.path || props.defaultValue;

  return (
    <div className="file-selector">
      {/* <label htmlFor="file">{path}</label> */}
      <input
        type="text"
        id="file"
        accept={props.accept}
        name={props.label}
        onChange={onChange}
        defaultValue={props.defaultValue}
      />

      <button
        type="button"
        onClick={() => {
          invoke("open_file_dialog", { accept: props.accept });
        }}
      >
        <p>Select File</p>
      </button>
    </div>
  );
}

export default FileSelector;
