"use client"

import React from 'react';
import { signIn, useSession } from 'next-auth/react';
import VideoGenerator from '@/components/data/video/VideoGenerator';

export default function Page() {
  const { data: session, status } = useSession();

  if (status === 'loading') {
    return <div>Loading...</div>;
  }

  if (status === 'unauthenticated') {
    signIn();
    return <div>Redirecting...</div>;
  }

  return (
    <div>
      <VideoGenerator session={session} />
    </div>
  );
}
