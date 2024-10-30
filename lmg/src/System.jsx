import React, { Component } from "react";
import { Form } from "react-bootstrap";
import { Button, Card, CardContent } from "@mui/material";
import { io } from "socket.io-client";


export default class System extends Component {
  constructor(props) {
    super(props);

    this.state = {
      socket_url: "ws://localhost:5501",
    };
  }

  watch_Socket = () => {
    if (this.props.socket) {
      this.props.socket.on("connect", () => {
        console.log(`CONNECTED`)
        this.props.is_database_found(true);

        this.props.socket.emit("database_init");
      })

      this.props.socket.on("admin_pass", (admin_pass) => {
        this.props.set_admin_pass(admin_pass)
        console.log(`Pass: ${admin_pass}`)
      });

      this.props.socket.on("database_init", (value) => {
        console.log(`DatabaseInit: ${value}`)
        if (value == "True") {
          this.props.set_admin_created(true);
          this.props.set_component("Dashboard_Login_Signup")
        } else {
          this.props.set_admin_created(false);
        }
      })
    }
  }

  socket_url_change = (event) => {
    let url = event.target.value;

    if (cause !== this.state.url) {
      this.setState({
        socket_url: url,
      });
      this.props.setSocket(url)
    }
  }

  submit_socket = () => {
    console.log(`Submit`);

    console.log(`Data out`);

    this.props.setSocket(this.state.socket_url)
  }

  render() {
    return (
      <div className="row-centered" style={{ maxWidth: 500 }} >
        <Card>
          <CardContent>
            {this.watch_Socket()}

            <h5>System</h5>

            <h4>Enter Wade URL</h4>

            <form className="row-centered" style={{ color: "black" }}>
              <Form.Group>
                <Form.Control
                  type="URL"
                  id="InputURL"
                  placeholder="ws://localhost:5501"
                  defaultValue="ws://localhost:5501"
                  onChange={() => {
                    this.socket_url_change()
                  }}
                />
              </Form.Group>

              <br />

              <Button
                color="primary"
                variant="outlined"
                onClick={() => {
                  console.log("Clicked!");
                  this.submit_socket();
                }}
                role="button"
                tabIndex={0}
              >
                Connect
              </Button>
            </form>


          </CardContent>
        </Card>
      </div >
    );
  }
}
