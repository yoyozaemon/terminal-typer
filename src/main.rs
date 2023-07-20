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

