/**
 * @file Introduction.tsx
 * @description Introduction component for the Tauri app. Displays the track library and the track editor.
 * @author
 * @version 0.0.1
 * @date 2021-06-01
 */

import React from "react";

import "../../styles/Pages.scss";
import PageControl from "./PageControl";

type PagesProps = {
  children: React.ReactNode;
};

type PagesState = {
  page: number;
};

class Pages extends React.Component<PagesProps, PagesState> {
  constructor(props: PagesProps) {
    super(props);
    this.state = {
      page: 0,
    };
  }

  render() {
    let pages = React.Children.count(this.props.children);

    return (
      <div className="pages">
        {/* <>{this.props.children}</> 

          inside this.props.children there should be a Page component

          the Page component should have a prop called index
          the index prop should be the index of the page in the Pages component

          if the index prop of the Page component is equal to the page state of the Pages component, then the Page component should be rendered
        */}
        <div className="pages__page">
          {React.Children.map(this.props.children, (child) => {
            // hide all pages except the current page
            let index = (child as React.ReactElement).props.index;
            if (index !== this.state.page) return null;

            return React.cloneElement(child as React.ReactElement, {
              index: this.state.page,
            });
          })}
        </div>
        <div className="pages__pagecontrol">
          <PageControl
            onChange={(page: number): void => {
              this.setState({ page: page });
            }}
            pages={pages}
          />
        </div>
      </div>
    );
  }
}

export default Pages;
