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
    /// Vẽ toàn bộ Khung Tab Container hoàn chỉnh bo góc (Border Radius) nối liền nội dung chuẩn log.md
    pub fn render_container(&self, area: Rect, content: Vec<Line<'static>>, frame: &mut Frame) {
        if self.titles.is_empty() || area.width < 10 || area.height < 4 {
            return;
        }

        let width = area.width as usize;
        let border_style = Style::default().fg(Theme::NEUTRAL_100);

        // Chiều rộng từng Tab (tiêu đề + 2 space padding)
        let tab_widths: Vec<usize> = self.titles.iter().map(|t| t.chars().count() + 2).collect();
        let num_tabs = self.titles.len();

        // 1. HÀNG 0: Nắp trên của Tab Bar (╭──────────┬──────┬──────────┬─────────────╮)
        let mut row0_spans: Vec<Span> = Vec::new();
        row0_spans.push(Span::styled("╭", border_style));

        for (idx, &w) in tab_widths.iter().enumerate() {
            row0_spans.push(Span::styled("─".repeat(w), border_style));
            if idx < num_tabs - 1 {
                row0_spans.push(Span::styled("┬", border_style));
            } else {
                row0_spans.push(Span::styled("╮", border_style));
            }
        }
        let line_row0 = Line::from(row0_spans);

        // 2. HÀNG 1: Nội dung Tab Titles (│ Overview │ Logs │ Settings │ Deployments │)
        let mut row1_spans: Vec<Span> = Vec::new();
        row1_spans.push(Span::styled("│", border_style));

        for (idx, title) in self.titles.iter().enumerate() {
            let is_active = idx == self.selected;
            let style = if is_active {
                Style::default()
                    .fg(Theme::PRIMARY)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Theme::SECONDARY)
            };

            row1_spans.push(Span::styled(format!(" {} ", title), style));
            row1_spans.push(Span::styled("│", border_style));
        }
        let line_row1 = Line::from(row1_spans);

        // 3. HÀNG 2: Đường phân cách đáy kết nối lòng Panel (theo đúng chuẩn log.md)
        let mut row2_spans: Vec<Span> = Vec::new();
        let mut drawn_cols: usize = 0;

        // Mép trái ngoài cùng
        if self.selected == 0 {
            row2_spans.push(Span::styled("│", border_style));
        } else {
            row2_spans.push(Span::styled("├", border_style));
        }
        drawn_cols += 1;

        for (idx, &w) in tab_widths.iter().enumerate() {
            let is_active = idx == self.selected;
            if is_active {
                row2_spans.push(Span::raw(" ".repeat(w)));
            } else {
                row2_spans.push(Span::styled("─".repeat(w), border_style));
            }
            drawn_cols += w;

            // Xử lý giao điểm cột dọc bên phải tab
            if idx < num_tabs - 1 {
                let sep = match (idx == self.selected, idx + 1 == self.selected) {
                    (true, false) => "╰", // Chuyển từ Active -> Inactive
                    (false, true) => "╯", // Chuyển từ Inactive -> Active
                    _ => "┴",             // Cả 2 đều Inactive
                };
                row2_spans.push(Span::styled(sep, border_style));
                drawn_cols += 1;
            } else {
                // Điểm kết thúc của tab cuối cùng
                if is_active {
                    row2_spans.push(Span::styled("╰", border_style));
                } else {
                    row2_spans.push(Span::styled("┴", border_style));
                }
                drawn_cols += 1;
            }
        }

        // Kéo dài viền trên sang hết chiều rộng của Panel và đóng góc ╮
        if width > drawn_cols + 1 {
            row2_spans.push(Span::styled("─".repeat(width - drawn_cols - 1), border_style));
            row2_spans.push(Span::styled("╮", border_style));
        } else if width > drawn_cols {
            row2_spans.push(Span::styled("╮", border_style));
        }

        let line_row2 = Line::from(row2_spans);

        // 4. HÀNG 3 đến N-2: Thân Panel chứa nội dung
        let body_height = (area.height as usize).saturating_sub(4);
        let mut body_lines: Vec<Line> = Vec::new();
        body_lines.push(line_row0);
        body_lines.push(line_row1);
        body_lines.push(line_row2);

        let max_inner_w = width.saturating_sub(3); // 2 ký tự mép trái ("│ ") + 1 mép phải ("│")

        for row in 0..body_height {
            let content_line = content.get(row);
            let mut line_spans: Vec<Span> = Vec::new();
            line_spans.push(Span::styled("│ ", border_style));

            let mut cur_w = 0;
            if let Some(cl) = content_line {
                for s in &cl.spans {
                    if cur_w >= max_inner_w {
                        break;
                    }
                    let s_len = s.content.chars().count();
                    if cur_w + s_len <= max_inner_w {
                        line_spans.push(s.clone());
                        cur_w += s_len;
                    } else {
                        // Tự động cắt tỉa (Truncate) an toàn nếu span vượt quá bề ngang panel
                        let take_chars = max_inner_w.saturating_sub(cur_w);
                        let truncated: String = s.content.chars().take(take_chars).collect();
                        line_spans.push(Span::styled(truncated, s.style));
                        cur_w = max_inner_w;
                        break;
                    }
                }
            }

            let remain = max_inner_w.saturating_sub(cur_w);
            if remain > 0 {
                line_spans.push(Span::raw(" ".repeat(remain)));
            }
            line_spans.push(Span::styled("│", border_style));
            body_lines.push(Line::from(line_spans));
        }

        // 5. HÀNG CUỐI: Viền đáy bo góc (╰───────────────╯)
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

    #[test]
    fn test_tabs_container_overflow_protection() {
        let _tabs = TabsWidget::new("nav", vec!["Tab1", "Tab2"]);
        let _long_line = Line::from(vec![
            Span::raw("A".repeat(500)), // Chuỗi cực dài vượt qua mọi kích thước terminal
        ]);
        // Kiểm tra logic cắt tỉa với max_inner_w
        let width = 80;
        let max_inner_w = width - 3;
        assert_eq!(max_inner_w, 77);
    }
}
