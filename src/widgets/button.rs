// --- PHÂN ĐOẠN: BUTTON VỚI ID FORM ---

use crate::traits::{EventResult, FormWidget};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct ButtonWidget {
    pub id: String,
    pub title: String,
    pub icon: Option<String>,
    pub bg_color: Color,
    pub fg_color: Color,
    focused: bool,
}

impl ButtonWidget {
    pub fn new(id: impl Into<String>, title: impl Into<String>, bg_color: Color, fg_color: Color) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            icon: None,
            bg_color,
            fg_color,
            focused: false,
        }
    }

    /// Thêm biểu tượng icon (ví dụ: Icons::RUN, Icons::SUCCESS, Icons::STOP)
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

impl FormWidget for ButtonWidget {
    fn id(&self) -> &str {
        &self.id
    }

    fn render(&self, area: Rect, frame: &mut Frame) {
        let (bg, fg) = if self.focused {
            (self.fg_color, self.bg_color)
        } else {
            (self.bg_color, self.fg_color)
        };

        let button_text = match &self.icon {
            Some(ic) => format!(" [ {} {} ] ", ic, self.title),
            None => format!(" [ {} ] ", self.title),
        };
        let content = Line::from(vec![
            Span::styled(button_text, Style::default().bg(bg).fg(fg).add_modifier(Modifier::BOLD)),
        ]);

        frame.render_widget(Paragraph::new(content), area);
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

    fn focus(&mut self) { self.focused = true; }
    fn blur(&mut self) { self.focused = false; }
    fn is_focused(&self) -> bool { self.focused }
    fn preferred_height(&self) -> u16 { 2 }
}