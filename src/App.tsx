import type { Component } from 'solid-js';

import Timer from './components/Timer';
import Tower from "./components/Tower";

import "./styles/globals.css";
import css from './styles/App.module.css';

import {invoke} from "@tauri-apps/api/tauri";

const App: Component = () => {

    function get_data() {
        invoke("get_data").then(data => {
            console.log(data);
        });
    }

    function add_brick() {
        invoke("add_brick");
    }

    function timer_stop() {
        invoke("timer_stop");
    }

    return (
        <div class={css.app}>
            <div class={css.timer}>
                <Timer />
            </div>
            <div class={css.tower}>
                <Tower />
            </div>
        </div>
    );
};

export default App;
