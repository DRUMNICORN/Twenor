// Footer.jsx

import Image from 'next/image';
import React from 'react';
import styles from '@/styles/Footer.module.scss';

export default function Footer({ theme }) {


  // have all my social media links here on the right and a link to my resume on the left
  return (
    <footer className={styles.footer}>
      <div className={styles.footer__left}>
        {/* <a href='/resume.pdf'
          className={styles.footer__left__resume}
          target='_blank'
          rel='noopener noreferrer'
        >

        </a> */}
      </div>
      <div className={styles.footer__right + ' ' + styles[theme]}>
        <a href='https://www.patreon.com/Drumnicorn/about'>
          <Image
            src='/img/artist/patreon-svgrepo-com.svg'
            alt='patreon'
            width={30}
            height={30}
            className={styles.link}
          />
        </a>
        <a href='https://open.spotify.com/artist/07WSvNqUYT3u5Hgkao5qiT?si=kp-boHjkTdKlMvOMEJlFxg'>
          <Image
            src='/img/artist/spotify-svgrepo-com.svg'
            alt='spotify'
            width={30}
            height={30}
            className={styles.link}
          />
        </a>
        <a href='https://soundcloud.com/drumnicorn'>
          <Image
            src='/img/artist/soundcloud-svgrepo-com.svg'
            alt='soundcloud'
            width={30}
            height={30}
            className={styles.link}
          />
        </a>
        <a href='https://www.youtube.com/@drumnicorn'>
          <Image
            src='/img/artist/youtube-svgrepo-com.svg'
            alt='youtube'
            width={30}
            height={30}
            className={styles.link}
          />
        </a>
        <a href='https://www.instagram.com/drumnicorn/'>
          <Image
            src='/img/artist/instagram-svgrepo-com.svg'
            alt='instagram'
            width={30}
            height={30}
            className={styles.link}
          />
        </a>
      </div>
    </footer>
  );
}
