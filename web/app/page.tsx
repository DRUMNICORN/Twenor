import Link from 'next/link';
import styles from './Home.module.scss';
export default function Home() {
  return (
    <main className={styles.main}>
      <h1 className={styles.title}>Welcome back <a href="https://drumni.com" className={styles.link}>drumni.com</a></h1>
    </main>
  );
}