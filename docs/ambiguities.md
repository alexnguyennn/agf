# agf Ambiguities

Generated during the missing-features implementation pass.

## Vim Bindings

Plain `j`, `k`, `h`, and `l` conflict with agf's original "type anything to search" model. I replaced that with a focus model: normal mode owns vim navigation, `/` or `:` focuses search, and text entry goes to search only while focused.

## Ctrl+W

Interpreted as deleting the word before the search cursor while search is focused. `Ctrl+U` clears focused search; in normal mode `Ctrl+U` is half-page up and `Ctrl+D` is half-page down.

## Custom Keybindings

Deferred. The focus model removes the main key conflicts without adding a config format. If users still need deeper remapping, a future config should be declarative and mode-specific so search-mode text entry cannot be accidentally shadowed by global bindings.

## Project Pins

The existing data model stores pinned session IDs, not pinned projects. I preserved session-level pins and made project groups pin-aware: projects containing pinned sessions sort before unpinned projects, and grouped child rows can toggle pins directly.

## Tmux Focus

"Already open" is interpreted as a tmux pane in the same project directory whose current command is the selected agent executable. This avoids launching a duplicate agent when a matching project/agent pane already exists. It does not inspect process arguments for exact session IDs because tmux does not expose that portably.

## Custom Resume Args

Interpreted as per-agent configurable resume command bases, for example:

```toml
[resume_commands]
opencode = "OPENCODE_PORT=5020 opencode"
```

agf appends the agent's normal resume arguments unless the configured value contains `{session_id}`, in which case it is treated as a full template.
