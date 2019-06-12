import { Provider } from "react-redux";
import { BrowserRouter as Router, Route, Switch } from "react-router-dom";
import AboutComponent from "./About/about";
import ContactComponent from "./Contact/contact";
import FooterComponent from "./Footer/footer";
import HomeComponent from "./Home/home";
import NavigationComponent from "./Navigation/navigation";
import ProjectsComponent from "./Projects/projects";
import React = require("react");

export const baseUrl: string = (() => {
  if (typeof window !== "undefined") {
    return location.protocol + "//" + location.host + "/";
  }

  return "";
})();

class MainComponent extends React.Component<{ store: any }, {}> {
  constructor(props: any) {
    super(props);
  }

  render() {
    return (
      <Provider store={this.props.store}>
        <Router>
          <NavigationComponent />
          <Switch>
            <Route exact path="/" component={HomeComponent} />
            <Route exact path="/about" component={AboutComponent} />
            <Route path="/contact" component={ContactComponent} />
            <Route path="/projects" component={ProjectsComponent} />
          </Switch>
        </Router>
        <FooterComponent />
      </Provider>
    );
  }
}

export default MainComponent;
