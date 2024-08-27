import { useRef } from 'react'
import dynamic from 'next/dynamic'
import Header from '@/config'
import Navbar from '@/components/Navbar'
import Footer from '@/components/Footer'
import Layout from '@/layout/Layout'
import StyledCursor from '@/components/StyledCursor'

import ToggleThemeButton from '@/components/ToggleThemeButton'

import { useState, useEffect } from 'react';

import styles from '@/styles/_app.module.scss'

const Scene = dynamic(() => import('@/view/Scene'), { ssr: true })

// use src/bot/telegram-bot.jsx for the bot

import { ThemeProvider } from '@emotion/react';

const PALLETE = {
  Ebony: '#000505',
  Olive: '#FFA69E',
  Ivory: '#FDFFF0',
  Red: '#970C10',
  Shadow: '-1px 0px 0px 2px rgba(0, 0, 0, 0.2)',
  Shine: '0px 0px 1px 0px rgba(255, 255, 255, 1)',
}

const THEME = {
  light: {
    '--theme': '#FFF',
    '--box-shadow': PALLETE.Shadow,
    '--text-color': PALLETE.Ebony,
    '--background-color': PALLETE.Ivory,
    '--dark-color': PALLETE.Ebony,
    '--light-color': PALLETE.Ivory,
    '--primary-color': PALLETE.Olive,
    '--secondary-color': PALLETE.Red
  },
  dark: {
    '--theme': '#000',
    '--box-shadow': PALLETE.Shine,
    '--text-color': PALLETE.Ivory,
    '--background-color': PALLETE.Ebony,
    '--dark-color': PALLETE.Ivory,
    '--light-color': PALLETE.Ebony,
    '--primary-color': PALLETE.Olive,
    '--secondary-color': PALLETE.Red
  },
};

export default function App({ Component, pageProps = { title: 'index' } }) {
  const [theme, setTheme] = useState('');
  useEffect(() => {
    if (typeof window !== 'undefined') {
      const localTheme = localStorage.getItem('theme');
      if (localTheme) {
        setTheme(localTheme);
        return;
      }
    }
    const themeQuery = window.matchMedia('(prefers-color-scheme: light)')
    setTheme(themeQuery.matches ? 'light' : 'dark')
    themeQuery.addEventListener('change', ({ matches }) => {
      setTheme(matches ? 'light' : 'dark')
    })
  }, [])

  const ref = useRef()


  const toggleTheme = () => {
    if (theme === 'light') {
      setTheme('dark');
    } else {
      setTheme('light');
    }
  };

  const [GlobalStyle, setGlobalStyle] = useState(createGlobalStyle`
    :root {
      ${THEME.light}
    }
  `);

  useEffect(() => {
    if (theme === 'dark') {
      setGlobalStyle(createGlobalStyle`
        :root {
          ${THEME.dark}
        }
      `);
    } else {
      setGlobalStyle(createGlobalStyle`
        :root {
          ${THEME.light}
        }
      `);
    }
  }, [theme]);

  // save theme to local storage
  useEffect(() => {
    if (typeof window !== 'undefined') {
      localStorage.setItem('theme', theme);
    }
  }, [theme]);

  useEffect(() => {
    document.body.className = theme;
  }, [theme]);
  return (
    <ThemeProvider theme={theme === 'light' ? lightTheme : darkTheme}>
      <Layout ref={ref} className={styles.layout + ' ' + theme} >
        <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Manrope&display=optional" />

        <div className={styles.app}>
          <Header />
          <Navbar />
          <Component {...pageProps} />
          <ToggleThemeButton theme={theme} toggleTheme={toggleTheme} />
          <Footer theme={theme} />
        </div>
        {
          Component?.canvas && (
            <div className={styles.canvas}>
              <Scene className='pointer-events-none' eventSource={ref} eventPrefix='client'>
                {Component.canvas(pageProps)}
              </Scene>
            </div>
          )
        }
      </Layout >
    </ThemeProvider>
  );
}
