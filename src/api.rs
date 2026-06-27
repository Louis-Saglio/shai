use crate::config::Config;
use crate::domain::{Feedback, Model, Prompt, ShellCommand};
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
    ApiCall(String),
    BadResponse,
}

impl fmt::Display for SuggestionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApiCall(e) => write!(f, "API call failed: {}", e),
            Self::BadResponse => write!(f, "The API did not respond a usable response"),
        }
    }
}

pub enum ExplanationError {
    ApiCall(String),
    BadResponse,
}

impl fmt::Display for ExplanationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApiCall(e) => write!(f, "API call failed: {}", e),
            Self::BadResponse => write!(f, "The API did not respond a usable response"),
        }
    }
}

pub enum RefineError {
    ApiCall(String),
    BadResponse,
}

impl fmt::Display for RefineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApiCall(e) => write!(f, "API call failed: {}", e),
            Self::BadResponse => write!(f, "The API did not respond a usable response"),
        }
    }
}

pub struct OpenAIClient {
    client: Client<OpenAIConfig>,
    model: Model,
}

#[derive(Deserialize)]
struct CommandResponse {
    command: String,
}

impl OpenAIClient {
    pub fn new(config: Config) -> Self {
        let client = Client::with_config(
            OpenAIConfig::new()
                .with_api_key(config.api_key)
                .with_api_base(config.api_url),
        );
        let model = Model::new(config.model);
        Self { client, model }
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
                    .unwrap() // Can err only if a required field is missing. The only required field is content which is defined just above
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(prompt.as_str())
                    .build()
                    .unwrap() // Can err only if a required field is missing. The only required field is content which is defined just above
                    .into(),
            ])
            .build()
            .unwrap(); // Can err only if a required field is missing. The only required fields are messages and model which are defined just above

        let response = self
            .client
            .chat()
            .create(request)
            .await
            .map_err(|e| SuggestionError::ApiCall(e.to_string()))?;

        let choice = response
            .choices
            .first()
            .ok_or(SuggestionError::BadResponse)?;

        let content = choice
            .message
            .content
            .as_ref()
            .ok_or(SuggestionError::BadResponse)?;

        let command_response: CommandResponse =
            serde_json::from_str(content).map_err(|_| SuggestionError::BadResponse)?;

        Ok(ShellCommand::new(command_response.command))
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
                    .unwrap()  // Can err only if a required field is missing. The only required field is content which is defined just above
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(command.as_str())
                    .build()
                    .unwrap() // Can err only if a required field is missing. The only required field is content which is defined just above
                    .into(),
            ])
            .build()
            .unwrap(); // Can err only if a required field is missing. The only required fields are messages and model which are defined just above

        let response = self
            .client
            .chat()
            .create(request)
            .await
            .map_err(|e| ExplanationError::ApiCall(e.to_string()))?;

        let choice = response
            .choices
            .first()
            .ok_or(ExplanationError::BadResponse)?;

        let explanation = choice
            .message
            .content
            .as_ref()
            .ok_or(ExplanationError::BadResponse)?
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
                    .unwrap() // Can err only if a required field is missing. The only required field is content which is defined just above
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(format!("Original request: {}\nOriginal command: {}\nFeedback: {}", original_prompt, original_command, feedback))
                    .build()
                    .unwrap() // Can err only if a required field is missing. The only required field is content which is defined just above
                    .into(),
            ])
            .build()
            .unwrap(); // Can err only if a required field is missing. The only required fields are messages and model which are defined just above

        let response = self
            .client
            .chat()
            .create(request)
            .await
            .map_err(|e| RefineError::ApiCall(e.to_string()))?;

        let choice = response.choices.first().ok_or(RefineError::BadResponse)?;

        let content = choice
            .message
            .content
            .as_ref()
            .ok_or(RefineError::BadResponse)?;

        let command_response: CommandResponse =
            serde_json::from_str(content).map_err(|_| RefineError::BadResponse)?;

        Ok(ShellCommand::new(command_response.command))
    }
}
