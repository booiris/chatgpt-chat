use crate::dal::openai_api::init::OpenaiApi;
use crate::model::errors::ResultWarp;
use crate::model::interface::{QueryReq, QueryResp};

pub struct QueryHandler {
    req: QueryReq,
    open_ai_api: OpenaiApi,
}

impl QueryHandler {
    pub fn new(req: QueryReq, open_ai_api: OpenaiApi) -> Self {
        Self { req, open_ai_api }
    }
}

impl QueryHandler {
    pub async fn handle(&mut self) -> ResultWarp<QueryResp> {
        Ok(self.new_resp().await?)
    }

    async fn new_resp(&mut self) -> ResultWarp<QueryResp> {
        Ok(QueryResp {
            text: self.open_ai_api.chat(&self.req.text).await?,
            time: 0,
        })
    }
}
