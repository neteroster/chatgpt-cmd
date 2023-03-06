use core::fmt;
use reqwest::{
    self,
    header::{self, HeaderMap},
};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

    pub fn serialize_to_file<P>(&self, p: P) -> Result<(), std::io::Error>
    where
        P: Into<PathBuf>,
    {
        let mut f = File::create(p.into())?;
        let serialized = serde_json::to_string(&self)?;
        f.write_all(serialized.as_bytes())?;
        Ok(())
    }

    pub fn from_file(p: &Path) -> Result<Self, std::io::Error> {
        let f = File::open(p)?;

        Ok(serde_json::from_reader(f)?)
    }

    pub async fn send_to_gpt(&mut self) -> Result<(), APIError> {
        let payload = RequestPayload {
            model: "gpt-3.5-turbo".to_owned(),
            messages: &self.chat_context.messages,
        };

        let client = reqwest::Client::new();

        let mut headers = HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {}", self.api_key).parse().unwrap(),
        );

        let response = client
            .post(&self.api_url)
            .headers(headers)
            .json(&payload)
            .send()
            .await?;

        let completion: ChatCompletion = response.json().await?;
        let resp = completion
            .choices
            .into_iter()
            .next()
            .ok_or(APIError::ParseError(
                "error decoding response: encounts None.".to_owned(),
            ))?
            .message;

        self.chat_context.add_message(resp);

        Ok(())
    }
}

#[derive(Debug)]
pub enum APIError {
    NetworkError(String),
    ParseError(String),
}

impl From<reqwest::Error> for APIError {
    fn from(e: reqwest::Error) -> Self {
        Self::NetworkError(e.to_string())
    }
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NetworkError(t) => write!(f, "network error: {}", t),
            Self::ParseError(t) => write!(f, "parse response error: {}", t),
        }
    }
}

impl std::error::Error for APIError {}
