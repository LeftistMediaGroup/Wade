import React, { Component } from "react";

import { Card, CardContent } from "@mui/material";

import MoodChart from "./MoodChart.jsx";
import MoodSelector from "./MoodSelector.jsx";
import Tasks from "./Tasks/Tasks.jsx";
import TextArea from "./TextArea/TextArea.jsx";
import strftime from "strftime";



var dateOptions = {
  weekday: "long",
  year: "2-digit",
  month: "2-digit",
  day: "2-digit",
  hour12: false,
};


export default class Day extends Component {
  constructor(props) {
    super(props);

    this.state = {
      options: {
        animationEnabled: true,
        exportEnabled: true,
        theme: "light2", // "light1", "dark1", "dark2"
        title: {
          text: "Moods",
        },
        axisY: {
          title: "Mood Val",
          suffix: "%",
        },
        axisX: {
          title: "Time",
          prefix: "T",
          interval: 2,
        },
        data: [
          {
            type: "line",
            toolTipContent: "Week {x}: {y}%",
            dataPoints: [{ x: new Date(), y: 64 }],
          },
        ],
      },
      tasks: {
        newTaskName: "",
        newSubTaskName: "",
      },
      taskList: {},
      entry: "",
    };
  }

  updateEntry = (entry, socket) => {
    if ((entry !== this.state.entry && entry !== "Not Initalized")) {
      this.setState({ entry: entry })

      socket.emit("TextArea_Entry", ({
        entry: entry,
        date: strftime("%y%m%d", this.props.day)
      }));
    }
  }


  
  newTaskNameChange = (TaskName) => {
    this.setState({ tasks: { newTaskName: TaskName } });
  };

  newSubTaskNameChange = (TaskName) => {
    this.setState({ tasks: { newSubTaskName: TaskName } });
  };

  submit = () => {
    console.log(`Submit: ${JSON.stringify(this.props.tasks, null, 2)}`);
   
  };

  setTaskList = (taskList) => {
    this.setState({ taskList: taskList });
  };

  render() {
    return (
      <>
        <Card variant="outlined">
          <CardContent>
            <h1>Day {this.props.day.toLocaleDateString(dateOptions)}</h1>
            <MoodSelector getMoodChart={this.getMoodChart} />
            <MoodChart getMoodChart={this.getMoodChart} options={this.state.options} />
            <TextArea entry={this.state.entry} updateEntry={this.updateEntry} day={this.props.day} getMoodChart={this.getMoodChart} username={this.props.username} />

            <Tasks day={this.props.day} tasks={this.state.tasks} newTaskNameChange={this.newTaskNameChange} taskList={this.state.taskList} setTaskList={this.setTaskList} submit={this.submit} subTaskSubmit={this.subTaskSubmit} newSubTaskNameChange={this.newSubTaskNameChange} />
          </CardContent>
        </Card>
      </>
    );
  }
}
