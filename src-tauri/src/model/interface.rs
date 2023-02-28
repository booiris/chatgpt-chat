use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryReq {
    pub text: String,
}

#[cfg(test)]
#[test]
fn query_req_test() {
    let a = QueryReq {
        text: "Hello World".to_string(),
    };
    println!("{}", serde_json::to_string(&a).unwrap());
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResp {
    pub text: String,
}
