use std::rc::Rc;

use ratatui::{layout::{Constraint, Direction, Layout, Rect}, Frame};

fn get_layout(f: &Frame) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(100 - 60),
        ])
        .split(f.area())
}