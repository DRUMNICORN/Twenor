/**
 * @file Node.tsx
 * @description Node component for the Tauri app. Displays a node in the track library.
 * @author
 * @version 0.0.1
 * @date 2021-06-01
 *
 */

import React from "react";
import Button from "./Button";
import { Node, Track } from "./Datatypes";

type NodeProps = {
  node: Node;
  open: Node[];
  onToggle: (node: Node) => void;
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
            return (
              <NodeComponent
                node={node}
                key={node.PATH}
                onChoose={this.props.onChoose}
                onToggle={this.props.onToggle}
                open={this.props.open}
              />
            );
          })}
        </>
      );
    }
    return (
      <div className="explorer-node">
        <div className="explorer-node-wrapper">
          <div className="explorer-node-header">
            <div
              className="explorer-node-title"
              style={{ marginLeft: (depth - 1) * 10 }}
              onClick={() => {
                this.onChoose();
              }}
            >
              {this.props.node.Name}
            </div>
            <div
              className="explorer-node-open"
              onClick={() => {
                this.props.onToggle(this.props.node);
              }}
            >
              {nodes.length > 0 ? (
                this.props.open.includes(this.props.node) ? (
                  <Button
                    link="material-symbols/featured-play-list-rounded"
                    onClick={function (): void {
                      throw new Error("Function not implemented.");
                    }}
                  ></Button>
                ) : (
                  <Button
                    link="material-symbols/featured-play-list-outline"
                    onClick={function (): void {
                      throw new Error("Function not implemented.");
                    }}
                  ></Button>
                )
              ) : null}
            </div>
          </div>

          {this.props.open.includes(this.props.node) ? (
            <div className="explorer-node-content">
              {(nodes || []).map((node) => {
                return (
                  <NodeComponent
                    node={node}
                    key={node.PATH}
                    onChoose={this.props.onChoose}
                    onToggle={this.props.onToggle}
                    open={this.props.open}
                  />
                );
              })}
            </div>
          ) : null}
        </div>
      </div>
    );
  }
}

export default NodeComponent;

export type { Node, Track };
