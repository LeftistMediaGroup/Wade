import React, { Component } from "react";

import { Card, CardContent } from "@mui/material";

import Overview from "/Photos/Sophia/Overview.png";


export default class Sophia extends Component {
  render() {
    return (
      <Card variant="outlined" style={{ textAlign: "center" }}>
        <CardContent>
          <h4>Sophia</h4>

          <p>Ask Leftist questions to be answered, discussed, or debated.</p>

          <div className="Row">
            <p>Sophia is based on the Kialo Software</p>

            <p>
              Source: https://www.kialo.com
            </p>

            <div>
              <p>First, A Premise is made</p>
              <img src={Overview} style={{ maxHeight: "100%", maxWidth: "100%" }} />

            </div>
          </div>
        </CardContent>
      </Card>
    );
  }
}