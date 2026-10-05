// --- PHÂN ĐOẠN: DEMO RUNNER VỚI ĐÚNG THỨ TỰ FORM VÀ KHÔNG BỊ NÉN KHUNG ---

use cli_gui_form::{
    ButtonWidget, CheckboxWidget, EventResult, FormManager, InputMode, InputWidget,
    LoadingWidget, ProgressWidget, RadioWidget,
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

    // 1. Inputs
    form.add_widget(Box::new(InputWidget::new("Username", InputMode::Text)));
    form.add_widget(Box::new(InputWidget::new("Password", InputMode::Password)));

    // 2. Checkbox & Radio
    form.add_widget(Box::new(CheckboxWidget::new("Remember Me", false)));
    form.add_widget(Box::new(RadioWidget::new(
        "Environment",
        vec!["Dev".into(), "Staging".into(), "Prod".into()],
    )));

    // 3. Progress bars
    form.add_widget(Box::new(
        ProgressWidget::new("Separation", 90, 134, Color::Cyan)
            .with_unit("chunks")
            .with_duration("2m")
            .with_status("MDX-Net Native Inference")
            .with_bar_width(20),
    ));

    form.add_widget(Box::new(
        ProgressWidget::new("TTS Generator", 667, 667, Color::Green)
            .with_duration("0s")
            .with_status("Hoàn tất sinh audio các câu thoại")
            .with_bar_width(20),
    ));

    // 4. Loading Widget (Nằm TRƯỚC Button Submit)
    let loading = Arc::new(Mutex::new(
        LoadingWidget::new(
            "Processing",
            "Đang chạy inference mô hình giọng nói...",
            Color::Magenta,
        )
        .with_bar_width(20),
    ));
    form.add_widget(Box::new(LoadingShared(Arc::clone(&loading))));

    // 5. Submit Button (Nằm SAU Loading)
    form.add_widget(Box::new(ButtonWidget::new("SUBMIT", Color::Blue, Color::White)));

    let target_frame_duration = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            // Render toàn bộ form theo chiều cao tự tính, không cắt layout cứng
            form.render(f.area(), f);
        })?;

        // Cập nhật frame xoay spinner
        if last_tick.elapsed() >= Duration::from_millis(40) {
            if let Ok(mut l) = loading.lock() {
                l.tick();
            }
            last_tick = Instant::now();
        }

        // Non-blocking Poll
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

// Wrapper nhỏ để FormManager có thể sở hữu LoadingWidget trong khi main loop vẫn gọi tick()
struct LoadingShared(Arc<Mutex<LoadingWidget>>);

impl cli_gui_form::FormWidget for LoadingShared {
    fn render(&self, area: ratatui::layout::Rect, frame: &mut ratatui::Frame) {
        if let Ok(l) = self.0.lock() {
            l.render(area, frame);
        }
    }

    fn handle_event(&mut self, key: crossterm::event::KeyEvent) -> EventResult {
        if let Ok(mut l) = self.0.lock() {
            l.handle_event(key)
        } else {
            EventResult::Ignored
        }
    }

    fn focus(&mut self) {
        if let Ok(mut l) = self.0.lock() {
            l.focus();
        }
    }

    fn blur(&mut self) {
        if let Ok(mut l) = self.0.lock() {
            l.blur();
        }
    }

    fn is_focused(&self) -> bool {
        self.0.lock().map(|l| l.is_focused()).unwrap_or(false)
    }

    fn preferred_height(&self) -> u16 {
        self.0.lock().map(|l| l.preferred_height()).unwrap_or(3)
    }
}