import React, { Component } from "react";

import { Card, CardContent } from "@mui/material";

import SocialMediaManager from "./positions/SocialMediaManager.jsx";
import Contact from "./Contact.jsx";
import Programmer from "./positions/Programmer.jsx";
import MemeDistributer from "./positions/MemeDistributer.jsx";
import Educate from "./Educate/Educate.jsx";
import Organize from "./Organize/Organize.jsx";
import Resist from "./Resist/Resist.jsx";

export default class Home extends Component {
  render() {
    return (
      <Card variant="outlined">
        <CardContent>
          <div className="page-header">
            <nav aria-label="breadcrumb">
              <ol className="breadcrumb">
                <li className="breadcrumb-item">
                  <a href="!#" onClick={(event) => event.preventDefault()}>
                    LMG
                  </a>
                </li>
                <li className="breadcrumb-item active" aria-current="page">
                  Home
                </li>
              </ol>
            </nav>
          </div>

          <div className="row mt-3">
            <div className="row-centered">

              <h1>Leftist Media Group</h1>

              <p>
                Leftist Media group is dedicated to Organizing the Left and
                starting the Revolution.
              </p>

            </div>
          </div>

          <div className="row">
            <div className="row-centered">
              <Card>
                <CardContent>
                  <h4> Lets Overthrow Capitalism together</h4>

                  <div className="row">
                    <div className="col-sm">
                      <Card variant="outlined">
                        <CardContent>
                          <p>
                            Due to our our fascist leadership path we are nearing the end of our escalating climate catastrophe.
                          </p>
                        </CardContent>
                      </Card>
                    </div>

                    <div className="col-sm">
                      <Card variant="outlined">
                        <CardContent>
                          <p>
                            Overthrowing Capitalism will require a multifaceted
                            attack from every angle possible.
                          </p>
                        </CardContent>
                      </Card>
                    </div>
                  </div>
                </CardContent>
              </Card>
            </div>
          </div>


          <div className="row">
            <div style={{ textAlign: "center" }}>
              <h4>Projects</h4>

              <p>
                All projects are focused on a specific avenue of dismantling
                Capitalism one brick at a time.
              </p>
            </div>

            <div>
              <Educate />
              <Organize />
              <Resist />
            </div>
          </div>

          <Card>
            <CardContent>
              <div className="row">
                <h4>Join Our Team!</h4>

                <div className="col" style={{ padding: 15 }}>
                  <MemeDistributer />
                </div>

                <div className="col" style={{ padding: 15 }}>
                  <Programmer />
                </div>

                <div className="col" style={{ padding: 15 }}>
                  <SocialMediaManager />
                </div>

              </div>
            </CardContent>

            <div className="row-centered">
              <Contact />
            </div>
          </Card>
        </CardContent>
      </Card>
    );
  }
}