# Keybinding Reference

agf reads optional key overrides from `~/.config/agf/config.toml`:

```toml
[keybindings]
focus_search = ["/", ":"]
move_down = ["Down", "j", "Ctrl+J", "Ctrl+N"]
jump_top = ["gg"]
```

Each action maps to an array of text key specs. If an action is omitted, agf uses its built-in default. Set an action to an empty array to disable it.

## Key Syntax

- Printable keys: `"j"`, `"k"`, `"G"`, `"/"`, `"?"`, `"["`, `"]"`.
- Key sequences: `"gg"` for two key presses in order.
- Control keys: `"Ctrl+D"`, `"Ctrl+U"`, `"Ctrl+F"`, `"Ctrl+B"`, `"Ctrl+W"`, `"Ctrl+G"`.
- Modifiers: `Ctrl`, `Control`, `Alt`, `Option`, `Shift`, `Super`, `Cmd`, `Command`, `Meta`, `Hyper`.
- Named keys: `Enter`, `Esc`, `Tab`, `BackTab`, `Backspace`, `Delete`, `Insert`, `Up`, `Down`, `Left`, `Right`, `Home`, `End`, `PageUp`, `PageDown`, `Space`, `F1` through `F12`.

The named key set follows SuperlightTUI's `KeyCode` model. See the local crate reference in `superlighttui` / `slt` docs, especially `KeyCode` and `KeyModifiers`.

## Actions

| Action | Default keys | Used for |
|:--|:--|:--|
| `focus_search` | `["/", ":"]` | Focus fuzzy search in browse mode |
| `clear_search` | `["Ctrl+U"]` | Clear focused search |
| `delete_search_word` | `["Ctrl+W"]` | Delete previous search word |
| `move_up` | `["Up", "k", "Ctrl+K", "Ctrl+P"]` | Move up in lists and menus |
| `move_down` | `["Down", "j", "Ctrl+J", "Ctrl+N"]` | Move down in lists and menus |
| `move_left` | `["Left", "h", "Ctrl+H"]` | Back/left movement where supported |
| `move_right` | `["Right", "l", "Ctrl+L"]` | Detail/right movement where supported |
| `half_page_down` | `["Ctrl+D"]` | Move down half a page in browse mode |
| `half_page_up` | `["Ctrl+U"]` | Move up half a page in browse mode |
| `page_down` | `["Ctrl+F"]` | Move down one page in browse mode |
| `page_up` | `["Ctrl+B"]` | Move up one page in browse mode |
| `jump_top` | `["gg"]` | Jump to top |
| `jump_bottom` | `["G"]` | Jump to bottom |
| `project_view` | `["Ctrl+G"]` | Toggle project view |
| `bulk_delete` | `["D"]` | Enter bulk delete mode |
| `summary_prev` | `["["]` | Previous summary |
| `summary_next` | `["]"]` | Next summary |
| `sort` | `["Ctrl+S"]` | Cycle sort mode |
| `pin` | `["p"]` | Pin or unpin in project view |
| `toggle_selection` | `["Space"]` | Toggle project expansion or bulk-delete selection |
| `select` | `["Enter", "l"]` | Select/confirm in menus and preview |
| `back` | `["Esc", "h", "Left"]` | Back from menus, preview, and project view |
| `help` | `["?"]` | Open help |
