/**
 * @file Page.tsx
 * @description Page component for the Tauri app. Displays the track library and the track editor.
 * @author
 * @version 0.0.1
 * @date 2021-06-01
 */

import React from "react";

import "../../styles/Page.scss";

type PageProps = {
  index: number;
  children: React.ReactNode;
};

type PageState = {};

class Page extends React.Component<PageProps, PageState> {
  constructor(props: PageProps) {
    super(props);
    this.state = {
      index: 0,
    };
  }

  render() {
    return <div className="page">{this.props.children}</div>;
  }
}

export default Page;
