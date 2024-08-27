import React from 'react';
import styles from '@/styles/GithubCard.module.scss';
import Image from 'next/image';

export default function GithubCard({ repo, username, subscribed, pending, onSubscribe }) {
  const { id, name, description, image } = repo;

  const handleSubscribe = () => {
    onSubscribe(id); // Subscribe
  };

  return (
    <div className={`${styles.card} ${subscribed ? styles.subscribed : ''} ${pending ? styles.pending : ''}`}>
      <div className={styles.imageContainer}>
        <Image className={styles.image} src={image} alt={`./${username}/${name}`} width={100} height={20} />
      </div>
      <div className={styles.content}>
        <h3>{name}</h3>
        <p>{description}</p>
        <button className={styles.waitlistButton} onClick={handleSubscribe}>
          {pending ? 'Subscribing...' : subscribed ? 'Unsubscribe' : 'Join Waitlist'}
        </button>
      </div>
    </div>
  );
}
