use crate::model::errors::Result;
use crate::{
    handler::query::QueryHandler,
    model::interface::{QueryReq, QueryResp},
};

#[tauri::command]
pub fn query(req: QueryReq) -> Result<QueryResp> {
    QueryHandler::new(req).handle()
}
