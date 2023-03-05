use mongodb::{bson::doc, options::ClientOptions, Client};
use once_cell::sync::OnceCell;
use openai_api::api::ChatRole;

use crate::model::{
    errors::{Error, ResultWarp},
    msg_info::MsgInfo,
};

static MONGODB_CLIENT: OnceCell<Client> = OnceCell::new();

const USERNAME: &str = "";
const PASSWORD: &str = "";
const CLUSTER: &str = "";

pub async fn init_mongodb() -> ResultWarp<()> {
    let client_options = ClientOptions::parse(format!(
        "mongodb+srv://{}:{}@{}.kuc6uor.mongodb.net/?retryWrites=true&w=majority",
        USERNAME, PASSWORD, CLUSTER
    ))
    .await?;
    let client = Client::with_options(client_options)?;
    MONGODB_CLIENT.set(client).unwrap();
    Ok(())
}

pub struct MongoDBClient {}

impl MongoDBClient {
    pub fn new() -> Self {
        Self {}
    }
}

impl MongoDBClient {
    pub async fn insert_one_msg(
        sender_id: i64,
        receiver_id: i64,
        chat_role: ChatRole,
        msg: String,
        time: i64,
    ) -> ResultWarp<()> {
        let client = MONGODB_CLIENT
            .get()
            .ok_or(Error::Any("MongoDB client not initialized".to_string()))?;
        let db = client.database("unencrypted_chat");
        let coll = db.collection::<MsgInfo>("msg");
        coll.insert_one(
            MsgInfo {
                _id: None,
                sender_id,
                receiver_id,
                chat_role,
                content: msg,
                time,
            },
            None,
        )
        .await?;
        let data = coll.find_one(doc! {"sender_id": sender_id}, None).await?;
        log::info!("{:?}", data);
        Ok(())
    }
}
