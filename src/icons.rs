use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

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
    pub const SUCCESS: &'static str = "✓ ";       // U+2713 + space (Check Mark / Success)
    pub const SUCCESS_HEAVY: &'static str = "✔ "; // U+2714 + space (Heavy Check Mark)
    pub const CHECK: &'static str = "✓ ";         // Alias cho SUCCESS
    pub const ERROR: &'static str = "✗ ";         // U+2716 + space
    pub const WARNING: &'static str = "⚠ ";       // U+25B2 / U+26A0 + space
    pub const RUN: &'static str = "▶ ";           // U+25B6 (Black Right-Pointing Triangle) + space
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
    pub const POINTER: &'static str = "▹ ";       // U+25B9 / U+25B8 White small triangle + space
    pub const BRANCH: &'static str = "⤷ ";        // U+21B3 + space
    pub const DIAMOND_EMPTY: &'static str = "◇ "; // U+25C7 + space (White Diamond)
    pub const SNOWFLAKE: &'static str = "❅ ";     // U+2745 + space (Snowflake)
    pub const PROGRESS_FILLED: &'static str = "━"; // U+2501 (Clean Line Filled)
    pub const PROGRESS_EMPTY: &'static str = "─";  // U+2500 (Clean Line Empty)
    pub const PROGRESS_PARALLELOGRAM_FILLED: &'static str = "▰"; // U+25B0 (Black Parallelogram Filled)
    pub const PROGRESS_PARALLELOGRAM_EMPTY: &'static str = "▱";  // U+25B1 (White Parallelogram Empty)
    pub const PROGRESS_RECT_FILLED: &'static str = "▬";          // U+25AC (Black Rectangle Filled)
    pub const PROGRESS_RECT_EMPTY: &'static str = "▭";           // U+25AD (White Rectangle Empty)
    pub const PROGRESS_SQUARE_FILLED: &'static str = "◼";        // U+25FC (Black Medium Square Filled)
    pub const PROGRESS_SQUARE_EMPTY: &'static str = "◻";         // U+25FB (White Medium Square Empty)

    // Cặp ký tự tiến trình chuẩn hóa (Filled, Empty)
    pub const PROGRESS_CHARS_LINE: (&'static str, &'static str) = (Self::PROGRESS_FILLED, Self::PROGRESS_EMPTY);
    pub const PROGRESS_CHARS_PARALLELOGRAM: (&'static str, &'static str) = (Self::PROGRESS_PARALLELOGRAM_FILLED, Self::PROGRESS_PARALLELOGRAM_EMPTY);
    pub const PROGRESS_CHARS_RECT: (&'static str, &'static str) = (Self::PROGRESS_RECT_FILLED, Self::PROGRESS_RECT_EMPTY);
    pub const PROGRESS_CHARS_SQUARE: (&'static str, &'static str) = (Self::PROGRESS_SQUARE_FILLED, Self::PROGRESS_SQUARE_EMPTY);

    // Standard Directional Arrows (Bộ mũi tên định hướng chuẩn Unicode 2-cell)
    pub const ARROW_UP: &'static str = "↑ ";      // U+2191 + space (Upwards Arrow)
    pub const ARROW_DOWN: &'static str = "↓ ";    // U+2193 + space (Downwards Arrow)
    pub const ARROW_LEFT: &'static str = "← ";    // U+2190 + space (Leftwards Arrow)
    pub const ARROW_RIGHT: &'static str = "→ ";   // U+2192 + space (Rightwards Arrow)
    pub const ARROW_UP_DOWN: &'static str = "↕ "; // U+2195 + space (Up Down Arrow / Vertical Axis)
    pub const ARROW_LEFT_RIGHT: &'static str = "↔ "; // U+2194 + space (Left Right Arrow / Horizontal Axis)
    pub const ENTER: &'static str = "↵ ";         // U+21B5 + space (Downwards Arrow with Corner Left / Return Key)
    pub const RELOAD: &'static str = "↻ ";        // U+21BB + space (Clockwise Open Circle Arrow / Refresh)
    pub const UNDO: &'static str = "↺ ";          // U+21BA + space (Anticlockwise Open Circle Arrow / Undo)

    // Safe Geometric Glyphs (Bộ glyph hình học an toàn, zero-clipping trên Windows/Linux Terminal)
    // 1. Squares & Checkboxes
    pub const SQUARE_FILLED: &'static str = "■ ";         // U+25A0 + space (Black Square)
    pub const SQUARE_EMPTY: &'static str = "□ ";          // U+25A1 + space (White Square)
    pub const SQUARE_MEDIUM_FILLED: &'static str = "◼ ";  // U+25FC + space (Black Medium Square)
    pub const SQUARE_MEDIUM_EMPTY: &'static str = "◻ ";   // U+25FB + space (White Medium Square)
    pub const SQUARE_SMALL_FILLED: &'static str = "▪ ";   // U+25AA + space (Black Small Square)
    pub const SQUARE_SHADOW: &'static str = "❒ ";         // U+2752 + space (Upper Right Drop-Shadowed White Square / 3D Box)
    pub const BOX_SHADOW: &'static str = "❒ ";            // Alias cho SQUARE_SHADOW
    pub const BOX: &'static str = "❒ ";                   // Alias cho SQUARE_SHADOW

    // 2. Circles & Radio Targets
    pub const CIRCLE_FILLED: &'static str = "● ";         // U+25CF + space (Black Circle / Active)
    pub const CIRCLE_EMPTY: &'static str = "○ ";          // U+25CB + space (White Circle / Inactive / Off)
    pub const CIRCLE_TARGET: &'static str = "◉ ";         // U+25C9 + space (Fisheye / Bullseye Target)
    pub const RADIO_TARGET: &'static str = "◉ ";          // Alias cho CIRCLE_TARGET
    pub const CIRCLE_DOTTED: &'static str = "◌ ";         // U+25CC + space (Dotted Circle / Idle)
    pub const STATE_IDLE: &'static str = "◌ ";            // Alias cho CIRCLE_DOTTED (Trạng thái chờ / Idle)
    pub const STATE_INACTIVE: &'static str = "○ ";        // Alias cho CIRCLE_EMPTY (Trạng thái tắt / Inactive)
    pub const STATE_ACTIVE: &'static str = "● ";          // Alias cho CIRCLE_FILLED (Trạng thái bật / Active)
    pub const CIRCLE_HALF_LEFT: &'static str = "◐ ";      // U+25D0 + space (Circle Left Half Black)
    pub const CIRCLE_HALF_BOTTOM: &'static str = "◒ ";    // U+25D2 + space (Circle Lower Half Black)
    pub const CIRCLE_HALF_RIGHT: &'static str = "◑ ";     // U+25D1 + space (Circle Right Half Black)
    pub const CIRCLE_HALF_TOP: &'static str = "◓ ";       // U+25D3 + space (Circle Upper Half Black)

    // Chuỗi hoạt họa vòng xoay 4 pha bán cầu (Moon Spinner / Clockwise rotating circle)
    pub const SPINNER_MOON_FRAMES: &'static [&'static str] = &["◐", "◒", "◑", "◓"];

    // 3. Pointers & Triangles
    pub const TRIANGLE_UP: &'static str = "▲ ";           // U+25B2 + space (Black Up Triangle)
    pub const TRIANGLE_DOWN: &'static str = "▼ ";         // U+25BC + space (Black Down Triangle)
    pub const TRIANGLE_RIGHT_SMALL: &'static str = "▸ ";  // U+25B8 + space (Black Right Small Triangle)
    pub const POINTER_FILLED: &'static str = "▸ ";        // Alias cho TRIANGLE_RIGHT_SMALL

    // 4. Diamonds
    pub const DIAMOND_FILLED: &'static str = "◆ ";        // U+25C6 + space (Black Diamond)

    // 5. Media, Navigation & System Glyphs
    pub const HOURGLASS: &'static str = "⧗ ";             // U+29D7 + space (Black Hourglass / Waiting)
    pub const TAB: &'static str = "⇥ ";                   // U+21E5 + space (Rightwards Arrow To Bar / Tab / Jump)
    pub const MENU: &'static str = "≡ ";                  // U+2261 + space (Identical To / Hamburger Menu)
    pub const MUSIC: &'static str = "♪ ";                 // U+266A + space (Eighth Note / Audio Track)
    pub const MUSIC_DOUBLE: &'static str = "♫ ";          // U+266B + space (Beamed Eighth Notes / Playlist / Stereo)
    pub const SQUARE_CONTAINED: &'static str = "▣ ";      // U+25A3 + space (White Square Containing Black Small Square)

    // 6. Technical, Hardware & Targeting Glyphs
    pub const PIN: &'static str = "⚲ ";                   // U+26B2 + space (Location Pin / Anchor Marker)
    pub const SPARK: &'static str = "⌁ ";                 // U+2301 + space (Electric Spark / Volt / Fast Trigger)
    pub const BULLSEYE: &'static str = "◎ ";              // U+25CE + space (Bullseye / Concentric Circle / Target)
    pub const CROSSHAIR: &'static str = "⌖ ";             // U+2316 + space (Position Indicator / Crosshair / GPS)
    pub const APPROX: &'static str = "≈ ";                // U+2248 + space (Almost Equal To / ETA / Wave)

    // 7. Flags & Milestones
    pub const FLAG_FILLED: &'static str = "⚑ ";           // U+2691 + space (Black Flag / Priority Flag / Checkpoint)
    pub const FLAG_EMPTY: &'static str = "⚐ ";            // U+2690 + space (White Flag / Milestone / Unflagged)
    pub const FLAG: &'static str = "⚑ ";                  // Alias cho FLAG_FILLED

    // 8. Celestial, Stars & Links
    pub const SUN: &'static str = "☼ ";                   // U+263C + space (White Sun with Rays / Daylight / Energy)
    pub const STAR_FILLED: &'static str = "★ ";           // U+2605 + space (Black Star / Favorite / Rating)
    pub const STAR_EMPTY: &'static str = "☆ ";            // U+2606 + space (White Star / Unrated)
    pub const STAR: &'static str = "★ ";                  // Alias cho STAR_FILLED
    pub const LIGHTNING: &'static str = "☇ ";             // U+2607 + space (Lightning / Quick Action / Flash)
    pub const NODE_LINK: &'static str = "☌ ";             // U+260C + space (Conjunction / Node Link / Socket)
    pub const PEER_LINK: &'static str = "☍ ";             // U+260D + space (Opposition / Peer Link / Bridge)

    // 9. Ornaments & Angle Brackets (Cặp ngoặc nhọn, chevron, trích dẫn chuẩn Unicode)
    pub const BRACKET_HEAVY_LEFT: &'static str = "❰ ";     // U+2770 + space (Heavy Left-Pointing Angle Bracket Ornament)
    pub const BRACKET_HEAVY_RIGHT: &'static str = "❱ ";    // U+2771 + space (Heavy Right-Pointing Angle Bracket Ornament)
    pub const BRACKET_MEDIUM_LEFT: &'static str = "❬ ";    // U+276C + space (Medium Left-Pointing Angle Bracket Ornament)
    pub const BRACKET_MEDIUM_RIGHT: &'static str = "❭ ";   // U+276D + space (Medium Right-Pointing Angle Bracket Ornament)
    pub const QUOTE_ANGLE_LEFT: &'static str = "❮ ";       // U+276E + space (Heavy Left-Pointing Angle Quotation Mark Ornament)
    pub const QUOTE_ANGLE_RIGHT: &'static str = "❯ ";      // U+276F + space (Heavy Right-Pointing Angle Quotation Mark Ornament)

    // Aliases cho Chevron & Angles
    pub const CHEVRON_HEAVY_LEFT: &'static str = "❰ ";
    pub const CHEVRON_HEAVY_RIGHT: &'static str = "❱ ";
    pub const CHEVRON_MEDIUM_LEFT: &'static str = "❬ ";
    pub const CHEVRON_MEDIUM_RIGHT: &'static str = "❭ ";

    // Cặp ký tự bao quanh (Không khoảng trắng, dùng bọc thẻ hoặc span)
    pub const PAIR_BRACKET_HEAVY: (&'static str, &'static str) = ("❰", "❱");
    pub const PAIR_BRACKET_MEDIUM: (&'static str, &'static str) = ("❬", "❭");
    pub const PAIR_QUOTE_ANGLE: (&'static str, &'static str) = ("❮", "❯");

    /// Trả về chuỗi icon chuẩn ghép cùng text (Zero-overhead logic)
    #[inline]
    pub fn format(icon: &'static str, text: &str) -> String {
        if icon.ends_with(' ') {
            format!("{}{}", icon, text)
        } else {
            format!("{} {}", icon, text)
        }
    }

    /// Tạo Span an toàn cho Icon: Tự động loại bỏ Modifier::BOLD khỏi icon để bảo đảm
    /// glyph không bao giờ bị méo, co lại hay bị clipping trên terminal Windows/Linux
    /// ngay cả khi lập trình viên áp dụng style có BOLD.
    #[inline]
    pub fn span(icon: &'static str, style: Style) -> Span<'static> {
        let safe_style = style.remove_modifier(Modifier::BOLD);
        Span::styled(icon, safe_style)
    }

    /// Ghép Icon và Text thành Line an toàn chuẩn UI:
    /// - Icon tự động được bảo vệ khỏi Modifier::BOLD (không bao giờ co rúm).
    /// - Text bên cạnh vẫn giữ nguyên trọn vẹn toàn bộ style, màu sắc và Modifier::BOLD.
    #[inline]
    pub fn line(
        icon: &'static str,
        icon_color: Color,
        text: impl Into<String>,
        text_style: Style,
    ) -> Line<'static> {
        let icon_str = if icon.ends_with(' ') {
            icon.to_string()
        } else {
            format!("{} ", icon)
        };
        Line::from(vec![
            Span::styled(icon_str, Style::default().fg(icon_color)),
            Span::styled(text.into(), text_style),
        ])
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
        assert_eq!(Icons::SUCCESS, "✓ ");
        assert_eq!(Icons::SUCCESS_HEAVY, "✔ ");
        assert_eq!(Icons::CHECK, "✓ ");
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
        assert_eq!(Icons::ARROW_UP, "↑ ");
        assert_eq!(Icons::ARROW_DOWN, "↓ ");
        assert_eq!(Icons::ARROW_LEFT, "← ");
        assert_eq!(Icons::ARROW_RIGHT, "→ ");
        assert_eq!(Icons::ARROW_UP_DOWN, "↕ ");
        assert_eq!(Icons::ARROW_LEFT_RIGHT, "↔ ");
        assert_eq!(Icons::ENTER, "↵ ");
        assert_eq!(Icons::RELOAD, "↻ ");
        assert_eq!(Icons::UNDO, "↺ ");
        assert_eq!(Icons::BRANCH, "⤷ ");
        assert_eq!(Icons::DIAMOND_EMPTY, "◇ ");
        assert_eq!(Icons::SNOWFLAKE, "❅ ");
        assert_eq!(Icons::PROGRESS_FILLED, "━");
        assert_eq!(Icons::PROGRESS_EMPTY, "─");
        assert_eq!(Icons::PROGRESS_PARALLELOGRAM_FILLED, "▰");
        assert_eq!(Icons::PROGRESS_PARALLELOGRAM_EMPTY, "▱");
        assert_eq!(Icons::PROGRESS_RECT_FILLED, "▬");
        assert_eq!(Icons::PROGRESS_RECT_EMPTY, "▭");
        assert_eq!(Icons::PROGRESS_SQUARE_FILLED, "◼");
        assert_eq!(Icons::PROGRESS_SQUARE_EMPTY, "◻");
        assert_eq!(Icons::PROGRESS_CHARS_LINE, ("━", "─"));
        assert_eq!(Icons::PROGRESS_CHARS_PARALLELOGRAM, ("▰", "▱"));
        assert_eq!(Icons::PROGRESS_CHARS_RECT, ("▬", "▭"));
        assert_eq!(Icons::PROGRESS_CHARS_SQUARE, ("◼", "◻"));

        // Safe Geometric Glyphs
        assert_eq!(Icons::SQUARE_FILLED, "■ ");
        assert_eq!(Icons::SQUARE_EMPTY, "□ ");
        assert_eq!(Icons::SQUARE_MEDIUM_FILLED, "◼ ");
        assert_eq!(Icons::SQUARE_MEDIUM_EMPTY, "◻ ");
        assert_eq!(Icons::SQUARE_SMALL_FILLED, "▪ ");
        assert_eq!(Icons::CIRCLE_FILLED, "● ");
        assert_eq!(Icons::CIRCLE_EMPTY, "○ ");
        assert_eq!(Icons::CIRCLE_TARGET, "◉ ");
        assert_eq!(Icons::RADIO_TARGET, "◉ ");
        assert_eq!(Icons::CIRCLE_DOTTED, "◌ ");
        assert_eq!(Icons::STATE_IDLE, "◌ ");
        assert_eq!(Icons::STATE_INACTIVE, "○ ");
        assert_eq!(Icons::STATE_ACTIVE, "● ");
        assert_eq!(Icons::CIRCLE_HALF_LEFT, "◐ ");
        assert_eq!(Icons::CIRCLE_HALF_BOTTOM, "◒ ");
        assert_eq!(Icons::CIRCLE_HALF_RIGHT, "◑ ");
        assert_eq!(Icons::CIRCLE_HALF_TOP, "◓ ");
        assert_eq!(Icons::SPINNER_MOON_FRAMES, &["◐", "◒", "◑", "◓"]);
        assert_eq!(Icons::TRIANGLE_UP, "▲ ");
        assert_eq!(Icons::TRIANGLE_DOWN, "▼ ");
        assert_eq!(Icons::TRIANGLE_RIGHT_SMALL, "▸ ");
        assert_eq!(Icons::POINTER_FILLED, "▸ ");
        assert_eq!(Icons::DIAMOND_FILLED, "◆ ");

        // Media, Navigation & System Glyphs
        assert_eq!(Icons::HOURGLASS, "⧗ ");
        assert_eq!(Icons::TAB, "⇥ ");
        assert_eq!(Icons::MENU, "≡ ");
        assert_eq!(Icons::MUSIC, "♪ ");
        assert_eq!(Icons::MUSIC_DOUBLE, "♫ ");
        assert_eq!(Icons::SQUARE_CONTAINED, "▣ ");

        // Technical, Hardware & Targeting Glyphs
        assert_eq!(Icons::PIN, "⚲ ");
        assert_eq!(Icons::SPARK, "⌁ ");
        assert_eq!(Icons::BULLSEYE, "◎ ");
        assert_eq!(Icons::CROSSHAIR, "⌖ ");
        assert_eq!(Icons::APPROX, "≈ ");

        // Flags & Milestones
        assert_eq!(Icons::FLAG_FILLED, "⚑ ");
        assert_eq!(Icons::FLAG_EMPTY, "⚐ ");
        assert_eq!(Icons::FLAG, "⚑ ");

        // Celestial, Stars & Links
        assert_eq!(Icons::SUN, "☼ ");
        assert_eq!(Icons::STAR_FILLED, "★ ");
        assert_eq!(Icons::STAR_EMPTY, "☆ ");
        assert_eq!(Icons::STAR, "★ ");
        assert_eq!(Icons::LIGHTNING, "☇ ");
        assert_eq!(Icons::NODE_LINK, "☌ ");
        assert_eq!(Icons::PEER_LINK, "☍ ");

        // Box Shadow & Ornaments
        assert_eq!(Icons::SQUARE_SHADOW, "❒ ");
        assert_eq!(Icons::BOX_SHADOW, "❒ ");
        assert_eq!(Icons::BOX, "❒ ");
        assert_eq!(Icons::BRACKET_HEAVY_LEFT, "❰ ");
        assert_eq!(Icons::BRACKET_HEAVY_RIGHT, "❱ ");
        assert_eq!(Icons::BRACKET_MEDIUM_LEFT, "❬ ");
        assert_eq!(Icons::BRACKET_MEDIUM_RIGHT, "❭ ");
        assert_eq!(Icons::QUOTE_ANGLE_LEFT, "❮ ");
        assert_eq!(Icons::QUOTE_ANGLE_RIGHT, "❯ ");
        assert_eq!(Icons::CHEVRON_HEAVY_LEFT, "❰ ");
        assert_eq!(Icons::CHEVRON_HEAVY_RIGHT, "❱ ");
        assert_eq!(Icons::CHEVRON_MEDIUM_LEFT, "❬ ");
        assert_eq!(Icons::CHEVRON_MEDIUM_RIGHT, "❭ ");
        assert_eq!(Icons::PAIR_BRACKET_HEAVY, ("❰", "❱"));
        assert_eq!(Icons::PAIR_BRACKET_MEDIUM, ("❬", "❭"));
        assert_eq!(Icons::PAIR_QUOTE_ANGLE, ("❮", "❯"));
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

    #[test]
    fn test_icons_safe_span_removes_bold() {
        let bold_style = Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD);
        let span = Icons::span(Icons::RUN, bold_style);
        assert!(!span.style.add_modifier.contains(Modifier::BOLD));
        assert_eq!(span.content, Icons::RUN);
    }
}
