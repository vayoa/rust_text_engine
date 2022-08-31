use std::{io, thread, time::Duration};
use std::io::Error;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use evalexpr::ValueType::String;
use regex::Regex;
use tui::{backend::CrosstermBackend, Frame, layout::{Constraint, Direction, Layout}, Terminal, widgets::{Block, Borders, Widget}};
use tui::backend::Backend;
use tui::widgets::Paragraph;

use crate::file_format::FileFormat;
use crate::initializer::Initializer;

mod section;
mod character;
mod initializer;
mod file_format;
mod capture;
mod condition;
mod switcher;
mod common;
mod text_input;
mod traits;
mod character_style;


fn main() {
    print!("{}[2J", 27 as char);
    handle_yaml();
    // let _ = setup_terminal();
}

fn setup_terminal() -> Result<(), Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10)
            ].as_ref()
        )
        .split(f.size());
    let block = Block::default()
        .title("Block")
        .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    let paragraph = Paragraph::new("Hey.").block(
        Block::default()
            .title("Block 2")
            .borders(Borders::ALL)
    );
    f.render_widget(paragraph, chunks[1]);
}

fn handle_yaml() {
    const ROOT: &str = r"C:\Users\ew0nd\Documents\DialogGame\story1";
    let mut initializer = Initializer::new(ROOT.to_owned(), FileFormat::Yaml);
    initializer.execute();
}

