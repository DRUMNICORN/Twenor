"use client"

import React from 'react';
import { signIn, useSession } from 'next-auth/react';
import Music2Video from '../../src/components/common/media/Music2Video';

export default function Video() {
  const { data: session, status } = useSession({
    required: true,
    onUnauthenticated: () => signIn(),
  });

  if (status === 'loading') {
    return <div>Loading...</div>;
  }

  if (!session)
    return (
      <div>
        <button onClick={() => signIn()}>Sign in</button>
      </div>
    );
  else
    return (
      <div>
        <Music2Video session={session} />
      </div>
    );

}
