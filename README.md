# shai (Shell AI)

`shai` is a seamless natural language interface for your terminal. Describe what you want to do in plain English, and `shai` will suggest a command, explain it, or refine it before you run it.

## Features

- **Natural Language to Shell**: Converts English descriptions to valid shell commands using xAI Grok.
- **Interactive Flow**: Choose to Run, Explain, Refine, or Cancel suggestions.
- **Shell Support**: Detects and uses your current shell (bash, zsh, etc.).
- **Concise Explanations**: Understand what the suggested command does before executing it.

## Installation

```bash
cargo install --path .
```

## Configuration

Set your xAI API key as an environment variable or in a `.env` file:

```bash
export XAI_API_KEY="your-api-key-here"
```

## Usage

```bash
shai "find all pdf files larger than 10MB"
shai "list all docker containers and show their resource usage"
shai "replace all occurrences of 'foo' with 'bar' in all .txt files"
```

## Interactive Options

- **Run**: Executes the suggested command directly.
- **Explain**: Provides a detailed breakdown of the command.
- **Refine**: Allows you to provide feedback to get an updated suggestion.
- **Cancel**: Exits without running anything.
