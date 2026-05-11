use crate::model::{Action, Agent, Session};
use crate::shell::CommandShell;

pub fn generate_command(
    session: &Session,
    action: Action,
    new_agent: Option<Agent>,
) -> Option<String> {
    let shell = CommandShell::from_env();
    let quoted_path = shell.quote(&session.project_path);

    match action {
        Action::Resume => Some(resume_with_flags(session, "")),
        Action::NewSession => {
            let agent = new_agent.unwrap_or(session.agent);
            let cmd = agent.new_session_cmd();
            Some(shell.cd_and(&quoted_path, cmd))
        }
        Action::Open => {
            let editor = detect_editor();
            Some(shell.cd_and(&quoted_path, &format!("{editor} .")))
        }
        Action::Cd => Some(shell.cd_only(&quoted_path)),
        Action::Delete | Action::Back | Action::Pin => None,
    }
}

pub fn action_preview(session: &Session, action: Action) -> String {
    match action {
        Action::Resume => resume_invocation(
            session,
            &crate::settings::Settings::load(),
            CommandShell::from_env(),
        ),
        Action::NewSession => "choose agent CLI...".to_string(),
        Action::Open => format!("{} .", detect_editor()),
        Action::Cd => CommandShell::from_env().cd_only(&session.display_path()),
        Action::Pin => "toggle pin".to_string(),
        Action::Delete => "remove session data".to_string(),
        Action::Back => "return to session list".to_string(),
    }
}

/// Detect editor from config, then $EDITOR, then $VISUAL, fallback to "vim".
pub fn detect_editor() -> String {
    let config = crate::settings::Settings::load();
    if let Some(ref editor) = config.editor {
        if !editor.is_empty() {
            return editor.clone();
        }
    }
    if let Ok(editor) = std::env::var("EDITOR") {
        if !editor.is_empty() {
            return editor;
        }
    }
    if let Ok(editor) = std::env::var("VISUAL") {
        if !editor.is_empty() {
            return editor;
        }
    }
    "vim".to_string()
}

pub fn resume_with_flags(session: &Session, flags: &str) -> String {
    let shell = CommandShell::from_env();
    if let Some(cmd) = crate::tmux::focus_command_for_session(session, shell) {
        return cmd;
    }
    resume_with_flags_for_shell_and_settings(
        session,
        flags,
        &crate::settings::Settings::load(),
        shell,
    )
}

pub fn resume_with_flags_for_shell_and_settings(
    session: &Session,
    flags: &str,
    settings: &crate::settings::Settings,
    shell: CommandShell,
) -> String {
    let quoted_path = shell.quote(&session.project_path);
    let base_cmd = resume_invocation(session, settings, shell);
    shell.cd_and(&quoted_path, &format!("{base_cmd}{flags}"))
}

pub fn new_session_with_flags(session: &Session, agent: Agent, flags: &str) -> Option<String> {
    let shell = CommandShell::from_env();
    let quoted_path = shell.quote(&session.project_path);
    let base = agent.new_session_cmd();
    Some(shell.cd_and(&quoted_path, &format!("{base}{flags}")))
}

fn resume_invocation(
    session: &Session,
    settings: &crate::settings::Settings,
    shell: CommandShell,
) -> String {
    let quoted_session_id = shell.quote(&session.session_id);
    let Some(custom) = settings.resume_command_for(session.agent) else {
        return session.agent.resume_cmd_quoted(&quoted_session_id);
    };

    if custom.contains("{session_id}") {
        custom.replace("{session_id}", &quoted_session_id)
    } else {
        format!(
            "{}{}",
            custom.trim_end(),
            session.agent.resume_arg_suffix(&quoted_session_id)
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::model::Agent;

    fn session(agent: Agent) -> Session {
        Session {
            agent,
            session_id: "sid".to_string(),
            project_name: "proj".to_string(),
            project_path: "/tmp/proj".to_string(),
            summaries: Vec::new(),
            timestamp: 1,
            git_branch: None,
            worktree: None,
            recap: None,
        }
    }

    #[test]
    fn resume_with_default_command_uses_agent_resume() {
        let cmd = resume_with_flags_for_shell_and_settings(
            &session(Agent::Codex),
            "",
            &crate::settings::Settings::default(),
            CommandShell::Posix,
        );

        assert_eq!(cmd, "cd '/tmp/proj' && codex resume 'sid'");
    }

    #[test]
    fn resume_with_custom_base_appends_agent_resume_suffix() {
        let mut settings = crate::settings::Settings::default();
        settings.resume_commands = HashMap::from([(
            "opencode".to_string(),
            "OPENCODE_PORT=5020 opencode".to_string(),
        )]);

        let cmd = resume_with_flags_for_shell_and_settings(
            &session(Agent::OpenCode),
            "",
            &settings,
            CommandShell::Posix,
        );

        assert_eq!(
            cmd,
            "cd '/tmp/proj' && OPENCODE_PORT=5020 opencode -s 'sid'"
        );
    }

    #[test]
    fn resume_with_custom_template_replaces_session_id_and_appends_flags() {
        let mut settings = crate::settings::Settings::default();
        settings.resume_commands = HashMap::from([(
            "codex".to_string(),
            "codex --profile work resume {session_id}".to_string(),
        )]);

        let cmd = resume_with_flags_for_shell_and_settings(
            &session(Agent::Codex),
            " --full-auto",
            &settings,
            CommandShell::Posix,
        );

        assert_eq!(
            cmd,
            "cd '/tmp/proj' && codex --profile work resume 'sid' --full-auto"
        );
    }
}
