import { Button, CssBaseline, Toolbar } from "@material-ui/core";
import AppBar from "@material-ui/core/AppBar";
import * as React from "react";
import { Link } from "react-router-dom";
import { baseUrl } from "..";
import "./navigation.scss";
import { IState } from "./types";

class NavigationComponent extends React.Component<any, IState> {
  constructor(props: any) {
    super(props);

    this.state = {
      profile: {
        fullName: ""
      },
      sites: []
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

    fetch(baseUrl + "./navigation.json")
      .then(res => res.json())
      .then(data => {
        this.setState({
          sites: data
        });
      });
  }

  render() {
    return (
      <div className="navigation-container">
        <CssBaseline />
        <AppBar position="static" className="header">
          <Toolbar>
            <h5 className="title">{this.state.profile.fullName}</h5>
            {this.state.sites.map((si, index) => (
              <Button key={index} className="menuItem">
                <Link to={si.path}>{si.displayName}</Link>
              </Button>
            ))}
          </Toolbar>
        </AppBar>
      </div>
    );
  }
}

export default NavigationComponent;
