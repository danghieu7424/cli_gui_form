use cli_gui_form::{
    ButtonWidget, CheckboxWidget, EventResult, FormManager, InputMode, InputWidget, RadioWidget,
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
                // Nhấn Enter tại nút SUBMIT
                break;
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}