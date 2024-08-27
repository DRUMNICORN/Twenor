const PALLETE = {
    Black: '#000505',
    White: '#FDFFF0',
    Green: '#FFA69E',
    Red: '#970C10',
    Blue: '#1A1A1D',
    Shadow: '-1px 0px 0px 2px rgba(0, 0, 0, 0.2)',
    Shine: '0px 0px 1px 0px rgba(255, 255, 255, 1)',
};


export interface ThemeProps {
    background: string;
    text: string;
}

export const darkTheme: ThemeProps = {
    background: PALLETE.Blue,
    text: PALLETE.White,
};

export const lightTheme: ThemeProps = {
    background: PALLETE.White,
    text: PALLETE.Blue,
};

