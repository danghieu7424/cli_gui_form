// --- PHÂN ĐOẠN: MASTER SHOWCASE TOÀN DIỆN DESIGN.MD VÀ MỌI WIDGET ---

use cli_gui_form::{
    ButtonWidget, CardWidget, CheckboxWidget, FormManager, FormWidget, Icons,
    InputMode, InputWidget, ListWidget, RadioWidget, SelectWidget, ShimmerWidget, SpinnerType, StatusBarWidget, TabsWidget,
    TaskWidget, Theme,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Terminal,
};
use std::{
    io::{self, stdout},
    time::{Duration, Instant},
};

struct TableRowItem {
    name: &'static str,
    status_icon: &'static str,
    status_text: &'static str,
    status_color: Color,
    branch: &'static str,
    commit: &'static str,
    latency: &'static str,
    time: &'static str,
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 1. TABS WIDGET VỚI 5 PHÂN HỆ DESIGN.MD
    let mut tabs = TabsWidget::new(
        "master_tabs",
        vec![
            "1. Forms & Inputs",
            "2. Table & Badges",
            "3. Tasks & Spinners",
            "4. Live Logs",
            "5. Icons & Theme",
        ],
    );
    tabs.focus();
    tabs.set_auto_scroll(false);

    // 2. KHỞI TẠO FORM SUITE (TAB 1)
    let mut form = FormManager::new();
    form.add_widget(Box::new(
        InputWidget::new("service_name", "Service Name", InputMode::Text)
            .with_placeholder("e.g. acme-auth-gateway"),
    ));
    form.add_widget(Box::new(
        InputWidget::new("api_secret", "API Secret Key", InputMode::Password)
            .with_placeholder("Enter secret key or paste token..."),
    ));
    form.add_widget(Box::new(CheckboxWidget::new(
        "enable_tls",
        "Enable Automatic TLS / SSL Certificate",
        true,
    )));
    form.add_widget(Box::new(CheckboxWidget::new(
        "telemetry",
        "Send Anonymous Telemetry & Crash Reports",
        false,
    )));
    form.add_widget(Box::new(RadioWidget::new(
        "environment",
        "Deployment Target",
        vec![
            "Production".into(),
            "Staging".into(),
            "Preview".into(),
            "Development".into(),
        ],
    )));
    form.add_widget(Box::new(
        SelectWidget::new(
            "region",
            "Hosting Region (HTML Select/Option Dropdown)",
            vec![
                ("us-east-1", "US East (N. Virginia)"),
                ("eu-central-1", "Europe (Frankfurt)"),
                ("ap-southeast-1", "Asia Pacific (Singapore)"),
                ("sa-east-1", "South America (São Paulo)"),
            ],
        )
        .with_placeholder("Select a deployment datacenter... ▾")
        .with_selected(0),
    ));
    form.add_widget(Box::new(
        ListWidget::new("route_menu")
            .with_label("Select Target Route (DESIGN.md Lists/Menus)")
            .with_item("api/routes.ts")
            .with_item("api/handler.ts")
            .with_disabled_item("lib/internal.ts (deprecated)")
            .with_item("config.json"),
    ));
    form.add_widget(Box::new(ButtonWidget::new(
        "btn_deploy",
        "Deploy Now",
        Theme::BG,
        Theme::FG,
    )));
    form.add_widget(Box::new(ButtonWidget::new(
        "btn_cancel",
        "Cancel",
        Theme::BG,
        Theme::MUTED,
    )));

    // 3. DỮ LIỆU TABLE (TAB 2)
    let table_items = vec![
        TableRowItem {
            name: "deploy-api",
            status_icon: Icons::SUCCESS,
            status_text: "Ready",
            status_color: Theme::SUCCESS,
            branch: "main",
            commit: "7670885",
            latency: "18ms",
            time: "2m ago",
        },
        TableRowItem {
            name: "deploy-web",
            status_icon: Icons::RUN,
            status_text: "Build",
            status_color: Theme::ORANGE,
            branch: "feat/tui",
            commit: "ffe5daf",
            latency: "45ms",
            time: "just now",
        },
        TableRowItem {
            name: "worker-ai",
            status_icon: Icons::WARNING,
            status_text: "High Load",
            status_color: Theme::WARNING,
            branch: "main",
            commit: "1a89c20",
            latency: "120ms",
            time: "8m ago",
        },
        TableRowItem {
            name: "deploy-docs",
            status_icon: Icons::ERROR,
            status_text: "Failed",
            status_color: Theme::CRITICAL,
            branch: "docs/v2",
            commit: "c381d09",
            latency: "---",
            time: "15m ago",
        },
        TableRowItem {
            name: "redis-cache",
            status_icon: Icons::PAUSE,
            status_text: "Paused",
            status_color: Theme::MUTED,
            branch: "infra",
            commit: "e29b11a",
            latency: "2ms",
            time: "1h ago",
        },
        TableRowItem {
            name: "cleanup-cron",
            status_icon: Icons::STOP,
            status_text: "Stopped",
            status_color: Theme::ERROR,
            branch: "infra",
            commit: "40fa882",
            latency: "---",
            time: "3h ago",
        },
    ];

    // Card 1: Plain border Deploy Status theo đúng chuẩn DESIGN.md mục 5
    let deploy_card = CardWidget::new("card_deploy", "Deploy Status")
        .with_item("Production", Some(Icons::SUCCESS), "Ready", Theme::SUCCESS)
        .with_item("Preview", Some(Icons::RUN), "Building", Theme::CYAN)
        .with_item("Staging", Some(Icons::SUCCESS), "Ready", Theme::SUCCESS);

    // Card 2: Rounded border Infrastructure Metrics (Màu sắc đa dạng sắc nét)
    let infra_card = CardWidget::new("card_infra", "Infrastructure")
        .with_rounded(true)
        .with_title_color(Theme::PURPLE)
        .with_item("PostgreSQL", Some(Icons::SUCCESS), "Healthy", Theme::EMERALD)
        .with_item("Redis Cluster", Some(Icons::WARNING), "Degraded", Theme::ORANGE)
        .with_item("Edge Router", Some(Icons::RUN), "Active", Theme::SKY);

    // 4. TASKS & ANIMATIONS (TAB 3)
    let mut pulse_task = TaskWidget::new_loading(
        "task_pulse",
        "Neural Inference Engine",
        "Synthesizing attention layers...",
        Theme::ACCENT,
    )
    .with_spinner_type(SpinnerType::Pulse);

    let mut braille_task = TaskWidget::new_loading(
        "task_braille",
        "Docker Layer Builder",
        "Exporting image snapshot sha256:7f8a9...",
        Theme::PRIMARY,
    )
    .with_spinner_type(SpinnerType::Dots);

    let progress_task = TaskWidget::new_progress(
        "task_progress",
        "WASM Optimization",
        340,
        500,
        "chunks",
        "12s",
        "Applying link-time dead code elimination...",
        Theme::SUCCESS,
    );

    let mut shimmer_bar = ShimmerWidget::new(
        "shimmer_bar",
        "Linear Sync Engine ─ Background Delta Syncing...",
    )
    .with_colors((69, 137, 255), (220, 240, 255));

    // 5. LIVE LOGS (TAB 4)
    let mut live_logs: Vec<Line<'static>> = vec![
        Line::from(vec![
            Span::styled("14:22:01 ", Style::default().fg(Theme::MUTED)),
            Span::styled("[INFO] ", Style::default().fg(Theme::ACCENT)),
            Span::styled("HTTP server listening at 127.0.0.1:3000", Style::default().fg(Theme::FG)),
        ]),
        Line::from(vec![
            Span::styled("14:22:05 ", Style::default().fg(Theme::MUTED)),
            Span::styled("[OK]   ", Style::default().fg(Theme::SUCCESS)),
            Span::styled("Database schema migrations verified", Style::default().fg(Theme::FG)),
        ]),
        Line::from(vec![
            Span::styled("14:22:18 ", Style::default().fg(Theme::MUTED)),
            Span::styled("[BUILD]", Style::default().fg(Theme::PRIMARY)),
            Span::styled("Native Rust backend compiled in 480ms", Style::default().fg(Theme::FG)),
        ]),
    ];

    let mut last_tick_60fps = Instant::now();
    let mut last_log_stream = Instant::now();
    let mut log_id = 1;

    loop {
        // Cập nhật nhịp animation 60 FPS
        if last_tick_60fps.elapsed() >= Duration::from_millis(16) {
            pulse_task.tick();
            braille_task.tick();
            shimmer_bar.tick();
            last_tick_60fps = Instant::now();
        }

        // Mô phỏng sinh log liên tục mỗi 1.6 giây
        if last_log_stream.elapsed() >= Duration::from_millis(1600) {
            let sec = 25 + log_id * 2;
            let time_str = format!("14:23:{:02}", sec % 60);
            let (level, color, msg) = match log_id % 5 {
                0 => ("[OK]   ", Theme::SUCCESS, format!("Event stream batch #{} acknowledged", log_id)),
                1 => ("[INFO] ", Theme::ACCENT, format!("Client session #{} authenticated via JWT", log_id)),
                2 => ("[DEBUG]", Theme::SECONDARY, format!("Redis cache hit for key 'cluster_metrics_{}'", log_id)),
                3 => ("[WARN] ", Theme::WARNING, format!("Garbage collection took 14.2ms on worker #{}", (log_id % 3) + 1)),
                _ => ("[BUILD]", Theme::PRIMARY, format!("WASM bundle hot-reloaded for client #{}", log_id)),
            };

            live_logs.push(Line::from(vec![
                Span::styled(format!("{} ", time_str), Style::default().fg(Theme::MUTED)),
                Span::styled(format!("{} ", level), Style::default().fg(color)),
                Span::styled(msg, Style::default().fg(Theme::FG)),
            ]));

            log_id += 1;
            last_log_stream = Instant::now();
        }

        terminal.draw(|f| {
            let full_area = f.area();

            // Layout 3 phân vùng: Header Title (3) -> Main Tabs & Container (Min 12) -> Status Bar (1)
            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Top Header H1
                    Constraint::Min(12),   // Master Tabbed Container
                    Constraint::Length(1), // Bottom Status Bar
                ])
                .split(full_area);

            // 1. TOP HEADER H1
            let header = Paragraph::new(Line::from(vec![
                Span::styled("  LINEAR / VERCEL DESIGN SYSTEM", Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
                Span::styled(" ─ Master Industrial Component Showcase", Style::default().fg(Theme::SECONDARY)),
            ]))
            .style(Style::default().bg(Theme::BG));
            f.render_widget(header, main_chunks[0]);

            // 2. NỘI DUNG TỪNG PHÂN HỆ TAB
            match tabs.selected() {
                0 => {
                    // TAB 1: FORMS & INPUTS
                    let mut form_lines = Vec::new();
                    form_lines.push(Line::from(vec![
                        Span::styled("  Form Inputs & Keyboard Navigation", Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
                        Span::styled(" (Tab/↑/↓: Form Focus ─ Lists: → to enter, ← to exit ─ Radios: ←/→)", Style::default().fg(Theme::MUTED)),
                    ]));
                    form_lines.push(Line::from(""));
                    tabs.render_container(main_chunks[1], form_lines, f);

                    // Render FormManager bên trong lòng Panel
                    let inner_form_area = Rect {
                        x: main_chunks[1].x + 2,
                        y: main_chunks[1].y + 4,
                        width: main_chunks[1].width.saturating_sub(4),
                        height: main_chunks[1].height.saturating_sub(5),
                    };
                    form.render(inner_form_area, f);
                }
                1 => {
                    // TAB 2: DATA TABLE & BADGES (Read-only Data Table theo chuẩn DESIGN.md mục 5)
                    let mut lines = Vec::new();
                    lines.push(Line::from(vec![
                        Span::styled("  Deployment Workloads & Status Badges", Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
                        Span::styled(" (Read-only Table ─ DESIGN.md)", Style::default().fg(Theme::MUTED)),
                    ]));
                    lines.push(Line::from(""));

                    // Header bảng theo chuẩn DESIGN.md: No outer border, header separated by ─, dim separator
                    lines.push(Line::from(vec![
                        Span::styled("  NAME              STATUS         BRANCH     COMMIT    LATENCY    TIME", Style::default().fg(Theme::SECONDARY).add_modifier(Modifier::BOLD)),
                    ]));
                    lines.push(Line::from(vec![
                        Span::styled("  ─────────────────────────────────────────────────────────────────────────────", Style::default().fg(Theme::NEUTRAL_100)),
                    ]));

                    for row in &table_items {
                        lines.push(Line::from(vec![
                            Span::raw("  "),
                            Span::styled(format!("{:<16} ", row.name), Style::default().fg(Theme::FG)),
                            Span::styled(format!("{} ", row.status_icon), Style::default().fg(row.status_color)),
                            Span::styled(format!("{:<12} ", row.status_text), Style::default().fg(row.status_color)),
                            Span::styled(format!("{:<10} ", row.branch), Style::default().fg(Theme::SECONDARY)),
                            Span::styled(format!("{:<9} ", row.commit), Style::default().fg(Theme::MUTED)),
                            Span::styled(format!("{:<10} ", row.latency), Style::default().fg(Theme::ACCENT)),
                            Span::styled(row.time, Style::default().fg(Theme::MUTED)),
                        ]));
                    }

                    tabs.render_container(main_chunks[1], lines, f);

                    // Render 2 Panels / Cards (DESIGN.md mục 5) bên dưới bảng KHI VÀ CHỈ KHI đủ chiều cao (>= 24 dòng)
                    // để đảm bảo khoảng cách an toàn, không bao giờ đè lên dòng cuối của bảng
                    if main_chunks[1].height >= 24 {
                        let card_w = (main_chunks[1].width.saturating_sub(8) / 2).min(38);
                        if card_w > 20 {
                            let card_deploy_area = Rect {
                                x: main_chunks[1].x + 3,
                                y: main_chunks[1].y + 15,
                                width: card_w,
                                height: 7,
                            };
                            deploy_card.render(card_deploy_area, f);

                            let card_infra_area = Rect {
                                x: main_chunks[1].x + 3 + card_w + 2,
                                y: main_chunks[1].y + 15,
                                width: card_w,
                                height: 7,
                            };
                            infra_card.render(card_infra_area, f);
                        }
                    }
                }
                2 => {
                    // TAB 3: TASKS, PROGRESS & SHIMMER
                    let mut lines = Vec::new();
                    lines.push(Line::from(vec![
                        Span::styled("  Background Task Lifecycle & Animation Engine", Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
                    ]));
                    lines.push(Line::from(""));
                    tabs.render_container(main_chunks[1], lines, f);

                    // Render các Tasks lồng nhau trong panel
                    let card_area = Rect {
                        x: main_chunks[1].x + 3,
                        y: main_chunks[1].y + 4,
                        width: main_chunks[1].width.saturating_sub(6),
                        height: 3,
                    };
                    pulse_task.render(card_area, f);

                    let card_area2 = Rect {
                        x: main_chunks[1].x + 3,
                        y: main_chunks[1].y + 8,
                        width: main_chunks[1].width.saturating_sub(6),
                        height: 3,
                    };
                    braille_task.render(card_area2, f);

                    let card_area3 = Rect {
                        x: main_chunks[1].x + 3,
                        y: main_chunks[1].y + 12,
                        width: main_chunks[1].width.saturating_sub(6),
                        height: 3,
                    };
                    progress_task.render(card_area3, f);

                    let shimmer_area = Rect {
                        x: main_chunks[1].x + 3,
                        y: main_chunks[1].y + 16,
                        width: main_chunks[1].width.saturating_sub(6),
                        height: 1,
                    };
                    shimmer_bar.render(shimmer_area, f);
                }
                3 => {
                    // TAB 4: LIVE LOGS (SMART STICKY FOLLOW & SCROLLBAR)
                    tabs.render_container(main_chunks[1], live_logs.clone(), f);
                }
                _ => {
                    // TAB 5: ICONS PALETTE & THEME MATRIX
                    let mut lines = Vec::new();
                    lines.push(Line::from(vec![
                        Span::styled("  Curated Zero-Noise Unicode 2-Cell Icons (DESIGN.md Mục 7)", Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
                    ]));
                    lines.push(Line::from(vec![
                        Span::styled("  ─────────────────────────────────────────────────────────────────────────────", Style::default().fg(Theme::NEUTRAL_100)),
                    ]));
                    lines.push(Line::from(""));

                    let icon_matrix = [
                        (Icons::SUCCESS, "Icons::SUCCESS", "✔ (U+2714)", "Theme::SUCCESS (#25A249)", Theme::SUCCESS),
                        (Icons::ERROR, "Icons::ERROR", "✗ (U+2716)", "Theme::ERROR   (#DA1E28)", Theme::ERROR),
                        (Icons::WARNING, "Icons::WARNING", "⚠ (U+26A0)", "Theme::WARNING (#F1C21B)", Theme::WARNING),
                        (Icons::RUN, "Icons::RUN", "▶ (U+25B6)", "Theme::ACCENT  (#0070F3)", Theme::ACCENT),
                        (Icons::BUILD, "Icons::BUILD", "⚙ (U+2699)", "Theme::PRIMARY (#4589FF)", Theme::PRIMARY),
                        (Icons::PAUSE, "Icons::PAUSE", "⏸ (U+23F8)", "Theme::MUTED   (#8D8D8D)", Theme::MUTED),
                        (Icons::STOP, "Icons::STOP", "■ (U+25A0)", "Theme::ERROR   (#DA1E28)", Theme::ERROR),
                        (Icons::CHECKBOX_ON, "Icons::CHECKBOX_ON", "☑ (U+2611)", "Theme::SUCCESS (#25A249)", Theme::SUCCESS),
                        (Icons::CHECKBOX_OFF, "Icons::CHECKBOX_OFF", "☐ (U+2610)", "Theme::MUTED   (#8D8D8D)", Theme::MUTED),
                        (Icons::RADIO_ON, "Icons::RADIO_ON", "● (U+25CF)", "Theme::ACCENT  (#0070F3)", Theme::ACCENT),
                        (Icons::RADIO_OFF, "Icons::RADIO_OFF", "○ (U+25CB)", "Theme::MUTED   (#8D8D8D)", Theme::MUTED),
                        (Icons::SPARKLE_FILLED, "Icons::SPARKLE_FILLED", "✦ (U+2726)", "Theme::WARNING (#F1C21B)", Theme::WARNING),
                        (Icons::STAR_OUTLINE, "Icons::STAR_OUTLINE", "⚝ (U+269D)", "Theme::PRIMARY (#4589FF)", Theme::PRIMARY),
                        (Icons::DIAMOND_EMPTY, "Icons::DIAMOND_EMPTY", "◇ (U+25C7)", "Theme::SECONDARY (#A8A8A8)", Theme::SECONDARY),
                        (Icons::SNOWFLAKE, "Icons::SNOWFLAKE", "❅ (U+2745)", "Theme::ACCENT  (#0070F3)", Theme::ACCENT),
                        (Icons::POINTER, "Icons::POINTER", "▹ (U+25B8)", "Theme::PRIMARY (#4589FF)", Theme::PRIMARY),
                        (Icons::ARROW_RIGHT, "Icons::ARROW_RIGHT", "→ (U+2192)", "Theme::MUTED   (#8D8D8D)", Theme::MUTED),
                        (Icons::BRANCH, "Icons::BRANCH", "⤷ (U+21B3)", "Theme::SECONDARY (#A8A8A8)", Theme::SECONDARY),
                    ];

                    for (glyph, const_name, unicode_char, color_name, color) in icon_matrix {
                        lines.push(Line::from(vec![
                            Span::styled(format!("    {}", glyph), Style::default().fg(color)),
                            Span::styled(format!("{:<22} ", const_name), Style::default().fg(Theme::FG)),
                            Span::styled(format!("{:<15} ", unicode_char), Style::default().fg(Theme::SECONDARY)),
                            Span::styled(color_name, Style::default().fg(color)),
                        ]));
                    }

                    // Bảng màu mở rộng Extended Palette Swatches
                    lines.push(Line::from(""));
                    lines.push(Line::from(vec![
                        Span::styled("  Extended Palette Swatches (Multi-Role System)", Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)),
                    ]));
                    lines.push(Line::from(vec![
                        Span::styled("  ─────────────────────────────────────────────────────────────────────────────", Style::default().fg(Theme::NEUTRAL_100)),
                    ]));
                    lines.push(Line::from(""));

                    let swatches = [
                        ("Theme::CYAN", "■ #50E3C2", Theme::CYAN, "Network, API Latency, Endpoints"),
                        ("Theme::PURPLE", "■ #7928CA", Theme::PURPLE, "AI Engine, Neural, GraphQL"),
                        ("Theme::MAGENTA", "■ #F81CE5", Theme::MAGENTA, "Auth Tokens, Webhooks, Secrets"),
                        ("Theme::ORANGE", "■ #FF8800", Theme::ORANGE, "Queues, Build Pipelines, Workers"),
                        ("Theme::INDIGO", "■ #5E6AD2", Theme::INDIGO, "Branches, PRs, Linear Tasks"),
                        ("Theme::EMERALD", "■ #10B981", Theme::EMERALD, "Healthy Uptime, In-Memory DBs"),
                        ("Theme::SKY", "■ #38BDF8", Theme::SKY, "Cloud Infra, Docker, Kubernetes"),
                        ("Theme::CRITICAL", "■ #FF0055", Theme::CRITICAL, "Fatal Panics, Immediate Alerts"),
                    ];

                    for (name, block, color, desc) in swatches {
                        lines.push(Line::from(vec![
                            Span::raw("    "),
                            Span::styled(format!("{:<12} ", block), Style::default().fg(color)),
                            Span::styled(format!("{:<18} ", name), Style::default().fg(Theme::FG)),
                            Span::styled(desc, Style::default().fg(Theme::SECONDARY)),
                        ]));
                    }

                    tabs.render_container(main_chunks[1], lines, f);
                }
            }

            // 3. STATUS BAR ĐÁY (Theo đúng chuẩn DESIGN.md Mục 5: Clean, Minimal, Tinh gọn)
            let mut status_bar = StatusBarWidget::new();
            status_bar.add_left(Span::styled("main", Style::default().fg(Theme::PRIMARY).add_modifier(Modifier::BOLD)));
            status_bar.add_left(Span::styled("3 files changed", Style::default().fg(Theme::SECONDARY)));
            status_bar.add_left(Span::styled("✔ Ready", Style::default().fg(Theme::SUCCESS)));

            status_bar.add_right(Span::styled(format!("Tab [{}/5]", tabs.selected() + 1), Style::default().fg(Theme::ACCENT)));
            status_bar.add_right(Span::styled("Esc: Exit", Style::default().fg(Theme::MUTED)));

            status_bar.render(main_chunks[2], f);
        })?;

        // Polling sự kiện phím và chuột
        if event::poll(Duration::from_millis(30))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        let is_ctrl_c = key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c');
                        if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc || is_ctrl_c {
                            break;
                        }

                        // 1. Chuyển Tab riêng biệt qua phím số 1..5 hoặc Tab / Shift+Tab (không chiếm phím Left/Right)
                        let mut target_tab = None;
                        match key.code {
                            KeyCode::Char('1') => target_tab = Some(0),
                            KeyCode::Char('2') => target_tab = Some(1),
                            KeyCode::Char('3') => target_tab = Some(2),
                            KeyCode::Char('4') => target_tab = Some(3),
                            KeyCode::Char('5') => target_tab = Some(4),
                            KeyCode::Tab => {
                                tabs.select_next();
                                if tabs.selected() == 3 {
                                    tabs.set_auto_scroll(true); // Tab 4 (Logs): Tiếp tục sticky follow
                                } else {
                                    tabs.set_auto_scroll(false); // Tab 1, 2, 3, 5: Khởi đầu từ đầu trang
                                    tabs.scroll_to_top();
                                }
                                continue;
                            }
                            KeyCode::BackTab => {
                                tabs.select_prev();
                                if tabs.selected() == 3 {
                                    tabs.set_auto_scroll(true); // Tab 4 (Logs): Tiếp tục sticky follow
                                } else {
                                    tabs.set_auto_scroll(false); // Tab 1, 2, 3, 5: Khởi đầu từ đầu trang
                                    tabs.scroll_to_top();
                                }
                                continue;
                            }
                            _ => {}
                        };

                        if let Some(tab_idx) = target_tab {
                            tabs.set_selected(tab_idx);
                            if tab_idx == 3 {
                                tabs.set_auto_scroll(true); // Tab 4 (Logs): Bật sticky follow
                            } else {
                                tabs.set_auto_scroll(false); // Tab 1, 2, 3, 5: Khởi đầu từ đầu trang
                                tabs.scroll_to_top();
                            }
                            continue;
                        }

                        // 2. Toàn bộ phím điều hướng Left / Right / Up / Down / PageUp / PageDown / Home / End
                        match tabs.selected() {
                            0 => {
                                // Tab 1 (Form): FormManager xử lý trọn vẹn (Radio Left/Right, Input Cursor, Up/Down Focus)
                                let _ = form.handle_event(key);
                            }
                            _ => {
                                // Mọi Tab còn lại (Tab 2: Table, Tab 3: Tasks, Tab 4: Logs, Tab 5: Icons & Theme):
                                // Đều hỗ trợ cuộn văn bản và danh sách mượt mà qua TabsWidget
                                tabs.handle_event(key);
                            }
                        }
                    }
                }
                Event::Mouse(mouse) => {
                    // Lăn chuột cuộn nội dung container ở bất kỳ Tab nào có thanh cuộn
                    tabs.handle_mouse_event(mouse);
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    println!("=== MASTER SHOWCASE HOÀN TẤT ===");
    Ok(())
}
