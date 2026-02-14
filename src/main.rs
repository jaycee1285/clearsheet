mod app;
mod config;
mod parser;
mod sheet;
mod ui;

use std::io;

use anyhow::{Context, Result};
use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::execute;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::app::{App, AppMode};
use crate::sheet::CheatSheet;

#[derive(Parser)]
#[command(name = "clearsheet", about = "TUI cheatsheet viewer")]
struct Cli {
    /// Name of the cheatsheet to open directly
    sheet: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let sheet_files = config::get_sheet_files()?;
    let sheets: Vec<CheatSheet> = sheet_files
        .iter()
        .filter_map(|path| parser::parse_cheatsheet(path).ok())
        .collect();

    if sheets.is_empty() {
        anyhow::bail!("No cheatsheets found in ~/.config/clearsheet/");
    }

    // Validate requested sheet exists
    if let Some(ref name) = cli.sheet {
        if !sheets.iter().any(|s| s.name.eq_ignore_ascii_case(name)) {
            let available: Vec<&str> = sheets.iter().map(|s| s.name.as_str()).collect();
            anyhow::bail!(
                "Cheatsheet '{}' not found.\nAvailable: {}",
                name,
                available.join(", ")
            );
        }
    }

    enable_raw_mode().context("Failed to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).context("Failed to enter alternate screen")?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("Failed to create terminal")?;

    let mut app = App::new(sheets, cli.sheet);
    let result = run_app(&mut terminal, &mut app);

    disable_raw_mode().context("Failed to disable raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("Failed to leave alternate screen")?;
    terminal.show_cursor().context("Failed to show cursor")?;

    result
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|frame| ui::render(frame, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            if app.filtering {
                match key.code {
                    KeyCode::Esc => {
                        app.filtering = false;
                        app.filter_query.clear();
                    }
                    KeyCode::Backspace => {
                        app.filter_query.pop();
                    }
                    KeyCode::Char(c) => {
                        app.filter_query.push(c);
                    }
                    _ => {}
                }
                continue;
            }

            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => match app.mode {
                    AppMode::Viewer => {
                        app.should_quit = true;
                    }
                    AppMode::Browser => {
                        app.should_quit = true;
                    }
                },
                KeyCode::Char('j') | KeyCode::Down => match app.mode {
                    AppMode::Browser => {
                        if app.selected_sheet + 1 < app.sheets.len() {
                            app.selected_sheet += 1;
                            app.scroll_offset = 0;
                        }
                    }
                    AppMode::Viewer => {
                        app.scroll_offset = app.scroll_offset.saturating_add(1);
                    }
                },
                KeyCode::Char('k') | KeyCode::Up => match app.mode {
                    AppMode::Browser => {
                        app.selected_sheet = app.selected_sheet.saturating_sub(1);
                        app.scroll_offset = 0;
                    }
                    AppMode::Viewer => {
                        app.scroll_offset = app.scroll_offset.saturating_sub(1);
                    }
                },
                KeyCode::Enter => {
                    if matches!(app.mode, AppMode::Browser) {
                        app.mode = AppMode::Viewer;
                        app.scroll_offset = 0;
                    }
                }
                KeyCode::Char('/') => {
                    app.filtering = true;
                    app.filter_query.clear();
                }
                _ => {}
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
