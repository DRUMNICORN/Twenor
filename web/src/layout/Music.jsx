import React, { useState } from 'react'
import YouTube from 'react-youtube';


// imports: react, next, and components
import Banner from '@/components/Banner'
import Bio from '@/components/Bio'
import Tracks from '@/components/Tracks'

// imports: styles
import styles from '@/styles/Music.module.scss'

export default function Music({ theme }) {
  const [playing, setPlaying] = useState(false);

  return (
    <div className={styles.container}>
      {/* <Banner /> */}
      <div className={styles.content}>

        <Tracks />
        <Bio />
        <div className={styles.video + ' ' + (playing ? styles.playing : '')}>
          <YouTube videoId='DPHbu6VzbO0' onPlay={() => setPlaying(true)} onPause={() => setPlaying(false)}
            opts={{ playerVars: { controls: 0, disablekb: 1, modestbranding: 1, rel: 0, showinfo: 0, start: 0 } }} />
        </div>
      </div>
    </div>
  )
}