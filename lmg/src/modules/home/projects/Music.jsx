import React, { Component } from "react";
import SpotifyPlayer from 'react-spotify-player';

import { Card, CardContent } from "@mui/material";


export default class Music extends Component {
  constructor(props) {
    super(props);

    this.state = {};
  }

  render() {
    return (
      <Card variant="outlined">
        <CardContent>
          <div style={{ textAlign: "center" }}>
            <h4>Leftism Through Music</h4>
          </div>

          <p>
            This course will provide listeners with a playlist
            of 80+ downloadable leftist songs to listen to and
            reflect on centered around leftist topics.
          </p>
          <br />
          
          <div className="row">
            <div className="col mx-auto" align="center" >

              <SpotifyPlayer
                uri="https://open.spotify.com/playlist/0Of0TS1QSKmeb7G8DrR40x?si=8d75011cd7ee427f"
                size={{
                  width: '90%',
                  height: 700,
                }}
                view={'list'}
                theme={'black'}
              />
            </div>
          </div>
          <a href="https://open.spotify.com/playlist/0Of0TS1QSKmeb7G8DrR40x?si=276bb70e04c94dda" target="_blank">Click here for Spotify playlist!</a>
        </CardContent>
      </Card>
    );
  }
}