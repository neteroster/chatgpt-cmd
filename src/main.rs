use tgchatgpt::data_struct::*;

const CHATGPT_API_URL: &'static str = "https://api.openai.com/v1/chat/completions";

#[tokio::main]
async fn main() {
    println!("Please enter your OpenAI API Key: ");
    let api_key = std::io::stdin().lines().next().unwrap().unwrap();
    println!("Thank you. Enjoy!");

    let mut ctx = ChatGPTAPIContext::build(api_key, CHATGPT_API_URL.to_owned());

    println!("API Context created. Enter `clear` to clean context and `quit` to exit.");
    println!("Enter `save <filepath>` to load api context from file.");

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
        if line.starts_with("save") {
            let fpath = line.split(' ').into_iter().skip(1).next().unwrap();
            ctx.serialize_to_file(fpath).unwrap();
            continue;
        }
        ctx.chat_context.add_user_chat(line);

        match ctx.send_to_gpt().await {
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
