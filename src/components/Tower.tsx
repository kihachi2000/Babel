import {Component, createSignal, onCleanup, For, Show} from "solid-js";

import {invoke} from "@tauri-apps/api/tauri";

import css from "../styles/Tower.module.css";

type BrickData = {date: string, color_num: number};

const Tower: Component = () => {
    const colors = ["#d0c91f", "#626262", "#008bba", "#df514c"];
    const [data, setData] = createSignal<Array<Array<BrickData>>>([]);

    function get_data() {
        invoke<Array<Array<BrickData>>>("get_data").then(data => {
            setData(data);
        });
    }

    get_data();
    const interval = setInterval(get_data, 60000);

    function tower_grid() {
        const data_len = data().length;

        return (
            <div>
                <For each={data()}>
                    {(step, index) => (
                        <>
                            <Show when={(data_len - index()) % 4 === 0}>
                                <div class={css.line}>
                                    <p class={css.line_num}>{(data_len - index()) * 5}</p>
                                </div>
                            </Show>
                            <div class={css.grid}>
                                <For each={step}>
                                    {({date, color_num}) => (
                                        <div 
                                            class={css.brick}
                                            title={date}
                                            style={{"background-color": colors[color_num % colors.length]}}
                                        ></div>
                                    )}
                                </For>
                            </div>
                        </>
                    )}
                </For>
            </div>
        );
    }

    onCleanup(() => clearInterval(interval));

    return (
        <>
            <div class={css.count_label}>{data().reduce((acc, x) => acc + x.length, 0)}</div>
            {tower_grid()}
        </>
    );
}

export default Tower;
