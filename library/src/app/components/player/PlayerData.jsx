import React from "react";

class Player extends React.Component {
  render() {
    return (
      <div className="player-data">
        <img hidden={true} id="player-image" src={this.image_src} alt="image not found"></img>
        <div className="player-title">{this.title}</div>
      </div>
    );
  }
}
export default Player;
