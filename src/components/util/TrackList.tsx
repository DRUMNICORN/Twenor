/**
 * @file TrackList.tsx
 * @description A component that renders a list of tracks.
 */

import React from "react";
import { Track } from "./Datatypes";
import TrackComponent from "./TrackComponent";

type TrackListProps = {
  tracks: Track[];
};

type TrackListState = {
  selected_tracks: Track[];
};

class TrackList extends React.Component<TrackListProps, TrackListState> {
  constructor(props: TrackListProps) {
    super(props);
    this.state = {
      selected_tracks: [],
    };
  }

  componentDidMount(): void {}

  render() {
    return (
      <div className="track-list">
        <h1>Tracks</h1>
        {this.props.tracks.map((track) => {
          return (
            <div key={track.TrackID}>
              <TrackComponent track={track} isSelected={this.state.selected_tracks.includes(track)} />
            </div>
          );
        })}
      </div>
    );
  }
}

export default TrackList;
