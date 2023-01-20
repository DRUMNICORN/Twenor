/**
 * The App should Cointain the following:
 *  - Titlebar
 *  - Sidebar
 *  - Content
 */

import React from "react";

import Titlebar from "./components/Titlebar";
import Sidebar from "./components/Sidebar";
import Content from "./components/Content";

// In this file we can import the tauri api
import { appWindow } from "@tauri-apps/api/window";
import Explorer from "./components/Explorer";
import Logo from "./components/design/Logo";

import "./App.scss";

function App(): JSX.Element {
  return (
    <div className="App">
      <Titlebar />
      <Sidebar>
        <Logo />
        <Explorer />
      </Sidebar>
      <Content />
    </div>
  );
}

export default App;
