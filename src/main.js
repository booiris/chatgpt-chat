import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";

import { debug_msg_data } from "./App.vue";
import { invoke } from '@tauri-apps/api/tauri';
import { emit, listen } from '@tauri-apps/api/event'
async function init_listen() {
    await listen('debug_print', (event) => {
        console.log(event)
        debug_msg_data.push(event.payload)
    })
    invoke("init_backend")
}
init_listen().then(() => { createApp(App).mount("#app"); })


