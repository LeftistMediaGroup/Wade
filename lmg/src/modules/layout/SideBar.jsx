import { Button, Card, CardContent } from "@mui/material";
import React from "react";

export default class SideBar extends React.Component {
  constructor(props) {
    super(props);

    this.state = {};
  }

  renderAccount = () => {
    if (this.props.is_loggedin === true) {
      return (
        <Button
          onClick={() => {
            if (this.state) {
              this.props.getComponent("Dashboard");
            }
          }}
        >
          Dashboard
        </Button>
      );
    } else {
      return (
        <>
          <div className="row ">
            <Button
              onClick={() => {
                if (this.state) {
                  this.props.getComponent("SignUp");
                }
              }}
            >
              {" "}
              SignUp
            </Button>
          </div>

          <div className="row ">
            <Button
              onClick={() => {
                if (this.state) {
                  this.props.getComponent("LogIn");
                }
              }}
            >
              {" "}
              Login
            </Button>
          </div>
        </>
      );
    }
  };

  renderSystem = () => {
    if (this.props.is_database_found === false) {
      return (
        <Button
          onClick={() => {
            if (this.state) {
              this.props.getComponent("System");
            }
          }}
        >
          System
        </Button>
      );
    } else if (this.props.is_database_found === true) {
      if (this.props.is_admin_created === true) {
        return (
          this.renderAccount()
        )
      } else if (this.props.is_admin_created === false) {
        return (
          <Button
            onClick={() => {
              if (this.state) {
                this.props.getComponent("RegisterAdmin");
              }
            }}
          >
            Register Admin
          </Button>
        )

      }
    }

  };

  renderAdminDashboard = () => {
    if (this.props.is_admin === true) {
      return (
        <div className="row ">
          <Button
            onClick={() => {
              if (this.state) {
                this.props.getComponent("AdminDashboard");
              }
            }}
          >
            Admin Dashboard
          </Button>
        </div>
      );
    }
  };

  render() {
    return (
      <Card variant="outlined">
        <CardContent>
          <Button
            onClick={() => {
              if (this.state) {
                this.props.getComponent("Home");
              }
            }}
          >Home
          </Button>
          <Button
            onClick={() => {
              if (this.state) {
                this.props.getComponent("Education");
              }
            }}
          >
            Education
          </Button>
          <Button
            onClick={() => {
              if (this.state) {
                this.props.getComponent("Music");
              }
            }}
          >
            Music
          </Button>

          <Button
            onClick={() => {
              if (this.state) {
                this.props.getComponent("Resources");
              }
            }}
          >
            Resources
          </Button>

          {this.renderAdminDashboard()}

          <Card variant="outlined">
            <CardContent>
              Wade
              <br />

              {this.renderSystem()}
            </CardContent>
          </Card>
        </CardContent>
      </Card>
    );
  }
}
