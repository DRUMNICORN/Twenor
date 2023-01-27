/**
 * @file FileSelector.tsx
 * @description Display input field with fileselector for xml library and a button next to it to open a file dialog.
 * @author roggen
 * @version 0.0.1
 * @date 2021-06-01
 */

import React from "react";
import Interface from "../../Interface";
import invoke from "../../Interface";
// import Api from "../../Api";

import { open } from "@tauri-apps/api/dialog";

import "../../styles/FileSelector.scss";

type FileSelectorProps = {
  label: string;
  accept: string;
  defaultValue?: string;
  onChange: (path: string) => void;
};

type FileSelectorState = {
  path: string;
};

function onChange(event: React.ChangeEvent<HTMLInputElement>): void {
  const path = event.target.value;
}

class FileSelector extends React.Component<FileSelectorProps, FileSelectorState> {
  constructor(props: FileSelectorProps) {
    super(props);
    this.state = {
      path: this.props.defaultValue ? this.props.defaultValue : "",
    };
  }

  componentDidMount() {
    Interface.get("XML_PATH").then((path) => {
      this.setState({ path: path });
      return path;
    });
  }

  render() {
    return (
      <div className="file-selector">
        <input
          type="text"
          id="file"
          accept={this.props.accept}
          name={this.props.label}
          onChange={onChange}
          defaultValue={this.state.path}
        />

        <button
          type="button"
          onClick={() => {
            open({
              title: "Select Recordbox Library",
              filters: [{ name: "XML", extensions: ["xml"] }],
              multiple: false,
            }).then((path) => {
              let pathString = path as string;
              this.setState({ path: pathString });
              this.props.onChange(pathString);
            });
          }}
        >
          <p>Select File</p>
        </button>
      </div>
    );
  }
}

export default FileSelector;
