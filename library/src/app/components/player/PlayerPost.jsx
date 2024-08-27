import { emit } from "@tauri-apps/api/event";
import React from "react";

class PlayerPost extends React.Component {
  componentDidUpdate() {
    let volume = document.getElementById("volume-bar");
    volume.value = this.props.volume;
  }

  render() {
    return (
      <div className="player-post">
        <div className="player-visualizer">
          {/** visualizer_container = div.visualizer */}
          <canvas className="player-visualizer"></canvas>
        </div>
        <div className="player-volume">
          <input
            type="range"
            id="volume-bar"
            className="player-volume"
            min="0"
            max="2"
            step="0.01"
            defaultValue={this.props.volume || 0.42}
            onInput={(e) => {
              emit("update-config", {
                KeyAndValue: {
                  key: "volume",
                  value: e.target.value.toString(),
                },
              });
            }}
          ></input>
        </div>
      </div>
    );
  }
}
export default PlayerPost;
