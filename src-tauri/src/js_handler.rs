use tauri::Manager;

use crate::dal::openai_api::init::OpenaiApi;
use crate::model::errors::ResultWarp;
use crate::{
    handler::query::QueryHandler,
    model::interface::{QueryReq, QueryResp},
};

#[tauri::command]
pub async fn query(req: QueryReq) -> ResultWarp<QueryResp> {
    let mut resp = QueryHandler::new(req, OpenaiApi::new()).handle().await?;
    let now = chrono::Utc::now().timestamp();
    resp.time = now;
    Ok(resp)
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
