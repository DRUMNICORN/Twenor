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
  onChoose: (node: Node) => void;
};

type NodeState = {};

class NodeComponent extends React.Component<NodeProps, NodeState> {
  constructor(props: NodeProps) {
    super(props);
    this.state = {};
  }

  componentDidUpdate(): void {}

  onChoose(): void {
    let node = this.props.node;
    this.props.onChoose(node);
  }

  render() {
    let nodes = this.props.node.NODE;
    let depth = this.props.node.PATH.split("/").length;

    // if the node is root dont show it
    if (depth == 0) {
      return (
        <>
          {(nodes || []).map((node) => {
            return <NodeComponent node={node} key={node.PATH} onChoose={this.props.onChoose} />;
          })}
        </>
      );
    }

    return (
      <div className="explorer-node">
        <div className="explorer-node-wrapper">
          <div
            className="explorer-node-title"
            style={{ marginLeft: (depth - 1) * 10 }}
            onClick={() => {
              this.onChoose();
            }}
          >
            {this.props.node.Name}
          </div>
          {(nodes || []).map((node) => {
            return <NodeComponent node={node} key={node.PATH} onChoose={this.props.onChoose} />;
          })}
        </div>
      </div>
    );
  }
}

export default NodeComponent;

export type { Node, Track };
