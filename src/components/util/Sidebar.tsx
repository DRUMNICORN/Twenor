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
  onResize: (width: number) => void;
  initial_width: number;
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
      sidebar_width: 0,
    };

    document.addEventListener("mouseup", this.onResizerBarStopHoldLocal);
    document.addEventListener("mousemove", this.onResizerBarDragLocal);
  }

  componentDidMount(): void {}

  componentDidUpdate(prevProps: Readonly<SidebarProps>, prevState: Readonly<SidebarState>, snapshot?: any): void {
    if (prevState.resizing !== this.state.resizing)
      if (this.state.sidebar_width > 0) this.props.onResize(this.state.sidebar_width);

    // check if initial width has changed
    if (prevProps.initial_width !== this.props.initial_width) {
      this.setState({
        sidebar_width: this.props.initial_width,
      });
      // this.props.onResize(this.props.initial_width);
    }
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

  requestUpdateConfigProperty = (property: string, value: string) => {
    // send a request to the backend to update the config property
    // this is a temporary solution, it should be replaced with a request to the backend
  };

  render() {
    return (
      <div className="sidebar" ref={this.state.sidebar_ref}>
        <div
          className="sidebar-content"
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
}

export default Sidebar;
