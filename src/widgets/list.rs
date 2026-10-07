// --- PHÂN ĐOẠN: LIST / MENU WIDGET CHUẨN DESIGN.MD MỤC 5 ---

use crate::{
    theme::Theme,
    traits::{EventResult, FormValue, FormWidget},
};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

/****
 * Struct: ListItem
 * Chức năng: Đại diện cho một phần tử trong danh sách chọn/menu.
 * Ranh giới: Hỗ trợ trạng thái kích hoạt (Active) hoặc vô hiệu hóa (Disabled).
 ****/
#[derive(Clone, Debug, PartialEq)]
pub struct ListItem {
    pub text: String,
    pub disabled: bool,
}

impl ListItem {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            disabled: false,
        }
    }

    pub fn disabled(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            disabled: true,
        }
    }
}

/****
 * Struct: ListWidget
 * Chức năng: Điều hướng danh sách / menu dọc theo đặc tả DESIGN.md mục 5 (Lists / Menus).
 *            - Selected: `▸` prefix + BOLD + Primary
 *            - Normal: 4-space indent + Foreground
 *            - Disabled: 4-space indent + Muted + dim
 * Ranh giới bảo vệ: Tự động bỏ qua các mục disabled khi di chuyển con trỏ qua Up/Down/j/k.
 ****/
pub struct ListWidget {
    pub id: String,
    pub label: Option<String>,
    pub items: Vec<ListItem>,
    pub selected: usize,
    focused: bool,
}

impl ListWidget {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: None,
            items: Vec::new(),
            selected: 0,
            focused: false,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_item(mut self, text: impl Into<String>) -> Self {
        self.items.push(ListItem::new(text));
        self
    }

    pub fn with_disabled_item(mut self, text: impl Into<String>) -> Self {
        self.items.push(ListItem::disabled(text));
        self
    }

    pub fn add_item(&mut self, text: impl Into<String>) -> &mut Self {
        self.items.push(ListItem::new(text));
        self
    }

    pub fn add_disabled_item(&mut self, text: impl Into<String>) -> &mut Self {
        self.items.push(ListItem::disabled(text));
        self
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn selected_item(&self) -> Option<&ListItem> {
        self.items.get(self.selected)
    }

    pub fn select_next(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let total = self.items.len();
        let mut next = (self.selected + 1) % total;
        // Tìm item khả dụng tiếp theo
        for _ in 0..total {
            if !self.items[next].disabled {
                self.selected = next;
                return;
            }
            next = (next + 1) % total;
        }
    }

    pub fn select_prev(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let total = self.items.len();
        let mut prev = if self.selected == 0 {
            total - 1
        } else {
            self.selected - 1
        };
        // Tìm item khả dụng phía trước
        for _ in 0..total {
            if !self.items[prev].disabled {
                self.selected = prev;
                return;
            }
            prev = if prev == 0 { total - 1 } else { prev - 1 };
        }
    }
}

impl FormWidget for ListWidget {
    fn id(&self) -> &str {
        &self.id
    }

    fn value(&self) -> FormValue {
        if let Some(item) = self.selected_item() {
            FormValue::Select(self.selected, item.text.clone())
        } else {
            FormValue::None
        }
    }

    fn render(&self, area: Rect, frame: &mut Frame) {
        let mut lines = Vec::new();

        // 1. Render nhãn tiêu đề (nếu có)
        if let Some(ref lbl) = self.label {
            let label_style = if self.focused {
                Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Theme::SECONDARY).add_modifier(Modifier::BOLD)
            };
            lines.push(Line::from(vec![Span::styled(format!("  {}", lbl), label_style)]));
        }

        // 2. Render các mục trong danh sách theo chuẩn DESIGN.md mục 5
        for (idx, item) in self.items.iter().enumerate() {
            let is_sel = idx == self.selected;

            let (prefix, text_style) = if item.disabled {
                // Disabled: 4-space indent + Muted + dim
                (
                    "    ",
                    Style::default()
                        .fg(Theme::NEUTRAL_200)
                        .add_modifier(Modifier::DIM),
                )
            } else if is_sel {
                // Selected: `▸` prefix + BOLD + Primary
                let style = Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD);
                ("  ▸ ", style)
            } else {
                // Normal: 4-space indent + Foreground
                ("    ", Style::default().fg(Theme::FG))
            };

            let prefix_span = if is_sel && !item.disabled {
                Span::styled(
                    prefix,
                    if self.focused {
                        Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Theme::SECONDARY).add_modifier(Modifier::BOLD)
                    },
                )
            } else {
                Span::raw(prefix)
            };

            lines.push(Line::from(vec![
                prefix_span,
                Span::styled(item.text.clone(), text_style),
            ]));
        }

        frame.render_widget(Paragraph::new(lines), area);
    }

    fn handle_event(&mut self, key: KeyEvent) -> EventResult {
        if key.kind != KeyEventKind::Press {
            return EventResult::Ignored;
        }

        match key.code {
            KeyCode::Down | KeyCode::Char('j') => {
                self.select_next();
                EventResult::Consumed
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.select_prev();
                EventResult::Consumed
            }
            KeyCode::Home => {
                // Chọn item đầu tiên không bị disable
                for (idx, it) in self.items.iter().enumerate() {
                    if !it.disabled {
                        self.selected = idx;
                        break;
                    }
                }
                EventResult::Consumed
            }
            KeyCode::End => {
                // Chọn item cuối cùng không bị disable
                for (idx, it) in self.items.iter().enumerate().rev() {
                    if !it.disabled {
                        self.selected = idx;
                        break;
                    }
                }
                EventResult::Consumed
            }
            KeyCode::Enter => EventResult::Submitted,
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
        (self.items.len() + if self.label.is_some() { 1 } else { 0 }) as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_widget_navigation_and_disabled_skip() {
        let mut list = ListWidget::new("routes")
            .with_item("api/routes.ts")
            .with_disabled_item("api/handler.ts (locked)")
            .with_item("lib/utils.ts")
            .with_item("config.json");

        assert_eq!(list.selected(), 0);
        assert_eq!(list.selected_item().unwrap().text, "api/routes.ts");

        // Di chuyển xuống -> phải nhảy qua disabled item (index 1) tới index 2
        list.select_next();
        assert_eq!(list.selected(), 2);
        assert_eq!(list.selected_item().unwrap().text, "lib/utils.ts");

        // Di chuyển lùi lại -> phải quay lại index 0
        list.select_prev();
        assert_eq!(list.selected(), 0);
    }

    #[test]
    fn test_list_widget_form_value() {
        let list = ListWidget::new("files")
            .with_item("main.rs")
            .with_item("lib.rs");

        assert_eq!(
            list.value(),
            FormValue::Select(0, "main.rs".to_string())
        );
    }
}
