import { Avatar, Grid } from "@material-ui/core";
import * as React from "react";
import Typist from "react-typist";
import "./home.scss";
import { IState } from "./types";
import { baseUrl } from "..";

class HomeComponent extends React.Component<any, IState> {
  constructor(props: any) {
    super(props);

    this.state = {
      profile: {
        fullName: "",
        avatarUrl: ""
      }
    };
  }

  componentDidMount() {
    fetch(baseUrl + "./profile.json")
      .then(res => res.json())
      .then(data => {
        this.setState({
          profile: data
        });
      });
  }

  render() {
    let titles: any = <div />;

    if (this.state.profile.titles) {
      titles = this.state.profile.titles.map((tit, index) => {
        return (
          <span key={index}>
            <h1>{tit}</h1>
            <Typist.Backspace count={tit.length} delay={1000} />
          </span>
        );
      });

      titles = (
        <Typist avgTypingDelay={50} startDelay={1000}>
          {titles}
          <h1>{this.state.profile.startTitle}</h1>
        </Typist>
      );
    }

    return (
      <div className="main-container">
        <Grid className="avatar">
          <Avatar
            alt={this.state.profile.fullName}
            src={this.state.profile.avatarUrl}
          />
          <div className="banner">{titles}</div>
        </Grid>
      </div>
    );
  }
}

export default HomeComponent;
