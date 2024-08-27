"use client";

import styles from './Layout.module.scss';
import { useRef } from 'react';
import dynamic from 'next/dynamic';
import Header from '@/components/dom/Header';
import Footer from '@/components/dom/Footer';
import Style from '@/helpers/style';
import { SessionProvider } from "next-auth/react";
import useThemeMode from '@/hooks/useThemeMode';
import React from 'react';
import ThemeContext from '@/context';
import styled, { ThemeProvider } from 'styled-components';
import { darkTheme, lightTheme } from '@/helpers/themes';

const Scene: any = dynamic(() => import('@/components/dom/Scene'), {
  ssr: false,
});

const LayoutContainer = styled.div`
  display: flex;
  flex-direction: column;
  min-height: 100vh;
`;

const ContentWrapper = styled.div`
  flex: 1 1 auto;
  overflow-y: auto;
`;

const SceneWrapper = styled.div`
  position: fixed;
  top: 10vh;
  left: 0;
  width: 100vw;
  bottom: 10vh;
  pointer-events: none;
`;

const HeaderWrapper = styled.div`
  position: sticky;
  top: 0;
  z-index: 10;
`;

const FooterWrapper = styled.div`
  position: sticky;
  bottom: 0;
  z-index: 10;
`;

const Layout = ({ children }) => {
  const ref = useRef(null);
  const { theme, themeToggler } = useThemeMode();
  const themeMode = theme === 'light' ? lightTheme : darkTheme;

  return (
    <LayoutContainer>
      <SessionProvider>
        <ThemeContext>
          <ThemeProvider theme={themeMode}>
            <div className={styles.app}>
              <HeaderWrapper>
                <Header themeToggler={themeToggler} theme={theme} />
              </HeaderWrapper>
              <ContentWrapper>
                <div className={styles.content}>
                  {children}

                  {/* <button onClick={themeToggler}>{theme}</button> */}

                </div>
              </ContentWrapper>
              <FooterWrapper>
                <Footer />
              </FooterWrapper>
            </div>

            <SceneWrapper>
              <Scene eventSource={ref} eventPrefix='client' />
            </SceneWrapper>

          </ThemeProvider>
        </ThemeContext>
      </SessionProvider>
    </LayoutContainer>
  );
};

export { Layout };
