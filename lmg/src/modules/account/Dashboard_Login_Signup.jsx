import React, { Component } from "react";

import { Card, CardContent } from "@mui/material";
import Login from "./Login";
import SignUpForm from "./SignUpForm";

export default class Dashboard_Login_Signup extends Component {
  constructor(props) {
    super(props);

    this.state = {};
  }

  render() {
    return (
      <Card>
        <CardContent>
          <h1>Dashboard Login / Signup</h1>
          <br />

          <SignUpForm />
          <br />
          <Login />
        </CardContent>
      </Card>
    );
  }
}
