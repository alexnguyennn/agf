# agf

[![CI](https://github.com/subinium/agf/actions/workflows/ci.yml/badge.svg)](https://github.com/subinium/agf/actions)
[![Release](https://img.shields.io/github/v/release/subinium/agf?include_prereleases&sort=semver)](https://github.com/subinium/agf/releases)
[![crates.io](https://img.shields.io/crates/v/agf.svg)](https://crates.io/crates/agf)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> Find the AI coding session you meant to resume.

`agf` is a local-first fuzzy finder for AI coding-agent sessions.
Search local sessions across **Claude Code**, **Codex**, **Gemini CLI**, **Cursor CLI**, **OpenCode**, **Kiro**, **pi**, and **Hermes** — then resume the right one in a keystroke.

![agf demo](./assets/demo.gif)

## Install

```bash
cargo install agf
agf setup
agf
```

Requires a Rust toolchain (`rustup` recommended). Prebuilt binaries for macOS, Linux, and Windows are also available on the [Releases page](https://github.com/subinium/agf/releases).

### Quick Resume (no TUI)

```bash
agf resume project-name   # fuzzy-matches and resumes the best match directly
```

## Why agf?

AI coding agents are great at keeping context — until you lose the terminal.

You switch projects, close a tab, forget the session ID, or resume the wrong agent.
Then you either dig through history files or start over.

`agf` gives you one searchable list of local agent sessions and resumes the right one.

## Supported agents

`agf` reads the session files each agent already stores locally. No account, no cloud sync, no extra agent process.

| Agent | Resume command | Local session source |
|:---|:---|:---|
| [Claude Code](https://github.com/anthropics/claude-code) | `claude --resume <id>` | `~/.claude/history.jsonl` + `~/.claude/projects/` |
| [Codex](https://github.com/openai/codex) | `codex resume <id>` | `~/.codex/sessions/**/*.jsonl` |
| [Gemini CLI](https://github.com/google-gemini/gemini-cli) | `gemini --resume <id>` | `~/.gemini/tmp/<project>/chats/session-*.json` |
| [Cursor CLI](https://docs.cursor.com/agent) | `cursor-agent --resume <id>` | `~/.cursor/projects/*/agent-transcripts/*.txt` |
| [OpenCode](https://github.com/opencode-ai/opencode) | `opencode -s <id>` | `~/.local/share/opencode/opencode.db` |
| [Kiro](https://kiro.dev) | `kiro-cli chat --resume` | `~/Library/Application Support/kiro-cli/data.sqlite3` |
| [pi](https://github.com/badlogic/pi-mono) | `pi --resume` | `~/.pi/agent/sessions/<cwd>/*.jsonl` |
| [Hermes](https://github.com/NousResearch/hermes-agent) | `hermes --resume <id>` | `~/.hermes/state.db` |

<details>
<summary>Full session storage paths</summary>

| Agent | Format | Default Path |
|:---|:---|:---|
| Claude Code | JSONL | `~/.claude/history.jsonl` (sessions)<br>`~/.claude/projects/*/` (worktree detection) |
| Codex | JSONL | `~/.codex/sessions/YYYY/MM/DD/rollout-*.jsonl` |
| OpenCode | SQLite | `~/.local/share/opencode/opencode.db` |
| pi | JSONL | `~/.pi/agent/sessions/--<encoded-cwd>--/<ts>_<id>.jsonl` |
| Kiro | SQLite | macOS: `~/Library/Application Support/kiro-cli/data.sqlite3`<br>Linux: `~/.local/share/kiro-cli/data.sqlite3` |
| Cursor CLI | SQLite + TXT | `~/.cursor/chats/*/<id>/store.db`<br>`~/.cursor/projects/*/agent-transcripts/<id>.txt` |
| Gemini | JSON | `~/.gemini/tmp/<project>/chats/session-<date>-<id>.json`<br>`<project>` is a named dir or SHA-256 hash of the project path<br>Project paths resolved via `~/.gemini/projects.json` |
| Hermes | SQLite | `~/.hermes/state.db` (sessions + messages)<br>JSON dumps in `~/.hermes/sessions/session_<id>.json`<br>Hermes is cwd-independent — resume runs in your current shell directory |

</details>

## Features

- **Cross-agent search** — see all supported agents in one list
- **Fuzzy search** — find sessions by project name, path, branch, or summary
- **One-key resume** — resume the selected session with the right agent command
- **Quick resume** — `agf resume <query>` skips the TUI entirely
- **Bulk delete** — `D` to multi-select and clean up stale sessions
- **Project awareness** — group by project, pin important sessions, and surface git branches / Claude Code `--worktree` sessions
- **Tmux aware resume** — when a matching project/agent pane is already open in tmux, agf offers an explicit focus action instead of launching a duplicate

Also supports Unicode/CJK search, mouse navigation, agent filters, permission/approval-mode picker, agent auto-detection, and shell wrappers for zsh, bash, fish, and PowerShell.

## Basic controls

| Key | Action |
|:---|:---|
| `/` / `:` | Focus search |
| `↑` `↓` / `j` `k` / `Ctrl+K` `Ctrl+J` | Navigate |
| `gg` / `G` | Jump to top/bottom |
| `Ctrl+D` `Ctrl+U` | Half-page down/up |
| `Ctrl+F` `Ctrl+B` | Page down/up |
| `Enter` | Open action menu |
| `Tab` / `Shift+Tab` | Cycle agent filter |
| `→` / `l` / `Ctrl+L` | Preview session |
| `Ctrl+G` | Project view |
| `D` | Bulk delete |
| `?` | Help / settings |
| `Esc` | Quit |

<details>
<summary>Full keybindings</summary>

### Browse

| Key | Action |
|:---|:---|
| `/` / `:` | Focus fuzzy search |
| `Esc` while search is focused | Unfocus search |
| `Ctrl+W` while search is focused | Delete previous search word |
| `Ctrl+U` while search is focused | Clear search |
| `↑` `↓` / `j` `k` / `Ctrl+K` `Ctrl+J` | Navigate |
| `gg` / `G` | Jump to top/bottom |
| `Ctrl+D` `Ctrl+U` | Half-page down/up |
| `Ctrl+F` `Ctrl+B` | Page down/up |
| `[` `]` | Cycle session summary |
| `Enter` | Open action menu |
| `→` / `l` / `Ctrl+L` | Preview session details |
| `Tab` / `Shift+Tab` | Cycle agent filter |
| `Ctrl+S` | Cycle sort (time / name / agent) |
| `Ctrl+G` | Toggle project view |
| `D` | Enter bulk delete mode |
| `?` | Help / settings |
| `Esc` | Quit |

### Project View (`Ctrl+G`)

| Key | Action |
|:---|:---|
| `↑` `↓` / `j` `k` | Navigate projects/sessions |
| `gg` / `G` | Jump to top/bottom |
| `Enter` / `Space` | Expand project or open session actions |
| `p` | Pin/unpin the selected project session |
| `Ctrl+G` / `Esc` | Return to flat view |

### Bulk Delete (`D`)

| Key | Action |
|:---|:---|
| `Space` | Toggle selection + move down |
| `↑` `↓` / `j` `k` / `Ctrl+K` `Ctrl+J` | Navigate |
| `Enter` | Confirm deletion (when items selected) |
| `Esc` | Cancel and return to browse |

### New Session (Agent Select)

| Key | Action |
|:---|:---|
| `1`-`9` | Quick select agent |
| `Tab` | Open permission/approval mode picker |
| `Enter` | Launch with default mode |
| `Esc` | Back |

</details>

## Configuration

Optional. Create `~/.config/agf/config.toml`:

```toml
sort_by = "time"            # "time" | "name" | "agent"
max_sessions = 200
search_scope = "name_path"  # "name_path" (default) | "all" (include summaries)
summary_search_count = 5    # number of summaries included when search_scope = "all"
last_view = "project"       # optional: remember project view between launches

[resume_commands]
opencode = "OPENCODE_PORT=5020 opencode"

# Optional keybinding overrides. Omitted actions keep the built-in defaults.
# See docs/keybindings.md for all action names and supported key syntax.
#
# [keybindings]
# focus_search = ["/", ":"]
# move_up = ["Up", "k", "Ctrl+K", "Ctrl+P"]
# move_down = ["Down", "j", "Ctrl+J", "Ctrl+N"]
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
```

You can also edit `search_scope` and `summary_search_count` interactively by pressing `?` in the TUI.
agf also writes `last_session_id`, `last_project_path`, and `expanded_projects` to remember cursor state between launches.

Custom `resume_commands` replace the command base for that agent while agf still appends the normal resume arguments. If a value contains `{session_id}`, agf treats it as a full template and substitutes the quoted session id.
See [docs/keybindings.md](docs/keybindings.md) for configurable keybinding actions and key names.

## Shell integration

`agf setup` auto-detects your shell and installs the wrapper. Supported shells:

- **zsh / bash** — appends to `~/.zshrc` or `~/.bashrc`
- **fish** — writes to `~/.config/fish/config.fish`
- **PowerShell** (Windows or cross-platform `pwsh`) — writes to `$PROFILE.CurrentUserAllHosts` (`Documents\PowerShell\profile.ps1` on Windows, `~/.config/powershell/profile.ps1` elsewhere)

If auto-detection misses your shell, run the matching `agf init` form manually:

```bash
eval "$(agf init zsh)"                               # zsh
eval "$(agf init bash)"                              # bash
agf init fish | source                               # fish
agf init powershell | Out-String | Invoke-Expression # PowerShell
```

After upgrading, run `agf setup` again (or restart your shell) to apply the latest wrapper.
See [CHANGELOG.md](CHANGELOG.md) for release notes.

## Requirements

- macOS, Linux, or Windows (PowerShell 5.1+ / PowerShell 7+)
- One or more of: `claude`, `codex`, `opencode`, `pi`, `kiro-cli`, `cursor-agent`, `gemini`

## Install from source

```bash
git clone https://github.com/subinium/agf.git
cd agf
cargo install --path .
agf setup
```

## Limitations

`agf` works best with agents that store resumable sessions locally.

**Amp** is not supported yet because its sessions are stored remotely, which makes it hard to reliably resolve local project paths from session metadata. We are monitoring upstream changes and will add support when feasible.

## Built with

`agf` is written in Rust and built with [SuperLightTUI (SLT)](https://github.com/subinium/SuperLightTUI) — an immediate-mode terminal UI library for Rust.

## Contributing

Issues and PRs are welcome.

[![Contributors](https://contrib.rocks/image?repo=subinium/agf)](https://github.com/subinium/agf/graphs/contributors)

## License

[MIT](LICENSE)
