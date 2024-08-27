import React, { Component } from "react";

import SidebarResizer from "./resizer/SidebarResizer";
import Nodes from "./nodes/Nodes";

class Sidebar extends Component {
  constructor(props) {
    super(props);
    this.state = {
      config: {},
      sidebar_ref: React.createRef(),
    };
  }

  componentDidMount() {
    console.log("Content.jsx: componentDidMount()");

    this.setState({
      config: {
        sidebar_width: "200",
      },
    });
  }

  render() {
    return (
      // <div
      //   ref={this.state.sidebar_ref}
      //   className="app-sidebar"
      //   style={{ width: `${parseInt(this.state.config.sidebar_width) / 16}em` }}
      // >
      //   <div className="app-sidebar-content">
      //     <Nodes
      //       selected_node={this.state.selected_node}
      //       nodes={this.state.nodes || []}
      //       dnd={this.state.dnd}
      //       dragging={this.state.file_dragging}
      //       onClickNode={this.onNodeClick}
      //     />
      //   </div>
      // </div>

      // rebuild sidebar with new react components for better organization
      // name of  resizer bar should be changed to something more appropriate like "sidebar-resizer"
      // Q: should it be a separate component?
      // A: yes, it should be a separate component#
      // Named it "SidebarResizer" and moved it to a separate file
      // stored it in src\app\components\sidebar\resizer\SidebarResizer.jsx

      <div className="app-sidebar">
        <div className="app-sidebar-content">
          <Nodes />
        </div>
        <SidebarResizer sidebar_ref={this.state.sidebar_ref} />
      </div>
    );
  }
}

export default Sidebar;

// Path: src\app\components\sidebar\Sidebar.jsx
