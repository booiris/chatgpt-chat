use crate::model::errors::{Error, ResultWarp};
use once_cell::sync::OnceCell;
use openai_api::{
    api::{CompletionArgs, CompletionArgsBuilder},
    Client,
};

static OPENAI_API_CLIENT: OnceCell<Client> = OnceCell::new();

pub fn init_openai_api() -> ResultWarp<()> {
    let api_token = std::env::var("OPENAI_SK")?;
    let client = Client::new(&api_token)?;
    OPENAI_API_CLIENT.set(client).unwrap();
    Ok(())
}
pub struct OpenaiApi {
    args: CompletionArgsBuilder,
    context: String,
}

impl OpenaiApi {
    pub fn new() -> Self {
        let args = CompletionArgs::builder()
            .model("text-davinci-003")
            .max_tokens(45)
            .stop(vec!["\n".into()])
            .frequency_penalty(0.5);
        let context = String::from("Say this is a test");
        Self { args, context }
    }
}

impl OpenaiApi {
    pub async fn query(&mut self, content: &str) -> ResultWarp<String> {
        let client = match OPENAI_API_CLIENT.get() {
            Some(client) => client,
            None => return Err(Error::Any("OpenaiApi client not initialized".to_string())),
        };
        self.context.push_str(&("\nHuman: ".to_owned() + content));
        self.context.push_str("\nAI: ");
        match client
            .complete_prompt(self.args.prompt(self.context.as_str()).build()?)
            .await
        {
            Ok(completion) => {
                let answer = &completion.choices[0].text;
                self.context.push_str(answer);
                Ok(answer.trim().to_owned())
            }
            Err(e) => Err(e.into()),
        }
    }
}
