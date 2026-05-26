use crate::config::Config;
use crate::domain::{Feedback, Model, ModelError, Prompt, ShellCommand, ShellCommandError};
use async_openai::{
    Client,
    config::OpenAIConfig,
    types::chat::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs, ResponseFormat,
    },
};
use serde::Deserialize;
use std::fmt;

pub enum SuggestionError {
    RequestBuild(&'static str, String),
    ApiCall(String),
    NoResponse,
    EmptyResponse,
    JsonParse { error: String, content: String },
    InvalidCommand(ShellCommandError),
}

impl fmt::Display for SuggestionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequestBuild(step, e) => write!(f, "Failed to build request at {}: {}", step, e),
            Self::ApiCall(e) => write!(f, "API call failed: {}", e),
            Self::NoResponse => write!(f, "No response from API"),
            Self::EmptyResponse => write!(f, "Empty response from API"),
            Self::JsonParse { error, content } => {
                write!(
                    f,
                    "Failed to parse JSON response: {}. Content: {}",
                    error, content
                )
            }
            Self::InvalidCommand(e) => write!(f, "Invalid command suggested: {}", e),
        }
    }
}

pub enum ExplanationError {
    RequestBuild(&'static str, String),
    ApiCall(String),
    NoResponse,
    EmptyResponse,
}

impl fmt::Display for ExplanationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequestBuild(step, e) => write!(f, "Failed to build request at {}: {}", step, e),
            Self::ApiCall(e) => write!(f, "API call failed: {}", e),
            Self::NoResponse => write!(f, "No response from API"),
            Self::EmptyResponse => write!(f, "Empty response from API"),
        }
    }
}

pub enum RefineError {
    RequestBuild(&'static str, String),
    ApiCall(String),
    NoResponse,
    EmptyResponse,
    JsonParse { error: String, content: String },
    InvalidCommand(ShellCommandError),
}

impl fmt::Display for RefineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequestBuild(step, e) => write!(f, "Failed to build request at {}: {}", step, e),
            Self::ApiCall(e) => write!(f, "API call failed: {}", e),
            Self::NoResponse => write!(f, "No response from API"),
            Self::EmptyResponse => write!(f, "Empty response from API"),
            Self::JsonParse { error, content } => {
                write!(
                    f,
                    "Failed to parse JSON response: {}. Content: {}",
                    error, content
                )
            }
            Self::InvalidCommand(e) => write!(f, "Invalid command suggested: {}", e),
        }
    }
}

pub enum GrokClientError {
    InvalidModel(ModelError),
}

impl fmt::Display for GrokClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidModel(e) => write!(f, "Invalid model: {}", e),
        }
    }
}

pub struct GrokClient {
    client: Client<OpenAIConfig>,
    model: Model,
}

#[derive(Deserialize)]
struct CommandResponse {
    command: String,
}

impl GrokClient {
    pub fn new(config: Config) -> Result<Self, GrokClientError> {
        let client = Client::with_config(
            OpenAIConfig::new()
                .with_api_key(config.api_key)
                .with_api_base(config.api_url),
        );
        let model = Model::new(config.model).map_err(GrokClientError::InvalidModel)?;
        Ok(Self { client, model })
    }

    pub async fn get_command_suggestion(
        &self,
        prompt: &Prompt,
    ) -> Result<ShellCommand, SuggestionError> {
        let request = CreateChatCompletionRequestArgs::default()
            .model(self.model.as_str())
            .response_format(ResponseFormat::JsonObject)
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content("You are a shell command assistant. Your goal is to translate natural language into a single, valid shell command. Return a JSON object with a single key 'command'. Target shell is bash/zsh.")
                    .build()
                    .map_err(|e| SuggestionError::RequestBuild("system message", e.to_string()))?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(prompt.as_str())
                    .build()
                    .map_err(|e| SuggestionError::RequestBuild("user message", e.to_string()))?
                    .into(),
            ])
            .build()
            .map_err(|e| SuggestionError::RequestBuild("request", e.to_string()))?;

        let response = self
            .client
            .chat()
            .create(request)
            .await
            .map_err(|e| SuggestionError::ApiCall(e.to_string()))?;

        let choice = response
            .choices
            .first()
            .ok_or(SuggestionError::NoResponse)?;

        let content = choice
            .message
            .content
            .as_ref()
            .ok_or(SuggestionError::EmptyResponse)?;

        let command_response: CommandResponse =
            serde_json::from_str(content).map_err(|e| SuggestionError::JsonParse {
                error: e.to_string(),
                content: content.clone(),
            })?;

        ShellCommand::new(command_response.command).map_err(SuggestionError::InvalidCommand)
    }

    pub async fn get_explanation(
        &self,
        command: &ShellCommand,
    ) -> Result<String, ExplanationError> {
        let request = CreateChatCompletionRequestArgs::default()
            .model(self.model.as_str())
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content("Explain the following shell command concisely and clearly. Break down what each part and flag does. The explanation will be displayed to a human user in a terminal, format accordingly: use only plain text, no markdown or other formatting")
                    .build()
                    .map_err(|e| ExplanationError::RequestBuild("system message", e.to_string()))?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(command.as_str())
                    .build()
                    .map_err(|e| ExplanationError::RequestBuild("user message", e.to_string()))?
                    .into(),
            ])
            .build()
            .map_err(|e| ExplanationError::RequestBuild("request", e.to_string()))?;

        let response = self
            .client
            .chat()
            .create(request)
            .await
            .map_err(|e| ExplanationError::ApiCall(e.to_string()))?;

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
        feedback: &Feedback,
    ) -> Result<ShellCommand, RefineError> {
        let request = CreateChatCompletionRequestArgs::default()
            .model(self.model.as_str())
            .response_format(ResponseFormat::JsonObject)
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content("You are a shell command assistant. The user wants to refine a previously suggested command. Return a JSON object with a single key 'command'. Target shell is bash/zsh.")
                    .build()
                    .map_err(|e| RefineError::RequestBuild("system message", e.to_string()))?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(format!("Original request: {}\nOriginal command: {}\nFeedback: {}", original_prompt, original_command, feedback))
                    .build()
                    .map_err(|e| RefineError::RequestBuild("user message", e.to_string()))?
                    .into(),
            ])
            .build()
            .map_err(|e| RefineError::RequestBuild("request", e.to_string()))?;

        let response = self
            .client
            .chat()
            .create(request)
            .await
            .map_err(|e| RefineError::ApiCall(e.to_string()))?;

        let choice = response.choices.first().ok_or(RefineError::NoResponse)?;

        let content = choice
            .message
            .content
            .as_ref()
            .ok_or(RefineError::EmptyResponse)?;

        let command_response: CommandResponse =
            serde_json::from_str(content).map_err(|e| RefineError::JsonParse {
                error: e.to_string(),
                content: content.clone(),
            })?;

        ShellCommand::new(command_response.command).map_err(RefineError::InvalidCommand)
    }
}
