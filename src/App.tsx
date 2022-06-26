import type { Component } from 'solid-js';

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

    function timer_update() {
        invoke("timer_update").then(data => {
            console.log(data);
        });
    }

    function timer_stop() {
        invoke("timer_stop");
    }

    return (
        <div class={css.app}>
            <div class={css.timer}>
            </div>
            <div class={css.tower}>
            </div>
        </div>
    );
};

export default App;
