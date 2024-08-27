import React from "react";

import { render } from "react-dom";

import "../../styles/effects/DropShadow.scss"

class DropShadow extends React.Component {
  constructor(props) {
    super(props);
  }

  render() {
    return (
      <div id="drop-shadow" className={this.props.active ? "active" : ""}>
        {this.props.children}
      </div>
    );
  }
}

export default DropShadow;
