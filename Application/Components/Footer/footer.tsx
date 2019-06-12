import * as React from 'react';
import './footer.scss';
import { IState } from './types';
import { baseUrl } from '..';

class FooterComponent extends React.Component<any, IState> {

  constructor(props: any) {
    super(props);

    this.state = {
      profile: {
        fullName: ""
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
    return (
      <footer className="footer">
          <h2>{this.state.profile.fullName} &copy;</h2>
      </footer>
    );
  }
}

export default FooterComponent;