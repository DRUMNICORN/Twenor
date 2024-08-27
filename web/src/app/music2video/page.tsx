"use client";
import React, { useState, useEffect } from 'react';
import styles from '@/styles/Music2Video.module.scss';
import FileUpload from './FileUpload';

const Music2Video: React.FC = () => {
  const [waveformLoaded, setWaveformLoaded] = useState(false);
  const [file, setFile] = useState<File | null>(null);

  // Simulating waveform loading
  useEffect(() => {
    const timer = setTimeout(() => {
      setWaveformLoaded(true);
    }, 2000);

    return () => {
      clearTimeout(timer);
    };
  }, []);

  const handleFileUpload = (file: File) => {
    // Handle the uploaded file here
    console.log(file);
    setFile(file);
  };

  return (
    <div className={styles.container}>
      <FileUpload onFileUpload={handleFileUpload} />
      {waveformLoaded && (
        <div className={styles.waveform}>
          <img src="/images/waveform.png" alt="Waveform" />
        </div>
      )}
    </div>
  );
};

export default Music2Video;
