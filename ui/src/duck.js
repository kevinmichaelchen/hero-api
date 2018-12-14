import { call, put, takeEvery, fork } from 'redux-saga/effects';

const BASE_API_URL = 'http://localhost:8000';

const Actions = {
  FETCH_HEROES_REQUEST: 'FETCH_HEROES_REQUEST',
  FETCH_HEROES_SUCCESS: 'FETCH_HEROES_SUCCESS',
  FETCH_HEROES_FAILURE: 'FETCH_HEROES_FAILURE',

  CREATE_HERO_REQUEST: 'CREATE_HERO_REQUEST',
  CREATE_HERO_SUCCESS: 'CREATE_HERO_SUCCESS',
  CREATE_HERO_FAILURE: 'CREATE_HERO_FAILURE',
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
  createHeroRequest: hero => ({
    type: Actions.CREATE_HERO_REQUEST,
    payload: hero,
  }),
  createHeroSuccess: () => ({
    type: Actions.CREATE_HERO_SUCCESS,
  }),
  createHeroFailure: error => ({
    type: Actions.CREATE_HERO_FAILURE,
    payload: error,
  }),
};

export const REDUCER_KEY = 'heroes';

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

export default function heroesReducer(state = initialState, action) {
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

    case Actions.CREATE_HERO_REQUEST:
      return {
        ...state,
        loading: true,
        error: null,
      };
    case Actions.CREATE_HERO_SUCCESS:
      return {
        ...state,
        loading: false,
        error: null,
      };
    case Actions.CREATE_HERO_FAILURE:
      return {
        ...state,
        loading: false,
        error: action.payload,
      };
    default:
      return state;
  }
}

class Api {
  static fetchHeroes() {
    return fetch(`${BASE_API_URL}/hero`)
      .then(r => r.json())
      .then(response => ({ response }))
      .catch(error => ({ error }));
  }

  static createHero(h) {
    const body = JSON.stringify(h);
    console.log('body =', body);
    return fetch(`${BASE_API_URL}/hero`, {
      method: 'POST',
      mode: 'cors',
      headers: {
        // "Content-Type": "application/json; charset=utf-8",
        'Content-Type': 'application/json',
      },
      body,
    })
      .then(r => r.json())
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

function* createHero(action) {
  const { payload } = action;
  const { response, error } = yield call(Api.createHero, payload);

  if (response) {
    yield put(ActionCreators.createHeroSuccess(response));
    yield put(ActionCreators.fetchHeroesRequest());
  } else {
    yield put(ActionCreators.createHeroFailure(error));
  }
}

function* createHeroSaga() {
  yield takeEvery(Actions.CREATE_HERO_REQUEST, createHero);
}

export const sagas = [fork(fetchHeroesSaga), fork(createHeroSaga)];
