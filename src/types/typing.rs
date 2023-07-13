use crate::types::line::Line;
use anyhow::{anyhow, Result};
use std::cmp;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub enum Typing{
    BeforeStart(State),
    Running(State),
    Finish(State),
}

#[derive(Clone, Debug)]
pub struct State{
    current_index: usize,
    display_lines: usize,
    end_time: Option<std::time::Instant>,
    is_error: bool,
    lines: Vec<Line>,
    remaining_time: Duration,
    start_time: Option<std::time::Instant>,
    typed : usize,
    typo: usize,
}

impl Typing{}
