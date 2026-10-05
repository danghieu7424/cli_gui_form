use cli_gui_form::{
    ButtonWidget, CheckboxWidget, EventResult, FormManager, FormWidget, InputMode, InputWidget,
    LoadingWidget, RadioWidget,
};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, style::Color, Terminal};
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

    let mut form = FormManager::new();
    form.add_widget(Box::new(InputWidget::new("Username", InputMode::Text)));
    form.add_widget(Box::new(InputWidget::new("Password", InputMode::Password)));
    form.add_widget(Box::new(CheckboxWidget::new("Remember Me", false)));
    form.add_widget(Box::new(RadioWidget::new(
        "Environment",
        vec!["Dev".into(), "Staging".into(), "Prod".into()],
    )));

    // Khởi tạo widget Loading động
    let mut loading = LoadingWidget::new(
        "Processing",
        "Đang chạy inference mô hình giọng nói...",
        Color::Magenta,
    )
    .with_bar_width(25);

    form.add_widget(Box::new(ButtonWidget::new("SUBMIT", Color::Blue, Color::White)));

    // Nhịp quét sự kiện và tốc độ khung hình (khoảng 60 FPS)
    let target_frame_duration = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    loop {
        // Render Frame
        terminal.draw(|f| {
            let chunks = ratatui::layout::Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .margin(1)
                .constraints([
                    ratatui::layout::Constraint::Length(12),
                    ratatui::layout::Constraint::Length(3), // Khu vực cho LoadingWidget
                    ratatui::layout::Constraint::Min(0),
                ])
                .split(f.area());

            form.render(chunks[0], f);
            loading.render(chunks[1], f);
        })?;

        // Cập nhật frame hoạt họa mỗi 40ms để chuyển động vừa mắt
        if last_tick.elapsed() >= Duration::from_millis(40) {
            loading.tick();
            last_tick = Instant::now();
        }

        // Non-blocking Polling: không chặn luồng vẽ của terminal
        if event::poll(target_frame_duration)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Esc {
                        break;
                    }
                    let res = form.handle_event(key);
                    if res == EventResult::Submitted {
                        break;
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}