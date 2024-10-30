import React, { Component } from "react";
import { Form } from "react-bootstrap";

import { Accordion, AccordionDetails, AccordionSummary, Button, Card, CardContent } from "@mui/material";


export default class SignUpForm extends Component {
  constructor(props) {
    super(props);

    this.state = {
      username: null,
      password: null,
      pass_error: false,
      ready: null,
      short_pass: null,
    };
  }

  set_pass_error = (result) => {
    this.setState({ pass_error: result })
  }

  render_pass_error = () => {
    if (this.state.pass_error) {
      return (
        <Card>
          <CardContent>
            <h1>INCORRECT PASSWORD</h1>
          </CardContent>
        </Card>
      )
    }
  }

  passChange(event) {
    let pass = event.target.value;

    if (pass !== this.state.password) {
      this.setState({
        password: pass,
      });
    }
  }

  get_password = () => {
    if (this.props.socket) {
      this.props.socket.on("admin_pass", (admin_pass) => {
        this.setState({ short_pass: admin_pass })
        console.log(`Admin Pass: ${admin_pass}`)
      })

      this.props.socket.on("user_details", (details) => {
        console.log(`User Details: ${JSON.stringify(details)}`)

        this.setState({ username: details.username, short_pass: details.local_key })
      })
    }
  }

  submit_user = () => {
    if (this.state.short_pass === this.state.password) {
      console.log(`Data out`);

      let data_out = {
        username: this.props.username,
        password: this.props.short_pass
      }

      this.props.socket.emit("create_user", data_out);


      this.set_pass_error(false);

    } else {
      this.set_pass_error(true);
    }
  }

  register() {
    this.props.socket.emit("new_user");
  }

  componentDidMount() {
    this.get_password();
  }

  render_register = () => {
    return (
      <Card variant="outlined">
        <CardContent>
          <p>To Register:</p>

          <ul>
            <li>1. WRITE DOWN username & password in SECURE place</li>
            <li>2. Enter and submit password </li>
            <li>3. Log in with password</li>
          </ul>

          <Accordion>
            <AccordionSummary>
              <div className="row">
                <Button
                  color="primary"
                  variant="outlined"
                  onClick={() => {
                    console.log("Clicked!");
                    this.register();
                  }}
                  type="button"
                >
                  Register
                </Button>
              </div>
            </AccordionSummary>

            <AccordionDetails>


              <Accordion>
                <AccordionDetails>
                  <Card>
                    <CardContent>
                      <p>Username: {this.props.username}</p>
                      <p>Password: {this.props.admin_pass}</p>
                      <br />

                      <form className="row-centered" style={{ color: "black" }}>
                        <Form.Group>
                          <Form.Control
                            type="manifest_pass"
                            id="Inputcause"
                            placeholder="Enter Above Password"
                            onChange={this.passChange.bind(this)}
                          />
                        </Form.Group>
                        <br />

                        <Button
                          color="primary"
                          variant="outlined"
                          onClick={() => {
                            this.submit_user();
                          }}
                          role="button"
                          tabIndex={0}
                        >
                          Submit
                        </Button>
                      </form>
                      <br />

                      <p>Enter Above Password to register user</p>
                      {this.render_pass_error()}
                    </CardContent>
                  </Card>
                </AccordionDetails>
              </Accordion>
            </AccordionDetails>
          </Accordion>
        </CardContent>
      </Card >
    )
  }

  render() {
    return (
      this.render_register()
    )
  }
}