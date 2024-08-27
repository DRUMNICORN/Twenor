import React, { Component } from "react";
import api from "./api";

class Content extends Component {
  constructor(props) {
    super(props);
    this.state = {
      xml_path: "",
      xml: {},
    };
  }

  componentDidMount() {
    console.log("Content.jsx: componentDidMount()");
  }

  render() {
    return (
      <div className="content">
        <h2>Content</h2>
        <p>{JSON.stringify(this.state.xml)}</p>
      </div>
    );
  }
}

export default Content;

// Path: src\app\Content.jsx
