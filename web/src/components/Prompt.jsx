import React from 'react';
import styles from '@/styles/ChatPrompt.module.scss';

export default function Prompt({ handleSubmit, input, handleInput, placeholder = 'Type your message here...', enableEmailValidation = false }) {
  const isValidEmail = (email) => {
    // Simple email validation using regular expression
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return emailRegex.test(email);
  };

  const handleChange = (e) => {
    const inputValue = e.target.value;
    handleInput(inputValue);

    if (enableEmailValidation) {
      // Check if the input is a valid email address
      if (isValidEmail(inputValue)) {
        // Handle valid email input
      } else {
        // Handle invalid email input
      }
    }
  };

  return (
    <form className={styles.promptForm} onSubmit={handleSubmit}>
      <input
        className={styles.promptInput}
        type="text"
        value={input}
        onChange={handleChange}
        placeholder={JSON.stringify(placeholder)}
        autoFocus
      />
      <button className={styles.promptButton} style={{ display: 'none' }} type="submit" name="submit" value="submit" id="submit">
        Send
      </button>
    </form>
  );
}
