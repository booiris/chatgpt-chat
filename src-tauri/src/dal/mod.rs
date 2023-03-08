use crate::model::errors::ResultWarp;

pub mod debug_print;
pub mod mysql;
pub mod openai_api;

pub async fn init_dal() -> ResultWarp<()> {
    openai_api::init::init_openai_api()?;
    mysql::init::init_mysqldb().await?;
    Ok(())
}
