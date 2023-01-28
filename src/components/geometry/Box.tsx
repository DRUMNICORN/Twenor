/**
 * @file Box.tsx
 * @description Box component for the Tauri app. Displays the track library.
 * @author
 * @version 0.0.1
 */

import React from "react";

// import "../../styles/Box.scss";

// create a new box component

type BoxProps = {
  title?: string;
  children?: React.ReactNode;
};

type BoxState = {};

class Box extends React.Component<BoxProps, BoxState> {
  constructor(props: BoxProps) {
    super(props);
    this.state = {};
  }

  componentDidUpdate(): void {}

  render() {
    return (
      <mesh>
        <boxGeometry attach="geometry" />
        <meshStandardMaterial attach="material" color="hotpink" />
      </mesh>
    );
  }
}

export default Box;
