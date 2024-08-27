"use client"

import { useEffect, useState } from 'react';


type Theme = {
    [key: string]: string;
};

export const CONFIG: Theme = {
    Add: '#FDFFF0',
    Sub: '#000505',
    Primary: '#6a11cb',
    Secondary: '#2575fc',
    SubRGB: '0, 5, 5',
    AddRGB: '253, 255, 240',
    PrimaryRGB: '106, 17, 203',
    SecondaryRGB: '37, 117, 252',
};


export const useThemeMode = () => {
    const [theme, setTheme] = useState('dark');

    const setMode = (mode: string) => {
        window.localStorage.setItem('theme', mode);
        setTheme(mode);
    };

    const themeToggler = () => (theme === 'dark' ? setMode('light') : setMode('dark'));

    useEffect(() => {
        const localTheme = window.localStorage.getItem('theme');
        localTheme && setTheme(localTheme);
    }, []);

    const THEME = {
        light: {
            '--color-dark': CONFIG.Sub,
            '--color-light': CONFIG.Add,

            '--color-bg': CONFIG.Add,

            '--color-sub': CONFIG.Sub,
            '--color-add': CONFIG.Add,
            '--color-primary': CONFIG.Primary,
            '--color-secondary': CONFIG.Secondary,

            '--color-sub-rgb': CONFIG.SubRGB,
            '--color-add-rgb': CONFIG.AddRGB,
            '--color-primary-rgb': CONFIG.PrimaryRGB,
            '--color-secondary-rgb': CONFIG.SecondaryRGB,

            '--filter': 'invert(0%)',
            '--filter-hover': 'invert(100%)',

            '--font-family': '\'Manrope\', sans-serif'
        },
        dark: {
            '--color-dark': CONFIG.Sub,
            '--color-light': CONFIG.Add,

            '--color-sub': CONFIG.Add,
            '--color-add': CONFIG.Sub,
            '--color-primary': CONFIG.Secondary,
            '--color-secondary': CONFIG.Primary,

            '--color-sub-rgb': CONFIG.AddRGB,
            '--color-add-rgb': CONFIG.SubRGB,
            '--color-primary-rgb': CONFIG.SecondaryRGB,
            '--color-secondary-rgb': CONFIG.PrimaryRGB,

            '--filter': 'invert(100%)',
            '--filter-hover': 'invert(0%)',

            '--font-family': '\'Manrope\', sans-serif'
        },
    };

    const themeStyles = THEME[theme];

    useEffect(() => {
        Object.keys(themeStyles).forEach((property) => {
            const value = themeStyles[property];
            document.documentElement.style.setProperty(property, value);
            // iniitilize Manrope font
        });
    }, [themeStyles]);

    return { theme, themeToggler };
};

export default useThemeMode;
