use tgchatgpt::cmdline::*;
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
        let cmd = CmdLine::try_from(TryFromWrapper(line.clone() /* del */)).unwrap();
        match cmd {
            CmdLine::Operation(CmdOperation::QuitCmd) => break,
            CmdLine::Operation(CmdOperation::ClearContext) => {
                ctx.reset();
                continue;
            }
            CmdLine::Operation(CmdOperation::SaveContext(pth)) => {
                ctx.serialize_to_file(pth).unwrap();
                continue;
            }
            CmdLine::Operation(CmdOperation::ReadContext(pth)) => {
                ctx = ChatGPTAPIContext::from_file(pth).unwrap();
                continue;
            }
            CmdLine::Chat(c) => ctx.chat_context.add_user_chat(c),
        }

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
