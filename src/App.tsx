/**
 * The App should Cointain the following:
 *  - Titlebar
 *  - Sidebar
 *  - Content
 */

import React from 'react';

import Titlebar from './components/Titlebar';
import Sidebar from './components/Sidebar';
import Content from './components/Content';

// In this file we can import the tauri api
import { appWindow } from "@tauri-apps/api/window";

function App(): JSX.Element {
  return (
    <div className="App">
      <Titlebar />
      <Sidebar />
      <Content />
    </div>
  );
}

export default App;