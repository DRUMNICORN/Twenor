const fetch = require('node-fetch');
const fs = require('fs');

// Fill in your client ID and client secret here
let config = JSON.parse(fs.readFileSync('../config.json', 'utf8'));
let CLIENT_ID = config.SPOTY_ID;
let CLIENT_SECRET = config.SPOTY_SECRET;

async function getAccessToken() {
  console.log('CLIENT_ID: ', CLIENT_ID);
  const response = await fetch('https://accounts.spotify.com/api/token', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded',
      'Authorization': `Basic ${Buffer.from(`${CLIENT_ID}:${CLIENT_SECRET}`).toString('base64')}`,
    },
    body: 'grant_type=client_credentials',
  });

  if (!response.ok) {
    throw new Error(`Failed to get access token: ${response.statusText}`);
  }

  const data = await response.json();

  return data.access_token;
}

async function main() {
  try {
    const accessToken = await getAccessToken();
    console.log(`Access token: ${accessToken}`);
    // overwrite the access token in the .env.local file
    // NEXT_PUBLIC_SPOTIFY_ACCESS_TOKEN=your_access_token
    fs.writeFileSync('.env.local', `NEXT_PUBLIC_SPOTIFY_ACCESS_TOKEN=${accessToken}`);
  } catch (err) {
    console.error(err);
  }
}

console.log(`
==============================
Spotify Access Token Generator
==============================

  client_id: ${CLIENT_ID}
  client_secret (last 5 chars): ...${CLIENT_SECRET.slice(-5)}
`)
main();

