use std::process::Command;

use crate::model::{Agent, Session};
use crate::shell::CommandShell;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TmuxPane {
    pub session_id: String,
    pub window_id: String,
    pub pane_id: String,
    pub current_path: String,
    pub current_command: String,
    pub pane_title: String,
    pub window_name: String,
}

pub fn focus_command_for_session(session: &Session, shell: CommandShell) -> Option<String> {
    std::env::var_os("TMUX")?;

    let output = Command::new("tmux")
        .args([
            "list-panes",
            "-a",
            "-F",
            "#{session_id}\t#{window_id}\t#{pane_id}\t#{pane_current_path}\t#{pane_current_command}\t#{pane_title}\t#{window_name}",
        ])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok()?;
    focus_command_for_panes(session, &parse_panes(&stdout), shell)
}

pub fn focus_command_for_panes(
    session: &Session,
    panes: &[TmuxPane],
    shell: CommandShell,
) -> Option<String> {
    let pane = panes
        .iter()
        .find(|pane| pane_matches_session(pane, session))?;
    Some(format!(
        "tmux switch-client -t {} ';' select-window -t {} ';' select-pane -t {}",
        shell.quote(&pane.session_id),
        shell.quote(&pane.window_id),
        shell.quote(&pane.pane_id)
    ))
}

pub fn parse_panes(output: &str) -> Vec<TmuxPane> {
    output
        .lines()
        .filter_map(|line| {
            let mut parts = line.splitn(7, '\t');
            let session_id = parts.next()?.trim();
            let window_id = parts.next()?.trim();
            let pane_id = parts.next()?.trim();
            let current_path = parts.next()?.trim();
            let current_command = parts.next()?.trim();
            let pane_title = parts.next()?.trim();
            let window_name = parts.next()?.trim();
            if session_id.is_empty()
                || window_id.is_empty()
                || pane_id.is_empty()
                || current_path.is_empty()
                || current_command.is_empty()
            {
                return None;
            }
            Some(TmuxPane {
                session_id: session_id.to_string(),
                window_id: window_id.to_string(),
                pane_id: pane_id.to_string(),
                current_path: current_path.to_string(),
                current_command: current_command.to_string(),
                pane_title: pane_title.to_string(),
                window_name: window_name.to_string(),
            })
        })
        .collect()
}

fn pane_matches_session(pane: &TmuxPane, session: &Session) -> bool {
    if session.project_path.is_empty() {
        return false;
    }
    same_path(&pane.current_path, &session.project_path)
        && (command_matches_agent(&pane.current_command, session.agent)
            || title_matches_agent(&pane.pane_title, session.agent)
            || title_matches_agent(&pane.window_name, session.agent))
}

fn same_path(left: &str, right: &str) -> bool {
    trim_trailing_separators(left) == trim_trailing_separators(right)
}

fn trim_trailing_separators(path: &str) -> &str {
    let trimmed = path.trim_end_matches(std::path::MAIN_SEPARATOR);
    if trimmed.is_empty() {
        path
    } else {
        trimmed
    }
}

fn command_matches_agent(command: &str, agent: Agent) -> bool {
    let command = std::path::Path::new(command)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(command);
    let cli = agent.cli_name();
    command == cli
        || command.strip_suffix(".exe") == Some(cli)
        || command
            .strip_prefix(cli)
            .map(|suffix| suffix.starts_with('-') || suffix.starts_with('_'))
            .unwrap_or(false)
}

fn title_matches_agent(title: &str, agent: Agent) -> bool {
    let title = title.to_ascii_lowercase();
    let display = agent.to_string().to_ascii_lowercase().replace(' ', "");
    let cli = agent.cli_name();
    title.contains(cli)
        || title.replace(' ', "").contains(&display)
        || (agent == Agent::OpenCode && title.starts_with("oc "))
        || (agent == Agent::OpenCode && title.starts_with("oc |"))
        || (agent == Agent::ClaudeCode && title.contains("claude code"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn session(agent: Agent, path: &str) -> Session {
        Session {
            agent,
            session_id: "sid".to_string(),
            project_name: "proj".to_string(),
            project_path: path.to_string(),
            summaries: Vec::new(),
            timestamp: 1,
            git_branch: None,
            worktree: None,
            recap: None,
        }
    }

    #[test]
    fn parse_panes_ignores_malformed_rows() {
        let panes = parse_panes(
            "$1\t@1\t%1\t/tmp/proj\tcodex\tCodex\twork\nbad\n$1\t@2\t%2\t\tcodex\tCodex\twork\n",
        );

        assert_eq!(
            panes,
            vec![TmuxPane {
                session_id: "$1".to_string(),
                window_id: "@1".to_string(),
                pane_id: "%1".to_string(),
                current_path: "/tmp/proj".to_string(),
                current_command: "codex".to_string(),
                pane_title: "Codex".to_string(),
                window_name: "work".to_string(),
            }]
        );
    }

    #[test]
    fn focus_command_matches_project_and_agent() {
        let panes = parse_panes(
            "$1\t@1\t%1\t/tmp/other\tcodex\tCodex\twork\n$1\t@2\t%2\t/tmp/proj/\topencode\tOpenCode\twork\n",
        );
        let cmd = focus_command_for_panes(
            &session(Agent::OpenCode, "/tmp/proj"),
            &panes,
            CommandShell::Posix,
        );

        assert_eq!(
            cmd.as_deref(),
            Some("tmux switch-client -t '$1' ';' select-window -t '@2' ';' select-pane -t '%2'")
        );
    }

    #[test]
    fn focus_command_matches_agent_wrapper_binary_names() {
        let panes = parse_panes("$1\t@2\t%2\t/tmp/proj\tcodex-aarch64-a\tfeature branch\twork\n");
        let cmd = focus_command_for_panes(
            &session(Agent::Codex, "/tmp/proj"),
            &panes,
            CommandShell::Posix,
        );

        assert!(cmd.is_some());
    }

    #[test]
    fn focus_command_can_match_agent_title_when_shell_is_current_command() {
        let panes = parse_panes("$1\t@2\t%2\t/tmp/proj\tzsh\tOC | Working on policy\twork\n");
        let cmd = focus_command_for_panes(
            &session(Agent::OpenCode, "/tmp/proj"),
            &panes,
            CommandShell::Posix,
        );

        assert!(cmd.is_some());
    }

    #[test]
    fn focus_command_does_not_match_wrong_agent() {
        let panes = parse_panes("$1\t@1\t%1\t/tmp/proj\tcodex\tCodex\twork\n");
        let cmd = focus_command_for_panes(
            &session(Agent::OpenCode, "/tmp/proj"),
            &panes,
            CommandShell::Posix,
        );

        assert!(cmd.is_none());
    }

    #[test]
    fn focus_command_ignores_cwd_independent_sessions() {
        let panes = parse_panes("$1\t@1\t%1\t/tmp/proj\thermes\tHermes\twork\n");
        let cmd = focus_command_for_panes(&session(Agent::Hermes, ""), &panes, CommandShell::Posix);

        assert!(cmd.is_none());
    }
}
