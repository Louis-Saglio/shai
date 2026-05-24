use crate::domain::ShellCommand;
use std::env;
use std::fmt;
use std::process::Command;

pub enum ExecutionError {
    CommandFailed(String),
}

impl fmt::Debug for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CommandFailed(e) => write!(f, "Command execution failed: {}", e),
        }
    }
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
