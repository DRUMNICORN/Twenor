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
  page: number;
  pages: number;
  onChange: (page: number) => void;
};

type PageControlState = {
  prevDisabled: boolean;
  nextDisabled: boolean;
};

class PageControl extends React.Component<PageControlProps, PageControlState> {
  constructor(props: PageControlProps) {
    super(props);
    this.state = {
      prevDisabled: true,
      nextDisabled: false,
    };
  }

  // check if page is first page or last page and disable the button
  componentDidUpdate() {
    if (this.props.page === 0) {
      if (!this.state.prevDisabled) {
        this.setState({ prevDisabled: true });
      }
    } else {
      if (this.state.prevDisabled) {
        this.setState({ prevDisabled: false });
      }
    }

    if (this.props.page === this.props.pages - 1) {
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
        <button
          className={`pagecontrol__button ${this.state.prevDisabled ? "disabled" : ""}`}
          onClick={() => {
            if (this.state.prevDisabled) return;
            if (this.props.page === 0) return;
            if (this.props.page > this.props.pages - 1) return;

            let new_index = this.props.page - 1;
            this.props.onChange(new_index);
          }}
        >
          {"<"}
        </button>
        <div className="pagecontrol__page">{this.props.page + 1}</div>
        <button
          className={`pagecontrol__button ${this.state.nextDisabled ? "disabled" : ""}`}
          onClick={() => {
            if (this.state.nextDisabled) return;
            if (this.props.page === this.props.pages - 1) return;
            if (this.props.page < 0) return;

            let new_index = this.props.page + 1;
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
