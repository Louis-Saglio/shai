use std::fmt;

#[derive(Debug, Clone)]
pub struct Prompt(String);

impl Prompt {
    pub fn new(content: String) -> Result<Self, String> {
        if content.trim().is_empty() {
            Err("Prompt cannot be empty".to_string())
        } else {
            Ok(Self(content))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Prompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct ShellCommand(String);

impl ShellCommand {
    pub fn new(command: String) -> Result<Self, String> {
        if command.trim().is_empty() {
            Err("Command cannot be empty".to_string())
        } else {
            Ok(Self(command))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ShellCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
