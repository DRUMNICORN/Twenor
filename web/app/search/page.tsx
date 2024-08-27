import React from "react";
import Result from "./Result";
import SearchBar from "./SearchBar";
import LoadingResult from "./LoadingResult";
import Query from "./Query";

import "./styles.scss";

export default function Component() {
    return (
        <div
            className="search-page" >
            <SearchBar />
            <Result />
            <Query />
            <LoadingResult />
        </div>
    );
}
