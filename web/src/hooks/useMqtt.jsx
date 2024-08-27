// This is a custom hook that is used to connect to an MQTT broker and subscribe to a topic. It also handles publishing messages to the topic. This hook is used in the Chat component in the Chat feature.

import { useState, useEffect } from 'react';
import mqtt from 'mqtt';

export default function useMqtt(topic) {
  const [client, setClient] = useState(null);
  const [message, setMessage] = useState('');

  // stats 
  const [connected, setConnected] = useState(false);
  const [offline, setOffline] = useState(false);
  const [closed, setClosed] = useState(false);

  const [error, setError] = useState(null);

  useEffect(() => {
    const client = mqtt.connect('ws://localhost:9001', {
      clientId: 'mqttjs_' + Math.random().toString(16).substr(2, 8)
    });


    client.on('connect', () => {
      client.subscribe(topic);
      setConnected(true);
    });

    client.on('message', (topic, message) => {
      setMessage(message.toString());
    });

    client.on('error', (error) => {
      setError(error);
    });

    client.on('offline', () => {
      setOffline(true);
    });

    client.on('close', () => {
      setClosed(true);
    });

    setClient(client);

    return () => {
      client.end();
    }
  }, [topic]);

  const publish = (topic, message) => {
    client.publish(topic, message);
  }

  return {
    message, publish, connected, offline, closed, error
  }
}