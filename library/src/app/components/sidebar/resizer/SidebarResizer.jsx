import React, { Component } from "react";

class SidebarResizer extends Component {
  constructor(props) {
    super(props);
    this.state = {
      config: {},
      sidebar_ref: props.sidebar_ref,
    };


    // check if sidebar_ref is passed as props
    // this.state.sidebar_ref = React.createRef();

    // sidebar_ref is referenced by parent given py props
    // this.state.resizing = false;

    // this.state.onResizerBarHold = this.onResizerBarHold.bind(this);
    // this.state.onResizerBarStopHold = this.onResizerBarStopHold.bind(this);
    // this.state.onResizerBarDrag = this.onResizerBarDrag.bind(this);
  }

  componentDidMount() {
    console.log("SidebarResizer.jsx: componentDidMount()");
    this.setState({
      config: {
        sidebar_width: "200",
      },
    });
  }

  onResizerBarHold = (e) => {
    this.setState({
      resizing: true,
    });
  };

  onResizerBarStopHold = (e) => {
    this.setState({
      resizing: false,
    });
  };

  onResizerBarDrag = (e) => {
    if (this.state.resizing) {
      this.requestUpdateConfigProperty(
        "sidebar_width",
        (e.clientX - this.state.sidebar_ref.current.getBoundingClientRect().left).toString()
      );
    }
  };

  requestUpdateConfigProperty = (property, value) => {
    console.log("SidebarResizer.jsx: requestUpdateConfigProperty()");
  };

  render() {
    return (
      <div
        className="app-sidebar-resizer"
        onMouseDown={this.onResizerBarHold}
        onMouseMove={this.onResizerBarDrag}
        onMouseUp={this.onResizerBarStopHold}
      />
    );
  }
}

export default SidebarResizer;

// Path: src\app\components\sidebar\resizer\SidebarResizer.jsx
