import React, { useState, useEffect } from "react";
import styles from "@/styles/Simple.module.scss";

export default function Simple({ onClick }) {
  // state  fopr the text
  // originally the text is an array of letters

  const innerText = "Start Simple";
  const [innerTextArray, setInnerTextArray] = useState(innerText.split(""));
  // a value of int arryy that counts up to the length of the text
  const [count, setCount] = useState(0);

  useEffect(() => {
    // count goes up every second until it reaches the length of the text
    // use CounterInterval to clear the interval on unmount and to not have multiple intervals
    const interval = setInterval(() => {
      let newCount = count + 1;
      setCount(newCount);
    }, 42);



    // when count reaches the length of the text, set the text to the innerText state
    if (count === innerTextArray.length) {
      clearInterval(interval)
    }

    // clear interval on unmount
    return () => clearInterval(interval);
  }, [count, innerTextArray]);

  // randomize the text every character, the count is the length of the text thats not randomized
  useEffect(() => {
    const interval = setInterval(() => {
      // for each character in the text, set a random letter
      // the count is the length of the text thats not randomized
      setInnerTextArray((innerTextArray) => {
        return innerTextArray.map((letter, index) => {
          if (index < count) {
            return innerText[index]
          } else {
            return String.fromCharCode(Math.floor(Math.random() * 26) + 97);
          }
        });
      });
    }, 42);

    // clear interval on unmount
    return () => clearInterval(interval);
  }, [innerTextArray, count]);

  useEffect(() => {
    setCount(0);
  }, []);

  return (
    <div className={styles.simple__container}>
      <div
        className={styles.simple}
        onClick={onClick}
      >
        {
          innerTextArray.map((letter, index) => {
            return (
              <span
                key={index}
                className={styles.letter}
              >
                {letter}
              </span>
            );
          }
          )
        }
      </div>
    </div>
  );
}

