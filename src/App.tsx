/**
 * The App should Cointain the following:
 *  - Titlebar
 *  - Sidebar
 *  - Content
 */

import React from "react";

import Titlebar from "./components/Titlebar";
import Sidebar from "./components/util/Sidebar";
import Content from "./components/Content";

import SelectRecordbox from "./components/pages/SelectRecordbox";
import Welcome from "./components/pages/Welcome";
import Explorer from "./components/pages/Explorer";

// In this file we can import the tauri api
import { appWindow } from "@tauri-apps/api/window";
import Logo from "./components/design/Logo";

import "./App.scss";
import Pages from "./components/util/Pages";
import Page from "./components/util/Page";

type AppProps = {};
type AppState = {
  width: number;
  height: number;
};

class App extends React.Component {
  constructor(props: any) {
    super(props);
  }

  render() {
    return (
      <div className="App">
        <Titlebar />
        <Sidebar>
          <>
            <Logo />
            <Explorer />
          </>
        </Sidebar>
        <Content>
          <Pages>
            <Page index={0}>
              <Welcome />
              <SelectRecordbox />
            </Page>
            <Page index={1}>
              <h1>Page 2</h1>
            </Page>
            <Page index={2}>
              <h1>Page 3</h1>
            </Page>
          </Pages>
        </Content>
      </div>
    );
  }
}

export default App;
