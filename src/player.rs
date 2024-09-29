use ratatui::{buffer::Buffer, layout::Rect, style::{Color, Style}, widgets::Widget};

pub struct Button {
    label: String
}

impl Widget for Button {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // let style = if self.is_pressed {
        //     self.pressed_style.unwrap_or_else(|| Style::default().fg(Color::Blue))
        // } else {
        //     self.style
        // };
        buf.set_string(area.left(), area.top(), &self.label, Style::default());
    }
}