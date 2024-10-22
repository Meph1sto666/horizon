use std::{fs::File, io::BufReader};

use color_eyre::Result;
use crossterm::event::KeyModifiers;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind::SLATE, Modifier, Style, Stylize},
    // symbols,
    // text::{Line, Text},
    widgets::{Block, BorderType, Borders, HighlightSpacing, List, StatefulWidget, Widget},
    DefaultTerminal,
};
use rodio::{Decoder, OutputStream, Sink};
mod library;
use library::Library;
mod playlist;
use playlist::{dir_to_songs, Queue, Song};
use tui_tree_widget::{Tree, TreeItem, TreeState};

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
// const FOCUS_BLOCK_BORDER_STYLE: Style = Style::new().color(ratatui::style::palette::tailwind::LIME);
// const BLOCK_BORDER_STYLE: Style = Style::new().gray();

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result =
        App::default(&mut Sink::try_new(&OutputStream::try_default().unwrap().1).unwrap())
            .run(terminal);
    ratatui::restore();
    app_result
}

struct App<'a> {
    pub audio_controls: &'a Sink,
    pub focus: i8,
    pub queue: Queue,
    pub should_exit: bool,
    pub library: Library<'a>,
}

impl<'a> App<'a> {
    fn default(controller: &'a mut Sink) -> Self {
        Self {
            should_exit: false,
            focus: 0,
            queue: Queue::new::<Song>(),
            library: Library::new::<Song>(),
            audio_controls: controller,
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
        if key.kind != KeyEventKind::Press { return; }
        if key.code == KeyCode::Char('q') && key.modifiers == KeyModifiers::ALT { self.should_exit = true }
        if key.modifiers == KeyModifiers::SHIFT && (key.code == KeyCode::Char('N') || key.code == KeyCode::Char('N')) { self.skip_one(); }
        match key.code {
            KeyCode::Char('t') => self.focus = 0,
            KeyCode::Char('p') => self.focus = 1,
            KeyCode::Char('q') => self.focus = 2,
            _ => {}
        }

        match self.focus {
            2 => match key.code {
                KeyCode::Enter => self.play(),
                KeyCode::Char(' ') => self.toggle_playback(),
                KeyCode::Down => self.select_next(),
                KeyCode::Up => self.select_previous(),
                KeyCode::Esc => self.select_none(),
                _ => {}
            },
            _ => {}
        }
    }

    fn select_none(&mut self) {
        self.queue.state.select(None);
    }

    fn select_next(&mut self) {
        self.queue.state.select_next();
    }
    fn select_previous(&mut self) {
        self.queue.state.select_previous();
    }

    fn play(&mut self) {
        // self.queue.clear();
        // self.queue.push();
        let index = self.queue.state.selected().unwrap();
        // self.audio_controls.append(self.queue.songs.get(index).unwrap().get_source());
        self.audio_controls.append(
            Decoder::new(BufReader::new(
                File::open(self.queue.songs.get(index).unwrap().path.clone()).unwrap(),
            ))
            .unwrap(),
        );
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

impl<'a> Widget for &mut App<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [queue_area, player_area, tree_area] = Layout::horizontal(Constraint::from_percentages([30, 40, 30]))
        .areas(area);

        self.render_queue(queue_area, buf);
        self.render_tree(tree_area, buf);
        self.render_selected_item(player_area, buf);
    }
}

impl<'a> App<'a> {
    fn render_tree(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
        .borders(Borders::all())
            .title("[T]ree / [ALT]+[Q] to exit")
            .border_type(BorderType::Rounded)
            .border_style(if self.focus==0 {Style::new().green()}else{Style::new().red()});
        
        let mut state = TreeState::default();
        
        let mut items = Vec::new(); // = TreeItem::new_leaf("l", "leaf");
        for song in &dir_to_songs(&"./music/".to_owned()) {
            items.push(TreeItem::new_leaf(song.path.to_string(), song.title.to_string()));
        }
        
        let tree_widget = Tree::new(&items)
            .expect("all item identifiers are unique")
            .block(block);
        StatefulWidget::render(tree_widget, area, buf, &mut state);
    }

    fn render_queue(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title("Playback [q]ueue")
            .borders(Borders::all())
            // .border_style(Style::new().red()); // define the border around the queue
            .border_style(if self.focus==2 {Style::new().green()}else{Style::new().red()});


        let items: Vec<playlist::Song> = self
            .queue
            .songs
            .iter()
            .enumerate()
            .map(|(_, s)| {
                // let color = alternate_colors(i);
                playlist::Song::from((*s).clone())
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
        // let block = Block::new();
        // .title(Line::raw("TODO Info").centered())
        // .borders(Borders::TOP)
        // .border_set(symbols::border::EMPTY)
        // .border_style(TODO_HEADER_STYLE)
        // .bg(NORMAL_ROW_BG)
        // .padding(Padding::horizontal(1));

        // We can now render the item info
        // Paragraph::new(info)
        //     .block(block)
        //     .fg(TEXT_FG_COLOR)
        //     .wrap(Wrap { trim: false })
        //     .render(area, buf);
        // StatefulWidget::render(list, area, buf, &mut self.queue.state);
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
