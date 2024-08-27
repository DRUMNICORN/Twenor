import React, { Component } from "react";

import "../styles/Welcome.scss";
// import Player from "./layout/Player";

class Welcome extends Component {
  constructor(props) {
    super(props);
    this.state = {
      mounted: false,
    };
  }

  componentDidMount() {
    this.setState({ mounted: true });
  }

  render() {
    return (
      <div className="welcome">
        <h1>Welcome to TRAL</h1>
        <p>Tauri</p>
        <p>Rust</p>
        <p>Audio</p>
        <p>Library</p>
      </div>
    );
  }
}

export default Welcome;
