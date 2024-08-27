// SearchBar.js

import React from "react";
import searchBarStyles from "./SearchBar.module.scss";
import SearchControl from "./SearchControl";

export default function SearchBar() {
    return (
        <div className={searchBarStyles.searchBar}>
            <div className={searchBarStyles.colLg8}>
                <div className={searchBarStyles.containerXl}>
                    <form className={searchBarStyles.mb3}>
                        <div className={`${searchBarStyles.inputGroup} ${searchBarStyles.shadow} ${searchBarStyles.borderMulticolor} ${searchBarStyles.darkmodeLight} ${searchBarStyles.positionRelative}`}>
                            <textarea
                                className={`${searchBarStyles.searchboxTextarea} ${searchBarStyles.formControl} ${searchBarStyles.bgWhite} ${searchBarStyles.darkmodeLight}`}
                                name="q"
                                autoComplete="off"
                                defaultValue="What is the answer?"
                                rows={1}
                                autoCapitalize="none"
                                autoCorrect="off"
                            />
                            <button className={`${searchBarStyles.btn} ${searchBarStyles.btnPrimary} ${searchBarStyles.p1} ${searchBarStyles.positionAbsolute} ${searchBarStyles.end0} ${searchBarStyles.bottom0} ${searchBarStyles.m2} ${searchBarStyles.textBlack} ${searchBarStyles.rounded}`} type="submit">
                                <i className={`${searchBarStyles.fs3} ${searchBarStyles.fe} ${searchBarStyles.feArrowRight} ${searchBarStyles.textBlack}`} />
                                <span className={searchBarStyles.visuallyHidden} />
                            </button>
                        </div>
                    </form>
                    <SearchControl />
                </div>
            </div>
        </div>
    );
}
