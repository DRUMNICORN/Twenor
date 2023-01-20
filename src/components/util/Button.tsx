/**
 * React Component for the Titlebar Buttons (Minimize, Maximize, Close)
 */

import React from "react";

interface ButtonProps {
  name: string;
  link: string;
  onClick: () => void;
  isToggable?: boolean;
  onToggle?: () => void;
}

function Button(props: ButtonProps) {
  const [isToggled, setToggled] = React.useState(false);

  let src = `https://api.iconify.design/${props.link}.svg`;

  function onToggle(e: React.MouseEvent<Element, MouseEvent>) {
    e.preventDefault();

    if (props.isToggable) {
      if (props.onToggle) props.onToggle();
      setToggled(!isToggled);
    }
  }

  return (
    <div
      className={`titlebar-button ${props.name} ${isToggled ? "toggled" : ""}`}
      onContextMenu={(e: React.MouseEvent<HTMLDivElement, MouseEvent>) => onToggle(e)}
    >
      <img id={props.name} src={src} alt={props.name} onClick={() => props.onClick()} />
    </div>
  );
}

export default Button;
