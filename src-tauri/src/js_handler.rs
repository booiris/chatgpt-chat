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
