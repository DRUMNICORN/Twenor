/**
 * @file Logo.tsx
 * @description Logo component for the Tauri app. Displays the logo of the app.
 * @author
 * @version 0.0.1
 * @date 2021-06-01
 *
 */

import React from "react";

import "../../styles/Logo.scss";

// create a new logo component
function Logo(): JSX.Element {
  // it should contain a logo component and a explorer component
  return (
    <div className="logo">
      <div className="image" />  
      <div className="background" />
    </div>
  );
}

export default Logo;
