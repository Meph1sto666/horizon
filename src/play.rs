use std::{io};
use crossterm::event::KeyModifiers;
mod img_to_ascii;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind}, style::{Stylize}, text::{Text}, DefaultTerminal
};

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
	let file = BufReader::new(File::open("./vowl-the-end.mp3").unwrap());
	let source = Decoder::new(file).unwrap();
	let (_stream, _stream_handle) = OutputStream::try_default().unwrap();
	let control: Sink = Sink::try_new(&_stream_handle).unwrap();
	control.append(source);

    loop {
        terminal.draw(|frame: &mut ratatui::Frame<'_>| {
            // let player: Text<'_> = Text::raw("Volume: ").light_green();
            
            let vol: i8 = (control.volume()*100.) as i8;
            frame.render_widget(
                Text::raw(
                    format!("Volume: {vol}")
            ).light_green(), frame.area());
            
        })?;
        
        if let event::Event::Key(key) = event::read()? {
            let volume: f32 = control.volume();
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                control.stop();
                return Ok(());
            }
            else if key.kind == KeyEventKind::Press && key.code == KeyCode::Char(',') {
                if volume <= 0. { control.set_volume(0.); continue };
                control.set_volume(volume - (if key.modifiers == KeyModifiers::ALT {0.01} else {0.05}));
            }
            else if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('.') {
                if volume >= 1. { control.set_volume(1.); continue };
                control.set_volume(volume + (if key.modifiers == KeyModifiers::ALT {0.01} else {0.05}));
            }
            else if key.kind == KeyEventKind::Press && key.code == KeyCode::Char(' ') {
                if control.is_paused() { control.play() }
                else {control.pause();}
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