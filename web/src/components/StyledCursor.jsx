// this will get the position of the mouse and set it to the state
// the renderer will draw a thin circle with radius 10px at the mouse position and a filled circle with radius 5px at the mouse position

import React, { useState, useEffect } from 'react';

import styles from '@/styles/StyledCursor.module.scss';

export default function StyledCursor({ ...props }) {
  const [mousePos, setMousePos] = useState({ x: 0, y: 0 });

  const handleMouseMove = (e) => {
    setMousePos({ x: e.clientX - 10, y: e.clientY - 10 });
  };

  useEffect(() => {
    window.addEventListener('mousemove', handleMouseMove);

    return () => {
      window.removeEventListener('mousemove', handleMouseMove);
    };
  }, []);
  return (
    <>
      <div className={styles.cursor} style={{
        left: mousePos.x, top: mousePos.y
      }} />
      <div className={styles.cursorFill} style={{ left: mousePos.x, top: mousePos.y }} />
    </>
  );
}


// Path: src\components\canvas\intelligence\Connection.jsx
