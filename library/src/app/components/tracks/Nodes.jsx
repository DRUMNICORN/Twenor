import React from "react";
import { render } from "react-dom";

class Nodes extends React.Component {
  constructor(props) {
    super(props);
  }

  handleMouseOver(e) {
    e.target.style.cursor = "pointer";
    //set max width of the node to computed width of max-content
    e.target.style.maxWidth = "100%";
  }

  render() {
    return (
      <div className="track-node-element scrollable">
        {/* {Object.keys(this.props.nodes).map((key) => {
          return (
            <div
              className="track-node"
              key={key}
              onClick={(e) => {
                this.props.onClickNode(this.props.nodes[key]);
              }}
              onMouseOver={(e) => {
                this.handleMouseOver(e);
              }}
              onMouseOut={(e) => {
                e.target.style.cursor = "default";
                e.target.style.maxWidth = ".5em";
              }}
            >
              #{this.props.nodes[key]}
            </div>
          );
        })} */}
      </div>
    );
  }
}

export default Nodes;
