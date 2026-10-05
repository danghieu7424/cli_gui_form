// --- PHÂN ĐOẠN: COLOR BUTTON WIDGET ---

use crate::traits::{EventResult, FormWidget};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::Paragraph,
    Frame,
};

pub struct ButtonWidget {
    pub title: String,
    pub bg_color: Color,
    pub fg_color: Color,
    focused: bool,
}

impl ButtonWidget {
    pub fn new(title: impl Into<String>, bg_color: Color, fg_color: Color) -> Self {
        Self {
            title: title.into(),
            bg_color,
            fg_color,
            focused: false,
        }
    }
}

impl FormWidget for ButtonWidget {
    fn render(&self, area: Rect, frame: &mut Frame) {
        let (bg, fg) = if self.focused {
            (self.fg_color, self.bg_color)
        } else {
            (self.bg_color, self.fg_color)
        };

        let label = format!(" [ {} ] ", self.title);
        let paragraph = Paragraph::new(label)
            .style(Style::default().bg(bg).fg(fg).add_modifier(Modifier::BOLD));

        frame.render_widget(paragraph, area);
    }

    fn handle_event(&mut self, key: KeyEvent) -> EventResult {
        if key.kind != KeyEventKind::Press {
            return EventResult::Ignored;
        }

        match key.code {
            KeyCode::Enter | KeyCode::Char(' ') => EventResult::Submitted,
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