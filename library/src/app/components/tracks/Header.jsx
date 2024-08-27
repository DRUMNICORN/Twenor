// a header element displays the name of the column

import React from "react";
import render from "react-dom";

class HeaderElement extends React.Component {
  constructor(props) {
    super(props);
  }

  render() {
    return (
      <thead className="node-table-header-element" >
        <tr>
          <th>Name</th>
          <th>Nodes</th>
        </tr>
      </thead>
    );
  }
}

export default HeaderElement;
