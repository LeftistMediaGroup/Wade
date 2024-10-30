import React, { Component } from "react";

import { Card, CardContent } from "@mui/material";


export default class Account extends Component {
  constructor(props) {
    super(props);

    this.state = {};
  }

  render() {
    return (
        <Card variant="outlined">
          <CardContent>
            <h1>Account</h1>
            <Card>
              <CardContent>
                <h3>Profile</h3>

                <p> Username: {this.props.username}</p>

              </CardContent>
            </Card>
          </CardContent>
        </Card>
    );
  }
}