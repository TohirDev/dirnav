use std::{
    env,
    ffi::OsString,
    fmt::format,
    fs::{self, File, Metadata},
    panic,
    path::PathBuf,
    process::Command,
};

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
    dirs: Vec<PathBuf>,
    lists: ListState,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}

#[allow(unused_must_use)]
fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let path = env::current_dir()?;
    let mut app_state = AppState::default();
    generate_dir(&path, &mut app_state);
    loop {
        terminal.draw(|f| render(f, &mut app_state))?;
        if let Event::Key(k) = event::read()? {
            match k.code {
                event::KeyCode::Esc => {
                    let path = env::current_dir()?;
                    env::set_current_dir(&path);
                    break;
                }
                event::KeyCode::Char(char) => match char {
                    'j' => {
                        app_state.lists.select_next();
                    }
                    'k' => {
                        app_state.lists.select_previous();
                    }
                    'b' => {
                        env::set_current_dir("../")?;
                        let path = env::current_dir()?;
                        generate_dir(&path, &mut app_state);
                        // run(terminal);
                    }
                    'c' => {
                        if let Some(i) = app_state.lists.selected() {
                            // panic!("{:?}", app_state.dirs)
                            let location = &app_state.dirs[i];
                            // let output = Command::new("xdg-open").arg(location).output();
                            env::set_current_dir(location);
                            let path = env::current_dir()?;
                            generate_dir(&path, &mut app_state);
                            // panic!("{:?} {:?}", output, location);
                        }
                        // env::set_current_dir("../")?; let path = env::current_dir()?;
                        // generate_dir(&path, &mut app_state);
                        // run(terminal);
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
            .map(|x| ListItem::from(x.file_name().unwrap().to_string_lossy().to_string())),
    )
    .highlight_symbol(">")
    .highlight_style(Style::default().fg(Color::Green));
    frame.render_stateful_widget(lists, inner_area, &mut state.lists);
}

fn generate_dir(path: &PathBuf, app_state: &mut AppState) -> Result<()> {
    app_state.dirs = vec![];
    let dir_icon = "\u{1F4C1}";
    let file_icon = "\u{1F4C4}";
    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = path.metadata()?;
        let file_name = path.file_name().unwrap();
        if path.is_dir() {
            app_state
                .dirs
                // .push(format!("{} {:?}", dir_icon, file_name.to_owned()));
                .push(path);
        } else if path.is_file() {
            app_state
                .dirs
                // .push(format!("{} {:?}", file_icon, file_name.to_owned()));
                .push(path);
        }
    }
    Ok(())
}
