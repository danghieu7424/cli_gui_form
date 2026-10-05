// --- PHÂN ĐOẠN: DEFINITION CỦA FORM WIDGET TRAIT ---

use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

/// Kết quả trả về sau khi widget xử lý một sự kiện phím
#[derive(Debug, PartialEq, Eq)]
pub enum EventResult {
    Consumed,
    Ignored,
    Submitted, // Dành riêng cho Button hoặc Enter trigger
}

/// Trait cốt lõi cho mọi phần tử form trong CLI GUI
pub trait FormWidget {
    /// Render widget lên vùng hiển thị Rect được cấp
    fn render(&self, area: Rect, frame: &mut Frame);

    /// Tiếp nhận và xử lý sự kiện phím khi đang được focus
    fn handle_event(&mut self, key: KeyEvent) -> EventResult;

    /// Thiết lập trạng thái focus
    fn focus(&mut self);

    /// Hủy trạng thái focus
    fn blur(&mut self);

    /// Kiểm tra trạng thái focus hiện tại
    fn is_focused(&self) -> bool;

    /// Chiều cao dòng khuyến nghị khi chia layout (Constraint::Length)
    fn preferred_height(&self) -> u16;
}