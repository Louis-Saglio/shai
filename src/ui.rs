use colored::*;
use dialoguer::{Input, Password, Select, theme::ColorfulTheme};
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

impl fmt::Debug for ChoiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputError(e) => write!(f, "Input error: {}", e),
        }
    }
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

pub fn prompt_for_api_key() -> Result<String, PromptError> {
    println!("\n{}", "X.AI API Key not found.".bright_yellow());
    println!("Please enter your API key (it will be saved for future use):");

    let api_key: String = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("API Key")
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

pub fn display_error<T: fmt::Display>(error: T) {
    eprintln!("{}: {}", "Error".red().bold(), error);
}

pub fn display_info(message: &str) {
    println!("{}", message.cyan());
}
