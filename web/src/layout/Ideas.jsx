import React, { useState, useEffect } from 'react';
import GithubCard from '@/components/GithubCard';
import useGithubProjects from '@/hooks/useGithubProjects';
import styles from '@/styles/Ideas.module.scss';
import Prompt from '@/components/Prompt';
import { useIdentity } from '@/hooks/useIdentity';
import { useChat } from '@/hooks/useChat';

export default function Ideas() {
  const { projects, loading } = useGithubProjects('drumni');
  const selectiveProjects = ['wave-lib', 'nodium', 'musictomovie'];

  const [subscribedCards, setSubscribedCards] = useState([]);
  const [validSubscribedCards, setValidSubscribedCards] = useState([]);
  const [showPrompt, setShowPrompt] = useState(false);
  const [selectedCardId, setSelectedCardId] = useState(null);
  const [pendingSubscriptions, setPendingSubscriptions] = useState([]); // Track cards in pending state
  const { identity } = useIdentity();
  const [messages, sendMessage] = useChat(identity); // Use the useChat hook
  const [input, setInput] = useState('');
  const [validEmail, setValidEmail] = useState(false); // Track email validity

  useEffect(() => {
    // Load subscribed cards from localStorage
    const storedSubscribedCards = localStorage.getItem('subscribedCards');
    if (storedSubscribedCards) {
      setSubscribedCards(JSON.parse(storedSubscribedCards));
    }
  }, []);

  useEffect(() => {
    // Save subscribed cards to localStorage
    localStorage.setItem('subscribedCards', JSON.stringify(subscribedCards));
  }, [subscribedCards]);

  if (loading) {
    return <div>Loading...</div>;
  }

  const handleSubscribe = (cardId) => {
    if (subscribedCards.includes(cardId)) {
      setSubscribedCards(subscribedCards.filter((id) => id !== cardId));
      sendMessage(`Unsubscribed from ${cardId}`);
    } else {
      setSelectedCardId(cardId);
      setShowPrompt(true);
    }
  };

  const handlePromptInput = (input) => {
    setInput(input);
    // Validate the email
    const isValidEmail = /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(input);
    setValidEmail(isValidEmail);
  };

  const handlePromptSubmit = async (e) => {
    e.preventDefault();
    if (!validEmail) {
      // Handle invalid email input
      return;
    }
    const email = input;
    try {
      setPendingSubscriptions([...pendingSubscriptions, selectedCardId]); // Move the card to the pending state
      await fetch('/api/subscribe', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ email, identity, cardId: selectedCardId, subscribe: true }),
      });

      sendMessage(`Subscribed to ${selectedCardId}`);
      setValidSubscribedCards([...validSubscribedCards, selectedCardId]); // Add the cardId to the validSubscribedCards immediately
      setSubscribedCards([...subscribedCards, selectedCardId]); // Add the cardId to the subscribedCards immediately
      setPendingSubscriptions(pendingSubscriptions.filter((id) => id !== selectedCardId)); // Remove the card from the pending state
      setShowPrompt(false);
    } catch (error) {
      // Handle error
      setPendingSubscriptions(pendingSubscriptions.filter((id) => id !== selectedCardId)); // Remove the card from the pending state on error
    }
  };

  return (
    <div className={styles.ideas}>
      {selectiveProjects.map((project) => {
        const repo = projects.find((repo) => repo.name === project);
        if (repo) {
          const isSubscribed = subscribedCards.includes(repo.id);
          const isPending = pendingSubscriptions.includes(repo.id); // Check if the card is in the pending state
          const isPromptVisible = showPrompt && selectedCardId === repo.id;
          const isUnsubscribeDisabled = isPromptVisible && !validEmail;

          return (
            <div key={repo.id} className={styles.projectCard}>
              <GithubCard
                username={'drumni'}
                repo={repo}
                subscribed={isSubscribed}
                pending={isPending} // Pass the pending state to the GithubCard component
                onSubscribe={handleSubscribe}
              />
              {isPromptVisible && (
                <Prompt
                  handleSubmit={handlePromptSubmit}
                  handleInput={handlePromptInput}
                  placeholder="Enter your email"
                  enableEmailValidation={true}
                  disableSubmit={!isSubscribed && isUnsubscribeDisabled} // Disable submit when not subscribed and email is invalid
                />
              )}
            </div>
          );
        }
        return null;
      })}
    </div>
  );
}


