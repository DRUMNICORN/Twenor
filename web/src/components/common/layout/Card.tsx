import React, { ReactNode } from 'react';
import styles from './Card.module.scss';

interface CardProps {
  title: string;
  children?: ReactNode;
}

const Card: React.FC<CardProps> = ({ title, children }) => {
  return (
    <div className={styles.card}>
      <h3 className={styles.title}>{title}</h3>
      {children}
      {/* Add more content to the card as needed */}
    </div>
  );
};

export default Card;
