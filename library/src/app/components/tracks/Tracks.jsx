import React from "react";
// import api from "../../api";
import { render } from "react-dom";

// import "../../../styles/Tracks.scss";

// import track element component
// import TrackElement from "./node/TrackElement";
import Details from "./tracks/Details";
import Header from "./tracks/Header";
import Track from "./tracks/Track";

import ReactDOM from "react-dom";
import { DragDropContext, Droppable, Draggable } from "react-beautiful-dnd";

import { emit, listen } from "@tauri-apps/api/event";

/**
 * On init this component will fetch the track node from the backend
 * and display it in a list.
 *
 *
 *
 *
 *
 */
class TrackList extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      tracks: [],
      selectedTracks: [],
    };
  }

  handleOnSelectTrack(e) {
    // check if shift is pressed
    if (e.shiftKey) {
      console.log("shift pressed");
      // get all tracks between last selected and current
      let tracks = this.props.node.tracks || {};
      let lastSelected = this.state.selectedTracks[this.state.selectedTracks.length - 1];
      if (lastSelected) {
        // get index of last selected
        tracks.map((track, index) => {
          if (track.path == lastSelected) {
            lastSelected = index;
          }
        });

        // get index of current
        let current = tracks.map((track) => track.path).indexOf(e.target.parentElement.id);

        let allIndexes = []; // all indexes between last selected and current

        console.log("last selected index", lastSelected);
        console.log("current index", current);
        if (lastSelected < current) {
          for (let i = lastSelected; i <= current; i++) {
            allIndexes.push(i);
          }
        } else {
          for (let i = lastSelected; i >= current; i--) {
            allIndexes.push(i);
          }
        }
        // get all tracks with indexes in allIndexes
        let tracksToSelect = [lastSelected, current];
        // add all numbers between last selected and current
        for (let i = 0; i < allIndexes.length; i++) {
          tracksToSelect.push(allIndexes[i]);
        }

        console.log("tracks to select", tracksToSelect);

        for (let trackIndex of tracksToSelect) {
          this.addSelectedTrack(tracks[trackIndex].path);
        }
      }
    } else if (e.ctrlKey) {
      console.log("ctrl pressed", e.target.parentElement.id);
      // check if track is already selected
      if (this.isSelected(e.target.parentElement.id)) {
        // remove from selected
        this.removeSelectedTrack(e.target.parentElement.id);
      } else {
        // add to selected
        this.addSelectedTrack(e.target.parentElement.id);
      }
    } else {
      console.log("no key pressed");
      const trackId = e.target.parentElement.id;
      if (this.isSelected(trackId)) {
        this.removeSelectedTrack(trackId);
      } else {
        this.setSelectedTracks(trackId);
      }
    }

    console.log("selected tracks === \n", this.state.selectedTracks);
  }

  setSelectedTracks(tracks) {
    // if track is single put in array
    if (typeof tracks === "string") {
      tracks = [tracks];
    }
    this.setState({ selectedTracks: tracks });
  }

  resetSelectedTracks() {
    this.setState({ selectedTracks: [] });
  }

  addSelectedTrack(trackId) {
    let selectedTracks = this.state.selectedTracks;
    if (selectedTracks == null) {
      selectedTracks = [];
    }
    selectedTracks.push(trackId);
    this.setState({ selectedTracks: selectedTracks });
  }

  removeSelectedTrack(trackId) {
    let selectedTracks = this.state.selectedTracks;
    if (selectedTracks == null) {
      selectedTracks = [];
    }
    selectedTracks = selectedTracks.filter((id) => id != trackId);
    this.setState({ selectedTracks: selectedTracks });
  }

  isSelected = (trackId) => (this.state.selectedTracks || []).includes(trackId);

  getStates(trackId, index) {
    // console.log("getStates", trackId, index);
    let is_selected = this.isSelected(trackId);

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

    let is_movable = !(is_dragging_node || (is_dragging_track && is_not_itself));

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
    ];
  }

  render() {
    let tracks = this.props.node ? this.props.node.tracks || [] : [];

    return (
      <div className="table">
        <Details node={this.props.node || {}}></Details>
        {/* <DropShadow active={this.props.dragging == "track"}> */}
        <div className="track-table">
          {/* <Header nodes={{}} /> */}
          <Droppable droppableId="tracks">
            {(provided, index) => (
              <ul {...provided.droppableProps} ref={provided.innerRef} id="track-list">
                {tracks.map((track, index) => {
                  let classNames = this.getStates(track.path, index);

                  let is_movable =
                    classNames.indexOf("dragging-node") != -1
                      ? // || (classNames.indexOf("dragging-node") != -1 && classNames.indexOf("not-itself") != -1)
                        false
                      : true;

                  return (
                    <Draggable className="track-drag" key={track.path} draggableId={track.path} index={index}>
                      {(provided, snapshot, rubric, state, _index) => {
                        /*
                        "dragstart",
                            function (evt) {
                              console.log("dragstart");
                              evt.dataTransfer.setData("DownloadURL", fileDetails);
                            },
                            false

                        */

                        return (
                          <li
                            onClick={(e) => {
                              // console.log("click");
                              this.handleOnSelectTrack(e);
                            }}
                            id={track.path}
                            key={track.path}
                            {...(classNames.indexOf("movable") != -1 ? provided.draggableProps : {})}
                            {...provided.dragHandleProps}
                            ref={provided.innerRef}
                            className={`track-row ${classNames.join(" ")}`}
                          >
                            <Track track={track.path} onClickNode={this.props.onClickNode} />
                          </li>
                        );
                      }}
                    </Draggable>
                  );
                })}

                {provided.placeholder}
              </ul>
            )}
          </Droppable>
        </div>
        {/* <Tracks node={this.props.node} dnd={this.props.dnd} /> */}
        {/* </DropShadow> */}
      </div>
    );
  }
}

export default TrackList;
