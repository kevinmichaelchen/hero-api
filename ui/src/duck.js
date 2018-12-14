import { call, put, takeEvery, fork } from 'redux-saga/effects';

const BASE_API_URL = 'http://localhost:8000';

const Actions = {
  FETCH_HEROES_REQUEST: 'FETCH_HEROES_REQUEST',
  FETCH_HEROES_SUCCESS: 'FETCH_HEROES_SUCCESS',
  FETCH_HEROES_FAILURE: 'FETCH_HEROES_FAILURE',
};

export const ActionCreators = {
  fetchHeroesRequest: () => ({
    type: Actions.FETCH_HEROES_REQUEST,
  }),
  fetchHeroesSuccess: heroes => ({
    type: Actions.FETCH_HEROES_SUCCESS,
    payload: heroes,
  }),
  fetchHeroesFailure: error => ({
    type: Actions.FETCH_HEROES_FAILURE,
    payload: error,
  }),
};

export const REDUCER_KEY = 'hero';

export const Selectors = {
  loadingSelector: state => state[REDUCER_KEY].loading,
  heroesSelector: state => state[REDUCER_KEY].heroes,
  errorSelector: state => state[REDUCER_KEY].error,
};

const initialState = {
  loading: false,
  heroes: null,
  error: null,
};

export default function auth(state = initialState, action) {
  switch (action.type) {
    case Actions.FETCH_HEROES_REQUEST:
      return {
        ...state,
        loading: true,
        error: null,
      };
    case Actions.FETCH_HEROES_SUCCESS:
      return {
        ...state,
        loading: false,
        error: null,
        heroes: action.payload,
      };
    case Actions.FETCH_HEROES_FAILURE:
      return {
        ...state,
        loading: false,
        heroes: null,
        error: action.payload,
      };
    default:
      return state;
  }
}

class Api {
  static fetchHeroes() {
    return fetch(`${BASE_API_URL}/hero`)
      .then(response => response.json())
      .then(response => ({ response }))
      .catch(error => ({ error }));
  }
}

function* fetchHeroes() {
  const { response, error } = yield call(Api.fetchHeroes);

  if (response) {
    yield put(ActionCreators.fetchHeroesSuccess(response));
  } else {
    yield put(ActionCreators.fetchHeroesFailure(error));
  }
}

function* fetchHeroesSaga() {
  yield takeEvery(Actions.FETCH_HEROES_REQUEST, fetchHeroes);
}

export const sagas = [fork(fetchHeroesSaga)];
