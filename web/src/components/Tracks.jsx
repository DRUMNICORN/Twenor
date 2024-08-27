import { useState, useEffect, useMemo } from 'react';
import styles from '@/styles/Tracks.module.scss';
import Image from 'next/image';
import { spotifyApi } from '@/libs/spotify-lib';
// import YoutubeEmbed from './YouTubeEmbed';


export default function Tracks() {
  const [tracks, setTracks] = useState([]);
  const [playing, setPlaying] = useState(false);

  useEffect(() => {
    spotifyApi.getArtistAlbums('07WSvNqUYT3u5Hgkao5qiT', { limit: 10 })
      .then(data => {
        setTracks(data.items)
      })
      .catch(err => console.error(err));
  }, []);

  const memoizedTracks = useMemo(() => {
    let rest = 4 - (tracks.length % 4);
    let tracksCopy = [...tracks];
    let placeholder = (id) => ({
      id: `placeholder-${id}`,
      images: [{ url: '/img/placeholder.jpg' }],
      name: 'placeholder'
    });
    for (let i = 0; i < rest; i++) {
      tracksCopy.unshift(placeholder(i));
    }
    return tracksCopy;
  }, [tracks]);

  const container = (
    <div className={styles.container}>
      <div className={styles.box}>
        {memoizedTracks.reduce((rows, track, index) => {
          if (index % 4 === 0) {
            rows.push([]);
          }
          rows[rows.length - 1].push(track);
          return rows;
        }, []).map((row, index) => (
          <div key={index} className={styles.imageContainer}>
            {row.map(track => (
              <div key={track.id} className={styles.image}>
                {
                  track.external_urls ? <a href={track.external_urls.spotify} target='_blank' rel='noreferrer'>
                    <Image src={track.images[0].url} width={500} height={500} layout='responsive' alt={track.name} />
                  </a> : <Image src={track.images[0].url} width={500} height={500} layout='responsive' alt={track.name} />
                }
              </div>
            ))}
          </div>
        ))}
      </div>
    </div>
  );

  return (
    <>
      {container}
    </>
  );
}


