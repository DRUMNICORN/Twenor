import React, { Component } from "react";
// import Player from "./layout/Player";
// import Nodes from "./layout/Nodes";
// import Tracks from "./layout/Tracks";
// import Welcome from "./Welcome";

import { DragDropContext } from "react-beautiful-dnd";

import Sidebar from "./components/sidebar/Sidebar";
import Content from "./Content";

import "../styles/Main.scss";

class Main extends Component {
  constructor(props) {
    super(props);

    this.state = {
      selected_node: null,
    };
  }

  render() {
    return (
      <div className="layout">
        <DragDropContext>
          <div className="app-container">
            <Sidebar />
            {this.state.selected_node ? <Content selected_node={this.state.selected_node} /> : <Content />}
          </div>
        </DragDropContext>
      </div>
    );
  }
}

export default Main;

// Path: src\app\Main.jsx
