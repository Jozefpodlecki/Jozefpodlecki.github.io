import { Button, Card, CardActionArea, CardActions, CardContent, Tab, Tabs, Typography } from "@material-ui/core";
import * as React from "react";
import { baseUrl } from "..";
import "./projects.scss";
import { IState } from "./types";

class ProjectsComponent extends React.Component<any, IState> {
  constructor(props: any) {
    super(props);

    this.state = {
      value: 0,
      tabNames: [],
      projects: [],
      data: []
    };
  }

  componentDidMount() {
    fetch(baseUrl + "./projects.json")
      .then(res => res.json())
      .then(data => {
        this.setState({
          projects: data[0].projects,
          tabNames: data.map(pr => pr.tabName),
          data: data
        });
      });
  }

  handleChange = (event: React.ChangeEvent<{}>, value: number) => {
    let projects = this.state.data[value].projects;

    this.setState({
      value: value,
      projects: projects
    });
  };

  render() {
    return (
      <div className="projectsContainer">
        <Tabs
          value={this.state.value}
          onChange={this.handleChange}
          indicatorColor="primary"
          textColor="primary"
          centered
        >
          {this.state.tabNames.map((pr, index) => {
            return <Tab key={index} label={pr} />;
          })}
        </Tabs>
        <div className="cardContainer">
          {this.state.projects.map((pr, index) => {
            return (
              <Card key={index} className="card">
                <CardActionArea>
                  <CardContent>
                    <Typography gutterBottom variant="h5" component="h2">
                      {pr.name}
                    </Typography>
                    <Typography
                      variant="body2"
                      color="textSecondary"
                      component="p"
                    >
                      {pr.description}
                    </Typography>
                  </CardContent>
                </CardActionArea>
                <CardActions>
                  <Button size="small" color="primary" href={pr.githubLink}>
                    Github
                  </Button>
                </CardActions>
              </Card>
            );
          })}
        </div>
      </div>
    );
  }
}

export default ProjectsComponent;
