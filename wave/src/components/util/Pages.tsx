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
  onChange: (page: number) => void;
  page: number;
};

class Pages extends React.Component<PagesProps> {
  constructor(props: PagesProps) {
    super(props);
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
            if (index !== this.props.page ) return null;

            return React.cloneElement(child as React.ReactElement, {
              index: this.props.page,
            });
          })}
        </div>
        <div className="pages__pagecontrol">
          <PageControl
            page={this.props.page}
            onChange={(page: number): void => {
              this.props.onChange(page);
            }}
            pages={pages}
          />
        </div>
      </div>
    );
  }
}

export default Pages;
