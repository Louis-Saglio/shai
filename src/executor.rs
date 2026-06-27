use crate::domain::ShellCommand;
use std::env;
use std::fmt;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

pub enum HistoryError {
    NoHistoryFile,
    WriteFailed(String),
}

impl fmt::Display for HistoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoHistoryFile => write!(f, "No history file configured"),
            Self::WriteFailed(e) => write!(f, "Failed to write to history file: {}", e),
        }
    }
}

pub fn append_to_history(command: &ShellCommand) -> Result<(), HistoryError> {
    let history_path = match env::var("HISTFILE") {
        Ok(path) => PathBuf::from(path),
        Err(_) => {
            let shell = env::var("SHELL").unwrap_or_default();
            let shell_name = shell.rsplit('/').next().unwrap_or(&shell);
            match shell_name {
                "bash" => dirs::home_dir()
                    .map(|home| home.join(".bash_history"))
                    .ok_or(HistoryError::NoHistoryFile)?,
                "zsh" => dirs::home_dir()
                    .map(|home| home.join(".zsh_history"))
                    .ok_or(HistoryError::NoHistoryFile)?,
                _ => return Err(HistoryError::NoHistoryFile),
            }
        }
    };

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&history_path)
        .map_err(|e| HistoryError::WriteFailed(e.to_string()))?;

    file.write_all(command.as_str().as_bytes())
        .and_then(|_| file.write_all(b"\n"))
        .map_err(|e| HistoryError::WriteFailed(e.to_string()))?;

    Ok(())
}

pub enum ExecutionError {
    CommandFailed(String),
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CommandFailed(e) => write!(f, "Command execution failed: {}", e),
        }
    }
}

pub fn execute_command(command: &ShellCommand) -> Result<(), ExecutionError> {
    // Detect the current shell from the environment variable, defaulting to "sh"
    let shell = env::var("SHELL").unwrap_or_else(|_| "sh".to_string());

    // Execute the command string provided by the AI in the context of the user's shell.
    // This allows for pipes, redirections, and shell-specific features.
    let status = Command::new(shell)
        .arg("-c")
        .arg(command.as_str())
        .status()
        .map_err(|e| ExecutionError::CommandFailed(e.to_string()))?;

    if status.success() {
        Ok(())
    } else {
        // Return an error if the command itself failed to execute properly or returned non-zero.
        Err(ExecutionError::CommandFailed(format!(
            "Command exited with status: {}",
            status
        )))
    }
}
