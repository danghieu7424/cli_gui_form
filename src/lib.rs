// --- PHÂN ĐOẠN: EXPORT GIAO DIỆN CRATE ---

pub mod manager;
pub mod traits;
pub mod widgets;

pub use manager::FormManager;
pub use traits::{EventResult, FormValue, FormWidget};
pub use widgets::*;