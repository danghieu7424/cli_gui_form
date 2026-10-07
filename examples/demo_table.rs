// --- PHÂN ĐOẠN: DEMO BẢNG (TABLE) CHUẨN MINIMAL TUI DESIGN SYSTEM ---

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
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table, TableState},
    Terminal,
};
use std::{
    io::{self, stdout},
    time::Duration,
};

/****
 * Struct: DeploymentItem
 * Chức năng: Lưu trữ dữ liệu dòng cho bảng danh sách tiến trình triển khai.
 ****/
struct DeploymentItem {
    name: &'static str,
    status_icon: &'static str,
    status_text: &'static str,
    status_color: ratatui::style::Color,
    branch: &'static str,
    commit: &'static str,
    time: &'static str,
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Dữ liệu mẫu theo chuẩn DESIGN.md mục 5. Tables
    let items = vec![
        DeploymentItem {
            name: "deploy-api",
            status_icon: Icons::SUCCESS,
            status_text: "Ready",
            status_color: Theme::SUCCESS,
            branch: "main",
            commit: "7670885",
            time: "2m ago",
        },
        DeploymentItem {
            name: "deploy-web",
            status_icon: Icons::RUN,
            status_text: "Build",
            status_color: Theme::ACCENT,
            branch: "feat/tui",
            commit: "ffe5daf",
            time: "just now",
        },
        DeploymentItem {
            name: "deploy-docs",
            status_icon: Icons::ERROR,
            status_text: "Error",
            status_color: Theme::ERROR,
            branch: "docs/update",
            commit: "5fed956",
            time: "5m ago",
        },
        DeploymentItem {
            name: "deploy-worker",
            status_icon: "●",
            status_text: "Thinking",
            status_color: Theme::MUTED,
            branch: "ai/agent",
            commit: "a12b3c4",
            time: "12m ago",
        },
        DeploymentItem {
            name: "deploy-auth",
            status_icon: Icons::WARNING,
            status_text: "Degraded",
            status_color: Theme::WARNING,
            branch: "hotfix/auth",
            commit: "c98d76e",
            time: "1h ago",
        },
    ];

    let mut table_state = TableState::default();
    table_state.select(Some(0));

    let mut needs_render = true;

    loop {
        if needs_render {
            terminal.draw(|f| {
                let full_area = f.area();

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3), // Top Header H1
                        Constraint::Min(10),   // Main Table Panel
                        Constraint::Length(1), // Bottom Status Bar
                    ])
                    .split(full_area);

                // 1. Top Header H1 (Minimal, figlet-style / BOLD Primary)
                let header_widget = Paragraph::new(Line::from(vec![
                    Span::styled("  LINEAR / VERCEL TUI", Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
                    Span::styled(" ─ Deployments Dashboard", Style::default().fg(Theme::SECONDARY)),
                ]))
                .style(Style::default().bg(Theme::BG));
                f.render_widget(header_widget, chunks[0]);

                // 2. Main Container Panel (Panel / Card với Single-line Box Drawing)
                let panel_block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .border_style(Style::default().fg(Theme::NEUTRAL_100))
                    .title(Span::styled("─ Services & Pipelines ─", Style::default().fg(Theme::SECONDARY)))
                    .style(Style::default().bg(Theme::BG));

                let inner_area = panel_block.inner(chunks[1]);
                f.render_widget(panel_block, chunks[1]);

                // 3. Table Header chuẩn DESIGN.md: BOLD + Secondary, no outer border, separator bằng ─
                let header_cells = ["  Name", "Status", "Branch", "Commit", "Time"]
                    .iter()
                    .map(|h| {
                        Cell::from(*h).style(
                            Style::default()
                                .fg(Theme::SECONDARY)
                                .add_modifier(Modifier::BOLD),
                        )
                    });
                let table_header = Row::new(header_cells)
                    .height(1)
                    .bottom_margin(1); // Tạo khoảng cách divider dòng phân cách

                // 4. Các hàng dữ liệu (Rows)
                let rows = items.iter().enumerate().map(|(idx, item)| {
                    let is_selected = table_state.selected() == Some(idx);

                    // Khi chọn (Selected): ▸ prefix + BOLD Primary, khi thường: 2-space indent + Foreground
                    let prefix = if is_selected { "▸ " } else { "  " };
                    let name_span = Span::styled(
                        format!("{}{}", prefix, item.name),
                        if is_selected {
                            Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Theme::FG)
                        },
                    );

                    let status_span = Line::from(vec![
                        Span::styled(format!("{} ", item.status_icon), Style::default().fg(item.status_color).add_modifier(Modifier::BOLD)),
                        Span::styled(item.status_text, Style::default().fg(item.status_color)),
                    ]);

                    let branch_span = Span::styled(item.branch, Style::default().fg(Theme::SECONDARY));
                    let commit_span = Span::styled(item.commit, Style::default().fg(Theme::MUTED));
                    let time_span = Span::styled(item.time, Style::default().fg(Theme::SECONDARY));

                    Row::new(vec![
                        Cell::from(name_span),
                        Cell::from(status_span),
                        Cell::from(branch_span),
                        Cell::from(commit_span),
                        Cell::from(time_span),
                    ])
                    .height(1)
                });

                // Cột căn lề: Left-align các cột nội dung, Right-align cột Time (DESIGN.md mục 6)
                let widths = [
                    Constraint::Length(22), // Name
                    Constraint::Length(16), // Status
                    Constraint::Length(18), // Branch
                    Constraint::Length(14), // Commit
                    Constraint::Min(10),    // Time
                ];

                let table = Table::new(rows, widths)
                    .header(table_header)
                    .highlight_style(Style::default().bg(Theme::SURFACE))
                    .style(Style::default().bg(Theme::BG));

                f.render_stateful_widget(table, inner_area, &mut table_state);

                // 5. Status Bar ở đáy màn hình (DESIGN.md mục 5)
                let status_line = Line::from(vec![
                    Span::styled(" main ", Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
                    Span::styled("─", Style::default().fg(Theme::MUTED)),
                    Span::styled(" 5 deployments loaded ", Style::default().fg(Theme::SECONDARY)),
                    Span::styled("─", Style::default().fg(Theme::MUTED)),
                    Span::styled(" ↑/↓: Navigate ", Style::default().fg(Theme::SECONDARY)),
                    Span::styled("─", Style::default().fg(Theme::MUTED)),
                    Span::styled(" Enter: Details ", Style::default().fg(Theme::SECONDARY)),
                    Span::styled("─", Style::default().fg(Theme::MUTED)),
                    Span::styled(" Esc: Exit ", Style::default().fg(Theme::SECONDARY)),
                    Span::styled("                                  ✓ Ready ", Style::default().fg(Theme::SUCCESS)),
                ]);
                let status_bar = Paragraph::new(status_line).style(Style::default().bg(Theme::BG));
                f.render_widget(status_bar, chunks[2]);
            })?;
            needs_render = false;
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let is_ctrl_c = key.modifiers.contains(KeyModifiers::CONTROL)
                        && key.code == KeyCode::Char('c');
                    if key.code == KeyCode::Esc || is_ctrl_c {
                        break;
                    }

                    match key.code {
                        KeyCode::Down | KeyCode::Char('j') => {
                            let curr = table_state.selected().unwrap_or(0);
                            let next = if curr + 1 < items.len() { curr + 1 } else { 0 };
                            table_state.select(Some(next));
                            needs_render = true;
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            let curr = table_state.selected().unwrap_or(0);
                            let prev = if curr > 0 { curr - 1 } else { items.len() - 1 };
                            table_state.select(Some(prev));
                            needs_render = true;
                        }
                        KeyCode::Enter => {
                            // Khi nhấn Enter có thể xem chi tiết hoặc trigger action
                            needs_render = true;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    println!("\n=== TABLE SELECTION ===");
    if let Some(sel) = table_state.selected() {
        println!("Selected: {} ({})", items[sel].name, items[sel].status_text);
    }

    Ok(())
}
