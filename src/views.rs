use std::path::PathBuf;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph},
    Frame,
};

use crate::app::App;
use crate::types::line::Line;
use crate::types::typing::Typing;

pub enum Theme{
    Dark, 
    Light,
}

impl Theme{
    pub fn new(theme: &str) -> Self{
        match theme {
            "dark" => Theme::Dark,
            "light" => Theme::Light,
            _ => Theme::Dark,
        }
    }

    pub fn fg(&self) -> Color{
        match self{
            Theme::Dark => Color::White,
            Theme::Light => Color::Black,
        }
    }
    
    pub fn bg(&self) -> Color{
        match self{
            Theme::Dark => Color::Reset,
            Theme::Light => Color::Reset,
        }
    }
}

pub fn view<B: Backend>(f: &mut Frame<B>, app: &App, theme: &Theme, file: PathBuf){
    if app.typing.is_finish(){
        let result = app.result();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                  [
                    Constraint::Percentage(10),
                    Constraint::Percentage(70),
                    Constraint::Percentage(20),
                  ]
                .as_ref(),
            )
            .split(f.size());
        f.render_widget(result_view(&app.typing, Borders::BOTTOM, theme), chunks[0]);
        f.render_widget(
            chart_view(app, &result.wpm_plot, &result.acc_plot, &theme),
            chunks[1],
        );
        f.render_widget(help_view(&theme, file), chunks[2]);
    } else if app.typing.is_before_start(){
            let chunks = Layout::default()
                     .direction(Direction::Vertical)
                     .constraints(
                        [
                            Constraint::Percentage(5),
                            Constraint::Percentage(85),
                            Constraint::Percentage(10),
                        ]
                    .as_ref(),
                )
                .split(f.size());
        f.render_widget(time_view(app, theme), chunks[0]);
        f.render_widget(
            lines(
                app.typing.display_lines(),
                app.typing.current_line_index(),
                app.typing.is_error(),
                theme,
            ),
            chunks[1],
        );
        f.render_widget(help_view(&theme, file), chunks[2]);
    } else {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                    [
                    Constraint::Percentage(5),
                    Constraint::Percentage(85),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(f.size());
        f.render_widget(remaining_time_view(&app.typing, theme), chunks[0]);
    }
}
