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
    /// Vẽ toàn bộ Khung Tab Container hoàn chỉnh bo góc (Border Radius) nối liền nội dung
    pub fn render_container(&self, area: Rect, content: Vec<Line<'static>>, frame: &mut Frame) {
        if self.titles.is_empty() || area.width < 10 || area.height < 4 {
            return;
        }

        let width = area.width as usize;
        let border_style = Style::default().fg(Theme::NEUTRAL_100);

        // Hàng 0: Tab Titles
        let mut title_spans: Vec<Span> = Vec::new();
        title_spans.push(Span::styled("  ", Style::default().fg(Theme::NEUTRAL_100)));

        for (idx, title) in self.titles.iter().enumerate() {
            let is_active = idx == self.selected;
            let style = if is_active {
                Style::default()
                    .fg(Theme::PRIMARY)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Theme::SECONDARY)
            };

            title_spans.push(Span::styled(title.as_str(), style));

            if idx < self.titles.len() - 1 {
                title_spans.push(Span::styled(" │ ", Style::default().fg(Theme::NEUTRAL_100)));
            }
        }

        let line_titles = Line::from(title_spans);

        // Hàng 1: Khung viền trên nối liền bo góc (╭───╯    ╰──────╮)
        let mut top_border_spans: Vec<Span> = Vec::new();
        let mut drawn_cols: usize = 0;

        // Góc trên cùng bên trái
        top_border_spans.push(Span::styled("╭─", border_style));
        drawn_cols += 2;

        for (idx, title) in self.titles.iter().enumerate() {
            let is_active = idx == self.selected;
            let tab_len = title.chars().count();

            if is_active {
                top_border_spans.push(Span::styled(" ".repeat(tab_len), border_style));
            } else {
                top_border_spans.push(Span::styled("─".repeat(tab_len), border_style));
            }
            drawn_cols += tab_len;

            if idx < self.titles.len() - 1 {
                let sep = match (idx == self.selected, idx + 1 == self.selected) {
                    (true, false) => " ╰─", // Nối góc ╰ từ active sang inactive
                    (false, true) => "─╯ ", // Nối góc ╯ từ inactive sang active
                    _ => "───",
                };
                top_border_spans.push(Span::styled(sep, border_style));
                drawn_cols += 3;
            }
        }

        // Lấp đầy phần còn lại của viền trên và đóng góc ╮
        if width > drawn_cols + 1 {
            top_border_spans.push(Span::styled("─".repeat(width - drawn_cols - 1), border_style));
            top_border_spans.push(Span::styled("╮", border_style));
        } else {
            top_border_spans.push(Span::styled("╮", border_style));
        }

        let line_top_border = Line::from(top_border_spans);

        // Hàng 2 đến N-2: Thân Panel chứa nội dung
        let body_height = (area.height as usize).saturating_sub(3);
        let mut body_lines: Vec<Line> = Vec::new();
        body_lines.push(line_titles);
        body_lines.push(line_top_border);

        for row in 0..body_height {
            let content_line = content.get(row);
            let mut line_spans: Vec<Span> = Vec::new();
            line_spans.push(Span::styled("│ ", border_style));

            let mut inner_len = 0;
            if let Some(cl) = content_line {
                for s in &cl.spans {
                    line_spans.push(s.clone());
                    inner_len += s.content.chars().count();
                }
            }

            let remain = width.saturating_sub(inner_len + 3);
            if remain > 0 {
                line_spans.push(Span::raw(" ".repeat(remain)));
            }
            line_spans.push(Span::styled("│", border_style));
            body_lines.push(Line::from(line_spans));
        }

        // Hàng cuối: Viền đáy bo góc (╰───────────────╯)
        let mut bottom_spans: Vec<Span> = Vec::new();
        bottom_spans.push(Span::styled("╰", border_style));
        if width > 2 {
            bottom_spans.push(Span::styled("─".repeat(width - 2), border_style));
        }
        bottom_spans.push(Span::styled("╯", border_style));
        body_lines.push(Line::from(bottom_spans));

        let paragraph = Paragraph::new(body_lines).style(Style::default().bg(Theme::BG));
        frame.render_widget(paragraph, area);
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

        // Hàng 1: Tiêu đề các Tab (Theo DESIGN.md: Active tab là BOLD + Primary, không có tiền tố tam giác)
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

            title_spans.push(Span::styled(title.as_str(), style));

            if idx < self.titles.len() - 1 {
                title_spans.push(Span::styled(" │ ", Style::default().fg(Theme::NEUTRAL_100)));
            }
        }

        let line_titles = Line::from(title_spans);

        // Hàng 2: Đường viền đáy nối chính xác 1-1 từng cột (─────────┘      └─────────)
        let mut bottom_spans: Vec<Span> = Vec::new();
        bottom_spans.push(Span::raw("  "));

        for (idx, title) in self.titles.iter().enumerate() {
            let is_active = idx == self.selected;
            let tab_len = title.chars().count();

            if is_active {
                // Active tab để trống đáy hoàn toàn
                bottom_spans.push(Span::styled(" ".repeat(tab_len), Style::default().fg(Theme::NEUTRAL_100)));
            } else {
                // Inactive tab gạch ngang bằng độ dài chữ
                bottom_spans.push(Span::styled("─".repeat(tab_len), Style::default().fg(Theme::NEUTRAL_100)));
            }

            if idx < self.titles.len() - 1 {
                let sep = match (idx == self.selected, idx + 1 == self.selected) {
                    (true, false) => " ╰─", // Nối góc ╰ từ active sang inactive
                    (false, true) => "─╯ ", // Nối góc ╯ từ inactive sang active
                    _ => "───",             // Cả 2 đều Inactive: Nối thẳng
                };
                bottom_spans.push(Span::styled(sep, Style::default().fg(Theme::NEUTRAL_100)));
            }
        }

        let line_bottom = Line::from(bottom_spans);
        let paragraph = Paragraph::new(vec![line_titles, line_bottom]).style(Style::default().bg(Theme::BG));
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
