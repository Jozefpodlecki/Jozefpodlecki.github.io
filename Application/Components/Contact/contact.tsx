import * as React from "react";
import "./contact.scss";
import { IState } from "./types";
import { baseUrl } from "..";

class ContactComponent extends React.Component<any, IState> {
  constructor(props: any) {
    super(props);

    this.state = {
      profile: {
        address: ""
      },
      links: []
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

    fetch(baseUrl + "./links.json")
      .then(res => res.json())
      .then(data => {
        this.setState({
          links: data
        });
      });
  }

  render() {
    let parsedLinks = this.state.links.map((res, index) => {

      if (res.src) {
        return (
          <div key={index}>
            <a className="contactLink" href={res.url} target="_blank">
              <img className="contactImage" src={res.src} />
            </a>
          </div>
        );
      }

      return (
        <div key={index}>
          <a className="contactLink" href={res.url} target="_blank">
            <i className={res.className} />
          </a>
        </div>
      );
    });

    return (
      <div className="container">
        <div className="subContainer">
          <i className="fas fa-map-marker-alt fa-10x" />
          <p className="address">{this.state.profile.address}</p>
        </div>
        <div className="subContainer">
          <div className="headline">
            <h1>You can find me at</h1>
          </div>
          <div className="links">{parsedLinks}</div>
        </div>
      </div>
    );
  }
}

export default ContactComponent;
