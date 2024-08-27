import styles from '@/styles/ToggleThemeButton.module.scss'

export default function ToggleThemeButton({ theme, toggleTheme }) {
  return (
    <div className={styles.toggleThemeContainer}>
      <button className={styles.toggleTheme} onClick={toggleTheme}>
        {theme === 'light' ? '🌙' : '☀️'}
      </button>
    </div>
  );
}
