/**
 * @file TrackOverview.tsx
 * @description TrackOverview component for the Tauri app. Displays the track library.
 * @author
 * @version 0.0.1
 * @date 2021-06-01
 */

import React from "react";

import "../../styles/TrackOverview.scss";

import { Node } from "../util/Datatypes";

import { Canvas } from "@react-three/fiber";
import Box from "../geometry/Box";
import { OrbitControls } from "@react-three/drei";

import Explorer from "../util/Explorer";

// create a new track overview component

type TrackOverviewProps = {
  nodes: Node[];
};

type TrackOverviewState = {};

class TrackOverview extends React.Component<TrackOverviewProps, TrackOverviewState> {
  constructor(props: TrackOverviewProps) {
    super(props);
    this.state = {};
  }

  componentDidUpdate(): void {}

  render() {
    return (
      <div className="track-overview">
        <h1>Three JS</h1>
        <p>
          This app is using <a href="https://threejs.org/">Three JS</a> to render 3D scenes.
        </p>
        <Canvas className="canvas">
          <OrbitControls />
          <ambientLight />
          <directionalLight position={[10, 10, 5]} />
          <Box />
        </Canvas>
        <p>
          In following updates, this will be used to display the track library.
        </p>
      </div>
    );
  }
}

export default TrackOverview;
