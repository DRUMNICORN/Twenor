import React, { useState, useEffect } from 'react';
import { Line } from '@react-three/drei';
import * as THREE from 'three';

export default function Connection({ from = { x: 0, y: 0, z: 0 }, to = { x: 0, y: 0, z: 0 }, weight = 1, ...props }) {
  const [hovered, setHover] = useState(false);
  const [active, setActive] = useState(false);

  const darkColor = getComputedStyle(document.documentElement).getPropertyValue('--dark-color');
  const primaryColor = getComputedStyle(document.documentElement).getPropertyValue('--primary-color');

  return (
    <>
      <Line
        key={`connection-${from.x}-${from.y}-${from.z}-${to.x}-${to.y}-${to.z}`}
        points={[
          new THREE.Vector3(from.x, from.y, from.z),
          new THREE.Vector3(to.x, to.y, to.z),
        ]}
        color={hovered ? primaryColor : darkColor}
        lineWidth={weight}
        {...props}
        position={[0, 0, 0]}
        scale={[1, 1, 1]}
        onClick={(e) => setActive(!active)}
        onPointerOver={(e) => setHover(true)}
        onPointerOut={(e) => setHover(false)}
      />
    </>
  );
}
