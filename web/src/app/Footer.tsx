import Image from 'next/image';
import React from 'react';
import styles from '@/styles/Footer.module.scss';

export const Footer: React.FC = () => {
  const config = [
    {
      href: 'https://www.patreon.com/Drumnicorn/about',
      alt: 'patreon',
      src: '/img/artist/patreon-svgrepo-com.svg',
    },
    {
      href: 'https://open.spotify.com/artist/07WSvNqUYT3u5Hgkao5qiT?si=kp-boHjkTdKlMvOMEJlFxg',
      alt: 'spotify',
      src: '/img/artist/spotify-svgrepo-com.svg',
    },
    {
      href: 'https://soundcloud.com/drumnicorn',
      alt: 'soundcloud',
      src: '/img/artist/soundcloud-svgrepo-com.svg',
    },
    {
      href: 'https://www.youtube.com/@drumnicorn',
      alt: 'youtube',
      src: '/img/artist/youtube-svgrepo-com.svg',
    },
    {
      href: 'https://www.instagram.com/drumnicorn/',
      alt: 'instagram',
      src: '/img/artist/instagram-svgrepo-com.svg',
    },
  ];

  return (
    <footer className={styles.footer}>
      <div className={styles.footer__left}>
        {config.map((item, index) => (
          <a href={item.href} key={index}>
            <Image src={item.src} alt={item.alt} width={30} height={30} className={styles.link} />
          </a>
        ))}
        </div>
      <div className={styles.footer__right}>
        <p>© 2023 Drumnicorn</p>
      </div>
    </footer>
  );
};
