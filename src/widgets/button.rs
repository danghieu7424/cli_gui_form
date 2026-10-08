// --- PHÂN ĐOẠN: BUTTON VỚI ID FORM ---

use crate::traits::{EventResult, FormWidget};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub struct ButtonWidget {
    pub id: String,
    pub title: String,
    pub icon: Option<String>,
    pub bg_color: Color,
    pub fg_color: Color,
    pub focused_bg_color: Option<Color>,
    pub focused_fg_color: Option<Color>,
    pub bordered: bool,
    pub centered: bool,
    pub full_width: bool,
    focused: bool,
}

impl ButtonWidget {
    pub fn new(id: impl Into<String>, title: impl Into<String>, bg_color: Color, fg_color: Color) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            icon: None,
            bg_color,
            fg_color,
            focused_bg_color: None,
            focused_fg_color: None,
            bordered: false,
            centered: false,
            full_width: false,
            focused: false,
        }
    }

    /// Thêm biểu tượng icon (ví dụ: Icons::RUN, Icons::SUCCESS, Icons::STOP)
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Tùy biến màu nền khi nút được chọn / hover
    pub fn with_focused_bg(mut self, bg: Color) -> Self {
        self.focused_bg_color = Some(bg);
        self
    }

    /// Tùy biến cả màu nền và màu chữ khi nút được chọn / hover
    pub fn with_focused_colors(mut self, bg: Color, fg: Color) -> Self {
        self.focused_bg_color = Some(bg);
        self.focused_fg_color = Some(fg);
        self
    }

    /// Đóng khung viền bo tròn (Rounded border) cho nút bấm dạng Box/Pill
    pub fn with_bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// Căn giữa nội dung nút trong khung
    pub fn with_centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }

    /// Tô màu nền toàn bộ chiều ngang container
    pub fn with_full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }
}

impl FormWidget for ButtonWidget {
    fn id(&self) -> &str {
        &self.id
    }

    fn render(&self, area: Rect, frame: &mut Frame) {
        let (prefix, bg, fg, is_focused) = if self.focused {
            let focused_bg = self.focused_bg_color.unwrap_or(crate::Theme::GRAY_33);
            let focused_fg = self.focused_fg_color.unwrap_or(crate::Theme::PRIMARY);
            ("▸ ", focused_bg, focused_fg, true)
        } else {
            ("  ", self.bg_color, self.fg_color, false)
        };

        // Ranh giới bảo vệ: Nếu nút có icon hoặc tiêu đề bắt đầu bằng icon '▶',
        // loại bỏ tiền tố '▸ ' hoặc điều chỉnh để tránh xuất hiện 2 tam giác '▸ ▶' cạnh nhau
        let has_run_icon = self.icon.is_some() || self.title.starts_with('▶');
        let effective_prefix = if has_run_icon {
            if self.bordered || self.centered { "" } else { "  " }
        } else {
            prefix
        };

        let mut spans = Vec::new();
        if !effective_prefix.is_empty() {
            spans.push(Span::styled(format!(" {}", effective_prefix), Style::default().fg(fg).bg(bg)));
        }

        // Tách riêng icon nếu có để bảo vệ glyph không bị bold co rút
        let mut display_title = self.title.as_str();
        let icon_to_render = if let Some(ic) = &self.icon {
            Some(ic.as_str())
        } else if let Some(stripped) = display_title.strip_prefix("▶ ") {
            display_title = stripped;
            Some("▶ ")
        } else if let Some(stripped) = display_title.strip_prefix('▶') {
            display_title = stripped.trim_start();
            Some("▶ ")
        } else {
            None
        };

        if let Some(ic) = icon_to_render {
            let icon_str = if ic.ends_with(' ') {
                ic.to_string()
            } else {
                format!("{} ", ic)
            };
            // Ranh giới bảo vệ thị giác: Icon tuyệt đối KHÔNG có Modifier::BOLD khi hover
            // để đảm bảo glyph 2-cell không bao giờ bị méo, co lại hay nhảy font trên terminal.
            let icon_style = Style::default().fg(fg).bg(bg).remove_modifier(Modifier::BOLD);
            spans.push(Span::styled(icon_str, icon_style));
        }

        let mut title_style = Style::default().fg(fg).bg(bg);
        if is_focused {
            title_style = title_style.add_modifier(Modifier::BOLD);
        }
        spans.push(Span::styled(format!("{} ", display_title), title_style));

        let content = Line::from(spans);
        let mut paragraph = Paragraph::new(content);

        if self.centered {
            paragraph = paragraph.alignment(ratatui::layout::Alignment::Center);
        }

        if self.full_width {
            paragraph = paragraph.style(Style::default().bg(bg).fg(fg));
        }

        if self.bordered {
            let border_color = if self.focused {
                self.focused_fg_color.unwrap_or(crate::Theme::BORDER_FOCUS)
            } else {
                crate::Theme::NEUTRAL_100
            };
            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(border_color))
                .style(Style::default().bg(bg));
            paragraph = paragraph.block(block);
        }

        frame.render_widget(paragraph, area);
    }

    fn handle_event(&mut self, key: KeyEvent) -> EventResult {
        if key.kind != KeyEventKind::Press {
            return EventResult::Ignored;
        }

        match key.code {
            KeyCode::Enter | KeyCode::Char(' ') => EventResult::Submitted,
            _ => EventResult::Ignored,
        }
    }

    fn focus(&mut self) { self.focused = true; }
    fn blur(&mut self) { self.focused = false; }
    fn is_focused(&self) -> bool { self.focused }
    fn preferred_height(&self) -> u16 {
        if self.bordered { 3 } else { 2 }
    }
}