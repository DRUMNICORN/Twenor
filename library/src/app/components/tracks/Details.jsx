{
  /* <div className="top">
<p>Interessante infos über die Node bla bla bla</p>
<p>
  Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore
  magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
  consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla
  pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est
  laborum.
</p>
</div> */
}

import React from "react";

import { render } from "react-dom";

function toUpperCase(str) {
  let res = "";
  for (let i = 0; i < str.length; i++) {
    // without .toUpperCase();
    res += String.fromCharCode(str.charCodeAt(i) & 223);
  }
  return res;
}

class NodeDetails extends React.Component {
  constructor(props) {
    super(props);
  }
  // this.props.node.name in CAPS
  render() {
    let name = this.props.node.name;
    name = toUpperCase(name || "");
    name = `#${name}`;

    return (
      <div className="node-details">
        <div className="title">
          <h1
            draggable={true}
            onDragStart={(e) => {
              console.log("dragging");
              e.dataTransfer.setData("text/plain", this.props.node.name);
            }}
          >
            {name}
          </h1>
        </div>

        <div className="info">
          {/* <p>{this.props.node.description}</p> */}
          <p>
            Lorem ipsum dolor, sit amet consectetur adipisicing elit. Dolor corrupti odit itaque aspernatur quia. Enim unde reprehenderit sed cupiditate. Dolore architecto saepe iste nemo earum veritatis, error placeat pariatur deleniti, dolorem sunt animi provident perferendis? Laborum sequi, culpa, tempore expedita iste dicta porro architecto, blanditiis facere iure quibusdam illo ea?
          </p>
        </div>



      </div>
    );
  }
}

export default NodeDetails;
