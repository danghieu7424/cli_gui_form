// --- PHÂN ĐOẠN: FORM MANAGER HỖ TRỢ ĐIỀU HƯỚNG MŨI TÊN LÊN / XUỐNG ---

use crate::traits::{EventResult, FormValue, FormWidget};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{layout::Rect, Frame};
use std::collections::HashMap;

pub struct FormManager {
    widgets: Vec<Box<dyn FormWidget>>,
    current_focus: usize,
    scroll_offset: u16,
}

impl Default for FormManager {
    fn default() -> Self {
        Self::new()
    }
}

impl FormManager {
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
            current_focus: 0,
            scroll_offset: 0,
        }
    }

    pub fn add_widget(&mut self, mut widget: Box<dyn FormWidget>) {
        if self.widgets.is_empty() {
            widget.focus();
        } else {
            widget.blur();
        }
        self.widgets.push(widget);
    }

    pub fn get_values(&self) -> HashMap<String, FormValue> {
        let mut map = HashMap::new();
        for w in &self.widgets {
            map.insert(w.id().to_string(), w.value());
        }
        map
    }

    pub fn focus_next(&mut self) {
        if self.widgets.is_empty() { return; }
        self.widgets[self.current_focus].blur();
        self.current_focus = (self.current_focus + 1) % self.widgets.len();
        self.widgets[self.current_focus].focus();
    }

    pub fn focus_prev(&mut self) {
        if self.widgets.is_empty() { return; }
        self.widgets[self.current_focus].blur();
        if self.current_focus == 0 {
            self.current_focus = self.widgets.len() - 1;
        } else {
            self.current_focus -= 1;
        }
        self.widgets[self.current_focus].focus();
    }

    pub fn render(&mut self, area: Rect, frame: &mut Frame) {
        if self.widgets.is_empty() { return; }

        let mut widget_positions: Vec<(u16, u16)> = Vec::new();
        let mut running_y: u16 = 0;
        for w in &self.widgets {
            let h = w.preferred_height();
            widget_positions.push((running_y, h));
            running_y += h;
        }

        let (focus_y, focus_h) = widget_positions[self.current_focus];
        if focus_y < self.scroll_offset {
            self.scroll_offset = focus_y;
        } else if focus_y + focus_h > self.scroll_offset + area.height {
            self.scroll_offset = (focus_y + focus_h).saturating_sub(area.height);
        }

        for (idx, widget) in self.widgets.iter().enumerate() {
            let (w_y, w_h) = widget_positions[idx];
            if w_y + w_h > self.scroll_offset && w_y < self.scroll_offset + area.height {
                let render_y = area.y + w_y.saturating_sub(self.scroll_offset);
                let render_h = w_h.min((area.y + area.height).saturating_sub(render_y));

                let widget_area = Rect {
                    x: area.x,
                    y: render_y,
                    width: area.width,
                    height: render_h,
                };

                widget.render(widget_area, frame);

                if widget.is_focused() {
                    if let Some((cx, cy)) = widget.cursor_position(widget_area) {
                        if cy < area.y + area.height {
                            frame.set_cursor_position((cx, cy));
                        }
                    }
                }
            }
        }
    }

    pub fn handle_event(&mut self, key: KeyEvent) -> EventResult {
        if key.kind != KeyEventKind::Press {
            return EventResult::Ignored;
        }

        // Ưu tiên điều hướng di chuyển form bằng Mũi tên Lên/Xuống hoặc Tab/Shift+Tab
        match key.code {
            KeyCode::Down | KeyCode::Tab => {
                self.focus_next();
                EventResult::Consumed
            }
            KeyCode::Up | KeyCode::BackTab => {
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
}