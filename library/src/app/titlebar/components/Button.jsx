function Button(props) {
  return (
    <div className="titlebar-button" id={props.name} onClick={props.onClick}>
      <img className="titlebar-img" src={props.link} alt={props.name} />
    </div>
  );
}

export default Button;
