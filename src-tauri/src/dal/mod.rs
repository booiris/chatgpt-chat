use crate::model::errors::ResultWarp;
pub mod debug_print;
pub mod mongodb;
pub mod openai_api;

pub async fn init_dal() -> ResultWarp<()> {
    openai_api::init::init_openai_api()?;
    mongodb::init::init_mongodb().await?;
    Ok(())
}
