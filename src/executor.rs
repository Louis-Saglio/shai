use std::process::Command;
use crate::errors::{Result, ShaiError};
use std::env;

pub fn execute_command(command: &str) -> Result<()> {
    // Detect the current shell from the environment variable, defaulting to "sh"
    let shell = env::var("SHELL").unwrap_or_else(|_| "sh".to_string());

    // Execute the command string provided by the AI in the context of the user's shell.
    // This allows for pipes, redirections, and shell-specific features.
    let status = Command::new(shell)
        .arg("-c")
        .arg(command)
        .status()
        .map_err(|e| ShaiError::ExecutionError(e.to_string()))?;

    if status.success() {
        Ok(())
    } else {
        // Return an error if the command itself failed to execute properly or returned non-zero.
        Err(ShaiError::ExecutionError(format!("Command exited with status: {}", status)))
    }
}
