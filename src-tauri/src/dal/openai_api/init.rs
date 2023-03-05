use crate::{
    dal::mongodb::init::MongoDBClient,
    model::errors::{Error, ResultWarp},
};
use once_cell::sync::OnceCell;
use openai_api::{
    api::{ChatArgs, ChatArgsBuilder, ChatFormat, ChatRole, CompletionArgs, CompletionArgsBuilder},
    Client,
};

static OPENAI_API_CLIENT: OnceCell<Client> = OnceCell::new();

pub fn init_openai_api() -> ResultWarp<()> {
    // let api_token = std::env::var("OPENAI_SK1")?;
    // let client = Client::new(&api_token)?;
    let client = Client::new("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")?;
    OPENAI_API_CLIENT.set(client).unwrap();
    Ok(())
}

#[allow(dead_code)]
pub struct OpenaiApi {
    complete_args: CompletionArgsBuilder,
    chat_args: ChatArgsBuilder,
    context: String,
    chat_history: Vec<ChatFormat>,
    mongo_db: MongoDBClient,
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
            .frequency_penalty(0.5);
        let chat_history = vec![ChatFormat::new(
            ChatRole::System,
            r#"I want you to act as a stand-up comedian. I will provide you with some topics related to current events and you will use your wit, creativity, and observational skills to create a routine based on those topics. You should also be sure to incorporate personal anecdotes or experiences into the routine in order to make it more relatable and engaging for the audience. My first request is "Please Give me some topics"."#.into(),
        )];
        Self {
            complete_args,
            chat_args,
            context,
            chat_history,
            mongo_db: MongoDBClient::new(),
        }
    }
}

impl OpenaiApi {
    pub async fn _query(&mut self, content: &str) -> ResultWarp<String> {
        let client = OPENAI_API_CLIENT
            .get()
            .ok_or(Error::Any("OpenaiApi client not initialized".to_string()))?;
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

    pub async fn chat(&mut self, content: &str) -> ResultWarp<(String, i64)> {
        let client = OPENAI_API_CLIENT
            .get()
            .ok_or(Error::Any("OpenaiApi client not initialized".to_string()))?;
        if content != "" {
            self.insert_msg(ChatRole::User, content.to_string()).await?;
        }
        match client
            .chat(self.chat_args.messages(self.chat_history.clone()).build()?) // TODO: clone is not good
            .await
        {
            Ok(answer) => {
                let answer = &answer.choices[0].message;
                let time = self
                    .insert_msg(ChatRole::Assistant, answer.to_string())
                    .await?;
                Ok((answer.content.trim().to_owned(), time))
            }
            Err(e) => Err(e.into()),
        }
    }

    async fn insert_msg(&mut self, role: ChatRole, content: String) -> ResultWarp<i64> {
        self.chat_history
            .push(ChatFormat::new(role.clone(), content.clone()));
        let time = chrono::Utc::now().timestamp();
        MongoDBClient::insert_one_msg(0, 1, role, content, time).await?;
        Ok(time)
    }
}
