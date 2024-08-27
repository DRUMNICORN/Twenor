"use client"

import React from "react";
import { useSession, SessionProvider } from "next-auth/react";
import styles from "@/styles/Header.module.scss";
import Link from "next/link";
import Hamburger from "@/components/Hamburger";
import Menu from "@/components/Menu";
import UserInfo from "@/components/UserInfo";

type HeaderProps = {
  session: any;
};

const Header: React.FC<HeaderProps> = ({ session }) => {
    const [menuOpen, setMenuOpen] = React.useState(false);

    const handleHamburgerClick = () => {
        setMenuOpen(!menuOpen);
    };

    return (
        <SessionProvider session={session}>
            <nav className={styles.header}>
                <Link href="/" className={styles.headerBrand}>
                    Drumni.com
                </Link>
                <UserInfo />
                <Hamburger onClick={handleHamburgerClick} />
                <Menu open={menuOpen} />
            </nav>
        </SessionProvider>
    );
};

export default Header;
