# agf Ambiguities

Generated during the missing-features implementation pass.

## Vim Bindings

Plain `j`, `k`, `h`, and `l` conflict with agf's "type anything to search" model. I treated plain vim navigation as active only when the search query is empty in the main session list, and always active in non-text modes such as project view, preview, and menus.

## Ctrl+W

Interpreted as deleting the word before the search cursor in the main search box. Existing `Ctrl+U` remains the full clear binding.

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
