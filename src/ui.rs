use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use ratatui::Frame;

use crate::app::{App, AppMode};

pub fn render(frame: &mut Frame, app: &mut App) {
    match app.mode {
        AppMode::Browser => render_browser(frame, app),
        AppMode::Viewer => render_viewer(frame, app),
    }
}

fn render_browser(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    // Main layout: list (30%) | content (70%)
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);

    // Left panel: sheet list
    let items: Vec<ListItem> = app
        .sheets
        .iter()
        .map(|s| ListItem::new(s.name.as_str()))
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Cheatsheets "),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    let mut list_state = ListState::default();
    list_state.select(Some(app.selected_sheet));
    frame.render_stateful_widget(list, chunks[0], &mut list_state);

    // Right panel: selected sheet content
    render_sheet_content(frame, app, chunks[1]);
}

fn render_viewer(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    render_sheet_content(frame, app, area);
}

fn render_sheet_content(frame: &mut Frame, app: &App, area: Rect) {
    let sheet = match app.current_sheet() {
        Some(s) => s,
        None => {
            let msg = Paragraph::new("No cheatsheet selected")
                .block(Block::default().borders(Borders::ALL));
            frame.render_widget(msg, area);
            return;
        }
    };

    // Split: content area + status bar
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);

    let content_area = chunks[0];
    let status_area = chunks[1];

    // Build content lines
    let mut lines: Vec<ContentLine> = Vec::new();

    // Title
    lines.push(ContentLine::Title(sheet.title.clone()));
    lines.push(ContentLine::Empty);

    for section in &sheet.sections {
        // Section header
        lines.push(ContentLine::SectionHeader(section.header.clone()));
        lines.push(ContentLine::Empty);

        // Table header
        lines.push(ContentLine::TableHeader);

        // Table rows (filtered)
        for row in &section.rows {
            if row.matches_query(&app.filter_query) {
                lines.push(ContentLine::TableRow(
                    row.shortcut.clone(),
                    row.description.clone(),
                ));
            }
        }

        lines.push(ContentLine::Empty);
    }

    // Apply scroll offset
    let visible_height = content_area.height as usize;
    let total_lines = lines.len();
    let max_offset = total_lines.saturating_sub(visible_height);
    let offset = app.scroll_offset.min(max_offset);

    let visible_lines: Vec<Line> = lines
        .iter()
        .skip(offset)
        .take(visible_height)
        .map(|line| match line {
            ContentLine::Title(t) => Line::from(Span::styled(
                format!(" {}", t),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )),
            ContentLine::SectionHeader(h) => Line::from(Span::styled(
                format!(" ## {}", h),
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            )),
            ContentLine::TableHeader => Line::from(vec![
                Span::styled(
                    format!(" {:<30}", "Shortcut"),
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(
                    "Description",
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::UNDERLINED),
                ),
            ]),
            ContentLine::TableRow(shortcut, description) => Line::from(vec![
                Span::styled(
                    format!(" {:<30}", shortcut),
                    Style::default().fg(Color::Cyan),
                ),
                Span::raw(description.as_str()),
            ]),
            ContentLine::Empty => Line::from(""),
        })
        .collect();

    let content = Paragraph::new(visible_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" {} ", sheet.name)),
    );
    frame.render_widget(content, content_area);

    // Status bar
    let status = if app.filtering {
        Line::from(vec![
            Span::styled(" /", Style::default().fg(Color::Yellow)),
            Span::raw(&app.filter_query),
            Span::styled("_", Style::default().fg(Color::Yellow)),
            Span::raw("  "),
            Span::styled("[Esc] cancel", Style::default().fg(Color::DarkGray)),
        ])
    } else {
        let hints = match app.mode {
            AppMode::Browser => {
                " j/k: navigate  Enter: open  /: filter  q: quit"
            }
            AppMode::Viewer => {
                " j/k: scroll  /: filter  q: quit"
            }
        };
        Line::from(Span::styled(hints, Style::default().fg(Color::DarkGray)))
    };

    frame.render_widget(Paragraph::new(status), status_area);
}

enum ContentLine {
    Title(String),
    SectionHeader(String),
    TableHeader,
    TableRow(String, String),
    Empty,
}
