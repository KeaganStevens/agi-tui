use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::{error::Error, io, process::exit};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
mod app;
mod fan_data;
mod sqlite;
mod ui;

/// AGI Dashboard Terminal UI
#[derive(Parser, Debug)]
#[clap(author, version, about = None, long_about = None)]
struct Args {
    /// Path to database file
    #[clap(short, long, value_parser)]
    database: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // get database path
    let args = Args::parse();
    let database = args.database;
    // create app and run it
    let app = match app::App::new(database) {
        Ok(good_app) => good_app,
        Err(error) => {
            println!("{error}");
            exit(1);
        }
    };
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, app);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: app::App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Right => app.next(),
                KeyCode::Left => app.previous(),
                _ => {}
            }
        }
    }
}
