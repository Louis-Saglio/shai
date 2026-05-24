use colored::*;
use dialoguer::{Input, Select, theme::ColorfulTheme};
use std::fmt;

pub enum InteractionResult {
    Run,
    Explain,
    Refine(String),
    Cancel,
}

pub enum UiError {
    InputError(String),
}

impl fmt::Debug for UiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputError(e) => write!(f, "Input error: {}", e),
        }
    }
}

impl fmt::Display for UiError {
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

pub fn get_user_choice() -> Result<InteractionResult, UiError> {
    let choices = vec!["Run", "Explain", "Refine", "Cancel"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .items(&choices)
        .default(0)
        .interact()
        .map_err(|e| UiError::InputError(e.to_string()))?;

    match selection {
        0 => Ok(InteractionResult::Run),
        1 => Ok(InteractionResult::Explain),
        2 => {
            let feedback: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter your feedback")
                .interact_text()
                .map_err(|e| UiError::InputError(e.to_string()))?;
            Ok(InteractionResult::Refine(feedback))
        }
        _ => Ok(InteractionResult::Cancel),
    }
}

pub fn display_error(error: &str) {
    eprintln!("{}: {}", "Error".red().bold(), error);
}

pub fn display_info(message: &str) {
    println!("{}", message.cyan());
}
