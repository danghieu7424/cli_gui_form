use cli_gui_form::{
    ButtonWidget, CheckboxWidget, EventResult, FormManager, FormWidget, InputMode, InputWidget,
    RadioWidget, TaskWidget,
};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, style::Color, Terminal};
use std::{
    io::{self, stdout},
    sync::{Arc, Mutex},
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

    // Tạo TaskWidget bắt đầu bằng pha Loading (Nạp mô hình)
    let task = Arc::new(Mutex::new(TaskWidget::new_loading(
        "Processing",
        "Đang nạp trọng số mô hình giọng nói...",
        Color::Magenta,
    )));
    form.add_widget(Box::new(TaskShared(Arc::clone(&task))));

    form.add_widget(Box::new(ButtonWidget::new("SUBMIT", Color::Blue, Color::White)));

    let target_frame_duration = Duration::from_millis(16);
    let mut last_tick = Instant::now();
    let start_time = Instant::now();
    let mut switched = false;
    let mut current_chunk = 0;

    loop {
        terminal.draw(|f| {
            form.render(f.area(), f);
        })?;

        // 1. Cập nhật nhịp animation và logic mô phỏng tiến trình
        if last_tick.elapsed() >= Duration::from_millis(40) {
            let mut t = task.lock().unwrap();

            // Giai đoạn 1: 0 -> 3 giây: Đang Loading (Nạp mô hình)
            if start_time.elapsed() < Duration::from_secs(3) {
                t.tick();
            } 
            // Giai đoạn 2: Sau 3 giây: Tự động chuyển sang chạy tiến trình
            else {
                if !switched {
                    t.color = Color::Cyan;
                    t.switch_to_progress(
                        "Separation",
                        134,
                        "chunks",
                        "0s",
                        "MDX-Net Native Inference",
                    );
                    switched = true;
                }

                // Mô phỏng tăng chunk xử lý
                if current_chunk < 134 {
                    current_chunk += 1;
                    let elapsed_sec = (start_time.elapsed().as_secs() - 3).max(1);
                    t.update_progress(current_chunk, format!("{}s", elapsed_sec));
                }
            }

            last_tick = Instant::now();
        }

        // 2. Bắt sự kiện bàn phím không chặn khung hình
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

// Wrapper chia sẻ con trỏ task giữa Main loop và FormManager
struct TaskShared(Arc<Mutex<TaskWidget>>);

impl FormWidget for TaskShared {
    fn render(&self, area: ratatui::layout::Rect, frame: &mut ratatui::Frame) {
        if let Ok(t) = self.0.lock() {
            t.render(area, frame);
        }
    }

    fn handle_event(&mut self, key: crossterm::event::KeyEvent) -> EventResult {
        if let Ok(mut t) = self.0.lock() {
            t.handle_event(key)
        } else {
            EventResult::Ignored
        }
    }

    fn focus(&mut self) {
        if let Ok(mut t) = self.0.lock() {
            t.focus();
        }
    }

    fn blur(&mut self) {
        if let Ok(mut t) = self.0.lock() {
            t.blur();
        }
    }

    fn is_focused(&self) -> bool {
        self.0.lock().map(|t| t.is_focused()).unwrap_or(false)
    }

    fn preferred_height(&self) -> u16 {
        self.0.lock().map(|t| t.preferred_height()).unwrap_or(3)
    }
}