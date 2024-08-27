// Gallery.jsx

import Image from "next/image";
import React, { useEffect, useState } from "react";

export default function Gallery() {
  const [media, setMedia] = useState([]);
  const [error, setError] = useState(null);

  useEffect(() => {
    const getInstagramMedia = async () => {
      try {
        // const res = await axios.get(
        //   `https://graph.instagram.com/me/media?fields=id,caption,media_url,thumbnail_url,permalink&access_token=${process.env.REACT_APP_INSTAGRAM_ACCESS_TOKEN}`
        // );
        let adress = "https://graph.instagram.com/me/media?fields=id,caption,media_url,thumbnail_url,permalink&access_token=";
        adress += process.env.REACT_APP_INSTAGRAM_ACCESS_TOKEN;
        fetch(adress).then(res => res.json()).then(data => {
          console.log(data);
          setMedia(data.data);
        });
      } catch (err) {
        setError(err);
      }
    };

    getInstagramMedia();
  }, []);

  return (
    <div style={{ display: "flex", flexWrap: "wrap", height: "100em" }}>
      {error && <p>Something went wrong. Please try again later.</p>}
      {media.map((item, index) => (
        <a href={item.permalink} target="_blank" rel="noreferrer" key={index}>
          <Image src={item.media_url} alt={item.caption} />
        </a>
      ))}
    </div>
  );
}


