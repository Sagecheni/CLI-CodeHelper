use crate::api::models::Message;
use colored::*;
use spinners::{Spinner, Spinners};
use std::{io::{self, Write}, thread, time::Duration};
pub struct DisplayManager;

impl DisplayManager {
    fn get_commands() -> Vec<(&'static str, &'static str)> {
        vec![
            ("/new", "Start a new conversation"),
            ("/stream", "Toggle stream mode"),
            ("/clear", "Clear screen"),
            ("/context", "Show conversation history"),
            ("/help", "Show this help message"),
            ("/exit", "Exit the program"),
        ]
    }

    pub fn print_welcome() {
        println!("{}", "\nWelcome to CLI Code Helper".bright_green().bold());
        println!("{}", "Available commands:".cyan());
        for (cmd, desc) in Self::get_commands() {
            println!("  {} - {}", cmd.yellow(), desc);
        }
        println!();
    }

    pub fn print_help() {
        println!("\n{}", "Commands:".cyan().bold());
        for (cmd, desc) in Self::get_commands() {
            println!("  {} - {}", cmd.yellow(), desc);
        }
    }

    pub fn print_prompt() {
        print!("{}", "\n> ".green().bold());
        io::stdout().flush().unwrap();
    }

    pub fn print_assistant_prefix() {
        println!("\n");
        print!("{}", "Assistant: ".blue().bold());
        io::stdout().flush().unwrap();
    }

    pub fn print_stream_status(enabled: bool) {
        let status = if enabled { "ON".green() } else { "OFF".red() };
        println!("{} {}", "Stream mode:".cyan(), status);
    }

    pub fn print_thinking() -> Spinner {
        Spinner::new(Spinners::Dots12, "Thinking...".into())
    }

    pub fn clear_screen() {
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().unwrap();
    }

    pub fn print_context(messages: &[&Message]) {
        println!("\n{}", "📝 Conversation History".cyan().bold());
        println!("{}", "─".repeat(60).bright_black());

        for msg in messages {
            let (prefix, content) = match msg.role.as_str() {
                "user" => ("You".green().bold(), msg.content.white()),
                "assistant" => ("Assistant".blue().bold(), msg.content.white()),
                _ => ("Unknown".red().bold(), msg.content.red()),
            };
            
            // 添加时间戳
            let timestamp = chrono::Local::now().format("%H:%M:%S");
            println!("[{}] {}", timestamp.to_string().bright_black(), prefix);
            // 缩进内容，使其更易读
            for line in content.to_string().lines() {
                println!("  {}", line);
            }
            println!("{}", "─".repeat(60).bright_black());
        }
    }

    pub fn print_error(msg: &str) {
        println!("{} {}", "Error:".red().bold(), msg);
    }

    // 添加打字机效果的方法
    pub fn print_typewriter(text: &str, delay: u64) {
        for c in text.chars() {
            print!("{}", c);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(delay));
        }
        println!();
    }
}