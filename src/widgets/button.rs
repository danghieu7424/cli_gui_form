// --- PHÂN ĐOẠN: BUTTON VỚI ID FORM ---

use crate::traits::{EventResult, FormWidget};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
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
}

impl FormWidget for ButtonWidget {
    fn id(&self) -> &str {
        &self.id
    }

    fn render(&self, area: Rect, frame: &mut Frame) {
        let (bg, fg) = if self.focused {
            let focused_bg = self.focused_bg_color.unwrap_or(crate::Theme::GRAY_33);
            let focused_fg = self.focused_fg_color.unwrap_or(crate::Theme::PRIMARY);
            (focused_bg, focused_fg)
        } else {
            (self.bg_color, self.fg_color)
        };

        // Nếu nút đã có icon riêng (ví dụ: Icons::RUN "▶ "), không chèn thêm tiền tố "▸ "
        // để tránh xuất hiện 2 hình tam giác "▸ ▶ " gây hiểu nhầm icon bị bold/méo hiển thị.
        let prefix = if self.icon.is_some() {
            "  "
        } else if self.focused {
            "▸ "
        } else {
            "  "
        };

        // Ranh giới bảo vệ thị giác: Tuyệt đối KHÔNG sử dụng Modifier::BOLD khi hover/focus
        // Giữ trọn vẹn nét thanh mảnh chuẩn Minimalist trên nền xám #333333,
        // bảo đảm icon glyph "▶ " không bao giờ bị dày lên hay co rút cell.
        let base_style = Style::default()
            .fg(fg)
            .bg(bg)
            .remove_modifier(Modifier::BOLD);

        let mut spans = Vec::new();
        spans.push(Span::styled(format!(" {}", prefix), base_style));

        if let Some(ic) = &self.icon {
            let icon_str = if ic.ends_with(' ') {
                ic.clone()
            } else {
                format!("{} ", ic)
            };
            spans.push(Span::styled(icon_str, base_style));
        }

        spans.push(Span::styled(format!("{} ", self.title), base_style));

        let content = Line::from(spans);
        frame.render_widget(Paragraph::new(content), area);
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
    fn preferred_height(&self) -> u16 { 2 }
}