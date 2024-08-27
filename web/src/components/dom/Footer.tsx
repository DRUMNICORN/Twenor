import React from 'react';
import Image from 'next/image';
import styles from './Footer.module.scss';

const Footer: React.FC = () => {
  const config = [
    {
      href: 'https://www.patreon.com/Drumnicorn/about',
      alt: 'patreon',
      src: '/icons/artist/patreon-svgrepo-com.svg',
    },
    {
      href: 'https://open.spotify.com/artist/07WSvNqUYT3u5Hgkao5qiT?si=kp-boHjkTdKlMvOMEJlFxg',
      alt: 'spotify',
      src: '/icons/artist/spotify-svgrepo-com.svg',
    },
    {
      href: 'https://soundcloud.com/drumnicorn',
      alt: 'soundcloud',
      src: '/icons/artist/soundcloud-svgrepo-com.svg',
    },
    {
      href: 'https://www.youtube.com/@drumnicorn',
      alt: 'youtube',
      src: '/icons/artist/youtube-svgrepo-com.svg',
    },
    {
      href: 'https://www.instagram.com/drumnicorn/',
      alt: 'instagram',
      src: '/icons/artist/instagram-svgrepo-com.svg',
    },
  ];

  return (
    <footer className={styles.footer}>
      <div className={styles.footer__left}>
        {config.map((item, index) => (
          <a href={item.href} key={index} className={styles.link}>
            <button className={styles.button}>
              <img src={item.src} alt={item.alt} width={20} height={20} />
            </button>
          </a>
        ))}
      </div>
      <div className={styles.footer__right}>
        <p>© 2023 Drumnicorn</p>
      </div>
    </footer>
  );
};

export default Footer;
