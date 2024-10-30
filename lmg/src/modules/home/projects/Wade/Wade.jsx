import React, { Component } from "react";
import { Doughnut } from "react-chartjs-2";
import { Card, CardContent, rgbToHex } from "@mui/material";

import StreamList from "/Photos/StreamList.png";
import Streams from "/Photos/Streams.png"
import Comms from "/Photos/Comms.png"
import CommSec from "/Photos/CommSec.png"
import Info from "/Photos/Info.png"
import Map from "/Photos/Map.jpg"
import SatMap from "/Photos/SatMap.png"
import Medical from "/Photos/Medical.png"
import Forehead from "/Photos/Forehead.png"



export class Wade extends Component {
  constructor(props) {
    super(props);

    this.state = {
      WadeData: {
        datasets: [
          {
            data: [50, 50],
            backgroundColor: [
              "rgba(255, 99, 132, 0.5)",
              "rgba(54, 162, 235, 0.5)",
              "rgba(255, 206, 86, 0.5)",
              "rgba(75, 192, 192, 0.5)",
              "rgba(153, 102, 255, 0.5)",
              "rgba(255, 159, 64, 0.5)",
            ],
            borderColor: [
              "rgba(255,99,132,1)",
              "rgba(54, 162, 235, 1)",
              "rgba(255, 206, 86, 1)",
              "rgba(75, 192, 192, 1)",
              "rgba(153, 102, 255, 1)",
              "rgba(255, 159, 64, 1)",
            ],
          },
        ],
        labels: ["Software 50%", "Personnel 50%"],
      },
      WadeOptions: {
        responsive: true,
        animation: {
          animateScale: true,
          animateRotate: true,
        },
      },
    };
  }
  render() {
    return (
      <Card variant="outlined">
        <CardContent>
          <div style={{ textAlign: "center" }}>
            <h4>Wade</h4>
            <h5>They’re already organized and watching, why aren’t we?</h5>
            <br />

            <img src={Info} style={{ maxHeight: "100%", maxWidth: "100%" }} />
          </div>

          <div style={{ textAlign: "center" }}>
            Community response system made to assist with the development,
            design, and organization of protests
          </div>
          <br />

          <div className="row" style={{ textAlign: "center" }}>
            <p>Wade is the mixture of Software and personnel. </p>
            <br />

            <div className="col vh-100 d-flex justify-content-center align-items-center" style={{ textAlign: "center", maxHeight: 450 }}>
              <br />
              <br />
              <Doughnut
                data={this.state.WadeData}
                options={this.state.WadeOptions}
              />
            </div>
          </div>
          <br />
          <br />
          <br />

          <div className="row">

            <div className="col">
              <Card variant="outlined" style={{ backgroundColor: "#36a1eb" }}>
                <CardContent>
                  <h5>Software</h5>

                  <p>The software element for Wade consists of:</p>
                  <Card>
                    <CardContent>


                      <ul>
                        <li>
                          Sending and viewing streams, encrypted chat, and a viewable library for education.
                        </li>

                        <li>
                          An Android app that does all that plus it can send your stream to organizers to view
                        </li>

                        <li>
                          A server for managing chats, streams, a library, and all other data.                </li>
                      </ul>

                      <h5>Software Tools:</h5>
                      <br />

                      <img src={Comms} style={{ maxHeight: "100%", maxWidth: "100%" }} />


                      <ul>
                        <li>
                          Communication

                          Encrypted communication channels to organize that evolve and collapse as needed
                          <br />

                          <img src={CommSec} style={{ maxHeight: "100%", maxWidth: "100%" }} />


                          <ul>
                            <li>
                              Medics
                              <br />

                              <img src={Medical} style={{ maxHeight: "100%", maxWidth: "100%" }} />

                            </li>
                            <li>
                              Security
                            </li>
                            <li>
                              Home Support
                            </li>
                            <li>
                              City Groups
                            </li>
                          </ul>
                        </li>
                        <br />

                        <li>
                          Maps

                          Mark and view points of intrest
                          <br />

                          <img src={SatMap} style={{ maxHeight: "100%", maxWidth: "100%" }} />

                          <ul>
                            <li>
                              Police
                            </li>
                            <li>
                              Medics
                            </li>
                            <li>
                              Danger Areas
                            </li>
                            <br />

                            <img src={Map} style={{ maxHeight: "100%", maxWidth: "100%" }} />

                          </ul>
                        </li>
                        <br />

                        <li>
                          Education

                          A collection of resources for people to know their rights, history about the movement and links to other helpful resources
                        </li>
                        <br />

                        <li>
                          Livestream phone as bodycamera
                          <br />

                          <img src={StreamList} style={{ maxHeight: "100%", maxWidth: "100%" }} />
                          <img src={Streams} style={{ maxHeight: "100%", maxWidth: "100%" }} />

                          <ul>
                            <li>
                              Sources come from all over the internet
                            </li>
                            <li>
                              Sources can steam directly to wade
                            </li>

                            <li>
                              Verbal Coordination from admin team
                            </li>
                          </ul>
                        </li>
                        <br />

                        <li>
                          I need a medic button

                          GPS sync between you and a medic
                        </li>
                        <br />

                        <li>
                          I’m being pulled over button

                          Connect you to home support and record everything
                        </li>
                        <br />

                        <li>
                          Fund Raising

                          Raise funds for direct payments to those in need
                        </li>
                        <br />

                        <li>
                          Audio sync

                          Sync event speakers to everyone's phones
                        </li>
                      </ul>
                    </CardContent>
                  </Card>
                </CardContent>
              </Card>
            </div>

            <div className="col">
              <Card variant="outlined" style={{ backgroundColor: "#ff6483" }}>
                <CardContent>
                  <h5>Personnel</h5>
                  <br />
                  <img src={Forehead} style={{ maxHeight: "100%", maxWidth: "100%" }} />


                  <p>Personnel for Wade is divided into two groups.</p>

                  <div className="row">
                    <div className="col">
                      <Card variant="outlined">
                        <CardContent>
                          <h4>Boots on the Ground</h4>
                          <br />
                          <p>The eyes, ears and heart of the protest</p>

                          <ul>
                            <li>
                              <h4>Streamers</h4>
                              Streamers can either stream indirectly to Twitch, Facebook or Youtube then the streams will be replayed on LeftistMediaGroup.com or they can stream directly to the LMG server to then be displayed on the website show the world what is going on
                              Streamers streaming indirectly get all the credit for their work as each stream is clearly labeled and linked to owner
                              Streamers streaming via the (open source) LMG app can choose to be anonymous.

                            </li>

                            <li>
                              <h4>Security</h4>
                              Security members will be the front line of defence between the police or other bad actors such as the proud boys and other white supremacists and the main group of protestors. They will train virtually and their main goal is to keep other protestors safe from police violence. Tactics are mainly consolidated from the Hong Kong protests like using umbrellas and homemade shields to form a shield wall and using thermoses and leaf blowers to neutralize tear gas. (https://en.wikipedia.org/wiki/Tactics_and_methods_surrounding_the_2019%E2%80%9320_Hong_Kong_protests)
                              Security will scout for police along the march route and will direct the flow of the protest to avoid police if possible and prevent kettling.

                            </li>

                            <li>
                              <h4>Suppliers</h4>
                              Suppliers will be focused on moving supplies through a protest. Some of these supplies can include masks, goggles, shields, umbrellas, first aid equipment and water

                            </li>
                          </ul>
                        </CardContent>
                      </Card>
                    </div>

                    <div className="col">
                      <Card variant="outlined">
                        <CardContent>
                          <h4> Home support </h4>
                          <br />

                          <p>General Helpline, manages communications, supplies, personnel and livestream</p>

                          <p>Made up of people with internet access that can help the boots on the ground from home (anywhere in the world).</p>

                          <ul>
                            <li>
                              <h4>Boots on the Ground support</h4>
                              On the ground support manages contacting emergency contacts in case of a medical emergency or arrest. They will also act as a general helpline for protestors looking for more information about the movement or help with finding a medic.
                              Ideal for people that can’t make it to the protests but want to help
                              Other tasks that they may do is manage and update live maps, communications, and tasking security personnel.
                            </li>

                            <li>
                              <h4>Stream Manager</h4>
                              The Stream Manager operates the live streaming software allowing for camera angles from across the country.
                            </li>

                            <li>
                              <h4>Community Outreach</h4>
                              Community outreach finds protestors and streamers that are expected to be protesting and start coordinating efforts.
                            </li>
                            <li>
                              <h4>Researchers</h4>
                              Researchers find information on the movement, community support, and compile them for people to understand on the website.
                            </li>

                            <li>
                              <h4>Organizers</h4>
                              Organizers will plan the events, logistics, speakers, security concerns ect.
                            </li>
                          </ul>
                        </CardContent>
                      </Card>
                    </div>
                  </div>
                </CardContent>
              </Card>
            </div>
          </div>
        </CardContent>
      </Card >
    );
  }
}

export default Wade;
