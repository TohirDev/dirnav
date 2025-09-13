use std::{
    env,
    ffi::{OsStr, OsString},
    fs,
};

use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
    layout::{Constraint, Layout},
    style::{Color, Stylize},
    widgets::{Block, BorderType, List, ListItem, Paragraph, Widget},
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let path = env::current_dir()?;
    let mut dirs = vec![];
    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap();
        if path.is_dir() {
            dirs.push(file_name.to_owned());
        }
    }

    loop {
        terminal.draw(|f| render(f, &dirs))?;
        if let Event::Key(k) = event::read()? {
            match k.code {
                event::KeyCode::Esc => {
                    break;
                }
                // event::KeyCode::Char(char) => match char {
                //     ""
                // },
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, list: &Vec<OsString>) -> () {
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
        list.iter()
            .map(|x| ListItem::from(x.to_string_lossy().to_string())),
    )
    .render(inner_area, frame.buffer_mut());
    // frame.render_stateful_widget(&lists, inner_area, & list);
}

// let path = env::current_dir()?;
// for entry in fs::read_dir(&path)? {
//     let entry = entry?;
//     let path = entry.path();

//     if path.is_dir() {
//         println!("Dir found: {:?}", path.file_name().unwrap())
//     } else if path.is_file() {
//         println!("File found: {:?}", path.file_name().unwrap())
//     }
// }
// println!("{:?}", fs::read_dir(&path));
