import React from "react";

import { listen } from "@tauri-apps/api/event";
class PlayerControls extends React.Component {
  constructor(props) {
    super(props);

    listen("play_track", (payload) => {
      this.playTrack(payload.payload);
    });
  }

  playTrack(track) {
    let track_player = document.getElementById("track");
    track_player.src = track;
    track_player.play();
    console.log("playing track");
  }

  onTrackPause() {
    let track_player = document.getElementById("track");
    track_player.pause();
  }

  onTrackEnded() {
    let track_player = document.getElementById("track");
    track_player.pause();
    track_player.currentTime = 0;

    let next_track = document.getElementById("next-track");
    if (next_track) {
      next_track.click();
    }
  }

  onTrackLoadedMetadata() {
    let track_player = document.getElementById("track");
    let track_duration = document.getElementById("track-duration");
    track_duration.innerHTML = this.formatTime(track_player.duration);
  }

  onTrackTimeUpdate() {
    let track_player = document.getElementById("track");
    let track_current_time = document.getElementById("track-current-time");
    track_current_time.innerHTML = this.formatTime(track_player.currentTime);
  }

  onTrackPlay() {
    let track_player = document.getElementById("track");
    let track_duration = document.getElementById("track-duration");
    track_duration.innerHTML = this.formatTime(track_player.duration);
  }

  onPlayToggle() {
    let track_player = document.getElementById("track");
    if (track_player.paused) {
      track_player.play();
    } else {
      track_player.pause();
    }
  }

  onShuffleToggle() {
    let shuffle_button = document.getElementById("shuffle-button");
    if (shuffle_button.classList.contains("active")) {
      shuffle_button.classList.remove("active");
    } else {
      shuffle_button.classList.add("active");
    }
  }

  onRepeatToggle() {
    let repeat_button = document.getElementById("repeat-button");
    if (repeat_button.classList.contains("active")) {
      repeat_button.classList.remove("active");
    } else {
      repeat_button.classList.add("active");
    }
  }

  formatTime(time) {
    let minutes = Math.floor(time / 60);
    let seconds = Math.floor(time - minutes * 60);
    return `${minutes}:${seconds < 10 ? "0" + seconds : seconds}`;
  }

  render() {
    return (
      <div className="track-player-controls">
        <track
          id="track"
          src=""
          hidden={true}
          autoPlay
          onPause={() => this.onTrackPause()}
          onEnded={() => this.onTrackEnded()}
          onLoadedMetadata={() => this.onTrackLoadedMetadata()}
          onTimeUpdate={() => this.onTrackTimeUpdate()}
          onPlay={() => this.onTrackPlay()}
        ></track>
        <div className="player-controls">
          <img
            className="player-img"
            id="img-shuffle"
            src="https://api.iconify.design/bi/shuffle.svg"
            alt="repeat"
            onClick={() => this.onShuffleToggle()}
          ></img>
          <img
            className="player-img darken"
            id="img-skip-back"
            src="https://api.iconify.design/bi/skip-start-fill.svg"
            alt="skip-back"
          ></img>
          <img
            className="player-img"
            id="img-play"
            src="https://api.iconify.design/bi/play-fill.svg"
            alt="play-button"
            onClick={() => this.onPlayToggle()}
          />
          <img
            className="player-img darken"
            id="img-skip-forwards"
            src="https://api.iconify.design/bi/skip-end-fill.svg"
            alt="skip-forwards"
          ></img>
          <img
            className="player-img"
            id="img-repeat"
            src="https://api.iconify.design/bi/repeat.svg"
            alt="repeat"
            onClick={() => this.onRepeatToggle()}
          ></img>
        </div>

        <div className="player-indicator">
          <span className="player-time" id="track-current-time">
            0:00
          </span>
          <input
            type="range"
            max="100"
            step="0.01"
            defaultValue="0"
            className="player-progress"
            onInput={() => this.onProgressBarSeek()}
          ></input>
          <span className="player-duration" id="track-duration">
            0:00
          </span>
        </div>
      </div>
    );
  }
}

export default PlayerControls;
