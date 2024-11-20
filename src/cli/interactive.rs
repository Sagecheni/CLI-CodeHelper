use crate::api::OpenAIClient;
use crate::utils::display::DisplayManager;
use anyhow::Result;
use colored::Colorize;
use std::io;

const VALID_COMMANDS: [&str; 6] = ["/stream", "/clear", "/exit", "/quit", "/help", "/new"];

pub async fn start_interactive_mode() -> Result<()> {
    let mut client = OpenAIClient::new()?;
    let mut stream_mode = false;

    DisplayManager::print_welcome();

    loop {
        DisplayManager::print_prompt();

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        match input {
            "exit" | "quit" | "/exit" | "/quit" => {
                println!("{}", "Goodbye!".bright_green());
                return Ok(());
            }
            "/stream" => {
                stream_mode = !stream_mode;
                DisplayManager::print_stream_status(stream_mode);
            }
            "/clear" => {
                DisplayManager::clear_screen();
                DisplayManager::print_welcome();
            }
            "/new" => {
                client.clear_context();
                println!("{}", "Started new conversation!".green());
            }
            "/context" => {
                DisplayManager::print_context(&client.show_context());
            }
            "/help" => {
                DisplayManager::print_help();
            }
            input if input.starts_with('/') && !VALID_COMMANDS.contains(&input) => {
                DisplayManager::print_error(&format!("Unknown command. Type {} for help.", "/help".yellow()));
            }
            "" => continue,
            input => {
                if stream_mode {
                    DisplayManager::print_assistant_prefix();
                    client.chat_stream(input).await?;
                } else {
                    let mut sp = DisplayManager::print_thinking();
                    let response = client.chat(input).await?;
                    sp.stop();
                    DisplayManager::print_assistant_prefix();
                    DisplayManager::print_typewriter(&response, 30); // 使用打字机效果，30ms 延迟
                }

                println!(
                    "\n{}",
                    format!("(Conversation turn: {})", client.context_length()).bright_black()
                );
            }
        }
    }
}