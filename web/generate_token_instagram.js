const fetch = require('node-fetch');
const fs = require('fs');

// Fill in your client ID and client secret here
let config = JSON.parse(fs.readFileSync('../config.json', 'utf8'));
let CLIENT_ID = config.IG_ID;
let CLIENT_SECRET = config.IG_SECRET;

async function getAccessToken() {
  const response = await fetch('https://graph.instagram.com/access_token', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded',
    },
    body: `grant_type=ig_exchange_token&client_secret=${CLIENT_SECRET}&access_token=${CLIENT_ID}`,
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

main();

