import React from "react";

import { render } from "react-dom";
import { listen } from "@tauri-apps/api/event";

import { convertFileSrc } from "@tauri-apps/api/tauri";

import PlayerData from "./player/PlayerData";
import PlayerControls from "./player/PlayerControls";
import PlayerPost from "./player/PlayerPost";
// import scss
// import "../../../styles/Player.scss";

class Player extends React.Component {
  render() {
    return (
      <div id="player" bar-width="5" bar-gap="2" preload="true">
        <Data />
        <Controls />
        <PlayerPost volume={parseFloat(this.props.config.volume || 0)} />
      </div>
    );
  }
}

export default Player;
