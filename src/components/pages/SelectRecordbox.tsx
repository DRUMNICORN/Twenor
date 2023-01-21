/**
 * @file SelectRecordbox.tsx
 * @description Display input field with fileselector for xml library and a button next to it to open a file dialog.
 * @author cherob
 * @version 0.0.1
 * @date 2021-06-01
 */

import React from "react";

import i from "../../Interface";
import "../../styles/FileSelector.scss";
import FileSelector from "../util/FileSelector";

class SelectRecordbox extends React.Component {
  state = {
    xmlPath: "",
  };

  componentDidMount() {
    i.get("XML_PATH").then((path) => {
      this.setState({ xmlPath: path });
      return path;
    });
  }

  render() {
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
            defaultValue={this.state.xmlPath}
          ></FileSelector>
        </div>
      </div>
    );
  }
}

export default SelectRecordbox;
