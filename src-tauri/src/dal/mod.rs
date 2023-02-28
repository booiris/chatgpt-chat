use crate::model::errors::ResultWarp;

pub mod openai_api;

pub fn init() -> ResultWarp<()> {
    openai_api::init::init_openai_api()?;
    Ok(())
}
