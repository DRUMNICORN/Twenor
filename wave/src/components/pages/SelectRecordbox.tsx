/**
 * @file SelectRecordbox.tsx
 * @description Display input field with fileselector for xml library and a button next to it to open a file dialog.
 * @author cherob
 * @version 0.0.1
 * @date 2021-06-01
 */

import React from "react";
import Interface from "../../Interface";

import i from "../../Interface";
import "../../styles/FileSelector.scss";
import Button from "../util/Button";
import FileSelector from "../util/FileSelector";

type SelectRecordboxProps = {
  path: string;
  onChange: (path: string) => void;
  onNodes: (nodes: any) => void;
};

type SelectRecordboxState = {
  xml_path: string;
};

class SelectRecordbox extends React.Component<SelectRecordboxProps, SelectRecordboxState> {
  constructor(props: SelectRecordboxProps) {
    super(props);
    this.state = {
      xml_path: this.props.path,
    };
  }

  componentDidMount() {
    if (this.props.path) {
      this.setState({ xml_path: this.props.path });
    }

    i.get("XML_PATH").then((path) => {
      this.setState({ xml_path: path });
      return path;
    });
  }

  render() {
    return (
      <div className="select-recordbox">
        <div className="select-recordbox-wrapper">
          <h2>Select your Recordbox library</h2>
          <p>
            In order to use this app, you need to export your&nbsp;
            <a href="https://www.recordbox.co/" target="_blank" rel="noreferrer">
              Recordbox
            </a>
            &nbsp;library to an XML file. You can do this by opening Recordbox, go to File -&gt; Export Library and select
            the XML format. Then select the file below.
          </p>
          <FileSelector
            label="Recordbox Library"
            accept=".xml"
            onChange={(path) => {
              this.setState({ xml_path: path });
              this.props.onChange(path);
            }}
            defaultValue={this.state.xml_path}
          ></FileSelector>
          <Button
            onClick={() =>
              Interface.get_nodes().then((nodes) => {
                this.props.onNodes(nodes);
              })
            }
          >
            Load
          </Button>
        </div>
      </div>
    );
  }
}

export default SelectRecordbox;
