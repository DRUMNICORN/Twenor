"use client"

import React, { useState } from "react";
import styles from "@/styles/Header.module.scss";

interface HambugerProps {
    onClick: () => void;
  }

const Hambuger: React.FC<HambugerProps> = ({ onClick }) => {
    const [menuOpen, setMenuOpen] = useState(false);

    const handleMenuClick = () => {
        setMenuOpen(!menuOpen);
        onClick();
    };


    return (
        <div
        className={`${styles.hamburger} ${menuOpen ? styles.show : ""}`}
        onClick={handleMenuClick}
      >
        <span></span>
        <span></span>
        <span></span>
      </div>
    );
  };

export default Hambuger;
