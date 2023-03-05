use crate::model::errors::ResultWarp;
pub mod debug_print;
pub mod openai_api;

pub fn init_dal() -> ResultWarp<()> {
    openai_api::init::init_openai_api()?;
    Ok(())
}
