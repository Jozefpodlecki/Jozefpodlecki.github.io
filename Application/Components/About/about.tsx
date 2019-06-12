import Chart from "chart.js";
import * as React from "react";
import "./about.scss";
import { IState } from "./types";
import { baseUrl } from "..";

Chart.defaults.global.defaultFontFamily = "'Indie Flower', cursive";
Chart.defaults.global.legend.display = false;

class AboutComponent extends React.Component<any, IState> {
  chartReference: React.RefObject<HTMLCanvasElement>;

  constructor(props) {
    super(props);

    this.state = {
      skills: [],
      education: [],
      career: [],
      profile: {
        summary: ""
      }
    };
    
    this.chartReference = React.createRef();
  }

  componentDidMount() {
    fetch(baseUrl + "./skills.json")
      .then(res => res.json())
      .then(data => {
        this.setState({
          skills: data
        });

        this.buildChart();
      });

    fetch(baseUrl + "./education.json")
      .then(res => res.json())
      .then(data => {
        this.setState({
          education: data
        });
      });

    fetch(baseUrl + "./career.json")
      .then(res => res.json())
      .then(data => {
        this.setState({
          career: data
        });
      });

    fetch(baseUrl + "./profile.json")
      .then(res => res.json())
      .then(data => {
        this.setState({
          profile: data
        });
      });
  }

  buildChart() {
    let canvasRenderingContext = this.chartReference.current.getContext("2d");

    let labels = this.state.skills.map(sk => sk.name);
    let data = this.state.skills.map(sk => sk.level);

    var radarChart = new Chart(canvasRenderingContext, {
      type: "radar",
      data: {
        labels: labels,
        datasets: [
          {
            data: data
          }
        ]
      }
    });
  }

  render() {
    return (
      <div className="container">
        <div className="summaryContainer">
          <div className="about">
            <div className="paragraphWrapper">
              <p className="paragraph">{this.state.profile.summary}</p>
            </div>
          </div>
          <div className="skillsRadar">
            <canvas id="chart" ref={this.chartReference} />
          </div>
        </div>
        <div className="summaryContainer">
          <ul className="timeline">
            {this.state.career.map((ca, index) => {
              return (
                <li key={index} className="event" data-date={ca.range}>
                  <h3>
                    {ca.title +
                      ", " +
                      ca.company +
                      ", " +
                      ca.city +
                      ", " +
                      ca.country}
                  </h3>
                  {ca.responsibilities.map((re, nindex) => (
                    <p key={"x" + nindex}>{re}</p>
                  ))}
                </li>
              );
            })}
          </ul>
        </div>
        <div>
          <ul>
            <li />
          </ul>
        </div>
      </div>
    );
  }
}

export default AboutComponent;
