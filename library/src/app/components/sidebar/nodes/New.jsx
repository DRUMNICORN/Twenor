// a plus button to add a new row to a table

// when you click on the plus button, it extends to an input field
// on pressing enter, it adds the new row to the table

// import { render } from "react-dom";
import React, { Component, useState } from "react";

import { emit, listen } from "@tauri-apps/api/event";

import ColorPicker from "../../../utils/ColorPicker";

class NewElement extends Component {
  constructor(props) {
    super(props);
    this.state = {
      isEditing: false,
      type: "",
      name: "",
      color: "#ffffff",
    };
  }

  render() {
    // a circle with a plus sign in it
    // when you click on the plus button, it extends to an input field
    // on pressing enter, it adds the new row to the table

    // it switches between the two states based on the isEditing state
    // edit mode: input field
    // normal mode: plus sign

    if (this.state.isEditing) {
      return (
        <div className="input-container">
          <div
            className="input-container__input"
            style={{
              borderColor: this.state.color,
            }}
          >
            #
            <input
              id="node_name"
              placeholder="node"
              className="right-element new-element-input"
              type="text"
              onKeyDown={(e) => {
                if (e.key === "Enter") {
                  // get node_type and node_name
                  // let node_type = document.getElementById("node_type").value;
                  let node_path = document.getElementById("node_name").value;
                  this.setState({ isEditing: false });
                  emit("add-node", {
                    NodePath: {
                      node_path: node_path,
                    },
                  });
                }

                // check if Escape key was pressed
                if (e.key === "Escape") {
                  this.setState({
                    isEditing: false,
                  });
                }
              }}
              // on right click, close the input field
              onContextMenu={(e) => {
                this.setState({
                  isEditing: false,
                });
                e.preventDefault();
              }}
            />
          </div>
          <div className="input-container__picker">
            <ColorPicker
              onColorChange={(color) => {
                this.setState({ color });
              }}
            />
          </div>
        </div>
      );
    } else {
      return (
        <div
          className="table-new-element"
          onClick={() => {
            this.setState({ isEditing: true });
            // emit("", {
            //   Node: {
            //     name: this.state.value,
            //     varient: "genre",
            //     color: "#ff0000",
            //   },
            // });
          }}
        ></div>
      );
    }
  }
}

export default NewElement;
