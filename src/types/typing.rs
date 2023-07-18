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

impl Typing{
    pub fn new(text: &str, remaining_time: Duration, display_lines: usize) -> Result<Self>{
        if text.is_empty(){
            Err(anyhow!("text is empty"))
        } else{
            let lines: Vec<Line> = text
                    .split("\n")
                    .enumerate()
                    .map(|(i, v)| Line::new(i + 1, &v.to_string()))
                    .collect();
            Ok(Typing::BeforeStart(State{
                    current_index: 0,
                    lines: lines.clone(),
                    start_time: None,
                    end_time: None,
                    remaining_time: remaining_time,
                    typed: 0,
                    typo: 0,
                    is_error: false,
                    display_lines: display_lines,
            })) 
        }
    }

    pub fn restart(&self, text: &str, remaining_time: Duration) -> Self{
        match self{
            Typing::Finish(s) => Typing::BeforeStart(State{
                    current_index: 0,
                    lines: Typing::to_lines(text),
                    start_time: None,
                    end_time: None,
                    remaining_time: remaining_time,
                    typed: 0,
                    typo: 0,
                    ..s.clone()
            }),
            Typing::Running(s) => Typing::Running(s.clone()),
            Typing::BeforeStart(s) => Typing::BeforeStart(s.clone()),
        }
    }

    pub fn start(&self) -> Self{
        match self{
            Typing::BeforeStart(s) => Typing::Running(State{
                    start_time: Some(Instant::now()),
                    ..s.clone()
            }),
            Typing::Running(t) => Typing::Running(t.clone()),
            Typing::Finish(t) => Typing::Finish(t.clone(),)
        }
    }
}
