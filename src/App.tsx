/**
 * The App should Cointain the following:
 *  - Titlebar
 *  - Sidebar
 *  - Content
 */

import React from "react";

import Titlebar from "./components/design/Titlebar";
import Sidebar from "./components/util/Sidebar";
import Content from "./components/util/Content";

import SelectRecordbox from "./components/pages/SelectRecordbox";
import Welcome from "./components/pages/Welcome";
import Explorer from "./components/util/Explorer";

// In this file we can import the tauri api
import { appWindow } from "@tauri-apps/api/window";

import Logo from "./components/design/Logo";

import "./App.scss";
import Pages from "./components/util/Pages";
import Page from "./components/util/Page";
import Interface from "./Interface";

import TrackOverview from "./components/pages/TrackOverview";
import TrackList from "./components/util/TrackList";

type AppProps = {};
type AppState = {
  width: number;
  height: number;
  xml_path: string;
  page: number;
  sidebar_width: number;
  nodes: any;
  tracks: any;
};
class App extends React.Component<AppProps, AppState> {
  constructor(props: AppProps) {
    super(props);
    this.state = {
      width: 0,
      height: 0,
      xml_path: "",
      page: 0,
      sidebar_width: 0,
      nodes: [],
      tracks: [],
    };
  }

  // on state change
  componentDidMount() {
    Interface.load_state().then((state: Array<any>) => {
      let new_state: any = {};
      state.forEach((element: any) => {
        new_state[element.key] = element.value;
      });
      this.setState(new_state);
      Interface.save_state(this.state);
    });
  }

  componentDidUpdate(prevProps: Readonly<AppProps>, prevState: Readonly<AppState>, snapshot?: any): void {
    Interface.save_state(this.state);
  }

  render() {
    return (
      <div className="App">
        <Titlebar />
        <Sidebar
          initial_width={this.state.sidebar_width}
          onResize={(width: number): void => this.setState({ sidebar_width: width })}
        >
          <>
            <Logo />
            <Explorer
              nodes={this.state.nodes}
              onChoose={(node: any): void => {
                Interface.request_tracks_by_path(node.PATH).then((tracks: any) => {
                  console.log(tracks);
                  this.setState({ page: 2 });
                  this.setState({ tracks: tracks });
                });
              }}
            />
          </>
        </Sidebar>
        <Content>
          <Pages
            page={this.state.page}
            onChange={(page: number): void => {
              this.setState({ page: page });
            }}
          >
            <Page index={0}>
              <Welcome />
              <SelectRecordbox
                path={this.state.xml_path}
                onChange={(path: string): void => {
                  this.setState({ xml_path: path });
                }}
                onNodes={(nodes: any): void => {
                  this.setState({ nodes: nodes });
                }}
              />
            </Page>
            <Page index={1}>
              <TrackOverview nodes={this.state.nodes} />
            </Page>
            <Page index={2}>
              <TrackList tracks={this.state.tracks} />
            </Page>
          </Pages>
        </Content>
      </div>
    );
  }
}

export default App;
