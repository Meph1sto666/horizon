use std::{borrow::{Borrow, BorrowMut}, fs, ops::ControlFlow, slice::from_ref};

use color_eyre::Result;
use crossterm::event::KeyModifiers;
use playlist::{Queue, Song};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{BLUE, GREEN, SLATE},
        Color, Modifier, Style, Stylize,
    },
    symbols,
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
        StatefulWidget, Widget, Wrap,
    },
    DefaultTerminal, Frame
};
use rodio::{OutputStream, Sink};
mod playlist;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}

struct App<'a> {
    audio_controls: Sink,
    focus: i8,
    queue: Queue<'a>,
    should_exit: bool
}

impl Default for App<'_> {
    fn default() -> Self {
        let folder: String = "./music/".to_owned();
        let dir = fs::read_dir(folder.clone()).unwrap();
        let mut songs: Vec<Song> = Vec::new();

        // for s in dir.enumerate() {songs.push(Song::new(folder.clone()+(s.1.unwrap().file_name().into_string().unwrap().borrow())));}
        let audio_controls: &mut Sink = &mut Sink::try_new(&OutputStream::try_default().unwrap().1).unwrap();
        // let queue = playlist::Queue::new::<String>(audio_controls, songs);
        Self {
            audio_controls: Sink::try_new(&OutputStream::try_default().unwrap().1).unwrap(),
            focus: 0,
            queue: playlist::Queue::new::<String>(),
            should_exit: false
        }
    }
}

impl App<'_> {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let should_exit: bool = self.should_exit.clone();
        while !should_exit {
            let mut render_callback = |mut frame:Frame| frame.render_widget(&mut self, frame.area());
            terminal.draw(|arg0: &mut ratatui::Frame<'_>| render_callback(*arg0))?;
            if let event::Event::Key(key) = event::read()? {
                self.handle_key_event(key);
            };
        }
        Ok(())
    }
    fn handle_key_event(self, key: event::KeyEvent) -> ControlFlow<()> {
        if key.code == KeyCode::Char('q') && key.modifiers == KeyModifiers::SHIFT {
            return ControlFlow::Break(())
        }
        // match key.code {
        //     _ => (),
        // }
        ControlFlow::Continue(())
    }
}


impl Widget for &mut App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        let [list_area, item_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(main_area);

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf);
        self.render_list(list_area, buf);
        self.render_selected_item(item_area, buf);
    }
}

/// Rendering logic for the app
impl App<'_> {
    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Ratatui List Example")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.")
            .centered()
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("TODO List").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY);
            // .border_style(TODO_HEADER_STYLE)
            // .bg(NORMAL_ROW_BG);

        // Iterate through all elements in the `items` and stylize them.
        // let items: Vec<ListItem> = self
            // .todo_list
            // .items
            // .iter()
            // .enumerate()
            // .map(|(i, todo_item)| {
                // let color = alternate_colors(i);
                // ListItem::from(todo_item).bg(color)
            // })
            // .collect();

        // Create a List from all list items and highlight the currently selected one
        // let list = List::new(items)
        //     .block(block)
        //     // .highlight_style(SELECTED_STYLE)
        //     .highlight_symbol(">")
        //     .highlight_spacing(HighlightSpacing::Always);

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        // StatefulWidget::render(list, area, buf, &mut self.todo_list.state);
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        // We get the info depending on the item's state.
        // let info = if let Some(i) = self.todo_list.state.selected() {
            // match self.todo_list.items[i].status {
                // Status::Completed => format!("✓ DONE: {}", self.todo_list.items[i].info),
                // Status::Todo => format!("☐ TODO: {}", self.todo_list.items[i].info),
            // }
        // } else {
        //     "Nothing selected...".to_string()
        // };

        // We show the list item's info under the list in this paragraph
        let block = Block::new()
            .title(Line::raw("TODO Info").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            // .border_style(TODO_HEADER_STYLE)
            // .bg(NORMAL_ROW_BG)
            .padding(Padding::horizontal(1));

        // We can now render the item info
        // Paragraph::new(info)
            // .block(block)
            // // .fg(TEXT_FG_COLOR)
            // .wrap(Wrap { trim: false })
            // .render(area, buf);
    }
}

// const fn alternate_colors(i: usize) -> Color {
//     if i % 2 == 0 {
//         NORMAL_ROW_BG
//     } else {
//         ALT_ROW_BG_COLOR
//     }
// }

// impl From<&TodoItem> for ListItem<'_> {
//     fn from(value: &TodoItem) -> Self {
//         let line = match value.status {
//             Status::Todo => Line::styled(format!(" ☐ {}", value.todo), TEXT_FG_COLOR),
//             Status::Completed => {
//                 Line::styled(format!(" ✓ {}", value.todo), COMPLETED_TEXT_FG_COLOR)
//             }
//         };
//         ListItem::new(line)
//     }
// }