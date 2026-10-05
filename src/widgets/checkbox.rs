// --- PHÂN ĐOẠN: CHECKBOX WIDGET MONOSPACE CHUẨN ---

use crate::traits::{EventResult, FormWidget};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct CheckboxWidget {
    pub label: String,
    pub checked: bool,
    focused: bool,
}

impl CheckboxWidget {
    pub fn new(label: impl Into<String>, checked: bool) -> Self {
        Self {
            label: label.into(),
            checked,
            focused: false,
        }
    }
}

impl FormWidget for CheckboxWidget {
    fn render(&self, area: Rect, frame: &mut Frame) {
        // Sử dụng định dạng đồng kích thước font
        let (box_symbol, color) = if self.checked {
            ("[✓]", Color::Green)
        } else {
            ("[ ]", Color::DarkGray)
        };

        let label_style = if self.focused {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let content = Line::from(vec![
            Span::styled(format!("{} ", box_symbol), Style::default().fg(color).add_modifier(Modifier::BOLD)),
            Span::styled(&self.label, label_style),
        ]);

        let paragraph = Paragraph::new(content);
        frame.render_widget(paragraph, area);
    }

    fn handle_event(&mut self, key: KeyEvent) -> EventResult {
        if key.kind != KeyEventKind::Press {
            return EventResult::Ignored;
        }

        match key.code {
            KeyCode::Char(' ') | KeyCode::Enter => {
                self.checked = !self.checked;
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
        2
    }
}