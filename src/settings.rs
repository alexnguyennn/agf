use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

use crate::model::Agent;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
    #[serde(default)]
    pub sort_by: Option<String>, // "time", "name", "agent"
    #[serde(default)]
    pub max_sessions: Option<usize>, // limit number of sessions loaded
    #[serde(default = "default_summary_search_count")]
    pub summary_search_count: usize, // number of summaries included in fuzzy search (default 5)
    #[serde(default = "default_search_scope")]
    pub search_scope: String, // "name_path" (default) | "all"
    #[serde(default)]
    pub editor: Option<String>, // editor command (e.g. "code", "cursor"). Falls back to $EDITOR/$VISUAL
    #[serde(default)]
    pub pinned_sessions: Vec<String>, // session IDs pinned to top of list
    #[serde(default)]
    pub show_recap: bool, // show Claude Code recap (away_summary) instead of last prompt
    #[serde(default)]
    pub last_view: Option<String>, // "browse" | "project"
    #[serde(default)]
    pub last_session_id: Option<String>, // last selected session id
    #[serde(default)]
    pub last_project_path: Option<String>, // last selected project path
    #[serde(default)]
    pub expanded_projects: Vec<String>, // project paths expanded in project view
    #[serde(default)]
    pub resume_commands: HashMap<String, String>, // per-agent custom resume command bases
    #[serde(default)]
    pub keybindings: HashMap<String, Vec<String>>, // action name -> key specs
}

fn default_summary_search_count() -> usize {
    5
}

fn default_search_scope() -> String {
    "name_path".to_string()
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            sort_by: None,
            max_sessions: None,
            summary_search_count: default_summary_search_count(),
            search_scope: default_search_scope(),
            editor: None,
            pinned_sessions: Vec::new(),
            show_recap: false,
            last_view: None,
            last_session_id: None,
            last_project_path: None,
            expanded_projects: Vec::new(),
            resume_commands: HashMap::new(),
            keybindings: HashMap::new(),
        }
    }
}

impl Settings {
    pub fn config_path() -> PathBuf {
        config_path()
    }
}

impl Settings {
    pub fn load() -> Self {
        let path = config_path();
        match fs::read_to_string(&path) {
            Ok(content) => parse_settings_or_default(&path, &content),
            Err(e) if e.kind() == ErrorKind::NotFound => legacy_config_path()
                .and_then(|legacy_path| {
                    fs::read_to_string(&legacy_path)
                        .ok()
                        .map(|content| parse_settings_or_default(&legacy_path, &content))
                })
                .unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    /// Persist settings to config.toml.
    pub fn save_editable(&self) {
        let path = config_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        // Load existing config and merge editable fields
        let mut existing: toml::Table = read_config_table(&path)
            .or_else(|| {
                legacy_config_path().and_then(|legacy_path| read_config_table(&legacy_path))
            })
            .unwrap_or_default();

        existing.insert(
            "search_scope".to_string(),
            toml::Value::String(self.search_scope.clone()),
        );
        existing.insert(
            "summary_search_count".to_string(),
            toml::Value::Integer(self.summary_search_count as i64),
        );
        if !self.pinned_sessions.is_empty() {
            existing.insert(
                "pinned_sessions".to_string(),
                toml::Value::Array(
                    self.pinned_sessions
                        .iter()
                        .map(|s| toml::Value::String(s.clone()))
                        .collect(),
                ),
            );
        } else {
            existing.remove("pinned_sessions");
        }
        if self.show_recap {
            existing.insert("show_recap".to_string(), toml::Value::Boolean(true));
        } else {
            existing.remove("show_recap");
        }
        match self.last_view.as_deref() {
            Some("project") => {
                existing.insert(
                    "last_view".to_string(),
                    toml::Value::String("project".to_string()),
                );
            }
            _ => {
                existing.remove("last_view");
            }
        }
        if let Some(session_id) = self
            .last_session_id
            .as_ref()
            .filter(|session_id| !session_id.is_empty())
        {
            existing.insert(
                "last_session_id".to_string(),
                toml::Value::String(session_id.clone()),
            );
        } else {
            existing.remove("last_session_id");
        }
        if let Some(project_path) = self
            .last_project_path
            .as_ref()
            .filter(|project_path| !project_path.is_empty())
        {
            existing.insert(
                "last_project_path".to_string(),
                toml::Value::String(project_path.clone()),
            );
        } else {
            existing.remove("last_project_path");
        }
        if !self.expanded_projects.is_empty() {
            existing.insert(
                "expanded_projects".to_string(),
                toml::Value::Array(
                    self.expanded_projects
                        .iter()
                        .map(|path| toml::Value::String(path.clone()))
                        .collect(),
                ),
            );
        } else {
            existing.remove("expanded_projects");
        }

        let content = editable_config_content(&existing);
        let tmp = path.with_extension("toml.tmp");
        if fs::write(&tmp, &content).is_ok() {
            let _ = fs::rename(&tmp, &path);
        }
    }

    pub fn resume_command_for(&self, agent: Agent) -> Option<&str> {
        self.resume_commands
            .get(agent.cli_name())
            .or_else(|| {
                let display_key = normalized_agent_key(&agent.to_string());
                self.resume_commands.get(&display_key)
            })
            .map(String::as_str)
    }
}

fn config_path() -> PathBuf {
    config_base_dir().join("agf").join("config.toml")
}

fn config_base_dir() -> PathBuf {
    config_base_dir_from(
        std::env::var_os("XDG_CONFIG_HOME"),
        dirs::home_dir().unwrap_or_default(),
    )
}

fn config_base_dir_from(xdg_config_home: Option<OsString>, home_dir: PathBuf) -> PathBuf {
    xdg_config_home
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| home_dir.join(".config"))
}

fn legacy_config_path() -> Option<PathBuf> {
    let current = config_path();
    dirs::config_dir()
        .map(|dir| dir.join("agf").join("config.toml"))
        .filter(|path| path != &current)
}

fn parse_settings_or_default(path: &std::path::Path, content: &str) -> Settings {
    match toml::from_str::<Settings>(content) {
        Ok(settings) => settings,
        Err(e) => {
            eprintln!(
                "[agf] config parse error at {}: {e} — using defaults",
                path.display()
            );
            Settings::default()
        }
    }
}

fn read_config_table(path: &std::path::Path) -> Option<toml::Table> {
    fs::read_to_string(path).ok().and_then(|c| c.parse().ok())
}

fn normalized_agent_key(name: &str) -> String {
    name.chars()
        .filter(|c| !c.is_whitespace())
        .flat_map(char::to_lowercase)
        .collect()
}

fn editable_config_content(existing: &toml::Table) -> String {
    format!("{CONFIG_COMMENTS}{}", existing)
}

const CONFIG_COMMENTS: &str = r#"# agf config.
# Keybindings are optional overrides. Omitted actions keep the built-in defaults.
# See docs/keybindings.md for all action names and supported key syntax.
#
# [keybindings]
# focus_search = ["/", ":"]
# move_up = ["Up", "k", "Ctrl+K", "Ctrl+P"]
# move_down = ["Down", "j", "Ctrl+J", "Ctrl+N"]
# move_left = ["Left", "h", "Ctrl+H"]
# move_right = ["Right", "l", "Ctrl+L"]
# half_page_down = ["Ctrl+D"]
# half_page_up = ["Ctrl+U"]
# page_down = ["Ctrl+F"]
# page_up = ["Ctrl+B"]
# jump_top = ["gg"]
# jump_bottom = ["G"]
# project_view = ["Ctrl+G"]
# bulk_delete = ["D"]
# clear_search = ["Ctrl+U"]
# delete_search_word = ["Ctrl+W"]

"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_parse_resume_commands_and_last_view() {
        let settings: Settings = toml::from_str(
            r#"
last_view = "project"
last_session_id = "session-two"
last_project_path = "/tmp/two"
expanded_projects = ["/tmp/two", "/tmp/three"]

[resume_commands]
opencode = "OPENCODE_PORT=5020 opencode"
codex = "codex --config model=gpt-5.4"

[keybindings]
focus_search = ["Ctrl+F"]
jump_top = ["Home"]
"#,
        )
        .unwrap();

        assert_eq!(settings.last_view.as_deref(), Some("project"));
        assert_eq!(settings.last_session_id.as_deref(), Some("session-two"));
        assert_eq!(settings.last_project_path.as_deref(), Some("/tmp/two"));
        assert_eq!(
            settings.expanded_projects,
            vec!["/tmp/two".to_string(), "/tmp/three".to_string()]
        );
        assert_eq!(
            settings.resume_command_for(Agent::OpenCode),
            Some("OPENCODE_PORT=5020 opencode")
        );
        assert_eq!(
            settings.resume_command_for(Agent::Codex),
            Some("codex --config model=gpt-5.4")
        );
        assert_eq!(
            settings.keybindings.get("focus_search"),
            Some(&vec!["Ctrl+F".to_string()])
        );
        assert_eq!(
            settings.keybindings.get("jump_top"),
            Some(&vec!["Home".to_string()])
        );
    }

    #[test]
    fn editable_config_content_includes_keybinding_reference_comments() {
        let mut table = toml::Table::new();
        table.insert(
            "search_scope".to_string(),
            toml::Value::String("all".to_string()),
        );

        let content = editable_config_content(&table);

        assert!(content.contains("See docs/keybindings.md"));
        assert!(content.contains("# focus_search = [\"/\", \":\"]"));
        assert!(content.contains("search_scope = \"all\""));
    }

    #[test]
    fn config_base_dir_uses_xdg_or_home_dot_config() {
        assert_eq!(
            config_base_dir_from(
                Some(OsString::from("/tmp/xdg-config")),
                PathBuf::from("/home/alex"),
            ),
            PathBuf::from("/tmp/xdg-config")
        );
        assert_eq!(
            config_base_dir_from(None, PathBuf::from("/home/alex")),
            PathBuf::from("/home/alex/.config")
        );
    }
}
