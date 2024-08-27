"use client"

import React from "react";
import styles from "@/styles/Header.module.scss";
import Link from "next/link";

interface MenuProps {
  open: boolean;
}

const Menu: React.FC<MenuProps> = ({ open }) => {
  const handleLinkClick = () => {
    // Handle link click logic
  };

  return (
    <ul className={`${styles.headerNav} ${open ? styles.show : ""}`}>
      <li className={styles.navItem}>
        <Link href="/code" className={styles.navLink} onClick={handleLinkClick}>
          Code
        </Link>
      </li>
      <li className={styles.navItem}>
        <Link href="/music" className={styles.navLink} onClick={handleLinkClick}>
          Music
        </Link>
      </li>
      <li className={styles.navItem}>
        <Link href="/chat" className={styles.navLink} onClick={handleLinkClick}>
          Chat
        </Link>
      </li>
    </ul>
  );
};

export default Menu;