import React, { useState, useEffect } from 'react';
import styles from '@/styles/ChatPrompt.module.scss';
// import useChat from './useChat';
// import useIdentity from './useIdentity';
// get from @ over hooks 

import { useChat } from '@/hooks/useChat';
import { useIdentity } from '@/hooks/useIdentity';
import Prompt from '@/components/Prompt';


export default function ChatPrompt() {
  const [input, setInput] = useState('');

  const { identity } = useIdentity();
  const [messages, setMessages] = useChat(identity);

  const handleInput = (e) => {
    setInput(e.target.value);
  };

  const handleSubmit = (e) => {
    e.preventDefault();
    if (input === '') return;
    setInput('');

    let message = {
      timestamp: Date.now(),
      text: input,
      type: 'user',
      identity: identity
    };
    setMessages([...messages, message]);
  };

  return (
    <div className={styles.container}>
      <div className={styles.prompt}>
          <div className={styles.promptMessages}>
            {messages.map((message, index) => {
              return (
                <div
                  key={index}
                  className={
                    message.type === 'user'
                      ? styles.promptMessageUser
                      : styles.promptMessageRoggen
                  }
                >
                  {/* display timestamp below left or right */}
                  <p className={styles.promptMessageText}>{message.text}</p>
                  {
                    message.timestamp ? <p className={styles.promptMessageTimestamp}>
                      {new Date(message.timestamp).toLocaleDateString('en-US', {
                        // show date and time
                        year: 'numeric',
                        month: 'short',
                        day: 'numeric',
                        hour: 'numeric',
                        minute: 'numeric'

                      })}
                    </p> : null
                  }
                </div>
              );
            })}
          </div>
        <Prompt input={input} handleInput={handleInput} handleSubmit={handleSubmit} placeholder="Type a message..." />
      </div>
    </div >
  );
}
