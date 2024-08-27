"use client"

import { createGlobalStyle, withTheme } from 'styled-components';
import { ThemeProps } from './themes';

type GlobalThemeProps = {
  theme: ThemeProps;
};

const PALLETE = {
  Ebony: '#000505',
  Olive: '#FFA69E',
  Ivory: '#FDFFF0',
  Red: '#970C10',
  Shadow: '-1px 0px 0px 2px rgba(0, 0, 0, 0.2)',
  Shine: '0px 0px 1px 0px rgba(255, 255, 255, 1)',
};

const THEME = {
  light: {
    '--theme': '#FFF',
    '--box-shadow': PALLETE.Shadow,
    '--color-background': PALLETE.Ivory,
    '--color-sub': PALLETE.Ebony,
    '--color-add': PALLETE.Ivory,
    '--color-primary': PALLETE.Olive,
    '--color-secondary': PALLETE.Red,
  },
  dark: {
    '--theme': '#000',
    '--box-shadow': PALLETE.Shine,
    '--color-background': PALLETE.Ebony,
    '--color-sub': PALLETE.Ivory,
    '--color-add': PALLETE.Ebony,
    '--color-primary': PALLETE.Olive,
    '--color-secondary': PALLETE.Red,
  },
};

const globalStyle = createGlobalStyle`
  :root {
    --theme: ${PALLETE.Ivory};
    --box-shadow: ${PALLETE.Shadow};
    --color-text: ${PALLETE.Ebony};
    --color-background: ${PALLETE.Ivory};
    --color-sub: ${PALLETE.Ebony};
    --color-add: ${PALLETE.Ivory};
    --color-primary: ${PALLETE.Olive};
    --color-secondary: ${PALLETE.Red};
  }

  * {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
    outline: 0;
  }

  body  {
    -webkit-font-smoothing: antialiased;
    height: 100vh;
    width: 50vw;
    margin: 0 auto;
    background-color: ${({ theme }: GlobalThemeProps) => theme.background};
    display: flex;
    justify-content: center;
    align-items: center;
  }

  h1 {
    font-size: 3.375rem;
    color: ${({ theme }: GlobalThemeProps) => theme.text};
  }

`;

export default withTheme(globalStyle);
