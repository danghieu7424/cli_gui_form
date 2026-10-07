// --- PHÂN ĐOẠN: DEMO TABS & STATUS BAR CHUẨN MINIMAL TUI DESIGN SYSTEM ---

use cli_gui_form::{FormWidget, StatusBarWidget, TabsWidget, Theme};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
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
    time::Duration,
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut tabs = TabsWidget::new("main_nav", vec!["Overview", "Logs", "Settings", "Deployments"]);
    tabs.focus();

    loop {
        terminal.draw(|f| {
            let size = f.area();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // 1. Tabs Header
                    Constraint::Min(8),    // 2. Main Content Panel
                    Constraint::Length(1), // 3. Status Bar
                ])
                .split(size);

            // 1. Render Tabs Component (DESIGN.md mục 5)
            tabs.render(chunks[0], f);

            // 2. Main Content theo Tab đang chọn
            let (tab_title, tab_content) = match tabs.selected() {
                0 => (
                    "Overview",
                    vec![
                        Line::from(vec![
                            Span::styled("● ", Style::default().fg(Theme::SUCCESS)),
                            Span::styled("System Status: ", Style::default().fg(Theme::SECONDARY)),
                            Span::styled("All services operational", Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
                        ]),
                        Line::from(""),
                        Line::from(vec![
                            Span::styled("  Uptime:       ", Style::default().fg(Theme::SECONDARY)),
                            Span::styled("99.98% (34 days, 12 hours)", Style::default().fg(Theme::FG)),
                        ]),
                        Line::from(vec![
                            Span::styled("  Active Nodes: ", Style::default().fg(Theme::SECONDARY)),
                            Span::styled("12 / 12 clusters ready", Style::default().fg(Theme::FG)),
                        ]),
                    ],
                ),
                1 => (
                    "Logs",
                    vec![
                        Line::from(vec![
                            Span::styled("14:22:01 ", Style::default().fg(Theme::MUTED)),
                            Span::styled("[INFO]  ", Style::default().fg(Theme::ACCENT)),
                            Span::styled("HTTP server listening at 127.0.0.1:3000", Style::default().fg(Theme::FG)),
                        ]),
                        Line::from(vec![
                            Span::styled("14:22:05 ", Style::default().fg(Theme::MUTED)),
                            Span::styled("[OK]    ", Style::default().fg(Theme::SUCCESS)),
                            Span::styled("Database migration applied (version 0.4.2)", Style::default().fg(Theme::FG)),
                        ]),
                        Line::from(vec![
                            Span::styled("14:22:18 ", Style::default().fg(Theme::MUTED)),
                            Span::styled("[BUILD] ", Style::default().fg(Theme::PRIMARY)),
                            Span::styled("Compiled client assets in 480ms", Style::default().fg(Theme::FG)),
                        ]),
                    ],
                ),
                2 => (
                    "Settings",
                    vec![
                        Line::from(vec![
                            Span::styled("  Theme Mode:     ", Style::default().fg(Theme::SECONDARY)),
                            Span::styled("Minimal Dark (256-color / TrueColor)", Style::default().fg(Theme::FG)),
                        ]),
                        Line::from(vec![
                            Span::styled("  Refresh Rate:   ", Style::default().fg(Theme::SECONDARY)),
                            Span::styled("60 FPS (16ms frame interval)", Style::default().fg(Theme::FG)),
                        ]),
                        Line::from(vec![
                            Span::styled("  Log Verbosity:  ", Style::default().fg(Theme::SECONDARY)),
                            Span::styled("Verbose / Debug", Style::default().fg(Theme::ACCENT)),
                        ]),
                    ],
                ),
                _ => (
                    "Deployments",
                    vec![
                        Line::from(vec![
                            Span::styled("  Branch: ", Style::default().fg(Theme::SECONDARY)),
                            Span::styled("main (commit 6fe5ff7)", Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
                        ]),
                        Line::from(vec![
                            Span::styled("  Latest Check: ", Style::default().fg(Theme::SECONDARY)),
                            Span::styled("✔ 10/10 tests passed", Style::default().fg(Theme::SUCCESS)),
                        ]),
                    ],
                ),
            };

            let panel = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Theme::NEUTRAL_100))
                .title(Line::from(vec![
                    Span::styled("─ ", Style::default().fg(Theme::NEUTRAL_100)),
                    Span::styled(format!("Panel: {}", tab_title), Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
                    Span::styled(" ─", Style::default().fg(Theme::NEUTRAL_100)),
                ]))
                .style(Style::default().bg(Theme::BG));

            let inner_area = panel.inner(chunks[1]);
            f.render_widget(panel, chunks[1]);
            f.render_widget(Paragraph::new(tab_content).style(Style::default().bg(Theme::BG)), inner_area);

            // 3. Status Bar Widget (DESIGN.md mục 5)
            let mut status_bar = StatusBarWidget::new();
            status_bar.add_left(Span::styled("main", Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)));
            status_bar.add_left(Span::styled("3 files changed", Style::default().fg(Theme::SECONDARY)));
            status_bar.add_left(Span::styled("✔ All checks passed", Style::default().fg(Theme::SUCCESS)));

            status_bar.add_right(Span::styled("127.0.0.1:3000", Style::default().fg(Theme::ACCENT)));
            status_bar.add_right(Span::styled("Press 'q' to exit", Style::default().fg(Theme::MUTED)));

            status_bar.render(chunks[2], f);
        })?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        _ => {
                            tabs.handle_event(key);
                        }
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    println!("=== DEMO TABS & STATUS BAR HOÀN TẤT ===");
    Ok(())
}
