// --- PHÂN ĐOẠN: TEXT INPUT VỚI CON TRỎ TERMINAL VÀ CHỈNH SỬA TẠI CHỖ ---

use crate::traits::{EventResult, FormValue, FormWidget};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Text,
    Password,
}

pub struct InputWidget {
    pub id: String,
    pub label: String,
    pub value: String,
    pub mode: InputMode,
    cursor_idx: usize,
    focused: bool,
}

impl InputWidget {
    pub fn new(id: impl Into<String>, label: impl Into<String>, mode: InputMode) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            value: String::new(),
            mode,
            cursor_idx: 0,
            focused: false,
        }
    }
}

impl FormWidget for InputWidget {
    fn id(&self) -> &str {
        &self.id
    }

    fn value(&self) -> FormValue {
        FormValue::Text(self.value.clone())
    }

    fn render(&self, area: Rect, frame: &mut Frame) {
        let display_text = match self.mode {
            InputMode::Text => self.value.clone(),
            InputMode::Password => "*".repeat(self.value.len()),
        };

        // Theo DESIGN.md:
        // - Active/Focused: Accent color border (#0070f3)
        // - Inactive: Muted color border (#555555 / Neutral 100)
        let border_color = if self.focused {
            crate::Theme::ACCENT
        } else {
            crate::Theme::MUTED
        };

        let title_formatted = format!("─ {} ─", self.label);

        // Bo góc theo yêu cầu người dùng (BorderType::Rounded) kết hợp format ╭─ Label ─
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(border_color))
            .padding(Padding::horizontal(1))
            .title(title_formatted);

        let paragraph = Paragraph::new(display_text)
            .block(block)
            .style(if self.focused {
                Style::default().fg(crate::Theme::PRIMARY).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(crate::Theme::FG)
            });

        frame.render_widget(paragraph, area);
    }

    fn handle_event(&mut self, key: KeyEvent) -> EventResult {
        if key.kind != KeyEventKind::Press {
            return EventResult::Ignored;
        }

        match key.code {
            KeyCode::Left => {
                if self.cursor_idx > 0 {
                    self.cursor_idx -= 1;
                    EventResult::Consumed
                } else {
                    EventResult::Ignored
                }
            }
            KeyCode::Right => {
                if self.cursor_idx < self.value.len() {
                    self.cursor_idx += 1;
                    EventResult::Consumed
                } else {
                    EventResult::Ignored
                }
            }
            KeyCode::Home => {
                self.cursor_idx = 0;
                EventResult::Consumed
            }
            KeyCode::End => {
                self.cursor_idx = self.value.len();
                EventResult::Consumed
            }
            KeyCode::Backspace => {
                if self.cursor_idx > 0 {
                    self.cursor_idx -= 1;
                    self.value.remove(self.cursor_idx);
                    EventResult::Consumed
                } else {
                    EventResult::Ignored
                }
            }
            KeyCode::Char(c) => {
                self.value.insert(self.cursor_idx, c);
                self.cursor_idx += 1;
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

    fn cursor_position(&self, area: Rect) -> Option<(u16, u16)> {
        if self.focused {
            // Tính toán vị trí con trỏ: area.x + 1 (border) + 1 (padding) + cursor_idx
            let x = area.x + 2 + (self.cursor_idx as u16);
            let y = area.y + 1; // Hàng thứ 2 (bên trong block border)
            Some((x, y))
        } else {
            None
        }
    }
}