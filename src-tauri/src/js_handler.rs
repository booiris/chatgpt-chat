use tauri::Manager;

use crate::dal::openai_api::init::OpenaiApi;
use crate::model::errors::ResultWarp;
use crate::{
    handler::query::QueryHandler,
    model::interface::{QueryReq, QueryResp},
};

#[tauri::command]
pub async fn query(req: QueryReq) -> ResultWarp<QueryResp> {
    Ok(QueryHandler::new(req, OpenaiApi::new()).handle().await?)
}

#[tauri::command]
pub async fn init_backend(app_handle: tauri::AppHandle) {
    if let Err(err) = crate::init(app_handle).await {
        println!("{:?}", err);
        crate::dal::debug_print::init::debug_print(&format!("{:?}", err));
    } else {
        log::info!("init_backend success");
        crate::dal::debug_print::init::debug_print("init_backend success");
    }
}

#[tauri::command]
pub fn get_dir(app_handle: tauri::AppHandle) {
    let mut a = std::collections::HashMap::<String, String>::new();
    a.insert(
        "cache".into(),
        app_handle
            .path_resolver()
            .app_cache_dir()
            .unwrap_or_default()
            .display()
            .to_string(),
    );
    a.insert(
        "config".into(),
        app_handle
            .path_resolver()
            .app_config_dir()
            .unwrap_or_default()
            .display()
            .to_string(),
    );
    a.insert(
        "data".into(),
        app_handle
            .path_resolver()
            .app_data_dir()
            .unwrap_or_default()
            .display()
            .to_string(),
    );
    a.insert(
        "local".into(),
        app_handle
            .path_resolver()
            .app_local_data_dir()
            .unwrap_or_default()
            .display()
            .to_string(),
    );
    a.insert(
        "resource".into(),
        app_handle
            .path_resolver()
            .resource_dir()
            .unwrap_or_default()
            .display()
            .to_string(),
    );
    a.insert(
        "log".into(),
        app_handle
            .path_resolver()
            .app_log_dir()
            .unwrap_or_default()
            .display()
            .to_string(),
    );
    app_handle.emit_all("get_dir", a).unwrap();
}
