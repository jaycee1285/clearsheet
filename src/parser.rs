use anyhow::{Context, Result};
use comrak::nodes::{AstNode, NodeValue};
use comrak::{Arena, Options, parse_document};
use std::path::Path;

use crate::sheet::{CheatSheet, Section, ShortcutRow};

pub fn parse_cheatsheet(path: &Path) -> Result<CheatSheet> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;

    let name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();

    let mut options = Options::default();
    options.extension.table = true;

    let arena = Arena::new();
    let root = parse_document(&arena, &content, &options);

    let mut title = String::new();
    let mut sections = Vec::new();
    let mut current_header = String::new();

    for node in root.children() {
        match &node.data.borrow().value {
            NodeValue::Heading(h) => {
                let text = extract_text(node);
                if h.level == 1 {
                    title = text;
                } else if h.level == 2 {
                    current_header = text;
                }
            }
            NodeValue::Table(_) => {
                let rows = parse_table_rows(node);
                if !rows.is_empty() {
                    sections.push(Section {
                        header: current_header.clone(),
                        rows,
                    });
                }
            }
            _ => {}
        }
    }

    if title.is_empty() {
        title = name.clone();
    }

    Ok(CheatSheet {
        name,
        title,
        sections,
    })
}

fn parse_table_rows<'a>(table_node: &'a AstNode<'a>) -> Vec<ShortcutRow> {
    let mut rows = Vec::new();

    for row_node in table_node.children() {
        let is_header = matches!(
            &row_node.data.borrow().value,
            NodeValue::TableRow(true)
        );
        if is_header {
            continue;
        }

        let cells: Vec<String> = row_node
            .children()
            .map(|cell| extract_text(cell).trim().to_string())
            .collect();

        if cells.len() >= 2 {
            rows.push(ShortcutRow {
                shortcut: cells[0].clone(),
                description: cells[1].clone(),
            });
        }
    }

    rows
}

fn extract_text<'a>(node: &'a AstNode<'a>) -> String {
    let mut text = String::new();
    collect_text(node, &mut text);
    text
}

fn collect_text<'a>(node: &'a AstNode<'a>, buf: &mut String) {
    match &node.data.borrow().value {
        NodeValue::Text(t) => buf.push_str(t),
        NodeValue::Code(c) => buf.push_str(&c.literal),
        NodeValue::SoftBreak | NodeValue::LineBreak => buf.push(' '),
        _ => {}
    }
    for child in node.children() {
        collect_text(child, buf);
    }
}
