import React from "react";

import "./styles.scss";
export default function Query() {
    return (
        <div style={{ boxSizing: "border-box", marginTop: "0" }}>
            <div
                className="col-lg-8 col-xl-7"
                style={{
                    paddingTop: "calc(1.5rem * 5.5)",
                    boxSizing: "border-box",
                    marginTop: "0",
                    maxWidth: "100%",
                    paddingLeft: "calc(32px*.5)",
                    paddingRight: "calc(32px*.5)",
                    flex: "0 0 auto",
                    flexShrink: 0,
                    width: "66.6667%",
                    position: "relative"
                }}
            >
                <div
                    className="container-xl"
                    style={{
                        boxSizing: "border-box",
                        marginLeft: "auto",
                        marginRight: "auto",
                        paddingLeft: "var(--bs-gutter-x,16px)",
                        paddingRight: "var(--bs-gutter-x,16px)",
                        width: "100%"
                    }}
                >
                    <div
                        className="mb-3"
                        style={{ boxSizing: "border-box", marginBottom: "0.75rem" }}
                    >
                        <span
                            className="fs-5 mb-3 font-monospace"
                            style={{
                                boxSizing: "border-box",
                                whiteSpace: "pre-wrap",
                                overflowWrap: "break-word",
                                cursor: "pointer",
                                marginBottom: "0.75rem",
                                fontFamily:
                                    'SFMono-Regular,Menlo,Monaco,Consolas,"Liberation Mono","Courier New",monospace',
                                fontSize: "1rem"
                            }}
                        >{`It is the number of ways to arrange the numbers 1 to 9 in a 3x3 matrix such that the numbers in each row and column are in ascending order.
42 is also...`}</span>
                        <i
                            className="fe fe-chevron-down"
                            style={{
                                boxSizing: "border-box",
                                fontVariant: "normal",
                                fontStyle: "normal",
                                fontWeight: 400,
                                lineHeight: "inherit",
                                textTransform: "none",
                                fontFamily: "Feather"
                            }}
                        />
                    </div>
                </div>
            </div>
            <style
                dangerouslySetInnerHTML={{
                    __html: `
html {
  box-sizing: border-box;
  overflow-x: hidden;
  background-color: rgb(23, 23, 25);
  color: rgb(232, 234, 236);
  color-scheme: dark;
}

body {
  box-sizing: border-box;
  margin: 0px;
  font-family: DM Sans,sans-serif;
  font-size: 1.125rem;
  font-weight: 400;
  line-height: 1.6;
  text-align: var(--bs-body-text-align);
  background-color: rgb(23, 23, 25);
  color: rgb(232, 234, 236);
}
`
                }}
            />
        </div>
    );
}
