import * as React from 'react';
import * as ReactDOM from 'react-dom';
import { render } from 'react-dom'
import { createStore } from 'redux'
import reducers from './Reducers'
import MainComponent from './Components';
import './index.scss'

const initialState: any = {
    sites: [],
    posts: []
};

const store = createStore(reducers, initialState);
const mainComponent = <MainComponent store={store} />
const mainElement: HTMLElement = document.getElementById("root");

ReactDOM.render (
    mainComponent,
    mainElement
);