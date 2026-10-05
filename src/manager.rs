// --- PHÂN ĐOẠN: FORM MANAGER VỚI CƠ CHẾ AUTO-FOCUS ---

use crate::traits::{EventResult, FormWidget};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub struct FormManager {
    widgets: Vec<Box<dyn FormWidget>>,
    current_focus: usize,
}

impl FormManager {
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
            current_focus: 0,
        }
    }

    /// Thêm widget vào form và tự động focus widget đầu tiên
    pub fn add_widget(&mut self, mut widget: Box<dyn FormWidget>) {
        if self.widgets.is_empty() {
            widget.focus();
        } else {
            widget.blur();
        }
        self.widgets.push(widget);
    }

    pub fn focus_next(&mut self) {
        if self.widgets.is_empty() {
            return;
        }
        self.widgets[self.current_focus].blur();
        self.current_focus = (self.current_focus + 1) % self.widgets.len();
        self.widgets[self.current_focus].focus();
    }

    pub fn focus_prev(&mut self) {
        if self.widgets.is_empty() {
            return;
        }
        self.widgets[self.current_focus].blur();
        if self.current_focus == 0 {
            self.current_focus = self.widgets.len() - 1;
        } else {
            self.current_focus -= 1;
        }
        self.widgets[self.current_focus].focus();
    }

    /// Tự động chia layout và render toàn bộ form widgets
    pub fn render(&self, area: Rect, frame: &mut Frame) {
        if self.widgets.is_empty() {
            return;
        }

        let mut constraints: Vec<Constraint> = self
            .widgets
            .iter()
            .map(|w| Constraint::Length(w.preferred_height()))
            .collect();
        constraints.push(Constraint::Min(0)); // Khoảng trống còn lại

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(constraints)
            .split(area);

        for (idx, widget) in self.widgets.iter().enumerate() {
            widget.render(chunks[idx], frame);
        }
    }

    /// Xử lý điều hướng Tab/BackTab hoặc chuyển tiếp event cho widget đang focus
    pub fn handle_event(&mut self, key: KeyEvent) -> EventResult {
        if key.kind != KeyEventKind::Press {
            return EventResult::Ignored;
        }

        // Tự động can thiệp các phím đổi focus
        match key.code {
            KeyCode::Tab => {
                self.focus_next();
                EventResult::Consumed
            }
            KeyCode::BackTab => {
                self.focus_prev();
                EventResult::Consumed
            }
            _ => {
                if let Some(widget) = self.widgets.get_mut(self.current_focus) {
                    widget.handle_event(key)
                } else {
                    EventResult::Ignored
                }
            }
        }
    }

    pub fn focused_index(&self) -> usize {
        self.current_focus
    }
}