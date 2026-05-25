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

    let config = match config::Config::load() {
        Ok(c) => c,
        Err(e) => {
            ui::display_error(e);
            if ui::confirm(
                "Configuration is missing or invalid. Would you like to configure it now?",
            ) {
                match ui::prompt_configuration() {
                    Ok(new_config) => {
                        if let Err(save_err) = new_config.save() {
                            ui::display_error(format!(
                                "Failed to save configuration: {}",
                                save_err
                            ));
                        }
                        new_config
                    }
                    Err(prompt_err) => {
                        ui::display_error(prompt_err);
                        return;
                    }
                }
            } else {
                return;
            }
        }
    };

    let client = GrokClient::new(config);

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
