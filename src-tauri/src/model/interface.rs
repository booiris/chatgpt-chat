use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryReq {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResp {
    pub text: String,
}
