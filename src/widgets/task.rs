// --- PHÂN ĐOẠN: TASK WIDGET TOÀN DIỆN (THAY THẾ HOÀN TOÀN CHO LOADING & PROGRESS) ---

use crate::traits::{EventResult, FormWidget};
use crossterm::event::KeyEvent;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub enum TaskState {
    Loading {
        message: String,
        frame_idx: usize,
        bouncing_pos: usize,
        moving_forward: bool,
    },
    Running {
        current: usize,
        total: usize,
        unit: String,
        duration: String,
        status: String,
    },
}

pub struct TaskWidget {
    pub id: String,
    pub label: String,
    pub state: TaskState,
    pub color: Color,
    pub bar_width: usize,
    focused: bool,
}

impl TaskWidget {
    /// 1. Khởi tạo ở chế độ thuần Loading (thay thế hoàn toàn cho LoadingWidget)
    pub fn new_loading(
        id: impl Into<String>,
        label: impl Into<String>,
        message: impl Into<String>,
        color: Color,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            state: TaskState::Loading {
                message: message.into(),
                frame_idx: 0,
                bouncing_pos: 0,
                moving_forward: true,
            },
            color,
            bar_width: 25,
            focused: false,
        }
    }

    /// 2. Khởi tạo ở chế độ thuần Progress (thay thế hoàn toàn cho ProgressWidget)
    pub fn new_progress(
        id: impl Into<String>,
        label: impl Into<String>,
        current: usize,
        total: usize,
        unit: impl Into<String>,
        duration: impl Into<String>,
        status: impl Into<String>,
        color: Color,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            state: TaskState::Running {
                current,
                total: total.max(1),
                unit: unit.into(),
                duration: duration.into(),
                status: status.into(),
            },
            color,
            bar_width: 25,
            focused: false,
        }
    }

    /// Chuyển đổi trạng thái từ Loading sang Running
    pub fn switch_to_progress(
        &mut self,
        label: impl Into<String>,
        total: usize,
        unit: impl Into<String>,
        duration: impl Into<String>,
        status: impl Into<String>,
    ) {
        self.label = label.into();
        self.state = TaskState::Running {
            current: 0,
            total: total.max(1),
            unit: unit.into(),
            duration: duration.into(),
            status: status.into(),
        };
    }

    /// Cập nhật tiến trình khi đang ở pha Running
    pub fn update_progress(&mut self, current: usize, duration: impl Into<String>) {
        if let TaskState::Running {
            current: c,
            total,
            duration: d,
            ..
        } = &mut self.state
        {
            *c = current.min(*total);
            *d = duration.into();
        }
    }

    /// Nhịp animation cho spinner ở pha Loading
    pub fn tick(&mut self) {
        if let TaskState::Loading {
            frame_idx,
            bouncing_pos,
            moving_forward,
            ..
        } = &mut self.state
        {
            *frame_idx = (*frame_idx + 1) % SPINNER_FRAMES.len();
            let block_size = 5;
            if self.bar_width > block_size {
                let max_pos = self.bar_width - block_size;
                if *moving_forward {
                    if *bouncing_pos >= max_pos {
                        *moving_forward = false;
                    } else {
                        *bouncing_pos += 1;
                    }
                } else if *bouncing_pos == 0 {
                    *moving_forward = true;
                } else {
                    *bouncing_pos -= 1;
                }
            }
        }
    }
}

impl FormWidget for TaskWidget {
    fn id(&self) -> &str {
        &self.id
    }

    fn render(&self, area: Rect, frame: &mut Frame) {
        let label_style = if self.focused {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        match &self.state {
            TaskState::Loading {
                message,
                frame_idx,
                bouncing_pos,
                ..
            } => {
                let spinner_char = SPINNER_FRAMES[*frame_idx];
                let block_size = 5;
                let left_empty = *bouncing_pos;
                let active_len = block_size.min(self.bar_width.saturating_sub(left_empty));
                let right_empty = self.bar_width.saturating_sub(left_empty + active_len);

                let line1 = Line::from(vec![
                    Span::styled(format!("{} ", spinner_char), Style::default().fg(self.color).add_modifier(Modifier::BOLD)),
                    Span::styled(format!("{}: ", self.label), label_style),
                    Span::styled("─".repeat(left_empty), Style::default().fg(Color::DarkGray)),
                    Span::styled("━".repeat(active_len), Style::default().fg(self.color).add_modifier(Modifier::BOLD)),
                    Span::styled("─".repeat(right_empty), Style::default().fg(Color::DarkGray)),
                ]);

                let line2 = Line::from(vec![
                    Span::styled("  ↳ ", Style::default().fg(Color::DarkGray)),
                    Span::styled(message, Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC)),
                ]);

                frame.render_widget(Paragraph::new(vec![line1, line2]), area);
            }
            TaskState::Running {
                current,
                total,
                unit,
                duration,
                status,
            } => {
                let ratio = (*current as f32 / *total as f32).clamp(0.0, 1.0);
                let percent = (ratio * 100.0).round() as u8;

                let filled_len = ((self.bar_width as f32) * ratio).round() as usize;
                let empty_len = self.bar_width.saturating_sub(filled_len);

                let metric_text = format!(" [{}/{} {} ({})]", current, total, unit, duration);

                let line1 = Line::from(vec![
                    Span::styled("  ", Style::default()), // Căn lề chuẩn với ký tự spinner 2 cột
                    Span::styled(format!("{}: ", self.label), label_style),
                    Span::styled("━".repeat(filled_len), Style::default().fg(self.color).add_modifier(Modifier::BOLD)),
                    Span::styled("─".repeat(empty_len), Style::default().fg(Color::DarkGray)),
                    Span::styled(format!(" {:>3}%", percent), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                    Span::styled(metric_text, Style::default().fg(Color::LightBlue)),
                ]);

                let line2 = Line::from(vec![
                    Span::styled("  ↳ ", Style::default().fg(Color::DarkGray)),
                    Span::styled(status, Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC)),
                ]);

                frame.render_widget(Paragraph::new(vec![line1, line2]), area);
            }
        }
    }

    fn handle_event(&mut self, _key: KeyEvent) -> EventResult {
        EventResult::Ignored
    }

    fn focus(&mut self) { self.focused = true; }
    fn blur(&mut self) { self.focused = false; }
    fn is_focused(&self) -> bool { self.focused }
    fn preferred_height(&self) -> u16 { 3 }
}