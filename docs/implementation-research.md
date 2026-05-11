# agf Implementation Research

Generated during the missing-features implementation pass.

## Architecture

- `src/main.rs` owns CLI parsing, cache loading, TUI startup, and command delivery through `deliver_command`.
- `src/tui/mod.rs` owns in-memory UI state, key handling, project grouping, pin toggling, settings persistence, and generated command selection.
- `src/action.rs` turns a selected `Session` plus `Action` into shell commands.
- `src/model.rs` owns core data types, agent metadata, built-in resume commands, and action labels.
- `src/settings.rs` loads and writes `~/.config/agf/config.toml`; `save_editable` preserves unknown config keys while updating UI-owned settings.
- `src/cache.rs` stores scanned sessions in `~/.cache/agf/sessions.json` and streams stale scans into the running TUI.

## Implementation Seams

- Resume command customisation belongs in `action.rs` with agent-specific command construction kept in `model.rs`.
- Durable UI preferences belong in `Settings`; transient modal state should stay inside `App`.
- Project view already exists as `Mode::GroupedBrowse`; feature work should improve persistence, pin awareness, and direct key actions.
- Most TUI behavior is difficult to exercise through SLT events, so new behavior should expose pure helper methods where practical and unit-test those helpers directly.

## Verification

- Local compile/test command in this environment is:

```bash
mise exec rust@stable -- cargo test
```

- `cargo` is not directly on PATH in the Codex shell, but `mise` has Rust installed.
