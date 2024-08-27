"use client"

import React, { useEffect } from 'react';
import { useSession, signOut } from 'next-auth/react';
import { useRouter } from 'next/router';
import styles from '@/styles/UserInfo.module.scss';

const UserInfo: React.FC = () => {
  const { data: session } = useSession();
  let router;

  useEffect(() => {
    router = useRouter();
  }, []);

  const handleLogout = () => {
    signOut();
  };

  const handleLogin = () => {
    router.push('/api/auth/signin');
  };

  return (
    <div className={styles.userInfo}>
      {session ? (
        <>
          <span className={styles.name}>Welcome, {session.user.name as string}</span>
          <button className={styles.button} onClick={handleLogout}>Logout</button>
        </>
      ) : (
        <>
          <button className={styles.button} onClick={handleLogin}>Login</button>
        </>
      )}
    </div>
  );
};

export default UserInfo;
