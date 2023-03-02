use crate::model::errors::{Error, ResultWarp};
use once_cell::sync::OnceCell;
use openai_api::{
    api::{ChatArgs, ChatArgsBuilder, ChatFormat, ChatRole, CompletionArgs, CompletionArgsBuilder},
    Client,
};

static OPENAI_API_CLIENT: OnceCell<Client> = OnceCell::new();

pub fn init_openai_api() -> ResultWarp<()> {
    let api_token = std::env::var("OPENAI_SK")?;
    let client = Client::new(&api_token)?;
    OPENAI_API_CLIENT.set(client).unwrap();
    Ok(())
}

#[allow(dead_code)]
pub struct OpenaiApi {
    complete_args: CompletionArgsBuilder,
    chat_args: ChatArgsBuilder,
    context: String,
    chat_history: Vec<ChatFormat>,
}

impl OpenaiApi {
    pub fn new() -> Self {
        let complete_args = CompletionArgs::builder()
            .model("text-davinci-003")
            .max_tokens(45)
            .stop(vec!["\n".into()])
            .frequency_penalty(0.5);
        let context = String::from("Say this is a test");
        let chat_args = ChatArgs::builder()
            .model("gpt-3.5-turbo")
            .max_tokens(45)
            .frequency_penalty(0.5);
        let chat_history = vec![ChatFormat::new(
            ChatRole::System,
            "You are a helpful assistant.".into(),
        )];
        Self {
            complete_args,
            chat_args,
            context,
            chat_history,
        }
    }
}

impl OpenaiApi {
    pub async fn _query(&mut self, content: &str) -> ResultWarp<String> {
        let client = match OPENAI_API_CLIENT.get() {
            Some(client) => client,
            None => return Err(Error::Any("OpenaiApi client not initialized".to_string())),
        };
        self.context.push_str(&("\nHuman: ".to_owned() + content));
        self.context.push_str("\nAI: ");
        match client
            .complete_prompt(self.complete_args.prompt(self.context.as_str()).build()?)
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

    pub async fn chat(&mut self, content: &str) -> ResultWarp<String> {
        let client = match OPENAI_API_CLIENT.get() {
            Some(client) => client,
            None => return Err(Error::Any("OpenaiApi client not initialized".to_string())),
        };
        self.chat_history
            .push(ChatFormat::new(ChatRole::User, content.to_string()));
        match client
            .chat(self.chat_args.messages(self.chat_history.clone()).build()?) // TODO: clone is not good
            .await
        {
            Ok(answer) => {
                let answer = &answer.choices[0].message;
                self.chat_history
                    .push(ChatFormat::new(ChatRole::Assistant, answer.to_string()));
                Ok(answer.content.trim().to_owned())
            }
            Err(e) => Err(e.into()),
        }
    }
}
