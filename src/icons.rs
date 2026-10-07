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
    // Trạng thái (Status Icons) - Đã bao gồm 1 space chuẩn 2 ô
    pub const SUCCESS: &'static str = "✔ ";       // U+2714 + space
    pub const ERROR: &'static str = "✗ ";         // U+2716 + space
    pub const WARNING: &'static str = "⚠ ";       // U+25B2 / U+26A0 + space
    pub const RUN: &'static str = "▶ ";           // U+25B6 + space
    pub const BUILD: &'static str = "⚙ ";         // U+2699 + space
    pub const INFO: &'static str = "ℹ ";          // U+2139 + space
    pub const PAUSE: &'static str = "⏸ ";         // U+23F8 + space
    pub const STOP: &'static str = "■ ";          // U+25A0 + space

    // Chuỗi nhịp đập Thinking / Loading (Giữ nguyên không space để ghép nhịp)
    pub const THINKING_FRAMES: &'static [&'static str] = &["·", "•", "●", "•", "·", " "];

    // Form Controls (Checkbox & Radio) - Đã bao gồm 1 space chuẩn
    pub const CHECKBOX_ON: &'static str = "☑ ";   // U+2611 + space
    pub const CHECKBOX_OFF: &'static str = "☐ ";  // U+2610 + space
    pub const RADIO_ON: &'static str = "● ";      // U+25CF + space
    pub const RADIO_OFF: &'static str = "○ ";     // U+25CB + space
    pub const BULLET: &'static str = "▪ ";        // U+25AA + space
    pub const SPARKLE_FILLED: &'static str = "✦ "; // U+2726 + space (Black Four Pointed Star)
    pub const SPARKLE_EMPTY: &'static str = "✧ ";  // U+2727 + space (White Four Pointed Star)
    pub const STAR_OUTLINE: &'static str = "⚝ ";   // U+269D + space (Outlined White Star)

    // Bộ chỉ hướng & tiến trình (Progress & Pointers)
    pub const POINTER: &'static str = "▹ ";       // U+25B8 + space
    pub const ARROW_RIGHT: &'static str = "→ ";   // U+2192 + space
    pub const ARROW_UP: &'static str = "▲ ";      // U+25B2 + space (Black Up-Pointing Triangle)
    pub const BRANCH: &'static str = "⤷ ";        // U+21B3 + space
    pub const DIAMOND_EMPTY: &'static str = "◇ "; // U+25C7 + space (White Diamond)
    pub const SNOWFLAKE: &'static str = "❅ ";     // U+2745 + space (Snowflake)
    pub const PROGRESS_FILLED: &'static str = "━"; // U+2501 (Không space để nối thanh bar)
    pub const PROGRESS_EMPTY: &'static str = "─";  // U+2500 (Không space để nối thanh bar)

    /// Trả về chuỗi icon chuẩn ghép cùng text (Zero-overhead logic)
    #[inline]
    pub fn format(icon: &'static str, text: &str) -> String {
        if icon.ends_with(' ') {
            format!("{}{}", icon, text)
        } else {
            format!("{} {}", icon, text)
        }
    }

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
        assert_eq!(Icons::SUCCESS, "✔ ");
        assert_eq!(Icons::ERROR, "✗ ");
        assert_eq!(Icons::WARNING, "⚠ ");
        assert_eq!(Icons::RUN, "▶ ");
        assert_eq!(Icons::BUILD, "⚙ ");
        assert_eq!(Icons::INFO, "ℹ ");
        assert_eq!(Icons::PAUSE, "⏸ ");
        assert_eq!(Icons::STOP, "■ ");
        assert_eq!(Icons::CHECKBOX_ON, "☑ ");
        assert_eq!(Icons::CHECKBOX_OFF, "☐ ");
        assert_eq!(Icons::RADIO_ON, "● ");
        assert_eq!(Icons::RADIO_OFF, "○ ");
        assert_eq!(Icons::BULLET, "▪ ");
        assert_eq!(Icons::SPARKLE_FILLED, "✦ ");
        assert_eq!(Icons::SPARKLE_EMPTY, "✧ ");
        assert_eq!(Icons::STAR_OUTLINE, "⚝ ");
        assert_eq!(Icons::POINTER, "▹ ");
        assert_eq!(Icons::ARROW_RIGHT, "→ ");
        assert_eq!(Icons::ARROW_UP, "▲ ");
        assert_eq!(Icons::BRANCH, "⤷ ");
        assert_eq!(Icons::DIAMOND_EMPTY, "◇ ");
        assert_eq!(Icons::SNOWFLAKE, "❅ ");
        assert_eq!(Icons::PROGRESS_FILLED, "━");
        assert_eq!(Icons::PROGRESS_EMPTY, "─");
    }

    #[test]
    fn test_pulse_and_thinking_frames() {
        assert_eq!(Icons::THINKING_FRAMES, &["·", "•", "●", "•", "·", " "]);
    }

    #[test]
    fn test_icons_theme_colors() {
        assert_eq!(Icons::color_success(), crate::Theme::SUCCESS);
        assert_eq!(Icons::color_error(), crate::Theme::ERROR);
        assert_eq!(Icons::color_warning(), crate::Theme::WARNING);
        assert_eq!(Icons::color_run(), crate::Theme::ACCENT);
        assert_eq!(Icons::color_build(), crate::Theme::PRIMARY);
        assert_eq!(Icons::color_info(), crate::Theme::ACCENT);
        assert_eq!(Icons::color_pending(), crate::Theme::MUTED);
        assert_eq!(Icons::color_stop(), crate::Theme::ERROR);
    }
}
