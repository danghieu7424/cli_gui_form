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
    widgets::{Block, BorderType, Borders, Paragraph, TableState},
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

    // Các cờ tùy chỉnh giao diện (Toggleable styles):
    let mut is_rounded = false;       // Bật / tắt bo góc (BorderType::Rounded vs Plain)
    let mut show_horizontal_lines = false; // Bật / tắt đường kẻ ngang giữa các hàng
    let mut show_column_borders = false;   // Bật / tắt đường kẻ dọc phân cách cột

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

                // 2. Main Container Panel (Màu viền và màu tiêu đề đồng bộ bằng Theme::NEUTRAL_100 / Theme::SECONDARY)
                let panel_border_type = if is_rounded {
                    BorderType::Rounded
                } else {
                    BorderType::Plain
                };

                let panel_block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(panel_border_type)
                    .border_style(Style::default().fg(Theme::NEUTRAL_100))
                    .title(Line::from(vec![
                        Span::styled("─ ", Style::default().fg(Theme::NEUTRAL_100)),
                        Span::styled("Services & Pipelines", Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
                        Span::styled(" ─", Style::default().fg(Theme::NEUTRAL_100)),
                    ]))
                    .style(Style::default().bg(Theme::BG));

                let inner_area = panel_block.inner(chunks[1]);
                f.render_widget(panel_block, chunks[1]);

                // 3. XÂY DỰNG TABLE GRID CHUẨN XÁC TỪ MỤC 4 DESIGN.MD
                // Bảng độ rộng thực tế của 5 cột dữ liệu:
                let widths = [22, 16, 18, 14, 12];
                let total_width: usize = widths.iter().sum::<usize>() + if show_column_borders { 4 } else { 0 };

                // Các ký tự box drawing
                let (c_top_left, c_top_right, c_bot_left, c_bot_right) = if is_rounded {
                    ("╭", "╮", "╰", "╯")
                } else {
                    ("┌", "┐", "└", "┘")
                };
                let c_tee_down = "┬";
                let c_tee_up = "┴";
                let c_cross = "┼";
                let c_tee_right = "├";
                let c_tee_left = "┤";
                let c_horiz = "─";
                let c_vert = "│";

                // Hàm tạo đường kẻ ngang phân cách nối chính xác với viền cột
                let make_divider = |l: &str, m: &str, r: &str| -> String {
                    let parts: Vec<String> = widths.iter().map(|&w| c_horiz.repeat(w)).collect();
                    if show_column_borders {
                        format!("{}{}{}", l, parts.join(m), r)
                    } else {
                        format!("{}{}{}", l, c_horiz.repeat(total_width), r)
                    }
                };

                let mut lines: Vec<Line> = Vec::new();

                // 1. Viền đỉnh của bảng (Top border) nếu có kẻ
                if show_horizontal_lines {
                    let top_line = make_divider(c_top_left, c_tee_down, c_top_right);
                    lines.push(Line::from(Span::styled(top_line, Style::default().fg(Theme::NEUTRAL_100))));
                }

                // 2. Hàng Header
                let h_cells = [" Name", "Status", "Branch", "Commit", "Time"];
                let mut header_spans: Vec<Span> = Vec::new();
                if show_horizontal_lines {
                    header_spans.push(Span::styled(c_vert, Style::default().fg(Theme::NEUTRAL_100)));
                } else {
                    header_spans.push(Span::styled(" ", Style::default()));
                }

                for (i, &title) in h_cells.iter().enumerate() {
                    let w = widths[i];
                    header_spans.push(Span::styled(
                        format!("{:<width$}", title, width = w),
                        Style::default().fg(Theme::SECONDARY).add_modifier(Modifier::BOLD),
                    ));
                    if show_column_borders && i < h_cells.len() - 1 {
                        header_spans.push(Span::styled(c_vert, Style::default().fg(Theme::NEUTRAL_100)));
                    }
                }
                if show_horizontal_lines {
                    header_spans.push(Span::styled(c_vert, Style::default().fg(Theme::NEUTRAL_100)));
                }
                lines.push(Line::from(header_spans));

                // 3. Đường phân cách Header (Header Separator: ├─┬─┤ hoặc ├─┼─┤)
                let header_sep = make_divider(
                    if show_horizontal_lines { c_tee_right } else { "" },
                    if show_column_borders { c_cross } else { c_horiz },
                    if show_horizontal_lines { c_tee_left } else { "" },
                );
                lines.push(Line::from(Span::styled(header_sep, Style::default().fg(Theme::NEUTRAL_100))));

                // 4. Các hàng dữ liệu (Data Rows)
                for (idx, item) in items.iter().enumerate() {
                    let is_selected = table_state.selected() == Some(idx);
                    let mut row_spans: Vec<Span> = Vec::new();

                    let row_border_style = Style::default().fg(Theme::NEUTRAL_100);

                    if show_horizontal_lines {
                        row_spans.push(Span::styled(c_vert, row_border_style));
                    } else {
                        row_spans.push(Span::styled(" ", Style::default()));
                    }

                    // Cột 1: Name (có ▸ khi selected)
                    let prefix = if is_selected { "▸ " } else { "  " };
                    let name_str = format!("{}{}", prefix, item.name);
                    let name_style = if is_selected {
                        Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Theme::FG)
                    };
                    row_spans.push(Span::styled(format!("{:<width$}", name_str, width = widths[0]), name_style));

                    if show_column_borders {
                        row_spans.push(Span::styled(c_vert, row_border_style));
                    }

                    // Cột 2: Status
                    let status_str = format!("{} {}", item.status_icon, item.status_text);
                    row_spans.push(Span::styled(
                        format!("{:<width$}", status_str, width = widths[1]),
                        Style::default().fg(item.status_color),
                    ));

                    if show_column_borders {
                        row_spans.push(Span::styled(c_vert, row_border_style));
                    }

                    // Cột 3: Branch
                    row_spans.push(Span::styled(
                        format!("{:<width$}", item.branch, width = widths[2]),
                        Style::default().fg(Theme::SECONDARY),
                    ));

                    if show_column_borders {
                        row_spans.push(Span::styled(c_vert, row_border_style));
                    }

                    // Cột 4: Commit
                    row_spans.push(Span::styled(
                        format!("{:<width$}", item.commit, width = widths[3]),
                        Style::default().fg(Theme::MUTED),
                    ));

                    if show_column_borders {
                        row_spans.push(Span::styled(c_vert, row_border_style));
                    }

                    // Cột 5: Time (Right-align theo DESIGN.md)
                    row_spans.push(Span::styled(
                        format!("{:>width$}", item.time, width = widths[4]),
                        Style::default().fg(Theme::SECONDARY),
                    ));

                    if show_horizontal_lines {
                        row_spans.push(Span::styled(c_vert, row_border_style));
                    }

                    let row_line = if is_selected {
                        Line::from(row_spans).style(Style::default().bg(Theme::SURFACE))
                    } else {
                        Line::from(row_spans)
                    };
                    lines.push(row_line);

                    // Đường kẻ ngang giữa các hàng dữ liệu (H-Line: ├─┼─┤)
                    if show_horizontal_lines && idx + 1 < items.len() {
                        let mid_sep = make_divider(c_tee_right, c_cross, c_tee_left);
                        lines.push(Line::from(Span::styled(mid_sep, Style::default().fg(Theme::NEUTRAL_100))));
                    }
                }

                // 5. Viền đáy của bảng (Bottom border: └─┴─┘)
                if show_horizontal_lines {
                    let bot_line = make_divider(c_bot_left, c_tee_up, c_bot_right);
                    lines.push(Line::from(Span::styled(bot_line, Style::default().fg(Theme::NEUTRAL_100))));
                }

                let paragraph = Paragraph::new(lines).style(Style::default().bg(Theme::BG));
                f.render_widget(paragraph, inner_area);

                // 5. Status Bar hiển thị phím tắt toggles
                let status_line = Line::from(vec![
                    Span::styled(" [B] ", Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
                    Span::styled(format!("Border:{} ", if is_rounded { "Rounded" } else { "Plain" }), Style::default().fg(Theme::SECONDARY)),
                    Span::styled("─", Style::default().fg(Theme::MUTED)),
                    Span::styled(" [H] ", Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
                    Span::styled(format!("H-Line:{} ", if show_horizontal_lines { "ON" } else { "OFF" }), Style::default().fg(Theme::SECONDARY)),
                    Span::styled("─", Style::default().fg(Theme::MUTED)),
                    Span::styled(" [V] ", Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
                    Span::styled(format!("V-Line:{} ", if show_column_borders { "ON" } else { "OFF" }), Style::default().fg(Theme::SECONDARY)),
                    Span::styled("─", Style::default().fg(Theme::MUTED)),
                    Span::styled(" ↑/↓: Navigate ", Style::default().fg(Theme::SECONDARY)),
                    Span::styled("─", Style::default().fg(Theme::MUTED)),
                    Span::styled(" Esc: Exit ", Style::default().fg(Theme::SECONDARY)),
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
                        KeyCode::Char('b') | KeyCode::Char('B') => {
                            // Toggle bo góc khung ngoài: Rounded <-> Plain
                            is_rounded = !is_rounded;
                            needs_render = true;
                        }
                        KeyCode::Char('h') | KeyCode::Char('H') => {
                            // Toggle đường kẻ ngang giữa các dòng
                            show_horizontal_lines = !show_horizontal_lines;
                            needs_render = true;
                        }
                        KeyCode::Char('v') | KeyCode::Char('V') => {
                            // Toggle đường kẻ dọc
                            show_column_borders = !show_column_borders;
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
