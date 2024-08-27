import React from "react";
import ReactDOM from "react-dom/client";

import "./styles/App.scss";
import TitleBar from "./app/titlebar/TitleBar";
import Main from "./app/Main";
ReactDOM.createRoot(document.getElementById("root")).render(
  <>
    <TitleBar />
    <Main />
  </>
);

