// --- PHÂN ĐOẠN: POPUP & FLOATING NODE WIDGET CHUẨN DESIGN.MD ---

use crate::theme::Theme;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Clear, Paragraph},
    Frame,
};

/****
 * Enum: PopupStyle
 * Chức năng: Định dạng ngoại quan cho Popup / Floating Card.
 * Ranh giới:
 *   - PipelineCard: Bo tròn viền trên lồng icon + title, viền dưới có progress baseline.
 *   - MicroBadge: Khung compact lồng tag ở cả viền trên (label) và viền dưới (port/status).
 *   - HudBracket: Khung viễn thám 4 góc rời (Corner brackets) cho telemetry & sensor metrics.
 *   - DiffCard: Khung kỹ thuật hỗ trợ nhãn phụ góc phải (top right tag) và khối diff code nổi bật.
 ****/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PopupStyle {
    /// Bo tròn viền trên lồng icon + title, viền dưới có accent baseline
    PipelineCard,
    /// Khung micro compact lồng tag ở cả viền trên (label) và viền dưới (port/status)
    MicroBadge,
    /// Khung viễn thám 4 góc rời (Corner brackets) cho telemetry & sensor metrics
    HudBracket,
    /// Khung kỹ thuật hỗ trợ nhãn phụ góc phải (top right tag) và khối diff code nổi bật
    DiffCard,
}

/****
 * Struct: PopupWidget
 * Chức năng: Component hiển thị cửa sổ nổi (Modal / Tooltip / Node Card) trên giao diện TUI.
 * Ranh giới bảo vệ:
 *   - Tự động gọi `Clear` để xóa sạch các cell buffer phía dưới, ngăn chặn hiện tượng rách hình hoặc chữ đè chéo.
 *   - Truncate nội dung theo kích thước Rect được cấp phát, triệt tiêu nguy cơ panic tràn biên.
 *   - 100% Zero-Emoji: Chỉ sử dụng các glyph hình học đơn sắc chuẩn BMP để bảo đảm độ rộng ô hiển thị.
 ****/
pub struct PopupWidget {
    pub style: PopupStyle,
    pub title: String,
    pub title_icon: Option<String>,
    pub title_color: Color,
    pub top_right_tag: Option<String>,
    pub top_right_tag_color: Color,
    pub border_color: Color,
    pub bg_color: Color,
    pub bottom_tag: Option<String>,
    pub bottom_tag_color: Color,
    pub lines: Vec<Line<'static>>,
}

impl PopupWidget {
    /****
     * Hàm: new
     * Chức năng: Khởi tạo PopupWidget với kiểu dáng và tiêu đề cơ bản.
     ****/
    pub fn new(style: PopupStyle, title: impl Into<String>) -> Self {
        Self {
            style,
            title: title.into(),
            title_icon: None,
            title_color: Theme::PRIMARY,
            top_right_tag: None,
            top_right_tag_color: Theme::SECONDARY,
            border_color: Theme::NEUTRAL_100,
            bg_color: Theme::SURFACE,
            bottom_tag: None,
            bottom_tag_color: Theme::SECONDARY,
            lines: Vec::new(),
        }
    }

    /// Đặt biểu tượng đứng trước tiêu đề (dùng hằng số từ Icons: Icons::DIAMOND_FILLED, Icons::POINTER...)
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.title_icon = Some(icon.into());
        self
    }

    /// Đặt màu sắc cho tiêu đề và biểu tượng
    pub fn with_title_color(mut self, color: Color) -> Self {
        self.title_color = color;
        self
    }

    /// Đặt nhãn phụ hiển thị ở góc phải viền trên (ví dụ: "callkit", "v1.2.0")
    pub fn with_top_right_tag(mut self, tag: impl Into<String>, color: Color) -> Self {
        self.top_right_tag = Some(tag.into());
        self.top_right_tag_color = color;
        self
    }

    /// Đặt màu viền bao quanh (Border Color)
    pub fn with_border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    /// Đặt màu nền của popup (Glassmorphic Surface)
    pub fn with_bg_color(mut self, color: Color) -> Self {
        self.bg_color = color;
        self
    }

    /// Đặt nhãn/trạng thái lồng ở viền dưới (thích hợp cho MicroBadge: "● leading")
    pub fn with_bottom_tag(mut self, tag: impl Into<String>, color: Color) -> Self {
        self.bottom_tag = Some(tag.into());
        self.bottom_tag_color = color;
        self
    }

    /// Thêm một dòng nội dung vào popup
    pub fn with_line(mut self, line: Line<'static>) -> Self {
        self.lines.push(line);
        self
    }

    /// Thêm danh sách dòng nội dung
    pub fn with_lines(mut self, lines: impl IntoIterator<Item = Line<'static>>) -> Self {
        self.lines.extend(lines);
        self
    }

    // -------------------------------------------------------------------------
    // HÀM TIỆN ÍCH TÍNH TOÁN VỊ TRÍ FLOATING (LAYOUT HELPERS)
    // -------------------------------------------------------------------------

    /****
     * Hàm: centered_rect
     * Chức năng: Tính toán vùng Rect nổi căn giữa màn hình theo phần trăm chiều rộng & chiều cao.
     ****/
    pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_width = (r.width as u32 * percent_x as u32 / 100) as u16;
        let popup_height = (r.height as u32 * percent_y as u32 / 100) as u16;
        let x = r.x + (r.width.saturating_sub(popup_width)) / 2;
        let y = r.y + (r.height.saturating_sub(popup_height)) / 2;
        Rect::new(x, y, popup_width, popup_height)
    }

    /****
     * Hàm: fixed_rect
     * Chức năng: Tính toán vùng Rect nổi với kích thước pixel/cell cố định, căn giữa màn hình.
     ****/
    pub fn fixed_rect(width: u16, height: u16, r: Rect) -> Rect {
        let w = width.min(r.width);
        let h = height.min(r.height);
        let x = r.x + (r.width.saturating_sub(w)) / 2;
        let y = r.y + (r.height.saturating_sub(h)) / 2;
        Rect::new(x, y, w, h)
    }

    // -------------------------------------------------------------------------
    // RENDER ENGINE
    // -------------------------------------------------------------------------

    /****
     * Hàm: render
     * Chức năng: Thực hiện vẽ đè (floating render) toàn bộ nội dung popup lên Frame.
     * Ranh giới bảo vệ:
     *   - Bắt buộc gọi `f.render_widget(Clear, area)` để xóa layer phía dưới.
     *   - Vẽ từng đường biên, title lồng viền (embedded title) và nội dung theo Style đã chọn.
     ****/
    pub fn render(&self, area: Rect, f: &mut Frame) {
        if area.width < 4 || area.height < 3 {
            // Không đủ không gian hiển thị tối thiểu để dựng khung
            return;
        }

        // 1. Xóa sạch lớp nền buffer phía dưới để tránh text bị xuyên thấu
        f.render_widget(Clear, area);

        match self.style {
            PopupStyle::PipelineCard => self.render_pipeline_card(area, f),
            PopupStyle::MicroBadge => self.render_micro_badge(area, f),
            PopupStyle::HudBracket => self.render_hud_bracket(area, f),
            PopupStyle::DiffCard => self.render_diff_card(area, f),
        }
    }

    /// Vẽ Popup kiểu Pipeline Task Node (Rounded border + Accent baseline)
    fn render_pipeline_card(&self, area: Rect, f: &mut Frame) {
        let w = area.width as usize;
        let mut rendered_lines: Vec<Line<'static>> = Vec::new();

        // 1. Dòng đỉnh: ╭─ [icon ]title ──────╮
        let mut top_spans = Vec::new();
        top_spans.push(Span::styled("╭─ ", Style::default().fg(self.border_color)));
        if let Some(ref icon) = self.title_icon {
            top_spans.push(Span::styled(format!("{} ", icon), Style::default().fg(self.title_color)));
        }
        top_spans.push(Span::styled(
            self.title.clone(),
            Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD),
        ));

        let prefix_len = 3 + self.title_icon.as_ref().map(|i| i.chars().count() + 1).unwrap_or(0) + self.title.chars().count();
        let remaining = w.saturating_sub(prefix_len + 1); // chừa đúng 1 ô cho ký tự '╮'
        let fill = "─".repeat(remaining.saturating_sub(1));
        top_spans.push(Span::styled(format!(" {}╮", fill), Style::default().fg(self.border_color)));
        rendered_lines.push(Line::from(top_spans));

        // 2. Thân card
        let inner_height = area.height.saturating_sub(2) as usize;
        let inner_width = area.width.saturating_sub(2) as usize;

        for i in 0..inner_height {
            if let Some(line) = self.lines.get(i) {
                let mut body_spans = Vec::new();
                body_spans.push(Span::styled("│ ", Style::default().fg(self.border_color)));
                
                let mut line_len = 0;
                for span in &line.spans {
                    let count = span.content.chars().count();
                    if line_len + count <= inner_width.saturating_sub(2) {
                        body_spans.push(span.clone());
                        line_len += count;
                    } else {
                        let avail = inner_width.saturating_sub(2).saturating_sub(line_len);
                        if avail > 0 {
                            let truncated: String = span.content.chars().take(avail).collect();
                            body_spans.push(Span::styled(truncated, span.style));
                            line_len += avail;
                        }
                        break;
                    }
                }

                let pad = inner_width.saturating_sub(line_len + 1);
                body_spans.push(Span::raw(" ".repeat(pad)));
                body_spans.push(Span::styled("│", Style::default().fg(self.border_color)));
                rendered_lines.push(Line::from(body_spans));
            } else {
                rendered_lines.push(Line::from(vec![
                    Span::styled("│", Style::default().fg(self.border_color)),
                    Span::raw(" ".repeat(inner_width)),
                    Span::styled("│", Style::default().fg(self.border_color)),
                ]));
            }
        }

        // 3. Đáy card (Baseline màu cam/đồng đặc trưng)
        let bot_fill = "─".repeat(w.saturating_sub(2));
        rendered_lines.push(Line::from(vec![
            Span::styled("╰", Style::default().fg(self.border_color)),
            Span::styled(bot_fill, Style::default().fg(Theme::ORANGE).add_modifier(Modifier::BOLD)),
            Span::styled("╯", Style::default().fg(self.border_color)),
        ]));

        let p = Paragraph::new(rendered_lines).style(Style::default().bg(self.bg_color));
        f.render_widget(p, area);
    }

    /// Vẽ Popup kiểu Micro Node Badge (Label lồng viền trên & Status lồng viền dưới)
    fn render_micro_badge(&self, area: Rect, f: &mut Frame) {
        let w = area.width as usize;
        let mut rendered_lines: Vec<Line<'static>> = Vec::new();

        // 1. Dòng đỉnh: ╭─ [icon ]title ──────╮
        let mut top_spans = Vec::new();
        top_spans.push(Span::styled("╭─ ", Style::default().fg(self.border_color)));
        if let Some(ref icon) = self.title_icon {
            top_spans.push(Span::styled(format!("{} ", icon), Style::default().fg(self.title_color)));
        }
        top_spans.push(Span::styled(
            self.title.clone(),
            Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD),
        ));

        let prefix_len = 3 + self.title_icon.as_ref().map(|i| i.chars().count() + 1).unwrap_or(0) + self.title.chars().count();
        let remaining = w.saturating_sub(prefix_len + 1);
        let fill = "─".repeat(remaining.saturating_sub(1));
        top_spans.push(Span::styled(format!(" {}╮", fill), Style::default().fg(self.border_color)));
        rendered_lines.push(Line::from(top_spans));

        // 2. Thân nội dung
        let inner_height = area.height.saturating_sub(2) as usize;
        let inner_width = area.width.saturating_sub(2) as usize;

        for i in 0..inner_height {
            if let Some(line) = self.lines.get(i) {
                let mut body_spans = Vec::new();
                body_spans.push(Span::styled("│ ", Style::default().fg(self.border_color)));

                let mut line_len = 0;
                for span in &line.spans {
                    let count = span.content.chars().count();
                    if line_len + count <= inner_width.saturating_sub(2) {
                        body_spans.push(span.clone());
                        line_len += count;
                    } else {
                        let avail = inner_width.saturating_sub(2).saturating_sub(line_len);
                        if avail > 0 {
                            let truncated: String = span.content.chars().take(avail).collect();
                            body_spans.push(Span::styled(truncated, span.style));
                            line_len += avail;
                        }
                        break;
                    }
                }

                let pad = inner_width.saturating_sub(line_len + 1);
                body_spans.push(Span::raw(" ".repeat(pad)));
                body_spans.push(Span::styled("│", Style::default().fg(self.border_color)));
                rendered_lines.push(Line::from(body_spans));
            } else {
                rendered_lines.push(Line::from(vec![
                    Span::styled("│", Style::default().fg(self.border_color)),
                    Span::raw(" ".repeat(inner_width)),
                    Span::styled("│", Style::default().fg(self.border_color)),
                ]));
            }
        }

        // 3. Đáy lồng status pin: ╰── ● leading ──╯
        if let Some(ref bot_tag) = self.bottom_tag {
            let mut bot_spans = Vec::new();
            bot_spans.push(Span::styled("╰── ", Style::default().fg(self.border_color)));
            bot_spans.push(Span::styled(bot_tag.clone(), Style::default().fg(self.bottom_tag_color)));
            let bot_len = 4 + bot_tag.chars().count();
            let bot_rem = w.saturating_sub(bot_len + 1);
            let bot_fill = "─".repeat(bot_rem.saturating_sub(1));
            bot_spans.push(Span::styled(format!(" {}╯", bot_fill), Style::default().fg(self.border_color)));
            rendered_lines.push(Line::from(bot_spans));
        } else {
            let bot_fill = "─".repeat(w.saturating_sub(2));
            rendered_lines.push(Line::from(vec![
                Span::styled("╰", Style::default().fg(self.border_color)),
                Span::styled(bot_fill, Style::default().fg(self.border_color)),
                Span::styled("╯", Style::default().fg(self.border_color)),
            ]));
        }

        let p = Paragraph::new(rendered_lines).style(Style::default().bg(self.bg_color));
        f.render_widget(p, area);
    }

    /// Vẽ Popup kiểu Cyber HUD Bracket (4 góc rời)
    fn render_hud_bracket(&self, area: Rect, f: &mut Frame) {
        let w = area.width as usize;
        let mut rendered_lines: Vec<Line<'static>> = Vec::new();
        let bracket_color = if self.border_color == Theme::NEUTRAL_100 {
            Theme::ACCENT
        } else {
            self.border_color
        };

        // 1. Dòng đỉnh: ┌                                   ┐
        let top_gap = w.saturating_sub(2);
        rendered_lines.push(Line::from(vec![
            Span::styled("┌", Style::default().fg(bracket_color).add_modifier(Modifier::BOLD)),
            Span::raw(" ".repeat(top_gap)),
            Span::styled("┐", Style::default().fg(bracket_color).add_modifier(Modifier::BOLD)),
        ]));

        // 2. Thân nội dung
        let inner_height = area.height.saturating_sub(2) as usize;
        let inner_width = area.width.saturating_sub(4) as usize;

        let mut line_idx = 0;
        if !self.title.is_empty() && inner_height > 0 {
            let mut h_spans = Vec::new();
            h_spans.push(Span::raw("  "));
            if let Some(ref icon) = self.title_icon {
                h_spans.push(Span::styled(format!("{} ", icon), Style::default().fg(self.title_color)));
            }
            h_spans.push(Span::styled(
                self.title.clone(),
                Style::default().fg(Theme::SECONDARY).add_modifier(Modifier::BOLD),
            ));
            let h_len = 2 + self.title_icon.as_ref().map(|i| i.chars().count() + 1).unwrap_or(0) + self.title.chars().count();
            let pad = w.saturating_sub(h_len);
            h_spans.push(Span::raw(" ".repeat(pad)));
            rendered_lines.push(Line::from(h_spans));
            line_idx += 1;
        }

        for i in line_idx..inner_height {
            let data_idx = i - line_idx;
            if let Some(line) = self.lines.get(data_idx) {
                let mut body_spans = Vec::new();
                body_spans.push(Span::raw("  "));

                let mut line_len = 0;
                for span in &line.spans {
                    let count = span.content.chars().count();
                    if line_len + count <= inner_width {
                        body_spans.push(span.clone());
                        line_len += count;
                    } else {
                        let avail = inner_width.saturating_sub(line_len);
                        if avail > 0 {
                            let truncated: String = span.content.chars().take(avail).collect();
                            body_spans.push(Span::styled(truncated, span.style));
                            line_len += avail;
                        }
                        break;
                    }
                }

                let pad = w.saturating_sub(line_len + 2);
                body_spans.push(Span::raw(" ".repeat(pad)));
                rendered_lines.push(Line::from(body_spans));
            } else {
                rendered_lines.push(Line::from(Span::raw(" ".repeat(w))));
            }
        }

        // 3. Dòng đáy: └                                   ┘
        let bot_gap = w.saturating_sub(2);
        rendered_lines.push(Line::from(vec![
            Span::styled("└", Style::default().fg(bracket_color).add_modifier(Modifier::BOLD)),
            Span::raw(" ".repeat(bot_gap)),
            Span::styled("┘", Style::default().fg(bracket_color).add_modifier(Modifier::BOLD)),
        ]));

        let p = Paragraph::new(rendered_lines).style(Style::default().bg(self.bg_color));
        f.render_widget(p, area);
    }

    /// Vẽ Popup kiểu Code Diff & Technical Build Card (Top Right Tag + Full-Width Diff Highlighting)
    fn render_diff_card(&self, area: Rect, f: &mut Frame) {
        let w = area.width as usize;
        let mut rendered_lines: Vec<Line<'static>> = Vec::new();

        // 1. Dòng đỉnh: ╭─ [icon ]title ──────── callkit ─╮
        let mut top_spans = Vec::new();
        top_spans.push(Span::styled("╭─ ", Style::default().fg(self.border_color)));
        if let Some(ref icon) = self.title_icon {
            top_spans.push(Span::styled(format!("{} ", icon), Style::default().fg(self.title_color)));
        }
        top_spans.push(Span::styled(
            self.title.clone(),
            Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD),
        ));

        let prefix_len = 3 + self.title_icon.as_ref().map(|i| i.chars().count() + 1).unwrap_or(0) + self.title.chars().count();
        if let Some(ref right_tag) = self.top_right_tag {
            let right_len = right_tag.chars().count() + 3; // " tag ─╮"
            if w > prefix_len + right_len + 2 {
                let fill_len = w.saturating_sub(prefix_len + right_len + 1);
                top_spans.push(Span::styled(format!(" {} ", "─".repeat(fill_len)), Style::default().fg(self.border_color)));
                top_spans.push(Span::styled(right_tag.clone(), Style::default().fg(self.top_right_tag_color)));
                top_spans.push(Span::styled(" ─╮", Style::default().fg(self.border_color)));
            } else {
                top_spans.push(Span::styled(" ─╮", Style::default().fg(self.border_color)));
            }
        } else {
            let remaining = w.saturating_sub(prefix_len + 1);
            let fill = "─".repeat(remaining.saturating_sub(1));
            top_spans.push(Span::styled(format!(" {}╮", fill), Style::default().fg(self.border_color)));
        }
        rendered_lines.push(Line::from(top_spans));

        // 2. Thân nội dung với hỗ trợ kéo dài màu nền diff block
        let inner_height = area.height.saturating_sub(2) as usize;
        let inner_width = area.width.saturating_sub(2) as usize;

        for i in 0..inner_height {
            if let Some(line) = self.lines.get(i) {
                let mut body_spans = Vec::new();
                body_spans.push(Span::styled("│", Style::default().fg(self.border_color)));

                // Kiểm tra xem span đầu tiên có background màu diff hay không (add/remove block)
                let line_bg = line.spans.first().and_then(|s| s.style.bg);

                let mut line_len = 0;
                for span in &line.spans {
                    let count = span.content.chars().count();
                    if line_len + count <= inner_width {
                        body_spans.push(span.clone());
                        line_len += count;
                    } else {
                        let avail = inner_width.saturating_sub(line_len);
                        if avail > 0 {
                            let truncated: String = span.content.chars().take(avail).collect();
                            body_spans.push(Span::styled(truncated, span.style));
                            line_len += avail;
                        }
                        break;
                    }
                }

                let pad = inner_width.saturating_sub(line_len);
                if let Some(bg) = line_bg {
                    body_spans.push(Span::styled(" ".repeat(pad), Style::default().bg(bg)));
                } else {
                    body_spans.push(Span::raw(" ".repeat(pad)));
                }
                body_spans.push(Span::styled("│", Style::default().fg(self.border_color)));
                rendered_lines.push(Line::from(body_spans));
            } else {
                rendered_lines.push(Line::from(vec![
                    Span::styled("│", Style::default().fg(self.border_color)),
                    Span::raw(" ".repeat(inner_width)),
                    Span::styled("│", Style::default().fg(self.border_color)),
                ]));
            }
        }

        // 3. Dòng đáy: ╰────────────────────────╮
        let bot_fill = "─".repeat(w.saturating_sub(2));
        rendered_lines.push(Line::from(vec![
            Span::styled("╰", Style::default().fg(self.border_color)),
            Span::styled(bot_fill, Style::default().fg(self.border_color)),
            Span::styled("╯", Style::default().fg(self.border_color)),
        ]));

        let p = Paragraph::new(rendered_lines).style(Style::default().bg(self.bg_color));
        f.render_widget(p, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::icons::Icons;

    #[test]
    fn test_popup_widget_creation() {
        let p = PopupWidget::new(PopupStyle::PipelineCard, "fable-4")
            .with_icon(Icons::SPARKLE_FILLED.trim())
            .with_border_color(Theme::NEUTRAL_100)
            .with_line(Line::from("pipes  91%"));

        assert_eq!(p.title, "fable-4");
        assert_eq!(p.title_icon.as_deref(), Some(Icons::SPARKLE_FILLED.trim()));
        assert_eq!(p.lines.len(), 1);
    }

    #[test]
    fn test_diff_card_creation() {
        let p = PopupWidget::new(PopupStyle::DiffCard, "astra-1")
            .with_icon(Icons::DIAMOND_FILLED.trim())
            .with_top_right_tag("callkit", Theme::SECONDARY);

        assert_eq!(p.title, "astra-1");
        assert_eq!(p.top_right_tag.as_deref(), Some("callkit"));
        assert_eq!(p.style, PopupStyle::DiffCard);
    }

    #[test]
    fn test_popup_rect_helpers() {
        let screen = Rect::new(0, 0, 100, 50);
        let center = PopupWidget::centered_rect(50, 40, screen);
        assert_eq!(center.width, 50);
        assert_eq!(center.height, 20);
        assert_eq!(center.x, 25);
        assert_eq!(center.y, 15);

        let fixed = PopupWidget::fixed_rect(30, 10, screen);
        assert_eq!(fixed.width, 30);
        assert_eq!(fixed.height, 10);
        assert_eq!(fixed.x, 35);
        assert_eq!(fixed.y, 20);
    }
}
