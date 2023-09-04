use anyhow::{anyhow, Result};
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ignore::Walk;
use rand::prelude::*;
use std::io;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tui::{backend::CrosstermBackend, Terminal};

mod app;
mod reader;
mod types;
mod views;
use crate::views::{view, Theme};
use app::App;
use reader::file::FileReader;
use types::typing::Typing;

const QUIT_COMMAND: char = 'q';
const EXIT_COMMAND: char = 'c';
const RESTART_COMMAND: char = 'r';
const ONE_SEC: Duration = Duration::from_secs(1);

#[derive(Parser, Debug)]
#[clap(author, about, long_about = None, version = "v0.1.0")]
struct Args{
   #[clap(long, default_value_t = 30)]
    time: usize,
  
   #[clap(long, default_value_t = 20)]
    line: usize,

   #[clap(short = 'f', parse(from_os_str), value_name = "file", value_hint = clap::ValueHint::FilePath)]
    file: Option<PathBuf>,

   #[clap(short = 'd', parse(from_os_str), value_name = "dir", value_hint = clap::ValueHint::DirPath)]
    dir: Option<PathBuf>,

   #[clap(short = 'e', long)]
    extension: Option<String>,

   #[clap(short = 't', default_value = "dark")]
    theme: String,
}

fn close_app() -> Result<()> {
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

fn run_app(mut app: App, text: &str, theme: Theme, file: PathBuf) ->io::Result<()>{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| view(f, &app, &theme, file.clone()))?;
        let timeout = ONE_SEC
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)?{
            if let Event::Key(key) = event::read()?{
                match app.typing{
                    Typing::BeforeStart(_) => match key.code{
                        KeyCode::Right =>{
                            app = app.next_time();
                        }
                        KeyCode::Left =>{
                            app = app.prev_time();
                        }
                        KeyCode::Char(QUIT_COMMAND) =>{
                            return Ok(());
                        }
                        KeyCode::Char(EXIT_COMMAND) if key.modifiers == KeyModifiers::CONTROL =>{
                            return Ok(());
                        }
                        KeyCode::Char(c) =>{
                            app = app.start().input(c);
                        }
                        _ => (),
                    },
                    Typing::Running(_) => match key.code{
                        KeyCode::Enter =>{
                            app = app.input('\n');
                        }
                        KeyCode::Char(EXIT_COMMAND) if key.modifiers == KeyModifiers::CONTROL =>{
                            app = app.finish();
                        }
                        KeyCode::Char(c) =>{
                            app = app.input(c);
                        }
                        _ => (),
                    },
                    Typing::Finish(_) => match key.code{
                        KeyCode::Char(RESTART_COMMAND) => app = app.restart(text),
                        KeyCode::Char(QUIT_COMMAND) =>{
                            return Ok(());
                        }
                        KeyCode::Char(EXIT_COMMAND) if key.modifiers == KeyModifiers::CONTROL =>{
                            return Ok(());
                        }
                        _ => (),
                    },
                }
            }
        }
        if last_tick.elapsed() >= ONE_SEC{
            match app.typing{
                Typing::Running(_) =>{
                    app = app.tick();
                    last_tick = Instant::now();
                }
                _ => (),
            }
        }
    }
}

fn start_typing(file: PathBuf, time: Duration, display_line: usize, theme: Theme) -> Result<()>{}
