// Error.jsx

// Path: src\components\util\Error.jsx

// dispalys a message if an error occurs 
// in the useGithubProjects hook

import Image from 'next/image';
import React from 'react';
import styles from '@/styles/ErrorCat.module.scss';

export default function ErrorCat({ message, code }) {
  return (
    <div className={styles.error}>
      <h1 className={styles.title}>Error</h1>
      <div className={styles.image}>
        <Image src={`https://http.cat/${code}`} alt={message} width={500} height={500} />
      </div>
      <p className={styles.message}>{message}</p>
    </div>
  );
}

