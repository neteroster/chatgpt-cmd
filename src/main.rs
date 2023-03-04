use reqwest::{
    self,
    header::{self, HeaderMap},
};
use tgchatgpt::data_struct::*;

const CHATGPT_API_URL: &'static str = "https://api.openai.com/v1/chat/completions";

async fn send_to_gpt(api_context: &mut ChatGPTAPIContext) -> Result<(), APIError> {
    let payload = RequestPayload {
        model: "gpt-3.5-turbo".to_owned(),
        messages: &api_context.chat_context.messages,
    };

    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(
        header::AUTHORIZATION,
        format!("Bearer {}", api_context.api_key).parse().unwrap(),
    );

    let response = client
        .post(CHATGPT_API_URL)
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

    api_context.chat_context.add_message(resp);

    Ok(())
}

#[tokio::main]
async fn main() {
    println!("Please enter your OpenAI API Key: ");
    let api_key = std::io::stdin().lines().next().unwrap().unwrap();
    println!("Thank you. Enjoy!");

    let mut ctx = ChatGPTAPIContext::build(api_key, CHATGPT_API_URL.to_owned());

    println!("API Context created. Enter `clear` to clean context and `quit` to exit.");
    println!("Enter `context` to view the information of current context."); //TBD

    loop {
        println!("You: ");
        let line = std::io::stdin().lines().next().unwrap().unwrap();
        if line == "quit" {
            break;
        }
        if line == "clear" {
            ctx.reset();
            continue;
        }
        ctx.chat_context.add_user_chat(line);

        match send_to_gpt(&mut ctx).await {
            Ok(_) => (),
            Err(e) => {
                println!("sorry, something went wrong: {}. Retrying...", e);
                if ctx.chat_context.latest().role == "user" {
                    ctx.chat_context.pop_latest();
                    continue;
                }
            }
        }

        println!("CHATGPT: ");
        println!("{}", ctx.chat_context.latest().content.trim());
    }
}
