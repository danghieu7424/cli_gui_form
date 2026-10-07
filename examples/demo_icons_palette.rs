// --- PHÂN ĐOẠN: DEMO KIỂM THỬ TRỰC QUAN TOÀN BỘ ICONS & COLOR PALETTE (DESIGN.MD) ---

use cli_gui_form::{Icons, Theme};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Terminal,
};
use std::{
    io::{self, stdout},
    time::{Duration, Instant},
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut pulse_idx = 0;
    let mut think_idx = 0;
    let mut last_tick = Instant::now();
    let mut needs_render = true;

    loop {
        // Cập nhật nhịp nháy pulse 150ms
        if last_tick.elapsed() >= Duration::from_millis(150) {
            pulse_idx = (pulse_idx + 1) % Icons::PULSE_FRAMES.len();
            think_idx = (think_idx + 1) % Icons::THINKING_FRAMES.len();
            last_tick = Instant::now();
            needs_render = true;
        }

        if needs_render {
            terminal.draw(|f| {
                let full_area = f.area();

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3), // Header
                        Constraint::Min(16),   // Content
                        Constraint::Length(1), // Footer
                    ])
                    .split(full_area);

                // 1. Header
                let header = Paragraph::new(Line::from(vec![
                    Span::styled("  LINEAR / VERCEL TUI", Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
                    Span::styled(" ─ Icons & Color Palette Showcase", Style::default().fg(Theme::SECONDARY)),
                ]))
                .style(Style::default().bg(Theme::BG));
                f.render_widget(header, chunks[0]);

                // 2. Chia đôi màn hình: Cột trái (Color Palette), Cột phải (Icons Showcase)
                let main_cols = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ])
                    .split(chunks[1]);

                // PANEL TRÁI: Color Palette (Semantic Roles & Neutral Scale)
                let color_block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Theme::NEUTRAL_100))
                    .title(Line::from(vec![
                        Span::styled("─ ", Style::default().fg(Theme::NEUTRAL_100)),
                        Span::styled("Color Palette (DESIGN.md)", Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
                        Span::styled(" ─", Style::default().fg(Theme::NEUTRAL_100)),
                    ]))
                    .style(Style::default().bg(Theme::BG));

                let color_lines = vec![
                    Line::from(Span::styled("  Semantic Roles:", Style::default().fg(Theme::SECONDARY).add_modifier(Modifier::BOLD))),
                    Line::from(vec![
                        Span::styled("   ■ ", Style::default().fg(Theme::BG)),
                        Span::styled("Background  ", Style::default().fg(Theme::FG)),
                        Span::styled("#0a0a0a  (ANSI 232)", Style::default().fg(Theme::MUTED)),
                    ]),
                    Line::from(vec![
                        Span::styled("   ■ ", Style::default().fg(Theme::FG)),
                        Span::styled("Foreground  ", Style::default().fg(Theme::FG)),
                        Span::styled("#ededed  (ANSI 255)", Style::default().fg(Theme::MUTED)),
                    ]),
                    Line::from(vec![
                        Span::styled("   ■ ", Style::default().fg(Theme::PRIMARY)),
                        Span::styled("Primary     ", Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
                        Span::styled("#ffffff  (ANSI 15)", Style::default().fg(Theme::MUTED)),
                    ]),
                    Line::from(vec![
                        Span::styled("   ■ ", Style::default().fg(Theme::SECONDARY)),
                        Span::styled("Secondary   ", Style::default().fg(Theme::SECONDARY)),
                        Span::styled("#888888  (ANSI 245)", Style::default().fg(Theme::MUTED)),
                    ]),
                    Line::from(vec![
                        Span::styled("   ■ ", Style::default().fg(Theme::ACCENT)),
                        Span::styled("Accent      ", Style::default().fg(Theme::ACCENT).add_modifier(Modifier::BOLD)),
                        Span::styled("#0070f3  (ANSI 33 - Vercel Blue)", Style::default().fg(Theme::MUTED)),
                    ]),
                    Line::from(vec![
                        Span::styled("   ■ ", Style::default().fg(Theme::SUCCESS)),
                        Span::styled("Success     ", Style::default().fg(Theme::SUCCESS)),
                        Span::styled("#00c853  (ANSI 41)", Style::default().fg(Theme::MUTED)),
                    ]),
                    Line::from(vec![
                        Span::styled("   ■ ", Style::default().fg(Theme::WARNING)),
                        Span::styled("Warning     ", Style::default().fg(Theme::WARNING)),
                        Span::styled("#f5a623  (ANSI 214)", Style::default().fg(Theme::MUTED)),
                    ]),
                    Line::from(vec![
                        Span::styled("   ■ ", Style::default().fg(Theme::ERROR)),
                        Span::styled("Error       ", Style::default().fg(Theme::ERROR)),
                        Span::styled("#ee0000  (ANSI 196)", Style::default().fg(Theme::MUTED)),
                    ]),
                    Line::from(vec![
                        Span::styled("   ■ ", Style::default().fg(Theme::MUTED)),
                        Span::styled("Muted       ", Style::default().fg(Theme::MUTED)),
                        Span::styled("#555555  (ANSI 240)", Style::default().fg(Theme::MUTED)),
                    ]),
                    Line::from(vec![
                        Span::styled("   ■ ", Style::default().fg(Theme::SURFACE)),
                        Span::styled("Surface     ", Style::default().fg(Theme::SURFACE)),
                        Span::styled("#1a1a1a  (ANSI 234)", Style::default().fg(Theme::MUTED)),
                    ]),
                    Line::from(""),
                    Line::from(Span::styled("  Neutral Scale (50 -> 500):", Style::default().fg(Theme::SECONDARY).add_modifier(Modifier::BOLD))),
                    Line::from(vec![
                        Span::styled("   ■ ", Style::default().fg(Theme::NEUTRAL_50)),
                        Span::styled("   ■ ", Style::default().fg(Theme::NEUTRAL_100)),
                        Span::styled("   ■ ", Style::default().fg(Theme::NEUTRAL_200)),
                        Span::styled("   ■ ", Style::default().fg(Theme::NEUTRAL_300)),
                        Span::styled("   ■ ", Style::default().fg(Theme::NEUTRAL_400)),
                        Span::styled("   ■ ", Style::default().fg(Theme::NEUTRAL_500)),
                    ]),
                ];
                let p_left = Paragraph::new(color_lines).block(color_block);
                f.render_widget(p_left, main_cols[0]);

                // PANEL PHẢI: Icons & Animations
                let icon_block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Theme::NEUTRAL_100))
                    .title(Line::from(vec![
                        Span::styled("─ ", Style::default().fg(Theme::NEUTRAL_100)),
                        Span::styled("Icons & Indicators (Mục 7 & 8)", Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
                        Span::styled(" ─", Style::default().fg(Theme::NEUTRAL_100)),
                    ]))
                    .style(Style::default().bg(Theme::BG));

                let cur_pulse = Icons::PULSE_FRAMES[pulse_idx];
                let cur_think = Icons::THINKING_FRAMES[think_idx];

                let icon_lines = vec![
                    Line::from(Span::styled("  Status Icons:", Style::default().fg(Theme::SECONDARY).add_modifier(Modifier::BOLD))),
                    Line::from(vec![
                        Span::styled(format!("   {} ", Icons::SUCCESS), Style::default().fg(Icons::color_success()).add_modifier(Modifier::BOLD)),
                        Span::styled("Success (✔)    ", Style::default().fg(Theme::FG)),
                        Span::styled(format!("   {} ", Icons::ERROR), Style::default().fg(Icons::color_error()).add_modifier(Modifier::BOLD)),
                        Span::styled("Error (✗)", Style::default().fg(Theme::FG)),
                    ]),
                    Line::from(vec![
                        Span::styled(format!("   {} ", Icons::WARNING), Style::default().fg(Icons::color_warning()).add_modifier(Modifier::BOLD)),
                        Span::styled("Warning (⚠)    ", Style::default().fg(Theme::FG)),
                        Span::styled(format!("   {} ", Icons::INFO), Style::default().fg(Icons::color_info()).add_modifier(Modifier::BOLD)),
                        Span::styled("Info (ℹ)", Style::default().fg(Theme::FG)),
                    ]),
                    Line::from(vec![
                        Span::styled(format!("   {} ", Icons::RUN), Style::default().fg(Icons::color_run()).add_modifier(Modifier::BOLD)),
                        Span::styled("Running (▶)    ", Style::default().fg(Theme::FG)),
                        Span::styled(format!("   {} ", Icons::BUILD), Style::default().fg(Icons::color_build()).add_modifier(Modifier::BOLD)),
                        Span::styled("Build (⚙)", Style::default().fg(Theme::FG)),
                    ]),
                    Line::from(vec![
                        Span::styled(format!("   {} ", Icons::STOP), Style::default().fg(Icons::color_stop()).add_modifier(Modifier::BOLD)),
                        Span::styled("Stop (■)       ", Style::default().fg(Theme::FG)),
                        Span::styled(format!("   {} ", Icons::PAUSE), Style::default().fg(Icons::color_pending()).add_modifier(Modifier::BOLD)),
                        Span::styled("Pause (⏸)", Style::default().fg(Theme::FG)),
                    ]),
                    Line::from(""),
                    Line::from(Span::styled("  Controls & Pointers:", Style::default().fg(Theme::SECONDARY).add_modifier(Modifier::BOLD))),
                    Line::from(vec![
                        Span::styled(format!("   {} ", Icons::CHECKBOX_ON), Style::default().fg(Theme::SUCCESS)),
                        Span::styled("Checkbox On    ", Style::default().fg(Theme::FG)),
                        Span::styled(format!("   {} ", Icons::CHECKBOX_OFF), Style::default().fg(Theme::MUTED)),
                        Span::styled("Checkbox Off", Style::default().fg(Theme::FG)),
                    ]),
                    Line::from(vec![
                        Span::styled(format!("   {} ", Icons::RADIO_ON), Style::default().fg(Theme::ACCENT)),
                        Span::styled("Radio On       ", Style::default().fg(Theme::FG)),
                        Span::styled(format!("   {} ", Icons::RADIO_OFF), Style::default().fg(Theme::MUTED)),
                        Span::styled("Radio Off", Style::default().fg(Theme::FG)),
                    ]),
                    Line::from(vec![
                        Span::styled(format!("   {} ", Icons::POINTER), Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
                        Span::styled("Selected (▸)   ", Style::default().fg(Theme::FG)),
                        Span::styled(format!("   {} ", Icons::ARROW_RIGHT), Style::default().fg(Theme::SECONDARY)),
                        Span::styled("Arrow (→)", Style::default().fg(Theme::FG)),
                    ]),
                    Line::from(""),
                    Line::from(Span::styled("  Live Animation Spinners (150ms):", Style::default().fg(Theme::SECONDARY).add_modifier(Modifier::BOLD))),
                    Line::from(vec![
                        Span::styled(format!("   {} ", cur_pulse), Style::default().fg(Theme::ACCENT).add_modifier(Modifier::BOLD)),
                        Span::styled(format!("Pulse Dot Animation: [{}]", cur_pulse), Style::default().fg(Theme::FG)),
                    ]),
                    Line::from(vec![
                        Span::styled(format!("   {} ", cur_think), Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
                        Span::styled(format!("Thinking Cycle:       [{}]", cur_think), Style::default().fg(Theme::FG)),
                    ]),
                ];
                let p_right = Paragraph::new(icon_lines).block(icon_block);
                f.render_widget(p_right, main_cols[1]);

                // 3. Status Bar
                let status_line = Line::from(vec![
                    Span::styled(" main ", Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
                    Span::styled("─", Style::default().fg(Theme::MUTED)),
                    Span::styled(" Press Esc to Exit ", Style::default().fg(Theme::SECONDARY)),
                    Span::styled("                                              ✓ Tests Passing ", Style::default().fg(Theme::SUCCESS)),
                ]);
                let status_bar = Paragraph::new(status_line).style(Style::default().bg(Theme::BG));
                f.render_widget(status_bar, chunks[2]);
            })?;
            needs_render = false;
        }

        if event::poll(Duration::from_millis(150))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let is_ctrl_c = key.modifiers.contains(KeyModifiers::CONTROL)
                        && key.code == KeyCode::Char('c');
                    if key.code == KeyCode::Esc || is_ctrl_c || key.code == KeyCode::Char('q') {
                        break;
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    println!("\n=== SHOWCASE HOÀN TẤT ===");
    Ok(())
}
