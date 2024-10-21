use color_eyre::Result;
use crossterm::event::KeyModifiers;
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
    DefaultTerminal,
};
use rodio::{OutputStream, Sink};
mod playlist;
use playlist::{Queue, Song};

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

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
    // audio_controls: Sink,
    focus: i8,
    queue: Queue<'a>,
    should_exit: bool,
}

struct TodoList {
    items: Vec<TodoItem>,
    state: ListState,
}

#[derive(Debug)]
struct TodoItem {
    todo: String,
    info: String,
    status: Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Status {
    Todo,
    Completed,
}

impl<'a> App<'a> {
    fn default(controller: &'a mut Sink) -> Self {
        Self {
            should_exit: false,
            focus: 0,
            queue: Queue::new::<Song>(controller),
        }
    }
}

impl FromIterator<(Status, &'static str, &'static str)> for TodoList {
    fn from_iter<I: IntoIterator<Item = (Status, &'static str, &'static str)>>(iter: I) -> Self {
        let items = iter
            .into_iter()
            .map(|(status, todo, info)| TodoItem::new(status, todo, info))
            .collect();
        let state = ListState::default();
        Self { items, state }
    }
}

impl TodoItem {
    fn new(status: Status, todo: &str, info: &str) -> Self {
        Self {
            status,
            todo: todo.to_string(),
            info: info.to_string(),
        }
    }
}

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
        match key.code {
            KeyCode::Char('a') | KeyCode::Char('A') => self.select_none(),
            // KeyCode::Char('c') | KeyCode::Char('C') => self.clear(),

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

    fn select_none(&mut self) {
        self.queue.state.select(None);
    }

    fn select_next(&mut self) {
        self.queue.state.select_next();
    }
    fn select_previous(&mut self) {
        self.queue.state.select_previous();
    }

    fn select_first(&mut self) {
        self.queue.state.select_first();
    }

    fn select_last(&mut self) {
        self.queue.state.select_last();
    }

    /// Changes the status of the selected list item
    fn toggle_status(&mut self) {
        // if let Some(i) = self.queue.state.selected() {
            // self.queue.songs[i] = match self.queue.songs[i] {
            //     Status::Completed => Status::Todo,
            //     Status::Todo => Status::Completed,
            // }
        // }
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

        let [list_area, item_area] = Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(main_area);

        // App::render_header(tree_area, buf);
        // App::render_footer(footer_area, buf);
        App::render_queue(queue_area, buf);
        self.render_list(list_area, buf);
        self.render_selected_item(item_area, buf);
    }
}

/// Rendering logic for the app
impl App<'_> {
    fn render_queue(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Queue")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_tree(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.")
            .centered()
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("TODO List").centered())
            .borders(Borders::all());
            // .border_set(symbols::border::EMPTY)
            // .border_style(TODO_HEADER_STYLE)
            // .bg(NORMAL_ROW_BG);

        // Iterate through all elements in the `items` and stylize them.
        // let items: Vec<playlist::Song> = self
        //     .queue
        //     .songs
        //     .iter()
        //     .enumerate()
        //     .map(|(i, s)| {
        //         let color = alternate_colors(i);
        //         let song = s;
        //         playlist::Song::from((*song).clone())
        //     })
        //     .collect();

        // Create a List from all list items and highlight the currently selected one
        let mut items: Vec<String> = Vec::new();
        for item in self.queue.songs {
            
            // items.push(item.artist);
        }
        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        // StatefulWidget::render(list, area, buf, &mut self.queue.state);
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        // We get the info depending on the item's state.
        let info = if let Some(i) = self.queue.state.selected() {
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

const fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}

impl From<&TodoItem> for ListItem<'_> {
    fn from(value: &TodoItem) -> Self {
        let line = match value.status {
            Status::Todo => Line::styled(format!(" ☐ {}", value.todo), TEXT_FG_COLOR),
            Status::Completed => {
                Line::styled(format!(" ✓ {}", value.todo), COMPLETED_TEXT_FG_COLOR)
            }
        };
        ListItem::new(line)
    }
}
