import { useState, useEffect } from 'react';
import { useQuery } from 'react-query';

const fetcher = async (url) => {
  const response = await fetch(url);
  return response.json();
};


export function useChat(identity) {
  const globalAddress = `/api/`;
  const { data, error, isLoading } = useQuery(['chat', identity], () => fetcher(`/api/get?identity=${identity}`), {
    refetchInterval: 1000,
  });
  const [messages, setMessages] = useState([]);
  const [lastMessage, setLastMessage] = useState(null);

  // Update messages and track last message
  useEffect(() => {
    if (messages.length === 0) return;
    if (lastMessage && lastMessage.type === 'user' && lastMessage.timestamp === messages.at(-1).timestamp) return;
    setLastMessage(messages.at(-1));
  }, [messages, lastMessage]);

  // Send user message to the server
  useEffect(() => {
    if (lastMessage && lastMessage.type === 'user') {
      console.log('Sending message to server:', lastMessage);
      const address = `${globalAddress}send?identity=${identity}&text=${encodeURIComponent(
        lastMessage.text
      )}&timestamp=${lastMessage.timestamp}`;
      console.log(address);
      fetch(address);
    }
  }, [lastMessage, globalAddress, identity]);

  // Process received data from the server
  useEffect(() => {
    console.log('Data:', data);
    if (data) {
      const parsedData = data.map((message) => ({
        ...message,
        timestamp: Number(message.timestamp),
      }));

      setMessages(parsedData);

      const lastMessageType = parsedData.at(-1)?.type;
      setIsBotReady(lastMessageType === 'bot');
    }
  }, [data]);

  // Handle errors
  useEffect(() => {
    console.log('Error:', error);
    setIsBotReady(false);
  }, [error]);

  const setIsBotReady = (isReady) => {
    // Implement your custom logic for handling bot readiness
    // ...
  };

  return [messages, setMessages];
}
