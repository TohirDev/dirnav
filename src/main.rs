use std::{env, ffi::OsString, fs};

use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, List, ListItem, ListState, Paragraph, Widget},
};

#[derive(Debug, Default)]
struct AppState {
    dirs: Vec<OsString>,
    lists: ListState,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let path = env::current_dir()?;
    let mut app_state = AppState::default();
    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap();
        if path.is_dir() {
            app_state.dirs.push(file_name.to_owned());
        }
    }

    loop {
        terminal.draw(|f| render(f, &mut app_state))?;
        if let Event::Key(k) = event::read()? {
            match k.code {
                event::KeyCode::Esc => {
                    break;
                }
                event::KeyCode::Char(char) => match char {
                    'j' => {
                        app_state.lists.select_next();
                    }
                    'k' => {
                        app_state.lists.select_previous();
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, state: &mut AppState) -> () {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());
    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(border_area);
    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(Color::Yellow)
        .render(border_area, frame.buffer_mut());
    let lists = List::new(
        state
            .dirs
            .iter()
            .map(|x| ListItem::from(x.to_string_lossy().to_string())),
    )
    .highlight_symbol(">")
    .highlight_style(Style::default().fg(Color::Green));
    frame.render_stateful_widget(lists, inner_area, &mut state.lists);
}
