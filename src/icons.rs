// --- PHÂN ĐOẠN: BẢNG KÝ TỰ BIỂU TƯỢNG (ICONS) VÀ BẢNG MÀU CHUẨN TỪ ICON_CLI.MD ---

use ratatui::style::Color;

/****
 * Module: Icons
 * Chức năng: Tập trung hóa toàn bộ ký tự trạng thái, biểu tượng UI và mã màu tương ứng
 *            theo quy chuẩn thiết kế trong icon_cli.md và DESIGN.md.
 * Ranh giới bảo vệ: Hằng số không thay đổi (zero-cost abstraction), đảm bảo tính nhất quán
 *            cho toàn bộ Widgets (Button, Task, Checkbox, Radio, Status).
 ****/
pub struct Icons;

impl Icons {
    // Trạng thái (Status Icons)
    pub const SUCCESS: &'static str = "✔";       // U+2714
    pub const ERROR: &'static str = "✗";         // U+2716
    pub const WARNING: &'static str = "⚠";       // U+25B2 / U+26A0
    pub const RUN: &'static str = "▶";           // U+25B6
    pub const BUILD: &'static str = "⚙";         // U+2699
    pub const INFO: &'static str = "ℹ";          // U+2139
    pub const PAUSE: &'static str = "⏸";         // U+23F8
    pub const STOP: &'static str = "■";          // U+25A0

    // Chuỗi nhịp đập Pending / Thinking
    pub const PULSE_FRAMES: &'static [&'static str] = &["·", "•", "●", "•"];
    pub const THINKING_FRAMES: &'static [&'static str] = &["·", "•", "●", "•", "·", " "];

    // Form Controls (Checkbox & Radio)
    pub const CHECKBOX_ON: &'static str = "☑";   // U+2611
    pub const CHECKBOX_OFF: &'static str = "☐";  // U+2610
    pub const RADIO_ON: &'static str = "●";      // U+25CF
    pub const RADIO_OFF: &'static str = "○";     // U+25CB

    // Bộ chỉ hướng & tiến trình (Progress & Pointers)
    pub const POINTER: &'static str = "▸";       // U+25B8
    pub const ARROW_RIGHT: &'static str = "→";   // U+2192
    pub const BRANCH: &'static str = "↳";        // U+21B3
    pub const PROGRESS_FILLED: &'static str = "━"; // U+2501
    pub const PROGRESS_EMPTY: &'static str = "─";  // U+2500

    /// Màu sắc khuyến nghị tương ứng từng trạng thái theo chuẩn DESIGN.md & Theme
    #[inline]
    pub const fn color_success() -> Color {
        crate::theme::Theme::SUCCESS
    }

    #[inline]
    pub const fn color_error() -> Color {
        crate::theme::Theme::ERROR
    }

    #[inline]
    pub const fn color_warning() -> Color {
        crate::theme::Theme::WARNING
    }

    #[inline]
    pub const fn color_run() -> Color {
        crate::theme::Theme::ACCENT
    }

    #[inline]
    pub const fn color_build() -> Color {
        crate::theme::Theme::PRIMARY
    }

    #[inline]
    pub const fn color_info() -> Color {
        crate::theme::Theme::ACCENT
    }

    #[inline]
    pub const fn color_pending() -> Color {
        crate::theme::Theme::MUTED
    }

    #[inline]
    pub const fn color_stop() -> Color {
        crate::theme::Theme::ERROR
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icons_constants() {
        assert_eq!(Icons::SUCCESS, "✔");
        assert_eq!(Icons::ERROR, "✗");
        assert_eq!(Icons::WARNING, "⚠");
        assert_eq!(Icons::CHECKBOX_ON, "☑");
        assert_eq!(Icons::CHECKBOX_OFF, "☐");
        assert_eq!(Icons::RADIO_ON, "●");
        assert_eq!(Icons::RADIO_OFF, "○");
    }
}
