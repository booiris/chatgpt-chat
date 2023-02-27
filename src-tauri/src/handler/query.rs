use crate::model::errors::Result;
use crate::model::interface::{QueryReq, QueryResp};

pub struct QueryHandler {
    req: QueryReq,
}

impl QueryHandler {
    pub fn new(req: QueryReq) -> Self {
        Self { req }
    }
}

impl QueryHandler {
    pub fn handle(&self) -> Result<QueryResp> {
        self.new_resp()
    }

    fn new_resp(&self) -> Result<QueryResp> {
        Ok(QueryResp {
            text: String::from(""),
        })
    }
}
