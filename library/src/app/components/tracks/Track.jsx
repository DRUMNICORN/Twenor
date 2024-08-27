import React from "react";
// import api from "../../api";
import { render } from "react-dom";
import { emit, listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

import Nodes from "./Nodes";

/**
 * On init this component will fetch the track node from the backend
 * and display it in a list.
 *
 *
 *
 *
 *
 */
class RowElement extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      is_mounted: false,
      is_loaded: false,
      track: null,
    };

    if (!this.props.track) throw new Error("RowElement requires an track prop");

    listen("receive-track", (track) => {
      track = JSON.parse(track.payload);

      if (!this.state.is_mounted) {
        return;
      }
      if (track.path == this.props.track) {
        this.setState({
          track: track,
          is_loaded: true,
        });
      }
    });
  }

  componentDidMount() {
    this.setState({
      is_mounted: true,
    });

    emit("request-track", {
      TrackPath: {
        track_path: this.props.track,
      },
    });
  }

  // it should render the track element from the backend
  // while the track is loading it should display a loading animation
  // when the track is loaded it should display the track element

  // it should have a play button on hover
  // it should have a pause button on hover
  // it should have a playing animation on play while not hovered (like spotify)

  render() {
    let track = this.state.track || this.props.track; // is an object or a path string
    let is_selected = this.props.is_selected;

    let name = "";
    if (track.path) {
      name = track.path.split("\\").pop();
    }

    if (!this.state.is_loaded) return <td>Loading...</td>;
    else
      return (
        <>
          {/* //render play button  */}
          <div className="track-play-element dragout" key={`play`} />
          <div className="track-name-element dragout" key={`name`}>
            <div className="text">
              <div className="name">
                {name}
              </div>
              <div className="artist">
                Artist
              </div>
            </div>
            <div className="trackform">
              Placeolder trackform
            </div>
          </div>
          {/* <div className="track-node-element" key={`nodes`} hidden={this.props.isMoving}> */}
            <Nodes nodes={track.nodes} onClickNode={this.props.onClickNode} />
          {/* </div> */}
        </>
      );
  }
}

export default RowElement;
