# StackBuild - ClearSheet

## Dev Stack
- **Language:** Rust (Edition 2024)
- **UI Framework:** Ratatui 0.30.0 (TUI)
- **Terminal Backend:** Crossterm 0.29.0
- **Markdown Parser:** Comrak 0.50.0 (GFM tables)
- **CLI:** Clap 4.5.57
- **Error Handling:** Anyhow 1.0.100

## Target
- **Desktop** (Linux terminal) — cross-platform terminal support via crossterm

## Additional Key Libraries (UI)
- Ratatui widgets: List, Paragraph, Block with color-coded columns (Cyan shortcuts, Magenta headers)
- Crossterm: raw mode, alternate screen, key events

## Key Features
TUI cheatsheet viewer that reads markdown tables from `~/.config/clearsheet/` and displays them in a crates-tui inspired two-panel interface. Open any cheatsheet with `clearsheet <program>` or browse all with a fuzzy picker.

- Unlike web-based cheatsheet tools (devhints.io, cheat.sh) — fully offline, instant launch, no browser needed
- Unlike `tldr` or `cheat` — displays your own custom markdown tables, not community-maintained command snippets
- Parses section headers (H1/H2) with multiple tables per file
- In-sheet search/filter with `/` key

---

## Building Instructions

### Nix develop?
No — no flake.nix or shell.nix. Uses standard Cargo toolchain.

### Dev server?
N/A (TUI app, not web-based)

### Tauri dev server?
N/A (pure Rust, no Tauri)

### Commands to run
```bash
cargo build                    # Debug build
cargo run                      # Run picker mode
cargo run -- yazi              # Open specific cheatsheet
```

---

## Android Build
N/A — terminal application only.

## Desktop Build
```bash
cargo build --release          # Release binary → target/release/clearsheet
cargo install --path .         # Install to ~/.cargo/bin/
```

- **Release script?** No
- **Last build:** 2026-02-05 (debug binary in target/debug/)

## Web Build
N/A — terminal application only.
