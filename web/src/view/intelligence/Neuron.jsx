// Neuron.jsx

import React, { useRef, useState } from 'react';
import { Sphere } from '@react-three/drei';

export default function Neuron({ position }) {
  // create a neuron at position with radius, color, opacity
  const mesh = useRef();
  const [hovered, setHover] = useState(false);
  const [active, setActive] = useState(false);
  // load theme colors

  // get $dark-color from theme
  const darkColor = getComputedStyle(document.documentElement).getPropertyValue('--dark-color');
  const primaryColor = getComputedStyle(document.documentElement).getPropertyValue('--primary-color');
  return (
    <Sphere
      ref={mesh}
      args={[0.1, 32, 32]}
      position={position
        ? [position.x, position.y, position.z]
        : [0, 0, 0]
      }
      scale={active ? [1.5, 1.5, 1.5] : [1, 1, 1]}
      onClick={(e) => setActive(!active)}
      onPointerOver={(e) => setHover(true)}
      onPointerOut={(e) => setHover(false)}
    >
      <meshStandardMaterial
        attach="material"
        color={hovered ? primaryColor : darkColor}
        opacity={1}
        transparent={true}
      />
    </Sphere>
  );

}