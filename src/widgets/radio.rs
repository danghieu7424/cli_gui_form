// --- PHÂN ĐOẠN: RADIO SELECT WIDGET CHUẨN MONOSPACE ---

use crate::traits::{EventResult, FormWidget};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct RadioWidget {
    pub label: String,
    pub options: Vec<String>,
    pub selected_index: usize,
    focused: bool,
}

impl RadioWidget {
    pub fn new(label: impl Into<String>, options: Vec<String>) -> Self {
        Self {
            label: label.into(),
            options,
            selected_index: 0,
            focused: false,
        }
    }
}

impl FormWidget for RadioWidget {
    fn render(&self, area: Rect, frame: &mut Frame) {
        let mut spans = vec![Span::styled(
            format!("{}: ", self.label),
            Style::default().fg(Color::Gray),
        )];

        for (idx, opt) in self.options.iter().enumerate() {
            let is_selected = idx == self.selected_index;
            let (symbol, opt_color) = if is_selected {
                ("(•)", Color::LightCyan)
            } else {
                ("( )", Color::DarkGray)
            };

            spans.push(Span::styled(
                format!("{} {}  ", symbol, opt),
                Style::default().fg(opt_color),
            ));
        }

        let block_style = if self.focused {
            Style::default().add_modifier(Modifier::UNDERLINED)
        } else {
            Style::default()
        };

        let paragraph = Paragraph::new(Line::from(spans)).style(block_style);
        frame.render_widget(paragraph, area);
    }

    fn handle_event(&mut self, key: KeyEvent) -> EventResult {
        if key.kind != KeyEventKind::Press {
            return EventResult::Ignored;
        }

        match key.code {
            KeyCode::Left | KeyCode::Up => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                    EventResult::Consumed
                } else {
                    EventResult::Ignored
                }
            }
            KeyCode::Right | KeyCode::Down => {
                if self.selected_index + 1 < self.options.len() {
                    self.selected_index += 1;
                    EventResult::Consumed
                } else {
                    EventResult::Ignored
                }
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