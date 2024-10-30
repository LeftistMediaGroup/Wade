import React, { Component } from "react";
import { Form } from "react-bootstrap";

import { Button, Card, CardContent } from "@mui/material";

import socket from "../socket/socket";

export default class Login extends Component {
  constructor(props) {
    super(props);

    this.state = {
      username: null,
      password: null,
    };
  }

  submit = () => {
    console.log(`Submit`);
  };

  usernameChange(event) {
    let username = event.target.value;
    if (username !== this.state.username) {
      this.setState({
        username: username,
      });
    }
  }

  passwordChange(event) {
    let password = event.target.value;

    if (password !== this.state.password) {
      this.setState({
        password: password,
      });
    }
  }


  submit_user = () => {
    if (this.state.short_pass === this.state.password) {
      console.log(`Data out`);

      let data_out = {
        username: this.state.username,
        password: this.state.short_pass
      }

      this.state.socket.emit("create_user", data_out);


      this.set_pass_error(false);

    } else {
      this.set_pass_error(true);
    }
  }

  render() {
    return (
      <div className="row-centered" style={{ maxWidth: 500 }}>

        <Card variant="outlined">
          <CardContent>

            <form className="row-centered">
              <p> Log in existing user</p>

              <Form.Group>
                <Form.Control
                  id="Inputusername"
                  placeholder="Username"
                  onChange={this.usernameChange.bind(this)}
                />
              </Form.Group>
              <br />

              <Form.Group>
                <Form.Control
                  type="password"
                  id="InputPassword"
                  placeholder="Password"
                  onChange={this.passwordChange.bind(this)}
                />
              </Form.Group>
              <br />

              <div className="row-centered">
                <Button
                  color="primary"
                  variant="outlined"
                  onClick={() => {
                    console.log("Clicked!");
                    this.submit_user();
                  }}
                  type="button"
                >
                  Submit
                </Button>
              </div>
            </form>
          </CardContent>
        </Card>
      </div>
    );
  }
}