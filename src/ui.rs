use crate::config::Config;
use colored::*;
use dialoguer::{Confirm, Input, Password, Select, theme::ColorfulTheme};
use std::fmt;

pub enum InteractionResult {
    Run,
    Explain,
    Refine(String),
    Cancel,
}

pub enum ChoiceError {
    InputError(String),
}

impl fmt::Display for ChoiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputError(e) => write!(f, "Input error: {}", e),
        }
    }
}

pub fn display_suggestion(command: &crate::domain::ShellCommand) {
    println!("\n{}", "Suggested command:".bright_blue().bold());
    println!("  {}\n", command.to_string().bright_green());
}

pub fn display_explanation(explanation: &str) {
    println!("\n{}", "Explanation:".bright_yellow().bold());
    println!("{}\n", explanation);
}

pub fn get_user_choice() -> Result<InteractionResult, ChoiceError> {
    let choices = vec!["Run", "Explain", "Refine", "Cancel"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .items(&choices)
        .default(0)
        .interact()
        .map_err(|e| ChoiceError::InputError(e.to_string()))?;

    match selection {
        0 => Ok(InteractionResult::Run),
        1 => Ok(InteractionResult::Explain),
        2 => {
            let feedback: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter your feedback")
                .interact_text()
                .map_err(|e| ChoiceError::InputError(e.to_string()))?;
            Ok(InteractionResult::Refine(feedback))
        }
        _ => Ok(InteractionResult::Cancel),
    }
}

pub enum PromptError {
    InputError(String),
}

impl fmt::Debug for PromptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputError(e) => write!(f, "Input error: {}", e),
        }
    }
}

impl fmt::Display for PromptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputError(e) => write!(f, "Input error: {}", e),
        }
    }
}

fn prompt_for_api_key() -> Result<String, PromptError> {
    let api_key: String = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("X.AI API Key")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.trim().is_empty() {
                Err("API key cannot be empty")
            } else {
                Ok(())
            }
        })
        .interact()
        .map_err(|e| PromptError::InputError(e.to_string()))?;

    Ok(api_key.trim().to_string())
}

pub fn prompt_configuration() -> Result<Config, PromptError> {
    println!("\n{}", "Shai Configuration".bright_blue().bold());
    println!("Please enter the following details to set up Shai:");

    let api_key = prompt_for_api_key()?;

    let api_url: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("API URL")
        .default("https://api.x.ai/v1".to_string())
        .interact_text()
        .map_err(|e| PromptError::InputError(e.to_string()))?;

    let model: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Model")
        .default("grok-4.20-0309-non-reasoning".to_string())
        .interact_text()
        .map_err(|e| PromptError::InputError(e.to_string()))?;

    Ok(Config {
        api_key,
        api_url,
        model,
    })
}

pub fn confirm(prompt: &str) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(true)
        .interact()
        .unwrap_or(false)
}

pub fn display_error<T: fmt::Display>(error: T) {
    eprintln!("{}: {}", "Error".red().bold(), error);
}

pub fn display_info(message: &str) {
    println!("{}", message.cyan());
}
