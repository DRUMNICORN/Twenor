/**
 * @file Node.tsx
 * @description Node component for the Tauri app. Displays a node in the track library.
 * @author
 * @version 0.0.1
 * @date 2021-06-01
 *
 */

import React from "react";
import { Node, Track } from "./Datatypes";

type NodeProps = {
  node: Node;
};

type NodeState = {};

class NodeComponent extends React.Component<NodeProps, NodeState> {
  constructor(props: NodeProps) {
    super(props);
    this.state = {};
  }

  componentDidUpdate(): void {}

  render() {
    let nodes = this.props.node.NODE;
    return (
      <div className="explorer-node">
        <div className="explorer-node-wrapper">
          <div className="explorer-node-title">{this.props.node.Name}</div>
          {(nodes || []).map((node) => {
            console.log(node);
            return <NodeComponent node={node} />;
          })}
        </div>
      </div>
    );
  }
}

export default NodeComponent;

export type { Node, Track };
