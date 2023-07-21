use crate::types::typing::Typing;
use anyhow::Result;
use encoding::all::ISO_8859_1;
use encoding::{DecoderTrap, EncoderTrap, Encoding};
use std::time::Duration;

const SELECTABLE_TIME: [&'static usize; 4] = [&15, &30, &60, &120];

#[derive(Clone, Debug)]
pub struct App{
    pub time: Duration,
    pub typing: Typing,
    progress: TypingProgress,
    custom_time: Duration,
}

#[derive(CLone, Debug)]
pub struct TypingResult{
    pub wpm: usize,
    pub acc: usize,
    pub typed: usize,
    pub typo: usize,
    pub wpm_max: f64,
    pub wpm_plot: Vec<(f64, f64)>,
    pub acc_plot: Vec<(f64, f64)>,   
}

impl App{
    pub fn new(text: &str, remaining_time: Duration, display_lines: usize) -> Result<App>{
            let text = App::filter_text(text);
            let typing = Typing::new(&text, remaining_time, display_lines)?;
            Ok(App{
                typing: typing,
                time: remaining_time,
                custom_time: remaining_time,
                progress: TypingProgress::new(),
        })
    }
}
