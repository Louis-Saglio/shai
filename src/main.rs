mod api;
mod config;
mod domain;
mod executor;
mod ui;

use api::GrokClient;
use clap::Parser;
use domain::Prompt;
use ui::InteractionResult;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The natural language description of what you want to do
    prompt: Vec<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.prompt.is_empty() {
        println!("Please provide a description of what you want to do.");
        return;
    }

    let initial_prompt_str = args.prompt.join(" ");
    let initial_prompt = match Prompt::new(initial_prompt_str) {
        Ok(p) => p,
        Err(e) => {
            ui::display_error(&e);
            return;
        }
    };

    let api_key = if let Ok(key) = std::env::var("XAI_API_KEY") {
        key
    } else {
        let mut config = config::Config::load().unwrap_or_else(|e| {
            ui::display_error(format!("Failed to load config: {}", e));
            config::Config::default()
        });

        if config.api_key.is_empty() {
            match ui::prompt_for_api_key() {
                Ok(key) => {
                    config.api_key = key.clone();
                    if let Err(e) = config.save() {
                        ui::display_error(format!("Failed to save config: {}", e));
                    }
                    key
                }
                Err(e) => {
                    ui::display_error(e);
                    return;
                }
            }
        } else {
            config.api_key
        }
    };

    let client = GrokClient::new(api_key);

    ui::display_info("Thinking...");
    let mut current_command = match client.get_command_suggestion(&initial_prompt).await {
        Ok(cmd) => cmd,
        Err(e) => {
            ui::display_error(e);
            return;
        }
    };

    loop {
        ui::display_suggestion(&current_command);

        match ui::get_user_choice() {
            Ok(InteractionResult::Run) => {
                ui::display_info("Executing...");
                if let Err(e) = executor::execute_command(&current_command) {
                    ui::display_error(e);
                }
                break;
            }
            Ok(InteractionResult::Explain) => {
                ui::display_info("Getting explanation...");
                match client.get_explanation(&current_command).await {
                    Ok(explanation) => ui::display_explanation(&explanation),
                    Err(e) => ui::display_error(e),
                }
            }
            Ok(InteractionResult::Refine(feedback)) => {
                ui::display_info("Refining...");
                match client
                    .refine_command(&initial_prompt, &current_command, &feedback)
                    .await
                {
                    Ok(new_cmd) => current_command = new_cmd,
                    Err(e) => ui::display_error(e),
                }
            }
            Ok(InteractionResult::Cancel) => {
                ui::display_info("Canceled.");
                break;
            }
            Err(e) => {
                ui::display_error(e);
                break;
            }
        }
    }
}
