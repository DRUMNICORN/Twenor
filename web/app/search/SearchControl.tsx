import React from "react";
import styles from "./SearchControl.module.scss";

export default function SearchControl() {
    return (
        <>
            <div className={styles.mt4}>
                <div className={styles.formcheck}>
                    <label className={styles.formchecklabel} htmlFor="UseBestModel">
                        Use Best Model (10 left)
                        <input
                            id="UseBestModel"
                            className={styles.formcheckinput}
                            type="checkbox"
                        />
                    </label>
                </div>
                <div className={styles.formcheck}>
                    <label className={styles.formchecklabel} htmlFor="BasicSearch">
                        Basic Search
                        <input
                            id="BasicSearch"
                            className={styles.formcheckinput}
                            type="checkbox"
                            defaultChecked
                        />
                    </label>
                </div>
            </div>
        </>
    );
}
