/**
 * @file Welcome.tsx
 * @description Welcome page component for the Tauri app. Displays the welcome page.
 * @author
 * @version 0.0.1
 * @date 2021-06-01
 */

import React from "react";


// create a new welcome component

function Welcome(): JSX.Element {
  // it should a large welcome message

  // and a text explaining what the app does

  // a input field with fileselector for xml library and a button next to it to select a file
  
  // and a button to inintiliaze the library

  return (
    <div className="welcome">
      <div className="welcome-message">
        <h1>Welcome to the Tauri app!</h1>
        <p>
          This is a Tauri app. It is a cross-platform desktop application that is
          built using web technologies like HTML, CSS, and JavaScript.
        </p>
        <p>
          It uses the Tauri CLI to build the app and the Tauri API to interact
          with the operating system.
        </p>
        <p>
          You can find more information about Tauri{" "}
          <a
            href="https://tauri.studio/en/docs/getting-started/intro"
            target="_blank"
            rel="noreferrer"
          >
            here
          </a>
          .
        </p>
      </div>
    </div>
  );
}

export default Welcome;