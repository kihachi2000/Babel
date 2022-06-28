import {Component, createSignal, Switch, Match} from "solid-js";

import {invoke} from "@tauri-apps/api/tauri";

import {SVGStart, SVGStop} from "../components/SVG";

import css from "../styles/Timer.module.css";

type TimerState = "Running" | "Stopping";

const Timer: Component = () => {
    const [remain, setRemain] = createSignal(0);
    const [state, setState] = createSignal<TimerState>("Stopping");

    let interval: number | null = null;

    invoke<number>("timer_sync").then(data => {
        setRemain(data);
    });

    function timer_update() {
        invoke<{next: boolean, remain: number}>("timer_update").then(data => {
            console.log(data);
            setRemain(data.remain);

            if (data.next) {
                // ストップ
                setState("Stopping");
                clearInterval(interval!);
                interval = null;
                invoke("timer_stop");

                // 効果音
                const music = new Audio("/end.mp3");
                music.volume = 0.3;
                music.muted = false;
                music.play();
            }
        });
    }

    function onClickHandler() {
        if (interval === null) {
            // スタート
            setState("Running");
            interval = setInterval(timer_update, 125);
        } else {
            // ストップ
            setState("Stopping");
            clearInterval(interval);
            interval = null;
            invoke("timer_stop");
        }
    }

    function remain_label() {
        const min = `0${Math.floor(remain() / 60)}`.slice(-2);
        const sec = `0${remain() % 60}`.slice(-2);

        return <p class={css.remain_label}>{`${min}:${sec}`}</p>;
    };

    return (
        <div class={css.wrapper}>
            {remain_label()}

            <Switch>
                <Match when={state() === "Running"}>
                    {SVGStop(css.stop, onClickHandler)}
                </Match>
                <Match when={state() === "Stopping"}>
                    {SVGStart(css.start, onClickHandler)}
                </Match>
            </Switch>
        </div>
    );
}

export default Timer;
