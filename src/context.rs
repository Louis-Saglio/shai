use std::env;
use std::fs;

const UNKNOWN: &str = "unknown";

/// Minimal description of the user's environment, sent to the LLM so that suggestions
/// fit the real OS and shell (e.g. GNU vs BSD flags). Deliberately contains nothing else:
/// no working directory, username, environment variables or history.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemContext {
    pub os: String,
    pub shell: String,
}

impl SystemContext {
    /// Detects the current environment. Never fails: anything undetectable becomes "unknown".
    pub fn detect() -> Self {
        Self {
            os: detect_os(),
            shell: shell_name(env::var("SHELL").ok().as_deref()),
        }
    }

    /// Line describing the OS and shell, e.g.
    /// `User environment: OS = Arch Linux, shell = zsh.`
    pub fn environment_line(&self) -> String {
        format!(
            "User environment: OS = {}, shell = {}.",
            self.os, self.shell
        )
    }

    /// Appended to the system prompts: the target shell and the environment line.
    pub fn prompt_suffix(&self) -> String {
        format!("Target shell: {}. {}", self.shell, self.environment_line())
    }
}

fn detect_os() -> String {
    let pretty_name = if cfg!(target_os = "linux") {
        fs::read_to_string("/etc/os-release")
            .ok()
            .and_then(|content| parse_os_release(&content))
    } else {
        None
    };
    pretty_name.unwrap_or_else(|| match env::consts::OS {
        "" => UNKNOWN.to_string(),
        os => os.to_string(),
    })
}

/// Extracts `PRETTY_NAME` from the content of an os-release file.
pub fn parse_os_release(content: &str) -> Option<String> {
    content.lines().find_map(|line| {
        let value = line.trim().strip_prefix("PRETTY_NAME=")?;
        let value = value.trim().trim_matches(|c| c == '"' || c == '\'').trim();
        (!value.is_empty()).then(|| value.to_string())
    })
}

/// Basename of the `$SHELL` path, defaulting to "unknown" when unset or empty.
pub fn shell_name(shell_path: Option<&str>) -> String {
    shell_path
        .and_then(|path| path.trim().rsplit('/').next())
        .filter(|name| !name.is_empty())
        .unwrap_or(UNKNOWN)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_quoted_pretty_name() {
        let content = "NAME=\"Arch Linux\"\nPRETTY_NAME=\"Arch Linux\"\nID=arch\n";
        assert_eq!(parse_os_release(content), Some("Arch Linux".to_string()));
    }

    #[test]
    fn parses_unquoted_and_single_quoted_pretty_name() {
        assert_eq!(
            parse_os_release("PRETTY_NAME=Gentoo\n"),
            Some("Gentoo".to_string())
        );
        assert_eq!(
            parse_os_release("PRETTY_NAME='Alpine Linux v3.20'\n"),
            Some("Alpine Linux v3.20".to_string())
        );
    }

    #[test]
    fn ignores_lookalike_keys() {
        assert_eq!(parse_os_release("XPRETTY_NAME=\"nope\"\n"), None);
    }

    #[test]
    fn missing_or_empty_pretty_name_is_none() {
        assert_eq!(parse_os_release(""), None);
        assert_eq!(parse_os_release("NAME=\"Arch Linux\"\n"), None);
        assert_eq!(parse_os_release("PRETTY_NAME=\"\"\n"), None);
    }

    #[test]
    fn shell_name_takes_basename() {
        assert_eq!(shell_name(Some("/usr/bin/zsh")), "zsh");
        assert_eq!(shell_name(Some("/bin/bash")), "bash");
        assert_eq!(shell_name(Some("fish")), "fish");
    }

    #[test]
    fn shell_name_falls_back_to_unknown() {
        assert_eq!(shell_name(None), "unknown");
        assert_eq!(shell_name(Some("")), "unknown");
        assert_eq!(shell_name(Some("/usr/bin/")), "unknown");
    }

    #[test]
    fn environment_line_and_prompt_suffix() {
        let ctx = SystemContext {
            os: "Arch Linux".to_string(),
            shell: "zsh".to_string(),
        };
        assert_eq!(
            ctx.environment_line(),
            "User environment: OS = Arch Linux, shell = zsh."
        );
        assert_eq!(
            ctx.prompt_suffix(),
            "Target shell: zsh. User environment: OS = Arch Linux, shell = zsh."
        );
    }

    #[test]
    fn detect_never_returns_empty_fields() {
        let ctx = SystemContext::detect();
        assert!(!ctx.os.is_empty());
        assert!(!ctx.shell.is_empty());
    }
}
