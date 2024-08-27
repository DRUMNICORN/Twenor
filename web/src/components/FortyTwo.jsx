
import { useState, useEffect, useMemo } from 'react';
import useMqtt from '@/hooks/useMqtt';

import styles from '@/styles/FortyTwo.module.scss';
import ChatPrompt from '@/layout/ChatPrompt';

export default function FortyTwo() {
  const [count, setCount] = useState(0);
  const memIs42 = useMemo(() => count > 42, [count]);
  const { message, publish, connected, offline, closed, error } = useMqtt('count');

  const handleClick = () => {
    if (memIs42) {
      return;
    }
    if (!connected) {
      let newCount = count + 1;
      setCount(newCount);
      return;
    }

    publish('increment', (count + 1).toString());
  };

  return (
    <div className={styles.container}>
      <h1 className={styles.title}>{memIs42 ? '42' : (count == 0 ? ("Whats that all about?") : (count))}</h1>
      {memIs42 ? (
        <ChatPrompt />
      ) : (
        <button className={styles.button} onClick={handleClick}> + </button>
      )}
    </div>
  );
}
