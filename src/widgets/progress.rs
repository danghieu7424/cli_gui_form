// --- PHÂN ĐOẠN: PROGRESS / SLIDER WIDGET (━ ĐÃ TẢI / ─ CHƯA TẢI) ---

use crate::traits::{EventResult, FormWidget};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct ProgressWidget {
    pub label: String,
    pub progress: f32, // Giá trị từ 0.0 đến 1.0
    pub color: Color,
    focused: bool,
}

impl ProgressWidget {
    pub fn new(label: impl Into<String>, initial_progress: f32, color: Color) -> Self {
        Self {
            label: label.into(),
            progress: initial_progress.clamp(0.0, 1.0),
            color,
            focused: false,
        }
    }
}

impl FormWidget for ProgressWidget {
    fn render(&self, area: Rect, frame: &mut Frame) {
        let percent = (self.progress * 100.0).round() as u8;
        let label_prefix = format!("{}: ", self.label);
        let percent_suffix = format!(" {:>3}%", percent);

        // Tính toán độ dài khả dụng cho thanh bar
        let used_width = label_prefix.chars().count() + percent_suffix.chars().count();
        let bar_width = (area.width as usize).saturating_sub(used_width);

        let filled_len = ((bar_width as f32) * self.progress).round() as usize;
        let empty_len = bar_width.saturating_sub(filled_len);

        // Phần đã chạy: ━ nét đậm tô màu (Cyan/Green/...)
        let filled_bar = "━".repeat(filled_len);
        // Phần chưa chạy: ─ nét mảnh màu xám tối (DarkGray)
        let empty_bar = "─".repeat(empty_len);

        let label_style = if self.focused {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let content = Line::from(vec![
            Span::styled(label_prefix, label_style),
            Span::styled(filled_bar, Style::default().fg(self.color).add_modifier(Modifier::BOLD)),
            Span::styled(empty_bar, Style::default().fg(Color::DarkGray)),
            Span::styled(percent_suffix, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]);

        let paragraph = Paragraph::new(content);
        frame.render_widget(paragraph, area);
    }

    fn handle_event(&mut self, key: KeyEvent) -> EventResult {
        if key.kind != KeyEventKind::Press {
            return EventResult::Ignored;
        }

        match key.code {
            KeyCode::Left => {
                self.progress = (self.progress - 0.05).max(0.0);
                EventResult::Consumed
            }
            KeyCode::Right => {
                self.progress = (self.progress + 0.05).min(1.0);
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