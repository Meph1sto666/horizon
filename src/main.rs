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
use playlist::{Queue, Song};
use tui_tree_widget::{Tree, TreeState};

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const FOCUS_ID_QUEUE: i8 = 0;
const FOCUS_ID_PLAYER: i8 = 1;
const FOCUS_ID_TREE: i8 = 2;
// const FOCUS_BLOCK_BORDER_STYLE: Style = Style::new().color(ratatui::style::palette::tailwind::LIME);
// const BLOCK_BORDER_STYLE: Style = Style::new().gray();

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result =
        App::default(
            &mut Sink::try_new(&OutputStream::try_default().unwrap().1).unwrap(),
            &mut TreeState::default()
        ).run(terminal);
    ratatui::restore();
    app_result
}

struct App<'a> {
    pub audio_controls: &'a Sink,
    pub focus: i8,
    pub queue: Queue,
    pub should_exit: bool,
    pub library: Library,
    pub tree_state: &'a mut TreeState<String>
}

impl<'a> App<'a> {
    fn default(controller: &'a mut Sink, tree_state: &'a mut TreeState<String>) -> Self {
        Self {
            should_exit: false,
            focus: 0,
            queue: Queue::new::<Song>(),
            library: Library::new(),
            tree_state: tree_state,
            audio_controls: controller,
        }
    }
}

impl<'a> App<'a> {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.library.update_tree_entries();
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
            KeyCode::Char('t') => self.focus = FOCUS_ID_TREE,
            KeyCode::Char('p') => self.focus = FOCUS_ID_PLAYER,
            KeyCode::Char('q') => self.focus = FOCUS_ID_QUEUE,
            _ => {}
        }

        match self.focus {
            FOCUS_ID_QUEUE => match key.code {
                KeyCode::Enter => self.play(),
                KeyCode::Char(' ') => self.toggle_playback(),
                KeyCode::Down => self.select_next(),
                KeyCode::Up => self.select_previous(),
                KeyCode::Esc => self.select_none(),
                _ => {}
            },
            FOCUS_ID_TREE => match key.code {
                KeyCode::Down => {self.tree_state.key_down();},
                KeyCode::Up => {self.tree_state.key_up();},
                KeyCode::Right => {self.tree_state.key_right();},
                KeyCode::Left => {self.tree_state.key_left();},
                KeyCode::Enter => {
                    let selected = self.tree_state.selected().get(0);
                    // self.library.tree_entries
                    print!("{}", selected.unwrap())
                    // self.queue.push_from_path(selected.unwrap());
                },
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
        let index = self.queue.state.selected().unwrap();
        // self.queue.go_to(index);
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
        let [queue_area, _player_area, tree_area] = Layout::horizontal(Constraint::from_percentages([30, 40, 30]))
        .areas(area);

        self.render_queue(queue_area, buf);
        self.render_tree(tree_area, buf);
        // self.render_selected_item(player_area, buf);
    }
}

impl<'a> App<'a> {
    fn render_tree(&mut self, area: Rect, buf: &mut Buffer) {
        let block: Block<'_> = Block::new()
        .borders(Borders::all())
            .title("[T]ree / [ALT]+[Q] to exit")
            .border_type(BorderType::Rounded)
            .border_style(if self.focus==FOCUS_ID_TREE {Style::new().green()}else{Style::new().red()});

        let tree_widget = Tree::new(&self.library.tree_entries)
            .expect("all item identifiers are unique")
            .highlight_style(SELECTED_STYLE)
            .block(block);
        StatefulWidget::render(tree_widget, area, buf, self.tree_state);
    }

    fn render_queue(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title("Playback [q]ueue")
            .borders(Borders::all())
            .border_style(if self.focus==FOCUS_ID_QUEUE {Style::new().green()}else{Style::new().red()});

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

    // fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
    //     let info = if let Some(_) = self.queue.state.selected() {
    //         "Nothing selected...".to_string()
    //     } else {
    //         "Nothing selected...".to_string()
    //     };
    // }
}
