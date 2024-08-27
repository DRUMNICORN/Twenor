// pages/api/subscribe.js

import { addUserToWaitlist, removeUserFromWaitlist } from '@/db/database'; // Import the functions to interact with the database

export default async function handler(req, res) {
  if (req.method === 'POST') {
    const { email, identity, cardId, subscribe } = req.body;

    try {
      if (subscribe) {
        await addUserToWaitlist(identity, cardId);
      } else {
        await removeUserFromWaitlist(identity, cardId);
      }

      // Handle other logic and response as needed
      res.status(200).json({ success: true });
    } catch (error) {
      // Handle error and response as needed
      res.status(500).json({ error: 'An error occurred' });
    }
  } else {
    res.status(405).json({ error: 'Method not allowed' });
  }
}
