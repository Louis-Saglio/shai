use dialoguer::{theme::ColorfulTheme, Select, Input};
use colored::*;
use crate::errors::{Result, ShaiError};

pub enum InteractionResult {
    Run,
    Explain,
    Refine(String),
    Cancel,
}

pub fn display_suggestion(command: &str) {
    println!("\n{}", "Suggested command:".bright_blue().bold());
    println!("  {}\n", command.bright_green());
}

pub fn display_explanation(explanation: &str) {
    println!("\n{}", "Explanation:".bright_yellow().bold());
    println!("{}\n", explanation);
}

pub fn get_user_choice() -> Result<InteractionResult> {
    let choices = vec!["Run", "Explain", "Refine", "Cancel"];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .items(&choices)
        .default(0)
        .interact()
        .map_err(|e| ShaiError::InputError(e.to_string()))?;

    match selection {
        0 => Ok(InteractionResult::Run),
        1 => Ok(InteractionResult::Explain),
        2 => {
            let feedback: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter your feedback")
                .interact_text()
                .map_err(|e| ShaiError::InputError(e.to_string()))?;
            Ok(InteractionResult::Refine(feedback))
        },
        _ => Ok(InteractionResult::Cancel),
    }
}

pub fn display_error(error: &str) {
    eprintln!("{}: {}", "Error".red().bold(), error);
}

pub fn display_info(message: &str) {
    println!("{}", message.cyan());
}
