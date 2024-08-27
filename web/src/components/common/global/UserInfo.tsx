"use client"
import React, { useEffect } from 'react';
import { useSession, signIn, signOut } from 'next-auth/react';
import Image from 'next/image';
import styles from './UserInfo.module.scss';

const UserInfo: React.FC<{ menuOpen: boolean }> = ({ menuOpen }) => {
  const { data: session } = useSession();

  useEffect(() => {
    // router = useRouter();
  }, []);

  const handleLogout = () => {
    signOut();
  };

  const handleLogin = () => {
    signIn();
  };

  return (
    <div className={styles.flex}>
      {session && (
        <>
          <span className={styles.spanned}>Welcome back {session?.user?.name}!</span>
          <button className={styles.button} onClick={handleLogout}>
            <img src="/icons/user/login.svg" alt="Logout" width={20} height={20} />
          </button>
        </>
      )}
      {!session && (
        <>
          <button className={styles.button} onClick={handleLogin}>
            <img src="/icons/user/logout.svg" alt="Login" width={20} height={20} />
          </button>
        </>
      )}
    </div>
  );
};

export default UserInfo;
