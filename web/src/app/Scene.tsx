'use client'

import { Canvas } from '@react-three/fiber';
import { OrbitControls, Preload } from '@react-three/drei';
import { useState, useEffect, useRef, FC } from 'react';

import styles from '@/styles/Scene.module.scss';

{/* <Scene className='pointer-events-none' eventSource={ref} eventPrefix='client'>
{Component.canvas(pageProps)}
</Scene> */}

interface SceneProps {
  className?: string;
  eventSource?: React.RefObject<HTMLElement>;
  eventPrefix?: string;

  children?: React.ReactNode;
  // Add any other props you need for the Scene component
}

const Scene: FC<SceneProps> = ({ className, children, ...props }) => {
  const [visible, setVisible] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    // Wait 100 ms before rendering the canvas
    setVisible(false);
    const timeout = setTimeout(() => {
      setVisible(true);
    }, 2000);

    return () => clearTimeout(timeout);
  }, []);

  const handleScroll = (event: React.UIEvent<HTMLDivElement>) => {
    event.stopPropagation();
  };

  return (
    <div
      ref={containerRef}
      className={`${styles.canvas} ${visible ? styles.visible : ''} ${className}`}
      onScroll={handleScroll}
    >
      <Canvas className={styles.canvas}>
        <directionalLight intensity={0.75} />
        <ambientLight intensity={0.75} />
        {children}
        <Preload all />
        <OrbitControls enabled={false} />
      </Canvas>
    </div>
  );
};

export default Scene;
