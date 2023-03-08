use once_cell::sync::OnceCell;
use openai_api::api::ChatRole;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

use crate::{
    dal::debug_print::init::debug_print,
    model::errors::{Error, ResultWarp},
};

static MYSQLDB_CLIENT: OnceCell<Pool<MySql>> = OnceCell::new();

pub async fn init_mysqldb() -> ResultWarp<()> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://preheated:%232x%24xcfTQw5G%24*WE@db4free.net:3306/gptdata")
        .await?;

    MYSQLDB_CLIENT.set(pool).unwrap();
    Ok(())
}

pub struct MysqlClient {}

impl MysqlClient {
    pub fn new() -> Self {
        Self {}
    }
}

impl MysqlClient {
    pub async fn insert_one_msg(
        &self,
        sender_id: i64,
        receiver_id: i64,
        chat_role: ChatRole,
        msg: String,
        time: i64,
    ) -> ResultWarp<()> {
        let client = MYSQLDB_CLIENT
            .get()
            .ok_or(Error::Any("MysqlDB client not initialized".to_string()))?;
        let res = sqlx::query(&format!(
            "insert into temp (sender_id,receiver_id,chat_role,content,time)
        values
        ({},{},'{:?}','{}',{})",
            sender_id, receiver_id, chat_role, msg, time
        ))
        .execute(client)
        .await?;
        debug_print(&format!("{:?}", res));
        Ok(())
    }
}
