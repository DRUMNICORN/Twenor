// a text area for the bio.

import React from 'react'
import styles from '@/styles/Bio.module.scss'

export default function Bio() {
  return (
    <div className={styles.bio}>
      <h1>Biography</h1>
      <p>
        Welcome to my world of creativity and innovation! I'm Drumnicorn, a multi-talented artist, music producer, and programming enthusiast. With a deep passion for blending art and technology, I strive to create captivating experiences that push the boundaries of what's possible.
      </p>
      <p>
        Through my electrifying beats and mesmerizing melodies, I aim to transport listeners into a realm where rhythm and imagination intertwine. As a relentless explorer of AI and programming, I'm constantly seeking new ways to enhance the artistic process and deliver unforgettable sonic journeys.
      </p>
      <p>
        By becoming a sponsor, you'll play a vital role in supporting my artistic endeavors and fueling my ongoing projects. In return, you'll gain exclusive access to behind-the-scenes content, early releases, interactive tutorials, and the opportunity to collaborate on innovative ventures.
      </p>
      <p>
        Join me on this incredible journey where music, technology, and creativity converge. Together, we'll create a symphony of innovation that resonates with the world.
      </p>
    </div>
  );
}
