// --- PHÂN ĐOẠN: DETAILED PROGRESS WIDGET VỚI METRICS & STATUS MESSAGE ---

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
    pub current: usize,
    pub total: usize,
    pub unit: String,             // Ví dụ: "chunks", "items", ""
    pub duration: Option<String>, // Ví dụ: "2m", "0s"
    pub status: Option<String>,   // Ví dụ: "MDX-Net Native Inference"
    pub color: Color,
    pub bar_width: usize,
    focused: bool,
}

impl ProgressWidget {
    /// Khởi tạo theo số lượng cụ thể (current / total)
    pub fn new(label: impl Into<String>, current: usize, total: usize, color: Color) -> Self {
        Self {
            label: label.into(),
            current,
            total: total.max(1),
            unit: String::new(),
            duration: None,
            status: None,
            color,
            bar_width: 25,
            focused: false,
        }
    }

    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = unit.into();
        self
    }

    pub fn with_duration(mut self, duration: impl Into<String>) -> Self {
        self.duration = Some(duration.into());
        self
    }

    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    pub fn with_bar_width(mut self, width: usize) -> Self {
        self.bar_width = width;
        self
    }

    pub fn ratio(&self) -> f32 {
        (self.current as f32 / self.total as f32).clamp(0.0, 1.0)
    }
}

impl FormWidget for ProgressWidget {
    fn render(&self, area: Rect, frame: &mut Frame) {
        let ratio = self.ratio();
        let percent = (ratio * 100.0).round() as u8;

        // Tính thanh bar
        let filled_len = ((self.bar_width as f32) * ratio).round() as usize;
        let empty_len = self.bar_width.saturating_sub(filled_len);

        let filled_bar = "━".repeat(filled_len);
        let empty_bar = "─".repeat(empty_len);

        let label_style = if self.focused {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        // Chuỗi metric: "[90/134 chunks (2m)]"
        let mut metric_parts = Vec::new();
        if self.unit.is_empty() {
            metric_parts.push(format!("{}/{}", self.current, self.total));
        } else {
            metric_parts.push(format!("{}/{} {}", self.current, self.total, self.unit));
        }

        if let Some(dur) = &self.duration {
            metric_parts.push(format!("({})", dur));
        }
        let metric_text = format!(" [{}]", metric_parts.join(" "));

        // Dòng 1: Label + Bar + % + Metrics
        let mut line1_spans = vec![
            Span::styled(format!("{}: ", self.label), label_style),
            Span::styled(filled_bar, Style::default().fg(self.color).add_modifier(Modifier::BOLD)),
            Span::styled(empty_bar, Style::default().fg(Color::DarkGray)),
            Span::styled(format!(" {:>3}%", percent), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled(metric_text, Style::default().fg(Color::LightBlue)),
        ];

        // Nếu không có dòng phụ, nối trực tiếp status vào cuối dòng 1
        let mut lines = Vec::new();
        if let Some(status) = &self.status {
            line1_spans.push(Span::styled(format!(" - {}", status), Style::default().fg(Color::Gray)));
            lines.push(Line::from(line1_spans));
        } else {
            lines.push(Line::from(line1_spans));
        }

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, area);
    }

    fn handle_event(&mut self, key: KeyEvent) -> EventResult {
        if key.kind != KeyEventKind::Press {
            return EventResult::Ignored;
        }

        let step = (self.total / 20).max(1);
        match key.code {
            KeyCode::Left => {
                self.current = self.current.saturating_sub(step);
                EventResult::Consumed
            }
            KeyCode::Right => {
                self.current = (self.current + step).min(self.total);
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