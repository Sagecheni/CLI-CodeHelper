use anyhow::Result;
use std::env;
use std::process::{Command, Output};

#[derive(Debug)]
enum ShellType {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Cmd,
}

#[derive(Debug)]
pub struct ShellCommand {
    shell_type: ShellType,
    command: String,
}

impl ShellCommand {
    pub fn new() -> Result<Self> {
        let shell_type = Self::detect_shell()?;
        Ok(Self {
            shell_type,
            command: String::new(),
        })
    }

    fn detect_shell() -> Result<ShellType> {
        let shell = env::var("SHELL")?;
        Ok(match shell.as_str() {
            s if s.contains("bash") => ShellType::Bash,
            s if s.contains("zsh") => ShellType::Zsh,
            s if s.contains("fish") => ShellType::Fish,
            s if s.contains("powershell") => ShellType::PowerShell,
            s if s.contains("cmd") => ShellType::Cmd,
            _ => ShellType::Bash,
        })
    }

    pub fn execute(&self) -> Result<Output> {
        match self.shell_type {
            ShellType::Bash | ShellType::Zsh => {
                Command::new("sh").arg("-c").arg(&self.command).output()
            }
            ShellType::PowerShell => Command::new("powershell")
                .arg("-Command")
                .arg(&self.command)
                .output(),
            ShellType::Cmd => Command::new("cmd").arg("/C").arg(&self.command).output(),
            ShellType::Fish => Command::new("fish").arg("-c").arg(&self.command).output(),
        }
        .map_err(Into::into)
    }

    pub fn set_command(&mut self, command: String) {
        self.command = command;
    }
}
