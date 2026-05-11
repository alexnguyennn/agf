use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BindingAction {
    Back,
    BulkDelete,
    ClearSearch,
    DeleteSearchWord,
    FocusSearch,
    HalfPageDown,
    HalfPageUp,
    Help,
    JumpBottom,
    JumpTop,
    MoveDown,
    MoveLeft,
    MoveRight,
    MoveUp,
    PageDown,
    PageUp,
    Pin,
    ProjectView,
    Select,
    Sort,
    SummaryNext,
    SummaryPrev,
    ToggleSelection,
}

impl BindingAction {
    pub fn config_key(self) -> &'static str {
        match self {
            Self::Back => "back",
            Self::BulkDelete => "bulk_delete",
            Self::ClearSearch => "clear_search",
            Self::DeleteSearchWord => "delete_search_word",
            Self::FocusSearch => "focus_search",
            Self::HalfPageDown => "half_page_down",
            Self::HalfPageUp => "half_page_up",
            Self::Help => "help",
            Self::JumpBottom => "jump_bottom",
            Self::JumpTop => "jump_top",
            Self::MoveDown => "move_down",
            Self::MoveLeft => "move_left",
            Self::MoveRight => "move_right",
            Self::MoveUp => "move_up",
            Self::PageDown => "page_down",
            Self::PageUp => "page_up",
            Self::Pin => "pin",
            Self::ProjectView => "project_view",
            Self::Select => "select",
            Self::Sort => "sort",
            Self::SummaryNext => "summary_next",
            Self::SummaryPrev => "summary_prev",
            Self::ToggleSelection => "toggle_selection",
        }
    }

    fn from_config_key(key: &str) -> Option<Self> {
        Some(match key {
            "back" => Self::Back,
            "bulk_delete" => Self::BulkDelete,
            "clear_search" => Self::ClearSearch,
            "delete_search_word" => Self::DeleteSearchWord,
            "focus_search" => Self::FocusSearch,
            "half_page_down" => Self::HalfPageDown,
            "half_page_up" => Self::HalfPageUp,
            "help" => Self::Help,
            "jump_bottom" => Self::JumpBottom,
            "jump_top" => Self::JumpTop,
            "move_down" => Self::MoveDown,
            "move_left" => Self::MoveLeft,
            "move_right" => Self::MoveRight,
            "move_up" => Self::MoveUp,
            "page_down" => Self::PageDown,
            "page_up" => Self::PageUp,
            "pin" => Self::Pin,
            "project_view" => Self::ProjectView,
            "select" => Self::Select,
            "sort" => Self::Sort,
            "summary_next" => Self::SummaryNext,
            "summary_prev" => Self::SummaryPrev,
            "toggle_selection" => Self::ToggleSelection,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyChord {
    code: slt::KeyCode,
    modifiers: Option<slt::KeyModifiers>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeySequence {
    chords: Vec<KeyChord>,
    display: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingKeySequence {
    action: BindingAction,
    remaining: Vec<KeyChord>,
}

#[derive(Debug, Clone)]
pub struct KeyBindings {
    bindings: HashMap<BindingAction, Vec<KeySequence>>,
}

impl KeyBindings {
    pub fn with_overrides(overrides: &HashMap<String, Vec<String>>) -> Self {
        let mut bindings = default_bindings();
        for (action_name, key_specs) in overrides {
            let Some(action) = BindingAction::from_config_key(action_name) else {
                continue;
            };
            let parsed = key_specs
                .iter()
                .filter_map(|spec| parse_key_sequence(spec).ok())
                .collect::<Vec<_>>();
            bindings.insert(action, parsed);
        }
        Self { bindings }
    }

    pub fn consume(
        &self,
        ui: &mut slt::Context,
        action: BindingAction,
        pending: &mut Option<PendingKeySequence>,
    ) -> bool {
        if let Some(active) = pending.as_ref().filter(|active| active.action == action) {
            if let Some(next) = active.remaining.first() {
                if let Some(event_idx) = matching_key_event(ui, next) {
                    ui.consume_event(event_idx);
                    let remaining = active.remaining[1..].to_vec();
                    if remaining.is_empty() {
                        *pending = None;
                        return true;
                    }
                    *pending = Some(PendingKeySequence { action, remaining });
                    return false;
                }
            }
            if any_key_press(ui) {
                *pending = None;
            }
        }

        let Some(sequences) = self.bindings.get(&action) else {
            return false;
        };
        for sequence in sequences {
            let Some(first) = sequence.chords.first() else {
                continue;
            };
            if let Some(event_idx) = matching_key_event(ui, first) {
                ui.consume_event(event_idx);
                if sequence.chords.len() == 1 {
                    *pending = None;
                    return true;
                }
                *pending = Some(PendingKeySequence {
                    action,
                    remaining: sequence.chords[1..].to_vec(),
                });
                return false;
            }
        }
        false
    }

    pub fn label(&self, action: BindingAction) -> String {
        self.bindings
            .get(&action)
            .and_then(|bindings| bindings.first())
            .map(|binding| binding.display.clone())
            .unwrap_or_else(|| action.config_key().to_string())
    }
}

fn default_bindings() -> HashMap<BindingAction, Vec<KeySequence>> {
    [
        (BindingAction::Back, &["Esc", "h", "Left"][..]),
        (BindingAction::BulkDelete, &["D"][..]),
        (BindingAction::ClearSearch, &["Ctrl+U"][..]),
        (BindingAction::DeleteSearchWord, &["Ctrl+W"][..]),
        (BindingAction::FocusSearch, &["/", ":"][..]),
        (BindingAction::HalfPageDown, &["Ctrl+D"][..]),
        (BindingAction::HalfPageUp, &["Ctrl+U"][..]),
        (BindingAction::Help, &["?"][..]),
        (BindingAction::JumpBottom, &["G"][..]),
        (BindingAction::JumpTop, &["gg"][..]),
        (
            BindingAction::MoveDown,
            &["Down", "j", "Ctrl+J", "Ctrl+N"][..],
        ),
        (BindingAction::MoveLeft, &["Left", "h", "Ctrl+H"][..]),
        (BindingAction::MoveRight, &["Right", "l", "Ctrl+L"][..]),
        (BindingAction::MoveUp, &["Up", "k", "Ctrl+K", "Ctrl+P"][..]),
        (BindingAction::PageDown, &["Ctrl+F"][..]),
        (BindingAction::PageUp, &["Ctrl+B"][..]),
        (BindingAction::Pin, &["p"][..]),
        (BindingAction::ProjectView, &["Ctrl+G"][..]),
        (BindingAction::Select, &["Enter", "l"][..]),
        (BindingAction::Sort, &["Ctrl+S"][..]),
        (BindingAction::SummaryNext, &["]"][..]),
        (BindingAction::SummaryPrev, &["["][..]),
        (BindingAction::ToggleSelection, &["Space"][..]),
    ]
    .into_iter()
    .map(|(action, specs)| {
        (
            action,
            specs
                .iter()
                .map(|spec| parse_key_sequence(spec).expect("default keybinding must parse"))
                .collect(),
        )
    })
    .collect()
}

fn parse_key_sequence(spec: &str) -> Result<KeySequence, String> {
    let spec = spec.trim();
    if spec.is_empty() {
        return Err("empty keybinding".to_string());
    }

    if let Ok(chord) = parse_key_chord(spec) {
        return Ok(KeySequence {
            chords: vec![chord],
            display: spec.to_string(),
        });
    }

    if spec.contains('+') {
        return Err(format!("invalid keybinding `{spec}`"));
    }

    let chars = spec.chars().collect::<Vec<_>>();
    if chars.len() > 4 {
        return Err(format!("unknown key `{spec}`"));
    }

    let chords = chars
        .into_iter()
        .map(|ch| KeyChord {
            code: slt::KeyCode::Char(ch),
            modifiers: None,
        })
        .collect::<Vec<_>>();
    if chords.is_empty() {
        return Err("empty keybinding".to_string());
    }

    Ok(KeySequence {
        chords,
        display: spec.to_string(),
    })
}

fn parse_key_chord(spec: &str) -> Result<KeyChord, String> {
    let parts = spec.split('+').map(str::trim).collect::<Vec<_>>();
    let key = parts
        .last()
        .copied()
        .filter(|part| !part.is_empty())
        .ok_or_else(|| format!("invalid keybinding `{spec}`"))?;

    let mut modifiers = slt::KeyModifiers::NONE;
    for modifier in &parts[..parts.len().saturating_sub(1)] {
        modifiers = slt::KeyModifiers(modifiers.0 | parse_modifier(modifier)?.0);
    }
    let explicit_modifiers = (modifiers != slt::KeyModifiers::NONE).then_some(modifiers);

    let code = parse_key_code(key, explicit_modifiers)?;
    Ok(KeyChord {
        code,
        modifiers: explicit_modifiers,
    })
}

fn parse_modifier(modifier: &str) -> Result<slt::KeyModifiers, String> {
    match normalized_name(modifier).as_str() {
        "ctrl" | "control" | "c" => Ok(slt::KeyModifiers::CONTROL),
        "alt" | "option" | "opt" => Ok(slt::KeyModifiers::ALT),
        "shift" | "s" => Ok(slt::KeyModifiers::SHIFT),
        "super" | "cmd" | "command" => Ok(slt::KeyModifiers::SUPER),
        "meta" => Ok(slt::KeyModifiers::META),
        "hyper" => Ok(slt::KeyModifiers::HYPER),
        _ => Err(format!("unknown modifier `{modifier}`")),
    }
}

fn parse_key_code(key: &str, modifiers: Option<slt::KeyModifiers>) -> Result<slt::KeyCode, String> {
    let normalized = normalized_name(key);
    if let Some(function_key) = normalized.strip_prefix('f') {
        if let Ok(number) = function_key.parse::<u8>() {
            return Ok(slt::KeyCode::F(number));
        }
    }

    let named = match normalized.as_str() {
        "enter" | "return" => Some(slt::KeyCode::Enter),
        "esc" | "escape" => Some(slt::KeyCode::Esc),
        "tab" => Some(slt::KeyCode::Tab),
        "backtab" | "shift-tab" | "shifttab" => Some(slt::KeyCode::BackTab),
        "backspace" | "bs" => Some(slt::KeyCode::Backspace),
        "delete" | "del" => Some(slt::KeyCode::Delete),
        "insert" | "ins" => Some(slt::KeyCode::Insert),
        "up" | "arrowup" => Some(slt::KeyCode::Up),
        "down" | "arrowdown" => Some(slt::KeyCode::Down),
        "left" | "arrowleft" => Some(slt::KeyCode::Left),
        "right" | "arrowright" => Some(slt::KeyCode::Right),
        "home" => Some(slt::KeyCode::Home),
        "end" => Some(slt::KeyCode::End),
        "pageup" | "page-up" | "pgup" => Some(slt::KeyCode::PageUp),
        "pagedown" | "page-down" | "pgdn" => Some(slt::KeyCode::PageDown),
        "null" => Some(slt::KeyCode::Null),
        "menu" => Some(slt::KeyCode::Menu),
        "keypadbegin" | "keypad-begin" => Some(slt::KeyCode::KeypadBegin),
        "space" => Some(slt::KeyCode::Char(' ')),
        _ => None,
    };
    if let Some(code) = named {
        return Ok(code);
    }

    let mut chars = key.chars();
    let Some(mut ch) = chars.next() else {
        return Err("empty key".to_string());
    };
    if chars.next().is_some() {
        return Err(format!("unknown key `{key}`"));
    }
    if modifiers
        .map(|mods| mods.contains(slt::KeyModifiers::CONTROL))
        .unwrap_or(false)
        && ch.is_ascii_alphabetic()
    {
        ch = ch.to_ascii_lowercase();
    }
    Ok(slt::KeyCode::Char(ch))
}

fn normalized_name(value: &str) -> String {
    value
        .chars()
        .filter(|ch| !ch.is_ascii_whitespace() && *ch != '_' && *ch != '-')
        .flat_map(char::to_lowercase)
        .collect()
}

fn matching_key_event(ui: &slt::Context, chord: &KeyChord) -> Option<usize> {
    ui.key_presses_when(true)
        .find_map(|(index, key)| chord_matches_key(chord, key).then_some(index))
}

fn chord_matches_key(chord: &KeyChord, key: &slt::KeyEvent) -> bool {
    if key.code != chord.code {
        return false;
    }
    if let Some(modifiers) = chord.modifiers {
        return key.modifiers.contains(modifiers);
    }
    match chord.code {
        slt::KeyCode::Char(ch) if ch.is_uppercase() => {
            key.modifiers == slt::KeyModifiers::NONE || key.modifiers == slt::KeyModifiers::SHIFT
        }
        _ => key.modifiers == slt::KeyModifiers::NONE,
    }
}

fn any_key_press(ui: &slt::Context) -> bool {
    ui.key_presses_when(true).next().is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_include_recent_vim_and_search_bindings() {
        let bindings = KeyBindings::with_overrides(&HashMap::new());

        assert_eq!(bindings.label(BindingAction::FocusSearch), "/");
        assert_eq!(bindings.label(BindingAction::MoveDown), "Down");
        assert_eq!(bindings.label(BindingAction::JumpTop), "gg");
        assert_eq!(bindings.label(BindingAction::JumpBottom), "G");
        assert_eq!(bindings.label(BindingAction::HalfPageDown), "Ctrl+D");
    }

    #[test]
    fn overrides_replace_defaults_and_empty_array_disables_action() {
        let overrides = HashMap::from([
            (
                "focus_search".to_string(),
                vec!["Ctrl+F".to_string(), "/".to_string()],
            ),
            ("move_down".to_string(), Vec::new()),
        ]);
        let bindings = KeyBindings::with_overrides(&overrides);

        assert_eq!(bindings.label(BindingAction::FocusSearch), "Ctrl+F");
        assert_eq!(bindings.label(BindingAction::MoveDown), "move_down");
    }

    #[test]
    fn parser_accepts_named_control_and_sequence_keys() {
        assert_eq!(
            parse_key_sequence("Ctrl+D").unwrap().chords[0].code,
            slt::KeyCode::Char('d')
        );
        assert_eq!(
            parse_key_sequence("PageDown").unwrap().chords[0].code,
            slt::KeyCode::PageDown
        );
        assert_eq!(parse_key_sequence("gg").unwrap().chords.len(), 2);
    }

    #[test]
    fn plain_char_binding_does_not_match_control_modified_key() {
        let chord = parse_key_sequence("j").unwrap().chords.remove(0);
        let event = slt::Event::key_ctrl('j');
        let ctrl_j = event.as_key().unwrap();

        assert!(!chord_matches_key(&chord, ctrl_j));
    }

    #[test]
    fn invalid_override_key_is_ignored_without_panicking() {
        let overrides = HashMap::from([(
            "jump_top".to_string(),
            vec!["DefinitelyNotAKey".to_string(), "Home".to_string()],
        )]);
        let bindings = KeyBindings::with_overrides(&overrides);

        assert_eq!(bindings.label(BindingAction::JumpTop), "Home");
    }
}
