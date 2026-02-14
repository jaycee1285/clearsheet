# Claude Instructions for ClearSheet

## Project Overview
A TUI cheatsheet viewer that reads markdown tables from `~/.config/clearsheet/`.
Built with Rust + Ratatui + crossterm. crates-tui inspired layout: list view
with fuzzy search, table content display. Local binary installed via `cargo install`.

## Key Paths
| Path | Purpose |
|------|---------|
| `src/main.rs` | Entry point, CLI arg parsing, app launch |
| `src/app.rs` | Application state and event loop |
| `src/ui.rs` | Ratatui rendering logic |
| `src/parser.rs` | Markdown table parsing (comrak) |
| `src/sheet.rs` | Cheatsheet data types |
| `~/.config/clearsheet/` | User's cheatsheet markdown files |
| `Cargo.toml` | Rust project manifest |

## Owner
- GitHub: jaycee1285
- User: john

---

## Documentation Requirements

**Update Obsidian after completing work.**

### Project File Location
`~/Sync/JMC/SideProjects/ClearSheet/ClearSheet.md`

### After Completing Tasks
Update the frontmatter:
- `last-completed`: What you just finished
- `next-tasks`: Remove done items, add new ones
- `blockers`: Set if you need human input, otherwise "None"

### Creating New Docs
Put reference docs in `~/Sync/JMC/SideProjects/ClearSheet/` with:
type: reference, parent: "[[ClearSheet]]", created date, and tags.

---

## Build/Run Commands

| Command | Purpose |
|---------|---------|
| `cargo build` | Build debug binary |
| `cargo run` | Run in dev mode (no arg = picker) |
| `cargo run -- yazi` | Open a specific cheatsheet |
| `cargo build --release` | Build release binary |
| `cargo install --path .` | Install to ~/.cargo/bin |

---

## Current State
**Read the Obsidian project file before starting work:**
`~/Sync/JMC/SideProjects/ClearSheet/ClearSheet.md`

That file has the canonical task list, known issues, blockers,
and PORTFOLIO-tagged features. Do not duplicate state here.
