/**
 * @file PageControl.tsx
 * @description PageControl component for the Tauri app. Displays the track library and the track editor.
 * @author
 * @version 0.0.1
 * @date 2021-06-01
 */

import React from "react";

import "../../styles/PageControl.scss";

type PageControlProps = {
  onChange: (page: number) => void;
  pages: number;
};

type PageControlState = {
  page: number;
  prevDisabled: boolean;
  nextDisabled: boolean;
};

class PageControl extends React.Component<PageControlProps, PageControlState> {
  constructor(props: PageControlProps) {
    super(props);
    this.state = {
      page: 0,
      prevDisabled: true,
      nextDisabled: false,
    };
  }

  // check if page is first page or last page and disable the button
  componentDidUpdate() {
    if (this.state.page === 0) {
      if (!this.state.prevDisabled) {
        this.setState({ prevDisabled: true });
      }
    } else {
      if (this.state.prevDisabled) {
        this.setState({ prevDisabled: false });
      }
    }

    if (this.state.page === this.props.pages - 1) {
      if (!this.state.nextDisabled) {
        this.setState({ nextDisabled: true });
      }
    } else {
      if (this.state.nextDisabled) {
        this.setState({ nextDisabled: false });
      }
    }
  }

  render() {
    return (
      <div className="pagecontrol">
        {/* three elements
          - button to go back (disabled if page is 0)
          - current page
          - button to go forward (disabled if page is last page)
      */}

        <button
          className={`pagecontrol__button ${this.state.prevDisabled ? "disabled" : ""}`}
          onClick={() => {
            if (this.state.prevDisabled) return;
            if (this.state.page === 0) return;
            if (this.state.page > this.props.pages - 1) return;

            let new_index = this.state.page - 1;
            this.setState({ page: new_index });
            this.props.onChange(new_index);
          }}
        >
          {"<"}
        </button>
        <div className="pagecontrol__page">{this.state.page + 1}</div>
        <button
          className={`pagecontrol__button ${this.state.nextDisabled ? "disabled" : ""}`}
          onClick={() => {
            if (this.state.nextDisabled) return;
            if (this.state.page === this.props.pages - 1) return;
            if (this.state.page < 0) return;

            let new_index = this.state.page + 1;
            this.setState({ page: new_index });
            this.props.onChange(new_index);
          }}
        >
          {">"}
        </button>
      </div>
    );
  }
}

export default PageControl;
