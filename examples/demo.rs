use cli_gui_form::{
    ButtonWidget, CheckboxWidget, EventResult, FormManager, InputMode, InputWidget,
    ProgressWidget, RadioWidget,
};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, style::Color, Terminal};
use std::io::{self, stdout};

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
    // Ví dụ 1: 90/134 chunks (2m) MDX-Net Native Inference
    form.add_widget(Box::new(
        ProgressWidget::new("Separation", 90, 134, Color::Cyan)
            .with_unit("chunks")
            .with_duration("2m")
            .with_status("MDX-Net Native Inference")
            .with_bar_width(20),
    ));

    // Ví dụ 2: 667/667 (0s) Hoàn tất sinh audio các câu thoại
    form.add_widget(Box::new(
        ProgressWidget::new("TTS Generator", 667, 667, Color::Green)
            .with_duration("0s")
            .with_status("Hoàn tất sinh audio các câu thoại")
            .with_bar_width(20),
));
    form.add_widget(Box::new(ButtonWidget::new("SUBMIT", Color::Blue, Color::White)));

    loop {
        terminal.draw(|f| {
            form.render(f.area(), f);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Esc {
                break;
            }

            let result = form.handle_event(key);
            if result == EventResult::Submitted {
                break;
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}