// --- PHÂN ĐOẠN: BẢNG MÀU VÀ GIAO DIỆN CHUẨN DESIGN.MD (MINIMAL TUI DESIGN SYSTEM) ---

use ratatui::style::Color;

/****
 * Module: Theme
 * Chức năng: Cung cấp bảng màu chuẩn Semantic Roles và Neutral Scale từ DESIGN.md.
 * Ranh giới bảo vệ: Hằng số biên dịch không phân bổ bộ nhớ động (Zero-Cost Abstraction).
 *            Áp dụng chuẩn phong cách Vercel/Linear: nền tối trầm, chữ xám/trắng, điểm nhấn Accent xanh dương.
 ****/
pub struct Theme;

impl Theme {
    // ----------------------------------------------------
    // 1. SEMANTIC ROLES (Màu theo ngữ nghĩa)
    // ----------------------------------------------------
    
    // Background: #0a0a0a (ANSI 232)
    pub const BG: Color = Color::Rgb(0x0a, 0x0a, 0x0a);

    // Foreground: #ededed (ANSI 255)
    pub const FG: Color = Color::Rgb(0xed, 0xed, 0xed);

    // Primary: #ffffff (ANSI 15)
    pub const PRIMARY: Color = Color::Rgb(0xff, 0xff, 0xff);

    // Secondary: #888888 (ANSI 245)
    pub const SECONDARY: Color = Color::Rgb(0x88, 0x88, 0x88);

    // Accent: #0070f3 (ANSI 33) - Vercel blue
    pub const ACCENT: Color = Color::Rgb(0x00, 0x70, 0xf3);

    // Success: #00c853 (ANSI 41)
    pub const SUCCESS: Color = Color::Rgb(0x00, 0xc8, 0x53);

    // Warning: #f5a623 (ANSI 214)
    pub const WARNING: Color = Color::Rgb(0xf5, 0xa6, 0x23);

    // Error: #ee0000 (ANSI 196)
    pub const ERROR: Color = Color::Rgb(0xee, 0x00, 0x00);

    // Muted: #555555 (ANSI 240)
    pub const MUTED: Color = Color::Rgb(0x55, 0x55, 0x55);

    // Surface: #1a1a1a (ANSI 234)
    pub const SURFACE: Color = Color::Rgb(0x1a, 0x1a, 0x1a);

    // ----------------------------------------------------
    // 2. EXTENDED ACCENT & SYSTEM PALETTE (Tránh lặp màu đơn điệu)
    // ----------------------------------------------------

    // Cyan / Teal: #50e3c2 (Vercel Cyan - Network, Latency, API Endpoints)
    pub const CYAN: Color = Color::Rgb(0x50, 0xe3, 0xc2);

    // Purple / Violet: #7928ca (Linear Violet - AI Inference, GraphQL, Plugins)
    pub const PURPLE: Color = Color::Rgb(0x79, 0x28, 0xca);

    // Magenta / Pink: #f81ce5 (Electric Pink - Auth, Secrets, Webhooks)
    pub const MAGENTA: Color = Color::Rgb(0xf8, 0x1c, 0xe5);

    // Orange / Amber: #ff8800 (Warm Amber - Pipelines, Queues, Workers)
    pub const ORANGE: Color = Color::Rgb(0xff, 0x88, 0x00);

    // Indigo: #5e6ad2 (Linear Indigo - Branches, Commits, Tasks)
    pub const INDIGO: Color = Color::Rgb(0x5e, 0x6a, 0xd2);

    // Emerald: #10b981 (Soft Green - Healthy Uptime, In-Memory DBs)
    pub const EMERALD: Color = Color::Rgb(0x10, 0xb9, 0x81);

    // Sky Blue: #38bdf8 (Cloud Infrastructure, Docker, K8s)
    pub const SKY: Color = Color::Rgb(0x38, 0xbd, 0xf8);

    // Critical: #ff0055 (Crimson Red - Fatal, Panic, Immediate Alert)
    pub const CRITICAL: Color = Color::Rgb(0xff, 0x00, 0x55);

    // Surface Elevated: #222222 (Card Header, Highlighted Row Background)
    pub const SURFACE_ELEVATED: Color = Color::Rgb(0x22, 0x22, 0x22);

    // Border Focus: #0070f3 (Viền khi ô form được kích hoạt)
    pub const BORDER_FOCUS: Color = Color::Rgb(0x00, 0x70, 0xf3);

    // ----------------------------------------------------
    // 3. NEUTRAL SCALE (Bảng thang độ xám)
    // ----------------------------------------------------
    pub const NEUTRAL_50: Color = Color::Rgb(0x1a, 0x1a, 0x1a);
    pub const NEUTRAL_100: Color = Color::Rgb(0x2a, 0x2a, 0x2a); // Borders, dividers
    pub const NEUTRAL_200: Color = Color::Rgb(0x44, 0x44, 0x44); // Disabled text
    pub const NEUTRAL_300: Color = Color::Rgb(0x66, 0x66, 0x66); // Placeholder text
    pub const NEUTRAL_400: Color = Color::Rgb(0x88, 0x88, 0x88); // Secondary text
    pub const NEUTRAL_500: Color = Color::Rgb(0xed, 0xed, 0xed); // Body text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_colors() {
        assert_eq!(Theme::BG, Color::Rgb(10, 10, 10));
        assert_eq!(Theme::ACCENT, Color::Rgb(0, 112, 243));
        assert_eq!(Theme::SUCCESS, Color::Rgb(0, 200, 83));
        assert_eq!(Theme::CYAN, Color::Rgb(0x50, 0xe3, 0xc2));
        assert_eq!(Theme::PURPLE, Color::Rgb(0x79, 0x28, 0xca));
        assert_eq!(Theme::ORANGE, Color::Rgb(0xff, 0x88, 0x00));
        assert_eq!(Theme::INDIGO, Color::Rgb(0x5e, 0x6a, 0xd2));
    }
}
