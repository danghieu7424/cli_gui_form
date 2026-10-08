// --- TEST ĐỘC LẬP: TEST HIỂN THỊ NÚT BẤM PODCAST VÀ KIỂM TRA CO RÚT ---
// File: examples/test_podcast_button.rs
// Mục đích: Cho phép người dùng test trực tiếp nút "▶ BẮT ĐẦU TẠO PODCAST (Enter)"
//          để kiểm tra xem glyph Unicode '▶' và tiếng Việt có bị co, méo, hay giật lề khi Focus/Blur.

use cli_gui_form::{
    ButtonWidget, EventResult, FormManager, Icons,
    InputMode, InputWidget, SelectWidget, Theme,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Terminal,
};
use std::{
    io::stdout,
    time::Duration,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 1. Khởi tạo Form Manager chứa form mẫu Podcast
    let mut form = FormManager::new();

    form.add_widget(Box::new(
        InputWidget::new("podcast_title", "Tiêu đề tập Podcast", InputMode::Text)
            .with_placeholder("Ví dụ: Hành trình khám phá AI Agents 2026..."),
    ));

    form.add_widget(Box::new(
        SelectWidget::new(
            "voice_model",
            "Mô hình giọng đọc (Voice TTS)",
            vec![
                ("gemini-flash", "Gemini 2.5 Flash Audio (Neural High-Def)"),
                ("elevenlabs", "ElevenLabs Multi-Lingual v2 (Studio Voice)"),
                ("openai-tts", "OpenAI TTS-1 HD (Onyx / Nova)"),
            ],
        )
        .with_selected(0),
    ));

    // Nút 1: [GỐC] Toàn bộ text BOLD khi focus (Tái hiện lỗi co glyph nếu có va chạm font)
    form.add_widget(Box::new(
        ButtonWidget::new(
            "btn_podcast_goc",
            "▶ [GỐC: Text BOLD] BẮT ĐẦU TẠO PODCAST (Enter)",
            Theme::GRAY_22,
            Theme::PRIMARY,
        )
        .with_bordered(true)
        .with_centered(true)
        .with_focused_colors(Theme::GRAY_33, Theme::PRIMARY),
    ));

    // Nút 2: [TEST 1 THEO ĐỀ XUẤT USER] Chữ đầu tiên KHÔNG BOLD, các chữ sau BOLD
    form.add_widget(Box::new(
        ButtonWidget::new(
            "btn_podcast_unbold_first",
            "▶ [TEST 1: Chữ đầu KHÔNG Bold] BẮT ĐẦU TẠO PODCAST (Enter)",
            Theme::GRAY_22,
            Theme::PRIMARY,
        )
        .with_bordered(true)
        .with_centered(true)
        .with_first_char_unbold(true)
        .with_focused_colors(Theme::GRAY_33, Theme::PRIMARY),
    ));

    // Nút 3: [TEST 2 ĐỆM 2 SPACES] Icon cách text 2 khoảng trắng đệm
    form.add_widget(Box::new(
        ButtonWidget::new(
            "btn_podcast_double_space",
            "▶ [TEST 2: Đệm 2 Spaces] BẮT ĐẦU TẠO PODCAST (Enter)",
            Theme::GRAY_22,
            Theme::PRIMARY,
        )
        .with_bordered(true)
        .with_centered(true)
        .with_double_space_icon(true)
        .with_focused_colors(Theme::GRAY_33, Theme::PRIMARY),
    ));

    // Nút 4: [TEST 3 CHUẨN UI VERCEL/LINEAR] Tắt hoàn toàn BOLD khi hover, chỉ highlight màu sắc
    form.add_widget(Box::new(
        ButtonWidget::new(
            "btn_podcast_no_bold",
            "▶ [TEST 3: Tắt BOLD khi Hover] BẮT ĐẦU TẠO PODCAST (Enter)",
            Theme::ACCENT,
            Theme::WHITE,
        )
        .with_full_width(true)
        .with_centered(true)
        .with_bold(false)
        .with_focused_colors(Theme::PRIMARY, Theme::BG),
    ));

    let mut last_action = String::from("Sẵn sàng. Dùng Tab / ↑ / ↓ để di chuyển focus vào các nút.");

    loop {
        terminal.draw(|f| {
            let size = f.area();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Header
                    Constraint::Min(16),   // Form & Buttons
                    Constraint::Length(4), // Footer & Status
                ])
                .split(size);

            // 1. Header
            let header = Paragraph::new(Line::from(vec![
                Span::styled(format!("  {} TEST CHỐNG CO RÚT NÚT BẤM (BUTTON INSPECTOR)", Icons::RUN), Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
                Span::styled(" ─ Kiểm tra độ ổn định Unicode & Font", Style::default().fg(Theme::SECONDARY)),
            ]))
            .block(Block::default().borders(Borders::BOTTOM).border_style(Style::default().fg(Theme::NEUTRAL_100)));
            f.render_widget(header, chunks[0]);

            // 2. Body Panel
            let panel_block = Block::default()
                .title("─ Podcast Generation Form ─")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Theme::BORDER_FOCUS))
                .style(Style::default().bg(Theme::BG));

            let inner_area = Rect {
                x: chunks[1].x + 2,
                y: chunks[1].y + 1,
                width: chunks[1].width.saturating_sub(4),
                height: chunks[1].height.saturating_sub(2),
            };

            f.render_widget(panel_block, chunks[1]);
            form.render(inner_area, f);

            // 3. Footer status
            let footer = Paragraph::new(vec![
                Line::from(vec![
                    Span::styled("  Trạng thái: ", Style::default().fg(Theme::MUTED)),
                    Span::styled(&last_action, Style::default().fg(Theme::SUCCESS).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(vec![
                    Span::styled("  Điều khiển: [Tab / ↑ / ↓] Chuyển nút  ─  [Enter] Kích hoạt nút  ─  [Esc] Thoát", Style::default().fg(Theme::MUTED)),
                ]),
            ])
            .block(Block::default().borders(Borders::TOP).border_style(Style::default().fg(Theme::NEUTRAL_100)));
            f.render_widget(footer, chunks[2]);
        })?;

        if event::poll(Duration::from_millis(30))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
                        break;
                    }

                    // Form xử lý phím
                    let res = form.handle_event(key);
                    if res == EventResult::Submitted {
                        last_action = format!("✔ Đã nhấn kích hoạt: '▶ BẮT ĐẦU TẠO PODCAST (Enter)' thành công!");
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    println!("=== TEST PODCAST BUTTON HOÀN TẤT ===");
    Ok(())
}
