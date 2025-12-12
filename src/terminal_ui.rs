//! Terminal UI module for Fusabi.
//!
//! Provides Ratatui/TUI widgets and helpers for building terminal user interfaces.

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::io::Stdout;

use crate::error::{Error, Result};
use fusabi_host::Value;

/// Terminal UI state container.
pub struct TerminalUI {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TerminalUI {
    /// Create a new terminal UI instance.
    pub fn new() -> Result<Self> {
        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).map_err(|e| Error::TerminalUI(e.to_string()))?;

        Ok(Self { terminal })
    }

    /// Get the terminal size.
    pub fn size(&self) -> Result<(u16, u16)> {
        let size = self
            .terminal
            .size()
            .map_err(|e| Error::TerminalUI(e.to_string()))?;
        Ok((size.width, size.height))
    }

    /// Clear the terminal.
    pub fn clear(&mut self) -> Result<()> {
        self.terminal
            .clear()
            .map_err(|e| Error::TerminalUI(e.to_string()))
    }
}

/// Convert a Fusabi Value to a styled text span.
pub fn value_to_span(value: &Value) -> Span<'static> {
    match value {
        Value::Null => Span::styled("null", Style::default().fg(Color::DarkGray)),
        Value::Bool(b) => Span::styled(
            if *b { "true" } else { "false" },
            Style::default().fg(Color::Yellow),
        ),
        Value::Int(n) => Span::styled(n.to_string(), Style::default().fg(Color::Cyan)),
        Value::Float(f) => Span::styled(f.to_string(), Style::default().fg(Color::Cyan)),
        Value::String(s) => Span::styled(format!("\"{}\"", s), Style::default().fg(Color::Green)),
        Value::List(items) => {
            let content = format!("[{} items]", items.len());
            Span::styled(content, Style::default().fg(Color::Magenta))
        }
        Value::Map(map) => {
            let content = format!("{{{} entries}}", map.len());
            Span::styled(content, Style::default().fg(Color::Blue))
        }
        Value::Bytes(b) => {
            let content = format!("<{} bytes>", b.len());
            Span::styled(content, Style::default().fg(Color::Red))
        }
        Value::Function(_) => Span::styled("<function>", Style::default().fg(Color::Yellow)),
        Value::Error(e) => Span::styled(format!("error: {}", e), Style::default().fg(Color::Red)),
    }
}

/// Create a bordered block with a title.
pub fn titled_block(title: &str) -> Block<'_> {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
}

/// Create a list widget from values.
pub fn value_list<'a>(values: &'a [Value]) -> List<'a> {
    let items: Vec<ListItem<'a>> = values
        .iter()
        .map(|v| ListItem::new(value_to_span(v)))
        .collect();

    List::new(items)
        .block(titled_block("Values"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
}

/// Create a simple status bar.
pub fn status_bar<'a>(status: &'a str) -> Paragraph<'a> {
    Paragraph::new(status).style(Style::default().fg(Color::White).bg(Color::DarkGray))
}

/// Common key event handling.
pub fn is_quit_key(event: &KeyEvent) -> bool {
    matches!(
        (event.code, event.modifiers),
        (KeyCode::Char('q'), _) | (KeyCode::Char('c'), KeyModifiers::CONTROL)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_to_span() {
        let span = value_to_span(&Value::Int(42));
        assert_eq!(span.content.as_ref(), "42");
    }

    #[test]
    fn test_titled_block() {
        let block = titled_block("Test");
        // Just verify it doesn't panic
    }
}
