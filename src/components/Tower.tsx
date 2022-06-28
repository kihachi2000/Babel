import {Component, createSignal, onCleanup} from "solid-js";

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

        return (
            <>
                {
                    data().map((step) => (
                        <div class={css.grid}>
                            {
                                step.map(({date, color_num}) => (
                                    <div
                                        class={css.brick}
                                        title={date}
                                        style={{"background-color": colors[color_num % colors.length]}}
                                    ></div>
                                ))
                            }
                        </div>
                    ))
                }
            </>
        );
    }

    onCleanup(() => clearInterval(interval));

    return (
        <>
            <div class={css.count_label}>{data().reduce((acc, x) => acc + x.length, 0)}</div>
            <div>
                {tower_grid()}
            </div>
        </>
    );
}

export default Tower;
