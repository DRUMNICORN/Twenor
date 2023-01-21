/**
 * @file Sidebar.tsx
 * @description Sidebar component for the Tauri app.
 * @author
 * @version 0.0.1
 * @date 2021-06-01
 *
 */

import React from "react";

import ResizeBar from "./ResizeBar";

import "../../styles/Sidebar.scss";

type SidebarProps = {
  children: React.ReactNode;
};

type SidebarState = {
  resizing: boolean;
  sidebar_ref: React.RefObject<HTMLDivElement>;
  sidebar_width: number;
};

class Sidebar extends React.Component<SidebarProps, SidebarState> {
  constructor(props: SidebarProps) {
    super(props);
    this.state = {
      resizing: false,
      sidebar_ref: React.createRef(),
      sidebar_width: 300,
    };

    document.addEventListener("mouseup", this.onResizerBarStopHoldLocal);
    document.addEventListener("mousemove", this.onResizerBarDragLocal);
  }

  onResizerBarStopHoldLocal = (e: MouseEvent) => {
    this.setState({
      resizing: false,
    });
  };

  onResizerBarDragLocal = (e: MouseEvent) => {
    if (this.state.resizing) {
      let bounding_left_ref = this.state.sidebar_ref.current;
      let bounding_left = 0;
      if (bounding_left_ref) {
        bounding_left = bounding_left_ref.getBoundingClientRect().left;
      } else {
        bounding_left = 0;
      }

      let new_sidebar_width = e.clientX - bounding_left;

      if (new_sidebar_width < 0) {
        new_sidebar_width = 0;
      }

      this.setState({
        sidebar_width: new_sidebar_width,
      });

    }
  };

  render() {
    return (
      <div className="sidebar" ref={this.state.sidebar_ref}>
        <div
          className="content"
          style={{
            width: this.state.sidebar_width,
          }}
        >
          {this.props.children}
        </div>
        <ResizeBar onResizerBarHold={this.onResizerBarHold} onResizerBarStopHold={this.onResizerBarStopHold}></ResizeBar>
      </div>
    );
  }

  onResizerBarHold = (e: React.MouseEvent) => {
    if (this.state.resizing) {
      return;
    }
    this.setState({
      resizing: true,
    });
  };

  onResizerBarStopHold = (e: React.MouseEvent) => {
    if (!this.state.resizing) {
      return;
    }
    this.setState({
      resizing: false,
    });
  };

  requestUpdateConfigProperty = (property: string, value: string) => {
    // send a request to the backend to update the config property
    // this is a temporary solution, it should be replaced with a request to the backend
  };
}

export default Sidebar;
