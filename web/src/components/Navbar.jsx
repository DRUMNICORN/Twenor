import React, { useState } from "react";
import styles from "@/styles/Navbar.module.scss";

import Link from "next/link";

const Navbar = () => {
  const [menuOpen, setMenuOpen] = useState(false);

  const handleMenuClick = () => {
    setMenuOpen(!menuOpen);
  };

  return (
    <nav className={styles.navbar}>
      <Link href="/" className={styles.navbarBrand}>
        Drumni.com
      </Link>
      <div className={`${styles.hamburger} ${menuOpen ? styles.show : null}`} onClick={handleMenuClick}>
        <span></span>
        <span></span>
        <span></span>
      </div>
      <ul
        className={
          menuOpen ? `${styles.navbarNav} ${styles.show}` : styles.navbarNav
        }
      >
        <li className={styles.navItem}>
          <Link href="/code" className={styles.navLink} onClick={handleMenuClick}> Code </Link>
        </li>
        <li className={styles.navItem}>
          <Link href="/music" className={styles.navLink} onClick={handleMenuClick}> Music </Link>
        </li>
        <li className={styles.navItem}>
          <Link href="/chat" className={styles.navLink} onClick={handleMenuClick}> Chat </Link>
        </li>
      </ul>
    </nav>
  );
};

export default Navbar;