use std::{io};
mod img_to_ascii;
mod player;

use player::Button;
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind}, style::{Stylize}, text::{Text}, DefaultTerminal
};

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(|frame: &mut ratatui::Frame<'_>| {
            // let player: Text<'_> = Text::raw("Volume: ").light_green();
            
            // frame.render_widget(

            // );
        })?;
        
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}

// fn main() {
fn main() -> io::Result<()> {
    let mut terminal: ratatui::Terminal<ratatui::prelude::CrosstermBackend<io::Stdout>> = ratatui::init();
    terminal.clear()?;
    let app_result: Result<(), io::Error> = run(terminal);
    ratatui::restore();
    app_result
}

// fn main() {
//     let paths = fs::read_dir("./").unwrap();
//     for path in paths {
//         println!("{}", to_ascii(path.unwrap().path().display().to_string(), 150))
//     }
// }