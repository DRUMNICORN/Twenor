import React from 'react';
import styles from '@/styles/Layout.module.scss';
import Header from './Header';
import { Footer } from './Footer';

type RootLayoutProps = {
  children: React.ReactNode;
};

const RootLayout: React.FC<RootLayoutProps> = ({ children }) => {
  return (
    <html>
      <body>
        <div className={styles.app}>   
            <Header session={undefined} />
            <div className={styles.wrapper}>
              <div className={styles.content}>
                {children}
              </div>
            </div>
            <Footer/>    
        </div>
      </body>
    </html>
  );
};

export default RootLayout;
