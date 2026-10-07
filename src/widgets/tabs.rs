// --- PHÂN ĐOẠN: TABS WIDGET CHUẨN MINIMAL TUI DESIGN SYSTEM ---

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
 * Struct: TabsWidget
 * Chức năng: Điều hướng theo thẻ (Tabs Component) theo chuẩn DESIGN.md mục 5.
 *            Active tab: BOLD + Primary text. Inactive tab: Secondary text.
 * Ranh giới bảo vệ: Giữ vững layout phẳng, điều hướng phím mũi tên Trái/Phải hoặc Tab/Shift+Tab.
 ****/
pub struct TabsWidget {
    id: String,
    titles: Vec<String>,
    selected: usize,
    focused: bool,
}

impl TabsWidget {
    pub fn new(id: impl Into<String>, titles: Vec<impl Into<String>>) -> Self {
        Self {
            id: id.into(),
            titles: titles.into_iter().map(Into::into).collect(),
            selected: 0,
            focused: false,
        }
    }

    pub fn with_selected(mut self, index: usize) -> Self {
        if !self.titles.is_empty() {
            self.selected = index.min(self.titles.len() - 1);
        }
        self
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn select_next(&mut self) {
        if !self.titles.is_empty() {
            self.selected = (self.selected + 1) % self.titles.len();
        }
    }

    pub fn select_prev(&mut self) {
        if !self.titles.is_empty() {
            if self.selected == 0 {
                self.selected = self.titles.len() - 1;
            } else {
                self.selected -= 1;
            }
        }
    }
}

impl FormWidget for TabsWidget {
    fn id(&self) -> &str {
        &self.id
    }

    fn value(&self) -> FormValue {
        if self.selected < self.titles.len() {
            FormValue::Select(self.selected, self.titles[self.selected].clone())
        } else {
            FormValue::None
        }
    }

    fn render(&self, area: Rect, frame: &mut Frame) {
        if self.titles.is_empty() {
            return;
        }

        // Hàng 1: Tiêu đề các Tab
        let mut title_spans: Vec<Span> = Vec::new();
        title_spans.push(Span::raw("  "));

        for (idx, title) in self.titles.iter().enumerate() {
            let is_active = idx == self.selected;
            let style = if is_active {
                Style::default()
                    .fg(Theme::PRIMARY)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Theme::SECONDARY)
            };

            let prefix = if is_active && self.focused { "▸ " } else { "  " };
            title_spans.push(Span::styled(format!("{}{}", prefix, title), style));

            if idx < self.titles.len() - 1 {
                title_spans.push(Span::styled(" │ ", Style::default().fg(Theme::NEUTRAL_100)));
            }
        }

        let line_titles = Line::from(title_spans);

        // Hàng 2: Đường phân cách đáy chuẩn DESIGN.md (─────────┘      └─────────)
        let mut bottom_spans: Vec<Span> = Vec::new();
        bottom_spans.push(Span::raw("  "));

        for (idx, title) in self.titles.iter().enumerate() {
            let is_active = idx == self.selected;
            let tab_width = title.len() + 2; // Gồm padding

            if is_active {
                // Active tab để trống đáy
                bottom_spans.push(Span::styled(" ".repeat(tab_width), Style::default().fg(Theme::NEUTRAL_100)));
            } else {
                // Inactive tab có gạch ngang
                bottom_spans.push(Span::styled("─".repeat(tab_width), Style::default().fg(Theme::NEUTRAL_100)));
            }

            if idx < self.titles.len() - 1 {
                let sep = if idx == self.selected {
                    "└─"
                } else if idx + 1 == self.selected {
                    "─┘"
                } else {
                    "───"
                };
                bottom_spans.push(Span::styled(sep, Style::default().fg(Theme::NEUTRAL_100)));
            }
        }

        let line_bottom = Line::from(bottom_spans);
        let paragraph = Paragraph::new(vec![line_titles, line_bottom]);
        frame.render_widget(paragraph, area);
    }

    fn handle_event(&mut self, key: KeyEvent) -> EventResult {
        if key.kind != KeyEventKind::Press {
            return EventResult::Ignored;
        }

        match key.code {
            KeyCode::Left | KeyCode::Char('h') => {
                self.select_prev();
                EventResult::Consumed
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.select_next();
                EventResult::Consumed
            }
            KeyCode::Tab => {
                self.select_next();
                EventResult::Consumed
            }
            KeyCode::BackTab => {
                self.select_prev();
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
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tabs_navigation() {
        let mut tabs = TabsWidget::new("nav", vec!["Overview", "Logs", "Settings"]);
        assert_eq!(tabs.selected(), 0);

        tabs.select_next();
        assert_eq!(tabs.selected(), 1);

        tabs.select_next();
        assert_eq!(tabs.selected(), 2);

        tabs.select_next(); // Wrap-around
        assert_eq!(tabs.selected(), 0);

        tabs.select_prev();
        assert_eq!(tabs.selected(), 2);
    }
}
