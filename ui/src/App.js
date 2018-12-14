import React, { Component } from 'react';
import logo from './logo.svg';
import './App.css';
import HeroList from './HeroList';

class App extends Component {
  state = {
    loading: true,
  };

  componentDidMount() {
    fetch('http://localhost:8000/hero')
      .then(response => response.json())
      .then(heroes => this.setState({ heroes, loading: false }));
  }

  render() {
    const { heroes } = this.state;
    return (
      <div className="App">
        <header className="App-header">
          <img src={logo} className="App-logo" alt="logo" />
          {heroes && <HeroList heroes={heroes} />}
        </header>
      </div>
    );
  }
}

export default App;
