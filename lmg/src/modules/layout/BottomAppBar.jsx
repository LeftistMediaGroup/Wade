import React from "react";

import Music from "../account/dashboard/Music.jsx";
import Journal from "../account/dashboard/Journal/Journal.jsx";


export default class BottomAppBar extends React.Component {
  constructor(props) {
    super(props);

    this.state = {};
  }

  renderBottomComponent = () => {
    if (this.props.BottomAppBarComponent === "Music") {
      return <Music />;
    } else if (this.props.BottomAppBarComponent === "Messages") {
      return <Chat username={this.props.username} />;
    } else if (this.props.BottomAppBarComponent === "RSS") {
      return <RSS />;
    } else if (this.props.BottomAppBarComponent === "Notifications") {
      return <h1> Notifications</h1>;
    } else if (this.props.BottomAppBarComponent === "Journal") {
      return <Journal username={this.props.username} />;
    }
  };

  render() {
    return (
      <>
        {this.renderBottomComponent()}
      </>
    );
  }
}