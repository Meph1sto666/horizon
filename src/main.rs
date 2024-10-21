use std::{fs::File, io::BufReader};

use color_eyre::Result;
use crossterm::event::KeyModifiers;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{BLUE, SLATE},
        Color, Modifier, Style, Stylize,
    },
    symbols,
    text::Line,
    widgets::{
        Block, BorderType, Borders, HighlightSpacing, List, Padding, Paragraph, StatefulWidget, Widget, Wrap
    },
    DefaultTerminal,
};
use rodio::{Decoder, OutputStream, Sink};
mod playlist;
use playlist::{Queue, Song};

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
// const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;
// const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default(
        &mut Sink::try_new(&OutputStream::try_default().unwrap().1).unwrap()
        // &mut Sink::new_idle().0
    ).run(terminal);
    ratatui::restore();
    app_result
}

struct App<'a> {
    pub audio_controls: &'a Sink,
    // pub focus: i8,
    pub queue: Queue,
    pub should_exit: bool,
}

// struct TodoList {
//     items: Vec<TodoItem>,
//     state: ListState,
// }

// #[derive(Debug)]
// struct TodoItem {
//     todo: String,
//     info: String,
//     status: Status,
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// enum Status {
//     Todo,
//     Completed,
// }

impl<'a> App<'a> {
    fn default(controller: &'a mut Sink) -> Self {
        Self {
            should_exit: false,
            // focus: 0,
            queue: Queue::new::<Song>(),
            audio_controls: controller
        }
    }
}

// impl FromIterator<(Status, &'static str, &'static str)> for TodoList {
//     fn from_iter<I: IntoIterator<Item = (Status, &'static str, &'static str)>>(iter: I) -> Self {
//         let items = iter
//             .into_iter()
//             .map(|(status, todo, info)| TodoItem::new(status, todo, info))
//             .collect();
//         let state = ListState::default();
//         Self { items, state }
//     }
// }

// impl TodoItem {
//     fn new(status: Status, todo: &str, info: &str) -> Self {
//         Self {
//             status,
//             todo: todo.to_string(),
//             info: info.to_string(),
//         }
//     }
// }

impl<'a> App<'a> {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        if key.code == KeyCode::Char('q') && key.modifiers == KeyModifiers::ALT {self.should_exit = true}
        if key.modifiers == KeyModifiers::SHIFT && (key.code == KeyCode::Char('N') || key.code == KeyCode::Char('N')) {
            self.skip_one();
        }
        match key.code {
            // KeyCode::Char('a') | KeyCode::Char('A') => self.select_none(),
            KeyCode::Enter => self.play(),
            // KeyCode::Char('c') | KeyCode::Char('C') => self.clear(),
            KeyCode::Char(' ') => self.toggle_playback(),

            KeyCode::Down => self.select_next(),
            KeyCode::Up => self.select_previous(),
            // KeyCode::Char('g') | KeyCode::Home => self.select_first(),
            // KeyCode::Char('G') | KeyCode::End => self.select_last(),
            // KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
            //     self.toggle_status();
            // }
            _ => {}
        }
    }

    // fn select_none(&mut self) {
    //     self.queue.state.select(None);
    // }

    fn select_next(&mut self) {
        self.queue.state.select_next();
    }
    fn select_previous(&mut self) {
        self.queue.state.select_previous();
    }

    // fn select_first(&mut self) {
    //     self.queue.state.select_first();
    // }

    // fn select_last(&mut self) {
    //     self.queue.state.select_last();
    // }

    fn play(&mut self) {
        // self.queue.clear();
        // self.queue.push();
        let index = self.queue.state.selected().unwrap();
        // self.audio_controls.append(self.queue.songs.get(index).unwrap().get_source());
        self.audio_controls.append(Decoder::new(BufReader::new(File::open(self.queue.songs.get(index).unwrap().path.clone()).unwrap())).unwrap());
        // if let Some(i) = self.queue.state.selected() {
            // self.queue.songs[i] = match self.queue.songs[i] {
            //     Status::Completed => Status::Todo,
            //     Status::Todo => Status::Completed,
            // }
        // }
    }
    fn skip_one(&mut self) {
        self.audio_controls.skip_one();
    }
    fn toggle_playback(&mut self) {
        if self.audio_controls.is_paused() {
            self.audio_controls.play();
        } else {
            self.audio_controls.pause();
        }
    }
}

impl Widget for &mut App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [tree_area, main_area, queue_area] = Layout::horizontal([
            // Constraint::from_percentages([20, 50, 30]),
            Constraint::Percentage(20),
            Constraint::Percentage(50),
            Constraint::Percentage(30)
        ])
        .areas(area);

        let [_, item_area] = Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(main_area);

        self.render_queue(queue_area, buf);
        App::render_tree(tree_area, buf);
        // App::render_list(tree_area, buf);
        self.render_selected_item(item_area, buf);
    }
}

impl App<'_> {
    // fn render_list(area: Rect, buf: &mut Buffer) {
    //     Paragraph::new("Queue")
    //         .bold()
    //         .centered()
    //         .render(area, buf);
    // }

    fn render_tree(area: Rect, buf: &mut Buffer) {
        Paragraph::new("[ALT]+[Q] to exit")
            .centered()
            .render(area, buf);
    }

    fn render_queue(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Playback queue").centered())
            .borders(Borders::all()).border_type(BorderType::Rounded);

        let items: Vec<playlist::Song> = self
            .queue
            .songs
            .iter()
            .enumerate()
            .map(|(_, s)| {
                // let color = alternate_colors(i);
                let song = s;
                playlist::Song::from((*song).clone())
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.queue.state);
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        // We get the info depending on the item's state.
        let info = if let Some(_) = self.queue.state.selected() {
            // match self.queue.songs[i] {
                // Status::Completed => format!("✓ DONE: {}", self.queue.songs[i].info),
                // Status::Todo => format!("☐ TODO: {}", self.queue.songs[i].info),
            // }
            "Nothing selected...".to_string()
        } else {
            "Nothing selected...".to_string()
        };

        // We show the list item's info under the list in this paragraph
        let block = Block::new()
            .title(Line::raw("TODO Info").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG)
            .padding(Padding::horizontal(1));

        // We can now render the item info
        Paragraph::new(info)
            .block(block)
            .fg(TEXT_FG_COLOR)
            .wrap(Wrap { trim: false })
            .render(area, buf);
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
