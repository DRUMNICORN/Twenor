import { Canvas } from '@react-three/fiber'
import { OrbitControls, Preload } from '@react-three/drei'
import { useState, useEffect } from 'react'

import styles from '@/styles/Scene.module.scss'

export default function Scene({ children, ...props }) {
  const [visible, setVisible] = useState(false);

  useEffect(() => {
    // wait 100 ms before rendering the canvas
    setVisible(false);
    const timeout = setTimeout(() => {
      setVisible(true);
    }
      , 2000);

    return () => clearTimeout(timeout);

  }, []);

  // Everything defined in here will persist between route changes, only children are swapped
  return (
    <Canvas  {...props} id="canvas" className={`${styles.canvas} ${visible ? styles.visible : ''}`}>
      <directionalLight intensity={0.75} />
      <ambientLight intensity={0.75} />
      {children}
      <Preload all />
      <OrbitControls />
    </Canvas>
  )
}
