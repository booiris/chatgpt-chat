use once_cell::sync::OnceCell;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, SyncSender};
use tauri::Manager;

static DEBUG_CHANNEL: OnceCell<SyncSender<String>> = OnceCell::new();

pub fn init_debug_print(app: tauri::AppHandle) {
    let (tx, mut rx) = mpsc::sync_channel::<String>(100);
    DEBUG_CHANNEL.set(tx).unwrap();
    listen_debug_print(app, &mut rx);
}

pub fn debug_print(msg: &str) {
    let tx = DEBUG_CHANNEL.get().unwrap();
    tx.send(msg.to_string()).unwrap();
}

pub fn listen_debug_print(app: tauri::AppHandle, rx: &mut Receiver<String>) {
    while let Ok(msg) = rx.recv() {
        app.emit_all("debug_print", msg).unwrap();
    }
}
