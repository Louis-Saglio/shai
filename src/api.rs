use crate::domain::{Prompt, ShellCommand, ShellCommandError};
use async_openai::{
    Client,
    config::OpenAIConfig,
    types::chat::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
};
use std::fmt;

pub enum SuggestionError {
    ApiError(String),
    NoResponse,
    EmptyResponse,
    InvalidCommand(ShellCommandError),
}

impl fmt::Display for SuggestionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApiError(e) => write!(f, "API error: {}", e),
            Self::NoResponse => write!(f, "No response from API"),
            Self::EmptyResponse => write!(f, "Empty response from API"),
            Self::InvalidCommand(e) => write!(f, "Invalid command suggested: {}", e),
        }
    }
}

pub enum ExplanationError {
    ApiError(String),
    NoResponse,
    EmptyResponse,
}

impl fmt::Display for ExplanationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApiError(e) => write!(f, "API error: {}", e),
            Self::NoResponse => write!(f, "No response from API"),
            Self::EmptyResponse => write!(f, "Empty response from API"),
        }
    }
}

pub enum RefineError {
    ApiError(String),
    NoResponse,
    EmptyResponse,
    InvalidCommand(ShellCommandError),
}

impl fmt::Display for RefineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApiError(e) => write!(f, "API error: {}", e),
            Self::NoResponse => write!(f, "No response from API"),
            Self::EmptyResponse => write!(f, "Empty response from API"),
            Self::InvalidCommand(e) => write!(f, "Invalid command suggested: {}", e),
        }
    }
}

pub struct GrokClient {
    client: Client<OpenAIConfig>,
}

impl GrokClient {
    pub fn new(api_key: String) -> Self {
        let config = OpenAIConfig::new()
            .with_api_key(api_key)
            .with_api_base("https://api.x.ai/v1");

        let client = Client::with_config(config);

        Self { client }
    }

    pub async fn get_command_suggestion(
        &self,
        prompt: &Prompt,
    ) -> Result<ShellCommand, SuggestionError> {
        let request = CreateChatCompletionRequestArgs::default()
            .model("grok-build-0.1")
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content("You are a shell command assistant. Your goal is to translate natural language into a single, valid shell command. Return ONLY the command itself, without any markdown formatting, backticks, or explanation. If multiple commands are needed, use pipes or subshells. Target shell is bash/zsh.")
                    .build()
                    .map_err(|e| SuggestionError::ApiError(e.to_string()))?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(prompt.as_str())
                    .build()
                    .map_err(|e| SuggestionError::ApiError(e.to_string()))?
                    .into(),
            ])
            .build()
            .map_err(|e| SuggestionError::ApiError(e.to_string()))?;

        let response = self
            .client
            .chat()
            .create(request)
            .await
            .map_err(|e| SuggestionError::ApiError(e.to_string()))?;

        let choice = response
            .choices
            .first()
            .ok_or(SuggestionError::NoResponse)?;

        let command_str = choice
            .message
            .content
            .as_ref()
            .ok_or(SuggestionError::EmptyResponse)?
            .trim()
            .to_string();

        ShellCommand::new(command_str).map_err(SuggestionError::InvalidCommand)
    }

    pub async fn get_explanation(
        &self,
        command: &ShellCommand,
    ) -> Result<String, ExplanationError> {
        let request = CreateChatCompletionRequestArgs::default()
            .model("grok-build-0.1")
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content("Explain the following shell command concisely and clearly. Break down what each part and flag does. The explanation will be displayed to a human user in a terminal, format accordingly: use only plain text, no markdown or other formatting")
                    .build()
                    .map_err(|e| ExplanationError::ApiError(e.to_string()))?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(command.as_str())
                    .build()
                    .map_err(|e| ExplanationError::ApiError(e.to_string()))?
                    .into(),
            ])
            .build()
            .map_err(|e| ExplanationError::ApiError(e.to_string()))?;

        let response = self
            .client
            .chat()
            .create(request)
            .await
            .map_err(|e| ExplanationError::ApiError(e.to_string()))?;

        let choice = response
            .choices
            .first()
            .ok_or(ExplanationError::NoResponse)?;

        let explanation = choice
            .message
            .content
            .as_ref()
            .ok_or(ExplanationError::EmptyResponse)?
            .trim()
            .to_string();

        Ok(explanation)
    }

    pub async fn refine_command(
        &self,
        original_prompt: &Prompt,
        original_command: &ShellCommand,
        feedback: &str,
    ) -> Result<ShellCommand, RefineError> {
        let request = CreateChatCompletionRequestArgs::default()
            .model("grok-build-0.1")
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content("You are a shell command assistant. The user wants to refine a previously suggested command. Return ONLY the new command itself, without any markdown formatting, backticks, or explanation.")
                    .build()
                    .map_err(|e| RefineError::ApiError(e.to_string()))?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(format!("Original request: {}\nOriginal command: {}\nFeedback: {}", original_prompt, original_command, feedback))
                    .build()
                    .map_err(|e| RefineError::ApiError(e.to_string()))?
                    .into(),
            ])
            .build()
            .map_err(|e| RefineError::ApiError(e.to_string()))?;

        let response = self
            .client
            .chat()
            .create(request)
            .await
            .map_err(|e| RefineError::ApiError(e.to_string()))?;

        let choice = response.choices.first().ok_or(RefineError::NoResponse)?;

        let command_str = choice
            .message
            .content
            .as_ref()
            .ok_or(RefineError::EmptyResponse)?
            .trim()
            .to_string();

        ShellCommand::new(command_str).map_err(RefineError::InvalidCommand)
    }
}
