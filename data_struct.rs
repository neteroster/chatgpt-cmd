use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Debug)]
pub struct RequestPayload<'a> {
    pub model: String,
    pub messages: &'a Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatCompletion {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub choices: Vec<ResponseChatChoice>,
    pub usage: ChatUsage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseChatChoice {
    pub index: usize,
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatUsage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

#[derive(Debug)]
pub struct ChatContext {
    pub messages: Vec<Message>,
}

impl ChatContext {
    pub fn new() -> Self {
        ChatContext {
            messages: Vec::new(),
        }
    }

    pub fn latest(&self) -> &Message {
        self.messages.last().unwrap()
    }

    pub fn pop_latest(&mut self) {
        self.messages.pop().unwrap();
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }

    pub fn add_user_chat<S>(&mut self, txt: S)
    where
        S: Into<String>,
    {
        self.messages.push(Message {
            role: "user".to_owned(),
            content: txt.into(),
        })
    }

    pub fn add_message(&mut self, msg: Message) {
        self.messages.push(msg);
    }
}

pub struct ChatGPTAPIContext {
    pub api_key: String,
    pub api_url: String,
    pub chat_context: ChatContext,
}

impl ChatGPTAPIContext {
    pub fn build<S>(api_key: S, api_url: S) -> Self
    where
        S: Into<String>,
    {
        ChatGPTAPIContext {
            api_key: api_key.into(),
            api_url: api_url.into(),
            chat_context: ChatContext::new(),
        }
    }

    pub fn reset(&mut self) {
        self.chat_context.clear();
    }
}
