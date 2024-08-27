import SpotifyWebApi from 'spotify-web-api-js';

export const spotifyApi = new SpotifyWebApi();
if (process.env.NEXT_PUBLIC_SPOTIFY_ACCESS_TOKEN) {
  spotifyApi.setAccessToken(process.env.NEXT_PUBLIC_SPOTIFY_ACCESS_TOKEN);
} else {
  console.error('No access token found');
}
