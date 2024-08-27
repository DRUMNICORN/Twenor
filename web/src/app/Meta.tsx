"use client";

import Head from 'next/head';
import './globals.css';
import { Inter } from 'next/font/google';

const inter = Inter({ subsets: ['latin'] });

const Meta: React.FC = () => (
  <Head>
    <link
      href="https://fonts.googleapis.com/css?family=Manrope&display=optional"
      rel="stylesheet"
    />
    <style jsx global>{`
      body {
        class: ${inter.className}
      }
    `}</style>
    {/* Add more meta tags as you need */}
  </Head>
);

export default Meta;
