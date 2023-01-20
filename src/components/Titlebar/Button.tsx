/**
 * React Component for the Titlebar Buttons (Minimize, Maximize, Close)
 */

import React from "react";

interface ButtonProps {
  name: string;
  link: string;
  onClick: () => void;
}

function Button(props: ButtonProps) {
  let src = `https://api.iconify.design/${props.link}.svg`;
  console.log(src);
  return (
    <div className="titlebar-button">
      <img src={src} alt={props.name} onClick={() => props.onClick()} />
    </div>
  );
}

export default Button;
