use crate::errors::{Result, ShaiError};
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use std::env;

pub struct GrokClient {
    client: Client<OpenAIConfig>,
}

impl GrokClient {
    pub fn new() -> Result<Self> {
        let api_key = env::var("XAI_API_KEY")
            .map_err(|_| ShaiError::EnvError("XAI_API_KEY environment variable not set".to_string()))?;

        let config = OpenAIConfig::new()
            .with_api_key(api_key)
            .with_api_base("https://api.x.ai/v1");

        let client = Client::with_config(config);

        Ok(Self { client })
    }

    pub async fn get_command_suggestion(&self, prompt: &str) -> Result<String> {
        let request = CreateChatCompletionRequestArgs::default()
            .model("grok-build-0.1")
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content("You are a shell command assistant. Your goal is to translate natural language into a single, valid shell command. Return ONLY the command itself, without any markdown formatting, backticks, or explanation. If multiple commands are needed, use pipes or subshells. Target shell is bash/zsh.")
                    .build()
                    .map_err(|e| ShaiError::ApiError(e.to_string()))?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(prompt)
                    .build()
                    .map_err(|e| ShaiError::ApiError(e.to_string()))?
                    .into(),
            ])
            .build()
            .map_err(|e| ShaiError::ApiError(e.to_string()))?;

        let response = self.client.chat().create(request).await
            .map_err(|e| ShaiError::ApiError(e.to_string()))?;

        let choice = response.choices.first()
            .ok_or_else(|| ShaiError::ApiError("No response from API".to_string()))?;

        let command = choice.message.content.as_ref()
            .ok_or_else(|| ShaiError::ApiError("Empty response from API".to_string()))?
            .trim()
            .to_string();

        Ok(command)
    }

    pub async fn get_explanation(&self, command: &str) -> Result<String> {
        let request = CreateChatCompletionRequestArgs::default()
            .model("grok-build-0.1")
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content("Explain the following shell command concisely and clearly. Break down what each part and flag does. The explanation will be displayed to a human user in a terminal, format accordingly: use only plain text, no markdown or other formatting")
                    .build()
                    .map_err(|e| ShaiError::ApiError(e.to_string()))?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(command)
                    .build()
                    .map_err(|e| ShaiError::ApiError(e.to_string()))?
                    .into(),
            ])
            .build()
            .map_err(|e| ShaiError::ApiError(e.to_string()))?;

        let response = self.client.chat().create(request).await
            .map_err(|e| ShaiError::ApiError(e.to_string()))?;

        let choice = response.choices.first()
            .ok_or_else(|| ShaiError::ApiError("No response from API".to_string()))?;

        let explanation = choice.message.content.as_ref()
            .ok_or_else(|| ShaiError::ApiError("Empty response from API".to_string()))?
            .trim()
            .to_string();

        Ok(explanation)
    }

    pub async fn refine_command(&self, original_prompt: &str, original_command: &str, feedback: &str) -> Result<String> {
        let request = CreateChatCompletionRequestArgs::default()
            .model("grok-build-0.1")
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content("You are a shell command assistant. The user wants to refine a previously suggested command. Return ONLY the new command itself, without any markdown formatting, backticks, or explanation.")
                    .build()
                    .map_err(|e| ShaiError::ApiError(e.to_string()))?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(format!("Original request: {}\nOriginal command: {}\nFeedback: {}", original_prompt, original_command, feedback))
                    .build()
                    .map_err(|e| ShaiError::ApiError(e.to_string()))?
                    .into(),
            ])
            .build()
            .map_err(|e| ShaiError::ApiError(e.to_string()))?;

        let response = self.client.chat().create(request).await
            .map_err(|e| ShaiError::ApiError(e.to_string()))?;

        let choice = response.choices.first()
            .ok_or_else(|| ShaiError::ApiError("No response from API".to_string()))?;

        let command = choice.message.content.as_ref()
            .ok_or_else(|| ShaiError::ApiError("Empty response from API".to_string()))?
            .trim()
            .to_string();

        Ok(command)
    }
}
