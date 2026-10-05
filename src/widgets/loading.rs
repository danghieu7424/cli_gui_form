// --- PHÂN ĐOẠN: HIGH-FPS LOADING & SPINNER WIDGET ---

use crate::traits::{EventResult, FormWidget};
use crossterm::event::KeyEvent;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

// Chuỗi ký tự Braille xoay mượt chuẩn Terminal
const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub struct LoadingWidget {
    pub label: String,
    pub message: String,
    pub color: Color,
    pub bar_width: usize,
    frame_idx: usize,
    bouncing_pos: usize,
    moving_forward: bool,
    focused: bool,
}

impl LoadingWidget {
    pub fn new(label: impl Into<String>, message: impl Into<String>, color: Color) -> Self {
        Self {
            label: label.into(),
            message: message.into(),
            color,
            bar_width: 25,
            frame_idx: 0,
            bouncing_pos: 0,
            moving_forward: true,
            focused: false,
        }
    }

    pub fn with_bar_width(mut self, width: usize) -> Self {
        self.bar_width = width;
        self
    }

    /// Hàm cập nhật frame animation (gọi liên tục trong render loop)
    pub fn tick(&mut self) {
        // Cập nhật frame xoay cho spinner
        self.frame_idx = (self.frame_idx + 1) % SPINNER_FRAMES.len();

        // Cập nhật vị trí bouncing bar ━ lướt qua lại
        let block_size = 5; // Độ dài cụm ━ chạy qua lại
        if self.bar_width > block_size {
            let max_pos = self.bar_width - block_size;
            if self.moving_forward {
                if self.bouncing_pos >= max_pos {
                    self.moving_forward = false;
                } else {
                    self.bouncing_pos += 1;
                }
            } else {
                if self.bouncing_pos == 0 {
                    self.moving_forward = true;
                } else {
                    self.bouncing_pos -= 1;
                }
            }
        }
    }
}

impl FormWidget for LoadingWidget {
    fn render(&self, area: Rect, frame: &mut Frame) {
        let spinner_char = SPINNER_FRAMES[self.frame_idx];

        // Tạo hiệu ứng bouncing: đoạn ━ trượt trên nền ─
        let block_size = 5;
        let left_empty = self.bouncing_pos;
        let active_len = block_size.min(self.bar_width.saturating_sub(left_empty));
        let right_empty = self.bar_width.saturating_sub(left_empty + active_len);

        let bar_left = "─".repeat(left_empty);
        let bar_active = "━".repeat(active_len);
        let bar_right = "─".repeat(right_empty);

        let label_style = if self.focused {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        // Dòng 1: Spinner xoay + Label + Indeterminate Bar
        let line1 = Line::from(vec![
            Span::styled(format!("{} ", spinner_char), Style::default().fg(self.color).add_modifier(Modifier::BOLD)),
            Span::styled(format!("{}: ", self.label), label_style),
            Span::styled(bar_left, Style::default().fg(Color::DarkGray)),
            Span::styled(bar_active, Style::default().fg(self.color).add_modifier(Modifier::BOLD)),
            Span::styled(bar_right, Style::default().fg(Color::DarkGray)),
        ]);

        // Dòng 2: Subtitle chi tiết đang loading
        let line2 = Line::from(vec![
            Span::styled("  ↳ ", Style::default().fg(Color::DarkGray)),
            Span::styled(&self.message, Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC)),
        ]);

        let paragraph = Paragraph::new(vec![line1, line2]);
        frame.render_widget(paragraph, area);
    }

    fn handle_event(&mut self, _key: KeyEvent) -> EventResult {
        // Widget loading không chặn phím gõ, bỏ qua xử lý phím
        EventResult::Ignored
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