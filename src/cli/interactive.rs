use crate::api::{config::ChatModel, OpenAIClient};
use crate::cli::shell::ShellCommand;
use crate::utils::display::DisplayManager;
use anyhow::Result;
use colored::Colorize;
use std::io;

const VALID_COMMANDS: [&str; 6] = ["/stream", "/clear", "/exit", "/quit", "/help", "/new"];

pub async fn handle_command_generation(
    client: &mut OpenAIClient,
    shell: &mut ShellCommand,
    prompt: &str,
) -> Result<()> {
    let command = client.generate_shell_command(prompt).await?;
    println!("\n{}", "Generated command:".cyan().bold());
    println!("{}", command.yellow());
    println!(
        "\n{}",
        "Do you want to execute this command? [y/N]:".bright_black()
    );

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => {
            shell.set_command(command);
            match shell.execute() {
                Ok(output) => {
                    if !output.stdout.is_empty() {
                        println!("{}", String::from_utf8_lossy(&output.stdout));
                    }
                    if !output.stderr.is_empty() {
                        eprintln!("{}", String::from_utf8_lossy(&output.stderr).red());
                    }
                }
                Err(e) => {
                    println!("{}", format!("Failed to execute command: {}", e).red());
                }
            }
        }
        _ => {
            println!("{}", "Command execution cancelled.".yellow());
        }
    }

    Ok(())
}

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
            "exit" | "quit" | "/exit" | "/quit" | "q" => {
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
            "/model" => {
                println!("\n{}", "Available models:".bright_black());
                for (model, description) in ChatModel::list_available_models() {
                    println!("{} - {}", model.bright_green(), description);
                }
                println!(
                    "Current model: {}",
                    client.get_config().model.as_str().bright_green()
                );
                println!(
                    "\nTo change the model, use {} followed by the model name.",
                    "/model".yellow()
                );
            }
            input if input.starts_with("/model") => {
                let model_name = input.trim_start_matches("/model ").trim();
                let new_model = match model_name {
                    "gpt-3.5-turbo" => ChatModel::Gpt35Turbo,
                    "gpt-4o" => ChatModel::Gpt4o,
                    "gpt-4o-mini" => ChatModel::Gpt4omini,
                    _ => {
                        DisplayManager::print_error(&format!(
                            "Invalid model name : {}",
                            model_name
                        ));
                        continue;
                    }
                };
                client.set_model(new_model)?;
                println!(
                    "{} {}",
                    "Model changed to:".bright_green(),
                    client.get_config().model.as_str().bright_green()
                );
            }
            input if input.starts_with('/') && !VALID_COMMANDS.contains(&input) => {
                DisplayManager::print_error(&format!(
                    "Unknown command. Type {} for help.",
                    "/help".yellow()
                ));
            }
            input if input.starts_with("!") => {
                let prompt = input.trim_start_matches('!').trim();
                let mut shell = ShellCommand::new()?;
                handle_command_generation(&mut client, &mut shell, prompt).await?;
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
