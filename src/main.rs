use std::ops::ControlFlow;

use color_eyre::Result;
use crossterm::event::KeyModifiers;
use ratatui::{
    crossterm::event::{ self, KeyCode },
    layout::{ Constraint, Direction, Layout },
    widgets::Paragraph,
    DefaultTerminal, Frame,
};
use rodio::{OutputStream, Sink};
// mod components;
mod playlist;
// pub use components::playlist::Queue;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal: ratatui::Terminal<ratatui::prelude::CrosstermBackend<std::io::Stdout>> = ratatui::init();
    let app_result: std::result::Result<(), color_eyre::eyre::Error> = run(terminal);
    ratatui::restore();
    app_result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let audio_controls: &mut Sink = &mut Sink::try_new(&OutputStream::try_default().unwrap().1).unwrap();
    loop {
        terminal.draw(|frame: &mut Frame<'_>| draw(frame, audio_controls))?;

        if let event::Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') && key.modifiers == KeyModifiers::ALT { // no clue why but it only works this way if you want CTRL shortcuts to kill the program
                audio_controls.stop();
                return Ok(());
            }
            if handle_key_event(key).is_break() {
                break;
            }
        }
    }
    Ok(())
}

fn draw(frame: &mut Frame, audio_controls: &mut Sink ) {
    let footer_chunks: std::rc::Rc<[ratatui::prelude::Rect]> = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(50),
            Constraint::Percentage(30)
        ])
        .split(frame.area());

        frame.render_widget(
        Paragraph::new("Playlist/album etc.")
        .centered(),
        footer_chunks[0]
    );
    frame.render_widget(Paragraph::new("music player\n- album cover\n- metadata etc."), footer_chunks[1]);
    frame.render_widget(playlist::Queue::new::<String>(audio_controls), footer_chunks[2]);

}

fn handle_key_event(
    key: event::KeyEvent
) -> ControlFlow<()> {
    if key.code == KeyCode::Char('q') && key.modifiers == KeyModifiers::SHIFT {
        return ControlFlow::Break(())
    }
    // match key.code {

    //     _ => (),
    // }
    ControlFlow::Continue(())
}