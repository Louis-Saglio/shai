use std::fmt;

#[derive(Debug, Clone)]
pub struct Prompt(String);

pub enum PromptError {
    Empty,
}

impl fmt::Debug for PromptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "Prompt cannot be empty"),
        }
    }
}

impl fmt::Display for PromptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "Prompt cannot be empty"),
        }
    }
}

impl Prompt {
    pub fn new(content: String) -> Result<Self, PromptError> {
        if content.trim().is_empty() {
            Err(PromptError::Empty)
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

pub enum ShellCommandError {
    Empty,
}

impl fmt::Debug for ShellCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "Command cannot be empty"),
        }
    }
}

impl fmt::Display for ShellCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "Command cannot be empty"),
        }
    }
}

impl ShellCommand {
    pub fn new(command: String) -> Result<Self, ShellCommandError> {
        if command.trim().is_empty() {
            Err(ShellCommandError::Empty)
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
