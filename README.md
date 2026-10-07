# cli_gui_form

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![Rust](https://img.shields.io/badge/Rust-2024%20Edition-orange.svg)](https://www.rust-lang.org)
[![Ratatui](https://img.shields.io/badge/Ratatui-0.28-green.svg)](https://ratatui.rs)

A minimal, professional, zero-noise TUI Form & Design System library for Rust powered by [`ratatui`](https://crates.io/crates/ratatui) and [`crossterm`](https://crates.io/crates/crossterm).

Designed for enterprise-grade command-line tools, AI pipelines, and interactive developer CLI interfaces that demand high visual fidelity, seamless keyboard navigation, zero heap allocation in render loops, and strict protection against terminal border/frame breakage.

---

## ✨ Features

- 🎯 **Focus Lifecycle & Viewport Management (`FormManager`)**:
  - Full keyboard navigation (`Tab` / `Shift+Tab`, `Up` / `Down` arrow keys).
  - Dynamic scrolling viewport when forms exceed terminal screen height.
  - Native inline cursor tracking (`frame.set_cursor_position`).
- 🧩 **Comprehensive Widget Suite**:
  - `InputWidget`: Single-line Text & Password inputs with real-time cursor editing.
  - `CheckboxWidget`: Boolean toggle controls (`[✔]` / `[ ]`).
  - `RadioWidget`: Single-choice option groups (`(•)` / `( )`).
  - `ButtonWidget`: Action triggers with submit signal integration.
  - `TaskWidget`: Unified multi-phase task runner supporting indeterminate pulse spinners and deterministic progress tracking.
  - `ShimmerWidget`: Smooth animated gradient highlight for status indicators.
  - `TabsWidget`: Clean top rounded tab navigation (`╭─┬─╮`, `╰`, `╯`, `┴`) with continuous border container rendering.
  - `StatusBarWidget`: Single-line bottom utility bar with split left/right metrics.
- 🎨 **Zero-Allocation Icon Engine (`Icons`)**:
  - 22 curated Unicode symbols locked to Text Presentation mode (`\u{FE0E}`) to prevent emoji font rendering glitches.
  - Pre-padded 2-cell terminal alignment for zero runtime `format!()` heap allocations.
- 🛡️ **Industrial Frame Overflow Protection**:
  - Auto-truncation guard in container rendering prevents inner text overflow from pushing or corrupting right-side borders.
- 📜 **Dual Licensed**: MIT OR Apache-2.0 for unrestricted commercial or open-source integration.

---

## 📦 Installation

Add `cli_gui_form` and `ratatui` to your `Cargo.toml`:

```toml
[dependencies]
cli_gui_form = "0.1.0"
crossterm = "0.28"
ratatui = "0.28"
```

Or via `cargo`:

```bash
cargo add cli_gui_form crossterm ratatui
```

---

## 🚀 Quick Start

Here is a minimal, complete example of creating an interactive form:

```rust,no_run
use cli_gui_form::{
    ButtonWidget, CheckboxWidget, EventResult, FormManager, FormValue, InputMode,
    InputWidget, RadioWidget,
};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, style::Color, Terminal};
use std::io::{self, stdout};

fn main() -> io::Result<()> {
    // 1. Setup terminal in raw mode
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    // 2. Initialize FormManager and register widgets
    let mut form = FormManager::new();
    form.add_widget(Box::new(InputWidget::new("user", "Username", InputMode::Text)));
    form.add_widget(Box::new(InputWidget::new("pass", "Password", InputMode::Password)));
    form.add_widget(Box::new(CheckboxWidget::new("remember", "Remember Me", true)));
    form.add_widget(Box::new(RadioWidget::new(
        "env",
        "Environment",
        vec!["Dev".into(), "Staging".into(), "Prod".into()],
    )));
    form.add_widget(Box::new(ButtonWidget::new("submit", "SUBMIT", Color::Blue, Color::White)));

    // 3. Main event loop
    loop {
        terminal.draw(|f| {
            form.render(f.area(), f);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if key.code == KeyCode::Esc {
                    break;
                }
                if form.handle_event(key) == EventResult::Submitted {
                    let values = form.get_values();
                    // Process submitted form data
                    break;
                }
            }
        }
    }

    // 4. Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
```

---

## 📚 Widget Catalog & Usage

### 1. `TabsWidget` & `StatusBarWidget`

Render a clean multi-tab layout with an overflow-safe container and status bar:

```rust
use cli_gui_form::{TabsWidget, StatusBarWidget};
use ratatui::style::Color;

// Create tabs
let tabs = TabsWidget::new(
    vec!["Overview".into(), "Processes".into(), "Logs".into(), "Settings".into()],
    Color::Cyan,
);

// In your render loop:
let inner_lines = vec![
    vec![ratatui::text::Span::raw("System operational. All metrics normal.")],
];
tabs.render_container(area, frame, &inner_lines);

// Bottom status bar
let status_bar = StatusBarWidget::new(
    "cli_gui_form v0.1.0",
    "FPS: 60  ─  CPU: 12%  ─  MEM: 412 MB",
);
status_bar.render(status_area, frame);
```

### 2. `TaskWidget` (Pulse Loading & Progress)

Handle background tasks with indeterminate spinners or discrete progress percentages:

```rust
use cli_gui_form::{TaskWidget, SpinnerType};
use ratatui::style::Color;

// Indeterminate spinner task
let mut loading_task = TaskWidget::new_loading(
    "inference_job",
    "Model Loading",
    "Fetching model weights from cache...",
    Color::Magenta,
).with_spinner_type(SpinnerType::Pulse);

// In event / tick loop:
loading_task.tick();

// Switch to deterministic progress when job starts
loading_task.switch_to_progress(
    "Processing",
    500,       // total items
    "frames",  // unit
    "12s",     // elapsed / ETA
    "Rendering audio waveforms...",
);
loading_task.update_progress(120, "3s");
```

### 3. `ShimmerWidget`

Render smooth animated gradient bars for activity indicators:

```rust
use cli_gui_form::ShimmerWidget;
use ratatui::style::Color;

let mut shimmer = ShimmerWidget::new(
    "Syncing Data",
    Color::Rgb(100, 149, 237), // Cornflower Blue
    Color::Rgb(220, 240, 255), // Shimmer Peak Light
    24,                         // Width in chars
);

// On each animation tick (e.g. 30ms):
shimmer.tick();
shimmer.render(area, frame);
```

---

## 🎨 Theme & Icons Reference

### Zero-Allocation Icons (`Icons`)

| Constant | Unicode | Presentation Mode | Standard Usage |
| :--- | :--- | :--- | :--- |
| `Icons::SUCCESS` | `✔ ` | `\u{2714}\u{FE0E} ` | Completed tasks, valid inputs, checked boxes |
| `Icons::ERROR` | `✘ ` | `\u{2718}\u{FE0E} ` | Failed jobs, syntax errors |
| `Icons::WARNING` | `▲ ` | `\u{25B2}\u{FE0E} ` | Alerts, resource limits, deprecations |
| `Icons::INFO` | `ℹ ` | `\u{2139}\u{FE0E} ` | Informational callouts, tooltips |
| `Icons::CHEVRON_RIGHT` | `▸ ` | `\u{25B8}\u{FE0E} ` | Active tab indicator, breadcrumb separator |
| `Icons::RADIO_ACTIVE` | `• ` | `\u{2022}\u{FE0E} ` | Selected radio item |
| `Icons::RADIO_INACTIVE` | `  ` | `  ` | Unselected radio item |
| `Icons::PULSE_FRAMES` | `[··, •·, ••, ·•]` | 4-state pulse animation | Smooth loading states |

### Design System Colors (`Theme`)

- **Primary**: `Theme::PRIMARY` (`#4589FF` / IBM Blue)
- **Success**: `Theme::SUCCESS` (`#25A249` / Terminal Green)
- **Warning**: `Theme::WARNING` (`#F1C21B` / Amber)
- **Error**: `Theme::ERROR` (`#DA1E28` / Crimson)
- **Neutral Dark Scale**:
  - `Theme::BG_DARK` (`#121212`)
  - `Theme::SURFACE` (`#1E1E1E`)
  - `Theme::BORDER` (`#393939`)
  - `Theme::TEXT_MUTED` (`#8D8D8D`)
  - `Theme::TEXT_MAIN` (`#F4F4F4`)

---

## 🧪 Running the Examples

This crate includes interactive examples demonstrating all components in action:

```bash
# Full interactive form with background worker thread
cargo run --example demo

# Multi-tab dashboard layout with overflow protection & bottom status bar
cargo run --example demo_tabs

# Complete 22-icon Unicode palette and styling test
cargo run --example demo_icons_palette

# Multi-phase task spinner and pulse lifecycle
cargo run --example demo_pulse_task

# Animated gradient shimmer progress bar
cargo run --example status_shimmer

# Responsive data table with selection cursor
cargo run --example demo_table

# Minimal TUI boilerplate
cargo run --example demo_minimal_tui
```

---

## 🏛️ Architecture & Philosophy

1. **YAGNI & Zero Fluff**: Clean, purposeful primitives. Every widget has a well-defined lifecycle (`focus`, `blur`, `render`, `handle_event`, `cursor_position`).
2. **Deterministic Layouts**: All borders, corner transitions (`╭`, `╮`, `╯`, `╰`, `┬`, `┴`, `├`, `┤`), and inner padding respect exact pixel-cell coordinates.
3. **Thread Safety**: Widgets and state can be safely wrapped in standard `Arc<Mutex<T>>` wrappers to receive real-time updates from background worker threads without UI blocking.

---

## 📄 License

This project is dual-licensed under either:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
