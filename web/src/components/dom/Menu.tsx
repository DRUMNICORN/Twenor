// Menu.tsx

import React from "react";
import styles from "./Menu.module.scss";
import Link from "next/link";
import UserInfo from "../common/global/UserInfo";
import { useRouter } from "next/router";
import Image from "next/image";

interface MenuProps {
  themeToggler: () => void;
  theme: string;
  open: boolean;
  onClose: () => void;
}

const Menu: React.FC<MenuProps> = ({ themeToggler, theme, open, onClose }) => {
  const handleLinkClick = () => {
    // Call the onClose function to close the header
    onClose();
  };

  const getCurrentPage = (path: string) => {
    const currentPath = "video";
    return currentPath === path ? styles.currentSelected : "";
  };

  let isMenuOpen = open === true ? styles.show : "";
  return (
    <ul className={`${styles.menu} ${isMenuOpen}`}>
      <li className={`${styles.menuItem} ${getCurrentPage("/video")}`}>
        {/* 
          Pages:
            - (Home) not displayed
            - Video
            - Jukebox
            - About
            
            < spacer >
            - The Box (1€)

        */}

        <Link href="/music2video" className={styles.menuLink} onClick={handleLinkClick}>
          Video
        </Link>
      </li>
      <li className={`${styles.menuItem} ${getCurrentPage("/jukebox")}`}>
        <Link href="/jukebox" className={styles.menuLink} onClick={handleLinkClick}>
          Jukebox
        </Link>
      </li>
      <li className={`${styles.menuItem} ${getCurrentPage("/about")}`}>
        <Link href="/about" className={styles.menuLink} onClick={handleLinkClick}>
          About
        </Link>
      </li>
      <li className={`${styles.menuItem} ${getCurrentPage("/thebox")} ${styles.theBox}`}>
        <Link href="/thebox" className={styles.menuLink} onClick={handleLinkClick}>
          The Box
        </Link>
      </li>
      <li className={`${styles.menuItem} ${styles.navFooter}`}>
        <UserInfo menuOpen={open} />

        <div className={styles.themeToggle}>
          <button className={styles.button} onClick={themeToggler}>
            {theme === "light" ? (
              <img src="/icons/theme/sun.svg" alt="Light Mode" width={20} height={20} />
            ) : (
              <img src="/icons/theme/moon.svg" alt="Dark Mode" width={20} height={20} />
            )}
          </button>
        </div>
      </li>
    </ul>
  );
};

export default Menu;
