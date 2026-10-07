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
                    Constraint::Min(10),   // 1. Tabbed Panel Container (Nối liền toàn bộ Tab + Panel bo góc)
                    Constraint::Length(1), // 2. Status Bar
                ])
                .split(size);

            // 1. Nội dung theo Tab đang chọn
            let tab_content = match tabs.selected() {
                0 => vec![
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
                1 => {
                    let mut logs = Vec::new();
                    let raw_entries = [
                        ("14:22:01", "[INFO] ", Theme::ACCENT, "HTTP server listening at 127.0.0.1:3000"),
                        ("14:22:05", "[OK]   ", Theme::SUCCESS, "Database migration applied (version 0.4.2)"),
                        ("14:22:18", "[BUILD]", Theme::PRIMARY, "Compiled client assets in 480ms"),
                        ("14:22:25", "[DEBUG]", Theme::SECONDARY, "Long payload stream: {\"user_id\":1092837,\"token\":\"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...\"}"),
                        ("14:22:30", "[INFO] ", Theme::ACCENT, "Worker thread pool initialized with 8 threads"),
                        ("14:22:35", "[INFO] ", Theme::ACCENT, "Connected to Redis cluster at redis://10.0.0.15:6379"),
                        ("14:22:42", "[WARN] ", Theme::WARNING, "Disk space threshold warning: 78% utilized on /var/log"),
                        ("14:22:50", "[OK]   ", Theme::SUCCESS, "SSL Certificate verified for api.internal.domain"),
                        ("14:23:02", "[DEBUG]", Theme::SECONDARY, "Received webhook event: payment.captured (id: evt_998124)"),
                        ("14:23:15", "[INFO] ", Theme::ACCENT, "Ingesting batch metrics: 1,420 data points written to TimescaleDB"),
                        ("14:23:22", "[INFO] ", Theme::ACCENT, "gRPC endpoint registered on port 50051 (reflection active)"),
                        ("14:23:31", "[OK]   ", Theme::SUCCESS, "Health probe succeeded on 12/12 container replicas"),
                        ("14:23:45", "[BUILD]", Theme::PRIMARY, "WASM module optimization completed (size reduced by 34%)"),
                        ("14:24:00", "[INFO] ", Theme::ACCENT, "Cron job scheduler triggered 'cleanup_stale_sessions'"),
                        ("14:24:12", "[DEBUG]", Theme::SECONDARY, "Query execution: SELECT * FROM users WHERE active = true (duration: 1.4ms)"),
                        ("14:24:25", "[INFO] ", Theme::ACCENT, "OAuth2 provider handshake validated with Keycloak SSO"),
                        ("14:24:38", "[WARN] ", Theme::WARNING, "Rate limit reached for IP 198.51.100.44 (429 Too Many Requests)"),
                        ("14:24:50", "[OK]   ", Theme::SUCCESS, "Automated snapshot backup created: s3://backups/snapshot-20261008.tar.zst"),
                        ("14:25:05", "[INFO] ", Theme::ACCENT, "Zero-downtime rolling update initiated for worker pods"),
                        ("14:25:18", "[BUILD]", Theme::PRIMARY, "Rust native library compiled with LTO = fat, codegen-units = 1"),
                    ];

                    for (time, level, color, msg) in raw_entries {
                        logs.push(Line::from(vec![
                            Span::styled(format!("{} ", time), Style::default().fg(Theme::MUTED)),
                            Span::styled(format!("{} ", level), Style::default().fg(color)),
                            Span::styled(msg, Style::default().fg(Theme::FG)),
                        ]));
                    }
                    logs
                },
                2 => vec![
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
                _ => vec![
                    Line::from(vec![
                        Span::styled("  Branch: ", Style::default().fg(Theme::SECONDARY)),
                        Span::styled("main (commit 1c905d4)", Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
                    ]),
                    Line::from(vec![
                        Span::styled("  Latest Check: ", Style::default().fg(Theme::SECONDARY)),
                        Span::styled("✔ 10/10 tests passed", Style::default().fg(Theme::SUCCESS)),
                    ]),
                ],
            };

            // 2. Render Tab Container nối liền bo góc hoàn chỉnh
            tabs.render_container(chunks[0], tab_content, f);

            // 3. Status Bar Widget (DESIGN.md mục 5)
            let mut status_bar = StatusBarWidget::new();
            status_bar.add_left(Span::styled("Tab: [Left/Right/Tab]", Style::default().fg(Theme::PRIMARY)));
            status_bar.add_left(Span::styled("Scroll: [Up/Down/PgUp/PgDn]", Style::default().fg(Theme::SECONDARY)));
            status_bar.add_left(Span::styled(format!("Offset: {}", tabs.scroll_offset()), Style::default().fg(Theme::SUCCESS)));

            status_bar.add_right(Span::styled("127.0.0.1:3000", Style::default().fg(Theme::ACCENT)));
            status_bar.add_right(Span::styled("Press 'q' to exit", Style::default().fg(Theme::MUTED)));

            status_bar.render(chunks[1], f);
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
