use std::fmt;

#[derive(Debug, Clone)]
pub struct Prompt(String);

pub enum PromptError {
    Empty,
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

#[derive(Debug, Clone)]
pub struct Model(String);

pub enum ModelError {
    Empty,
}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "Model name cannot be empty"),
        }
    }
}

impl Model {
    pub fn new(name: String) -> Result<Self, ModelError> {
        if name.trim().is_empty() {
            Err(ModelError::Empty)
        } else {
            Ok(Self(name))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Feedback(String);

pub enum FeedbackError {
    Empty,
}

impl fmt::Display for FeedbackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "Feedback cannot be empty"),
        }
    }
}

impl Feedback {
    pub fn new(content: String) -> Result<Self, FeedbackError> {
        if content.trim().is_empty() {
            Err(FeedbackError::Empty)
        } else {
            Ok(Self(content))
        }
    }
}

impl fmt::Display for Feedback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
