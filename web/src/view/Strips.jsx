// its a 3d animation of cruvy strips line moving in 3d space
// it is a react component

import React, { useState, useEffect } from 'react';

// its inside a <Scene /> component
// <Scene /> is a react-three-fiber component
// <Scene /> is a react component
// <Scene /> is a 3d scene
// <Scene /> is a 3d canvas
// <Scene /> is a 3d animation

// its inside a <Canvas /> component

// its inside a <group /> component


export default function Strips({ route, ...props }) {
  // there should be 10 strips in the animation
  // from bottom right to top left
  // each strip should be 10px wide
  // each strip should be 10px tall
  // each strip should be 10px deep

  const [strips, setStrips] = useState([]);

  useEffect(() => {
    let strips = [];
    for (let i = 0; i < 10; i++) {
      strips.push({
        x: 0,
        y: 0,
        z: 0,
      });
    }
    setStrips(strips);
  }, []);

  return (
    <group {...props}>
      {strips.map((strip, i) => (
        <mesh
          key={`strip-${i}`}
          position={[strip.x, strip.y, strip.z]}
          scale={[1, 1, 1]}
        >
          <boxBufferGeometry attach="geometry" args={[10, 10, 10]} />
          <meshStandardMaterial attach="material" color="red" />
        </mesh>
      ))}
    </group>
  );
}



