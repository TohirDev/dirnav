use std::{
    env,
    fs::{self},
    path::PathBuf,
    process::Command,
};

use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, List, ListItem, ListState, Widget},
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
                    }
                    'f' => {
                        if let Some(i) = app_state.lists.selected() {
                            let location = &app_state.dirs[i];
                            env::set_current_dir(location);
                            let path = env::current_dir()?;
                            generate_dir(&path, &mut app_state);
                        }
                    }
                    'z' => {
                        Command::new("zed")
                            .arg(".")
                            .spawn()
                            .expect("zed command failed");
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
        .fg(Color::White)
        .render(border_area, frame.buffer_mut());
    let lists = List::new(state.dirs.iter().map(|x| {
        ListItem::style(
            ListItem::from(x.file_name().unwrap().to_string_lossy()),
            Color::White,
        )
    }))
    .highlight_symbol("->")
    .highlight_style(Style::default().fg(Color::Green));
    frame.render_stateful_widget(lists, inner_area, &mut state.lists);
    // frame.render_widget(Block::bordered().title("「 ✦ dir_nav 🦀 ✦ 」"), inner_area);
}

fn generate_dir(path: &PathBuf, app_state: &mut AppState) -> Result<()> {
    app_state.dirs = vec![];
    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            app_state.dirs.push(path);
        } else if path.is_file() {
            app_state.dirs.push(path);
        }
    }
    Ok(())
}
