import React, { Component, useState } from "react";

import "../../styles/tools/ColorPicker.scss";

class ColorPicker extends Component {
  // window.addEventListener('scroll', () => {
  //   console.log('????');
  // });
  constructor(props) {
    super(props);
    this.state = {
      color: "#ffffff",
    };
  }

  render() {
    return (
      <div className="color-picker">
        <input
          className="color-picker__input"
          type="color"
          value={this.state.color}
          onChange={(e) => {
            this.setState({ color: e.target.value });
            this.props.onColorChange(e.target.value);
          }}
        />
      </div>
    );
  }
}

export default ColorPicker;
