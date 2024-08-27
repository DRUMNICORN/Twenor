import { useEffect, useState } from 'react';
import { v4 as uuidv4 } from 'uuid';

export function useIdentity() {
  const [identity, setIdentity] = useState(null);

  useEffect(() => {
    const identity = localStorage.getItem('identity');
    if (identity) {
      setIdentity(identity);
    } else {
      const newIdentity = uuidv4();
      localStorage.setItem('identity', newIdentity);
      setIdentity(newIdentity);
    }
  }, []);

  return {
    identity
  };
}
