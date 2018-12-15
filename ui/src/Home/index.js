import React from 'react';
import { connect } from 'react-redux';
import { createStructuredSelector } from 'reselect';
import { bindActionCreators } from 'redux';
import {
  Selectors as HeroesSelectors,
  ActionCreators as HeroesActionCreators,
} from '../duck';
import HeroList from '../HeroList';
import { Link } from 'react-router-dom';

class Home extends React.Component {
  componentDidMount() {
    const {
      actions: { fetchHeroesRequest },
    } = this.props;
    fetchHeroesRequest();
  }

  render() {
    const { heroes, loading, error } = this.props;
    console.log('heroes =', heroes);
    return (
      <div>
        <Link to="/create">Create New Hero</Link>
        {/*<button onClick={() => console.log('click')}>Create New Hero</button>*/}
        {heroes && <HeroList heroes={heroes} />}
      </div>
    );
  }
}

const mapStateToProps = createStructuredSelector({
  loading: HeroesSelectors.loadingSelector,
  error: HeroesSelectors.errorSelector,
  heroes: HeroesSelectors.heroesSelector,
});

const mapDispatchToProps = dispatch => ({
  actions: bindActionCreators(
    {
      fetchHeroesRequest: HeroesActionCreators.fetchHeroesRequest,
    },
    dispatch
  ),
});

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(Home);
