use ratatui::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget};
use rodio::Sink;

pub struct Queue<'a> {
    controller: &'a mut Sink,
    current_song: i32,
    songs: Vec<String>
}

impl<'a> Queue<'a> {
    pub fn new<T>(controller: &'a mut Sink) -> Self {
        Queue {
            controller: controller,
            current_song: -1,
            songs: Vec::new()
        }
    }

    pub fn add_song<T: Into<String>>(&mut self, song: T) {
        self.songs.push(song.into());
    }
}
impl<'a> Widget for Queue<'a> {
    // #[allow(clippy::cast_possible_truncation)]
    fn render(self, area: Rect, buf: &mut Buffer) {
        // let (background, text, shadow, highlight) = self.colors();
        buf.set_style(area, Style::new());

        // render top line if there's enough space
        if area.height > 2 {
            buf.set_string(
                area.x,
                area.y,
                "▔".repeat(area.width as usize),
                Style::new()
            );
        }
        // render bottom line if there's enough space
        if area.height > 1 {
            buf.set_string(
                area.x,
                area.y + area.height - 1,
                "▁".repeat(area.width as usize),
                Style::new()
            );
        }
        // render label centered
        // buf.set_line(
        //     area.x + (area.width.saturating_sub(self.label.width() as u16)) / 2,
        //     area.y + (area.height.saturating_sub(1)) / 2,
        //     &self.label,
        //     area.width,
        // );
    }
}

// impl Queue<'_> {
//     const fn colors(&self) -> (Color, Color, Color, Color) {
//         let theme = self.theme;
//         match self.state {
//             State::Normal => (theme.background, theme.text, theme.shadow, theme.highlight),
//             State::Selected => (theme.highlight, theme.text, theme.shadow, theme.highlight),
//             State::Active => (theme.background, theme.text, theme.highlight, theme.shadow),
//         }
//     }
// }