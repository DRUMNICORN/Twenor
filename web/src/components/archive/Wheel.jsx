// Color Wheel Component


// there are 2 seperate things, 1st the circle with the colors and 2nd the colors inside the circle.
// from default the ring colors are red, green, blue and the center is white

// there is an event that will add 3 colors to the ring (magenta, cyan, yellow)
// and the center will be white
// the event is triggered by clicking on the center

// the colors inside the circle are 12 colors (
//   "red",
// "orange",
//   "yellow",
//   "chartreuse",
//   "green",
//   "springgreen",
//   "cyan",
//   "turquoise",
//   "blue",
//   "violet",
//   "magenta",
//   "pink",
// )
// the color fields are divided into 2 parts, inner and outer
// the inner text are 12A, 1A 2A 3A 4A 5A 6A 7A 8A 9A 10A 11A
// the outer text are 12B, 1B 2B 3B 4B 5B 6B 7B 8B 9B 10B 11B

import styles from "./Wheel.module.scss";
import { useRouter } from 'next/router'
import React, { Component, useEffect } from 'react';
// router


const CONF = {
  className: "wheel__slice--12",
  centerText: "Scales",
  colors: [
    "turquoise",
    "blue",
    "violet",
    "magenta",
    "pink",
    "red",
    "orange",
    "yellow",
    "chartreuse",
    "green",
    "springgreen",
    "cyan",
  ],
  // text keys
  texts: [
    "B-Flat Major",
    "E-Flat Major",
    "A-Flat Major",
    "D-Flat Major",
    "F-Sharp Major",
    "B Major",
    "E Major",
    "A Major",
    "D Major",
    "G Major",
    "C Major",
    "F Major",
  ],
  audios: [
    "audio/Scale B-Flat Major.mp3",
    "audio/Scale E-Flat Major.mp3",
    "audio/Scale A-Flat Major.mp3",
    "audio/Scale D-Flat Major.mp3",
    "audio/Scale F-Sharp Major.mp3",
    "audio/Scale B Major.mp3",
    "audio/Scale E Major.mp3",
    "audio/Scale A Major.mp3",
    "audio/Scale D Major.mp3",
    "audio/Scale G Major.mp3",
    "audio/Scale C Major.mp3",
    "audio/Scale F Major.mp3",
  ],
  innnerTexts: [
    "G Minor",
    "C Minor",
    "F Minor",
    "B-Flat Minor",
    "E-Flat Minor",
    "A-Flat Minor",
    "D-Flat Minor",
    "F-Sharp Minor",
    "B Minor",
    "E Minor",
    "A Minor",
    "D Minor"
  ],
};

// a function to rotate arrays


export default function Wheel({ onClick }) {
  const colors = CONF.colors;
  const texts = CONF.texts;

  return (
    <div className={styles.container}>
      <WheelContent colors={colors} />
    </div>
  );

  function WheelSlices({ colors, setHoverText, isSimonAwake, setIsSimonAwake }) {
    const [activeStates, setActiveStates] = React.useState([false, false, false, false, false, false, false, false, false, false, false, false]);
    const [simonsHistory, setSimonsHistory] = React.useState([]);
    const [userHistory, setUserHistory] = React.useState([]);
    const [locked, setLocked] = React.useState(false);

    // is simon is not awake reset the game
    useEffect(() => {
      if (!isSimonAwake) {
        resetGame();
      }
    }, [isSimonAwake]);

    // reset game on page load
    useEffect(() => {
      resetGame();
    }, []);

    const resetGame = () => {
      setActiveStates([false, false, false, false, false, false, false, false, false, false, false, false]);
      setSimonsHistory([]);
      setUserHistory([]);
      setLocked(false);
    }

    const duration_in_seconds = 3;

    const handleMouseEnter = (event) => {
      // play audio on click
      setHoverText(event.target.id);
    };

    const handleSliceClick = (event) => {
      if (locked) return;
      let id = event.target.id;
      let index = parseInt(CONF.texts.indexOf(id));

      let newHistory = [...userHistory];
      newHistory.push(index);
      setUserHistory(newHistory);
      setActiveNote(index, setActiveStates);

      userPlay(index);
    };

    const setActiveNote = async (index, setState) => {
      console.log(`[Simon] Playing ${CONF.texts[index]}`); // [Simon] Playing [color]
      const newActiveStates = [...activeStates];
      newActiveStates[index] = true;
      setState(newActiveStates);
    }

    const setInactiveNote = (index) => {
      console.log(`[Simon] Stopping ${CONF.texts[index]}`); // [Simon] Stopping [color]
      const newActiveStates = [...activeStates];
      newActiveStates[index] = false;
      setActiveStates(newActiveStates);
    }

    const userPlay = (index) => {
      let audio = new Audio(CONF.audios[index]);
      console.log(`[User] Playing ${CONF.texts[index]}`); // [User] Playing [color]
      audio.play();
    }

    const simonPlay = (index) => {
      let audio = new Audio(CONF.audios[index]);
      audio.play();
    }

    const simonPlayHistory = (setState) => {
      let history = simonsHistory;
      let i = 0;
      setLocked(true);
      console.log(`[SIMON] play history: ${simonsHistory}`);
      let interval = setInterval(() => {
        // disable previus note
        if (i > 0)
          setInactiveNote(history[i - 1]);

        if (i >= history.length) {
          setLocked(false);
          clearInterval(interval);
          return;
        }
        setActiveNote(history[i], setState);
        simonPlay(history[i]);
        i++;
      }, duration_in_seconds * 1000);

    }

    // const simonPlayRandomSequence = async (c) => {
    //   let i = 0;

    //   setLocked(true);
    //   console.log(`[SIMON] play random sequence: ${c}`);
    //   let interval = setInterval(() => {
    //     if (i >= c) {
    //       setLocked(false);
    //       clearInterval(interval);
    //       return;
    //     }

    //     let randomIndex = Math.floor(Math.random() * 12);

    //     let newActiveStates = activeStates;
    //     newActiveStates[randomIndex] = true;
    //     console.log(`[Simon] activating state ${randomIndex}, seq`);

    //     setActiveStates(newActiveStates);
    //     simonPlay(randomIndex);
    //     i++;

    //     let newHistory = simonsHistory;
    //     newHistory.push(randomIndex);
    //     setSimonsHistory(newHistory);
    //   }, duration_in_seconds * 1000);
    // }

    // // if a state is active deactivate it after 10 seconds
    useEffect(() => {
      const interval = setInterval(() => {
        let newActiveStates = [...activeStates];
        for (let i = 0; i < newActiveStates.length; i++) {
          if (newActiveStates[i]) {
            newActiveStates[i] = false;
          }
        }
        setActiveStates(newActiveStates);
      }, duration_in_seconds * 1000);

      return () => clearInterval(interval);
    }, [activeStates]);

    useEffect(() => {
      // is simon is awake and simon history is empty, lock the wheel and play random sequence
      if (isSimonAwake && simonsHistory.length === 0 && !locked) {
        simonPlayRandomSequence(3);
      }
    }, [isSimonAwake, simonsHistory, locked]);

    // useEffect(() => {
    //   // if simon is awake and simon history is not empty
    //   // wait for the user to play, if the user has the same history as simon
    //   // clear the user history, play the simon history and after that a new random note
    //   if (isSimonAwake && simonsHistory.length > 0) {
    //     if (userHistory.length == simonsHistory.length) {
    //       console.log(`[SIMON] user history: ${userHistory}`);
    //       setLocked(true);
    //       setUserHistory([]);
    //       simonPlayHistory(setActiveStates);
    //       simonPlayRandomSequence(1);
    //     } else {
    //       console.log(`[SIMON] user count: ${userHistory.length}`, userHistory);
    //       console.log(`[SIMON] simon count: ${simonsHistory.length}`, simonsHistory);
    //     }
    //   }
    // }, [userHistory, simonsHistory]);

    return (<div className={styles.wheel__slices}>
      {colors.map((color, i) => {
        // if active console log
        if (activeStates[i]) {
          console.log(`[SIMON] active state ${i}`);
        }
        return (
          <div
            name={i}
            key={i} className={`${styles.wheel__slice} ${styles[color]} ${activeStates[i] ? styles.active : ""}`} onMouseEnter={handleMouseEnter} onMouseLeave={() => setHoverText("")} onClick={handleSliceClick} style={{ "--i": i }} // active state
            // active state
            id={texts[i]} />
        );
      })}
    </div>);
  }
  function WheelCenter({ handleCenterContexClick, handleCenterClick, hoverText, isSimonAwake }) {
    return (<div className={
      `${styles.wheel__center} ${(isSimonAwake ? styles.active : "")}` // active state
    } onContextMenu={handleCenterContexClick} onClick={handleCenterClick}>
      <div className={styles.title}>{hoverText || CONF.centerText}</div>
    </div>);
  }

  function WheelContent({ colors }) {
    const [isSimonAwake, setIsSimonAwake] = React.useState(false);
    const [hoverText, setHoverText] = React.useState("");
    const [wheelRotation, setWheelRotation] = React.useState(0);
    const [accelerationInterval, setAccelerationInterval] = React.useState(0);

    const handleCenterClick = (event) => {
      event.preventDefault();
      onClick();
    };

    const handleCenterContexClick = (event) => {
      event.preventDefault();
      // setIsSimonAwake(!isSimonAwake);
    };

    // when scrooling up or down the wheel rotation velecity will change by 1
    useEffect(() => {
      const handleWheel = (event) => {
        // for 100 ms the wheel will rotate by 1 degree
        setAccelerationInterval(100);
        // if the wheel is rotated up the wheel rotation will increase by 1
        if (event.deltaY < 0) {
          setWheelRotation((prev) => prev + 10);
        } else {
          // if the wheel is rotated down the wheel rotation will decrease by 1
          setWheelRotation((prev) => prev - 10);
        }
      };


      window.addEventListener("wheel", handleWheel);

      return () => {
        window.removeEventListener("wheel", handleWheel);
      };
    }, [wheelRotation]);

    return (<div className={`${styles.wheel} ${styles[CONF.className]}`} style={{
      transform: `rotate(${wheelRotation}deg)`,
      transition: `transform ${accelerationInterval}ms linear`,
      transitionTimingFunction: `cubic - bezier(0.1, 0.7, 1.0, 0.1)`
    }}>
      <WheelSlices colors={colors} setHoverText={setHoverText} isSimonAwake={isSimonAwake} setIsSimonAwake={setIsSimonAwake} />
      <WheelCenter handleCenterContexClick={handleCenterContexClick} handleCenterClick={handleCenterClick} hoverText={hoverText} isSimonAwake={isSimonAwake} />
    </div>);
  }
}