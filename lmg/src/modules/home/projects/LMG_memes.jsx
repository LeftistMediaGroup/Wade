import React, { Component } from "react";

import { Card, CardContent } from "@mui/material";


export default class LMG_memes extends Component {
  render() {
    return (
      <Card variant="outlined">
        <CardContent>
          <div style={{ textAlign: "center" }}>
            <h4>LMG - Memes</h4>
          </div>

          <div style={{ textAlign: "center" }}>
            A facebook page for sharing leftist propaganda
          </div>
        </CardContent>
      </Card>
    );
  }
}