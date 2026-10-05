// --- PHÂN ĐOẠN: TEXT & PASSWORD INPUT WIDGET (BO GÓC, KHÔNG NGOẶC VUÔNG) ---

use crate::traits::{EventResult, FormWidget};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Text,
    Password,
}

pub struct InputWidget {
    pub label: String,
    pub value: String,
    pub mode: InputMode,
    focused: bool,
}

impl InputWidget {
    pub fn new(label: impl Into<String>, mode: InputMode) -> Self {
        Self {
            label: label.into(),
            value: String::new(),
            mode,
            focused: false,
        }
    }
}

impl FormWidget for InputWidget {
    fn render(&self, area: Rect, frame: &mut Frame) {
        let display_text = match self.mode {
            InputMode::Text => self.value.clone(),
            InputMode::Password => "*".repeat(self.value.len()),
        };

        let border_color = if self.focused {
            Color::Yellow
        } else {
            Color::DarkGray
        };

        // Sử dụng BorderType::Rounded để bo tròn 4 góc của khung
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(border_color))
            .title(format!(" {} ", self.label));

        // Render trực tiếp chuỗi text mà không cần ký tự bao [ ]
        let paragraph = Paragraph::new(display_text)
            .block(block)
            .style(if self.focused {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            });

        frame.render_widget(paragraph, area);
    }

    fn handle_event(&mut self, key: KeyEvent) -> EventResult {
        if key.kind != KeyEventKind::Press {
            return EventResult::Ignored;
        }

        match key.code {
            KeyCode::Backspace => {
                self.value.pop();
                EventResult::Consumed
            }
            KeyCode::Char(c) => {
                self.value.push(c);
                EventResult::Consumed
            }
            _ => EventResult::Ignored,
        }
    }

    fn focus(&mut self) {
        self.focused = true;
    }

    fn blur(&mut self) {
        self.focused = false;
    }

    fn is_focused(&self) -> bool {
        self.focused
    }

    fn preferred_height(&self) -> u16 {
        3
    }
}