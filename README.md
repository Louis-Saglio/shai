# shai (Shell AI)

`shai` is a seamless natural language interface for your terminal. Describe what you want to do in plain English, and `shai` will suggest a command, explain it, or refine it before you run it.

## Features

- **Natural Language to Shell**: Converts English descriptions to valid shell commands using any OpenAI-compatible API (defaults to xAI Grok).
- **Interactive Flow**: Choose to Run, Explain, Refine, or Cancel suggestions.
- **Shell Support**: Detects and uses your current shell (bash, zsh, etc.).
- **Concise Explanations**: Understand what the suggested command does before executing it.
- **Customizable**: Configure your preferred API URL and model.

## Installation

```bash
cargo install --path .
```

## Configuration

`shai` requires an API key from an OpenAI-compatible provider (e.g., xAI, OpenAI, etc.).

### Interactive Setup

The easiest way to configure `shai` is to run it for the first time. If no configuration is found, `shai` will interactively prompt you for:
- **API Key**
- **API URL** (defaults to `https://api.x.ai/v1`)
- **Model** (defaults to `grok-4.20-0309-non-reasoning`)

### Configuration File

The configuration is stored in a TOML file at:
- **Linux/macOS**: `~/.config/shai/config.toml`
- **Windows**: `%AppData%\shai\config.toml`

Example `config.toml`:

```toml
api_key = "your-api-key-here"
api_url = "https://api.x.ai/v1"
model = "grok-4.20-0309-non-reasoning"
```

## Usage

```bash
shai find all pdf files larger than 10MB
shai list all docker containers and show their resource usage
shai replace all occurrences of 'foo' with 'bar' in all .txt files
```

## Interactive Options

- **Run**: Executes the suggested command directly.
- **Explain**: Provides a detailed breakdown of the command.
- **Refine**: Allows you to provide feedback to get an updated suggestion.
- **Cancel**: Exits without running anything.
