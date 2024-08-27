import React from "react";

import { render } from "react-dom";
import { DragDropContext, Droppable, Draggable } from "react-beautiful-dnd";

import Icon from "./Icon";
import NewElement from "./New";

import Api from "../../../Api";
// import Element from "./nodelist/NodeElement";

// import scss
// import "src\styles\Tracks\Tracks.scss";
// import DropShadow from "../utils/DropShadow";

class Nodes extends React.Component {
  // should be a list of nodes
  // get node from params

  constructor(props) {
    super(props);
    this.count = 0;

    this.state = {
      opened: [],
      nodes: null,
    };

    this.handleToggleNode = this.handleToggleNode.bind(this);
  }

  handleToggleNode(node_path) {
    let opened = this.state.opened;
    // check if node is already opened
    if (this.isOpened(node_path)) {
      // remove node from opened
      opened = opened.filter((path) => path != node_path);
    } else {
      // add node to opened
      opened.push(node_path);
    }

    this.setState({ opened: opened });
  }

  isOpened = (node_path) => this.state.opened.indexOf(node_path) != -1;

  getStates(node_path, index) {
    this.last_states = this.props.dnd;

    let is_selected = this.props.selected_node && this.props.selected_node.path == node_path;

    let is_dragging_node = false;
    let nodes = document.getElementsByClassName("node-row");
    for (let i = 0; i < nodes.length; i++) {
      if (parseInt((nodes[i].style || {}).zIndex || "0") == 5000) is_dragging_node = true;
    }

    let is_dragging_track = false;
    let tracks = document.getElementsByClassName("track-row");
    for (let i = 0; i < tracks.length; i++) {
      // parseInt(((document.getElementsByClassName("track-row")[index] || {}).style || {}).zIndex || "0") == 5000;
      if (parseInt((tracks[i].style || {}).zIndex || "0") == 5000) is_dragging_track = true;
    }

    let is_dragging_smt = is_dragging_node || is_dragging_track;

    let is_dragging_over = this.props.dnd && this.props.dnd.destination && this.props.dnd.destination.index == index;

    let is_not_itself = this.props.dnd && this.props.dnd.source && this.props.dnd.source.index != index;

    let is_itself = this.props.dnd && this.props.dnd.source && this.props.dnd.source.index == index;

    let is_destination_index = this.props.dnd && this.props.dnd.destination && this.props.dnd.destination.index == index;

    let is_destination_nodes =
      this.props.dnd && this.props.dnd.destination && this.props.dnd.destination.droppableId == "nodes";

    let is_destination_tracks =
      this.props.dnd && this.props.dnd.destination && this.props.dnd.destination.droppableId == "tracks";

    let is_destination_dropin =
      this.props.dnd && this.props.dnd.destination && this.props.dnd.destination.droppableId == "nodes-dropin";

    let is_source_index = this.props.dnd && this.props.dnd.source && this.props.dnd.source.index == index;

    let is_source_nodes = this.props.dnd && this.props.dnd.source && this.props.dnd.source.droppableId == "nodes";

    let is_source_tracks = this.props.dnd && this.props.dnd.source && this.props.dnd.source.droppableId == "tracks";

    let is_source_dropin = this.props.dnd && this.props.dnd.source && this.props.dnd.source.droppableId == "nodes-dropin";

    let is_opened = this.isOpened(node_path);

    let is_movable = !(is_dragging_track || (is_dragging_node && is_not_itself));

    return [
      is_movable ? "movable" : "",
      is_selected ? "selected" : "",
      is_dragging_over ? "dragging-over" : "",
      is_not_itself ? "not-itself" : "",
      is_itself ? "itself" : "",
      is_dragging_node ? "dragging-node" : "",
      is_dragging_track ? "dragging-track" : "",
      is_dragging_smt ? "dragging-smt" : "",
      is_destination_index ? "destination-index" : "",
      is_destination_nodes ? "destination-nodes" : "",
      is_destination_tracks ? "destination-tracks" : "",
      is_destination_dropin ? "destination-dropin" : "",
      is_source_index ? "source-index" : "",
      is_source_nodes ? "source-nodes" : "",
      is_source_tracks ? "source-tracks" : "",
      is_source_dropin ? "source-dropin" : "",
      is_opened ? "opened" : "",
    ];
  }

  getNodesFlattened = (nodes) => {
    let nodes_flattened = [];
    for (let i = 0; i < nodes.length; i++) {
      let node = nodes[i];
      nodes_flattened.push(node);
      if (node.NODE) {
        nodes_flattened = nodes_flattened.concat(this.getNodesFlattened(node.NODE));
      }
    }
    return nodes_flattened;
  };

  getNodesOrdered = () => {
    let nodes = this.getNodesFlattened(this.state.nodes || this.props.nodes || []);
    nodes.sort((a, b) => {
      return a.PATH > b.PATH ? 1 : -1;
    });
    return nodes;
  };

  updateState() { }

  render() {
    return (
      <div className="explorer">
        asd
        {/* //   <Icon />
      //    <DropShadow active={this.props.dragging == "node"}>
      //   <div className="node-list">
      //     <div className="container">
      //       <div className="left">
      //         <Droppable droppableId="nodes">
      //           {(provided) => {
      //             return (
      //               <div {...provided.droppableProps} ref={provided.innerRef}>
      //                 <ul>
      //                   {this.getNodesOrdered().map((node, index) => {
      //                     let classNames = this.getStates(node.PATH, index);

      //                     console.log("node.path", node);

      //                     let node_path_depth = (node.PATH.split("/").length - 1) | 0;

      //                     return (
      //                       <Draggable key={node.Name} draggableId={node.Name} index={index}>
      //                         {(provided) => (
      //                           console.log("node.path", node),
      //                           (
      //                             <li
      //                               className="node-row"
      //                               key={node.Name}
      //                               onClick={(event) => this.props.onClickNode(event.target.id)}
      //                               {...(classNames.indexOf("movable") != -1 ? provided.draggableProps : {})}
      //                               {...provided.dragHandleProps}
      //                               ref={provided.innerRef}
      //                             >
      //                               <div
      //                                 className={`node-element ${classNames.join(" ")}`}
      //                                 style={{
      //                                   paddingLeft: `${node_path_depth * 20}px`,
      //                                 }}
      //                               >
      //                                 <div className="node-element">
      //                                   <a id={node.PATH}>#{node.Name}</a>
      //                                 </div>
      //                               </div>
      //                             </li>
      //                           )
      //                         )}
      //                       </Draggable>
      //                     );
      //                   })}
      //                   {provided.placeholder}
      //                 </ul>
      //               </div>
      //             );
      //             // <div {...provided.droppableProps} ref={provided.innerRef}></div>
      //           }}
      //         </Droppable>
      //       </div>
      //       <div className="right">
      //         <Droppable droppableId="nodes-dropin">
      //           {(provided) => (
      //             <div {...provided.droppableProps} ref={provided.innerRef} className="node-open-element-table">
      //               <ul>
      //                 {(this.state.nodes || this.props.nodes)
      //                   .sort((a, b) => {
      //                     return a.PATH > b.PATH ? 1 : -1;
      //                   })
      //                   .map((node, index) => (
      //                     // </tr>
      //                     <Draggable key={`${node.name}-dropin`} draggableId={`${node.name}-dropin`} index={index}>
      //                       {(provided) => {
      //                         let classNames = this.getStates(node.PATH, index);
      //                         return (
      //                           <li
      //                             className="node-open-element-row"
      //                             key={`${node.NAME}-dropin`}
      //                             id={`${node.PATH}-dropin`}
      //                             onClick={(e) => {
      //                               this.handleToggleNode(e.target.parentElement.id);
      //                             }}
      //                             {...provided.dragHandleProps}
      //                             ref={provided.innerRef}
      //                           >
      //                             <div className={`node-open-element ${classNames.join(" ")}}`}></div>
      //                           </li>
      //                         );
      //                       }}
      //                     </Draggable>
      //                   ))}
      //                 {provided.placeholder}
      //               </ul>
      //             </div>
      //           )}
      //         </Droppable>
      //       </div>
      //     </div>
      //     <NewElement />
      //   </div>
      //   {/* </DropShadow> */}
      </div>
    );
  }
}

export default Nodes;
