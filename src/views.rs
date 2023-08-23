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

pub enum Theme {
    Dark,
    Light,
}

impl Theme {
    pub fn new(theme: &str) -> Self {
        match theme {
            "dark" => Theme::Dark,
            "light" => Theme::Light,
            _ => Theme::Dark,
        }
    }

    pub fn fg(&self) -> Color {
        match self {
            Theme::Dark => Color::White,
            Theme::Light => Color::Black,
        }
    }

    pub fn bg(&self) -> Color {
        match self {
            Theme::Dark => Color::Reset,
            Theme::Light => Color::Reset,
        }
    }
}

pub fn view<B: Backend>(f: &mut Frame<B>, app: &App, theme: &Theme, file: PathBuf) {
    if app.typing.is_finish() {
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
    } else if app.typing.is_before_start() {
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
        f.render_widget(
            lines(
                app.typing.display_lines(),
                app.typing.current_line_index(),
                app.typing.is_error(),
            ),
            chunks[1],
        );
        f.render_widget(result_view(&app.typing, Borders::TOP, theme),chunks[2]);
    }
}

pub fn chart_view<'a>(
    app: &App,
    wpm_dataset: &'a Vec<(f64, f64)>,
    acc_dataset: &'a Vec<(f64, f64)>,
    theme: &Theme,
) -> Chart<'a>{
    let elapsed_time = app.elapsed_time();
    let result = app.result();

    Chart::new(vec![
        Dataset::default
                .name("wpm")
                .marker(symbols::Marker::Dot)
                .graph_type(GraphType::Line)
                .style(Style::default().bg(theme.bg()).fg(Color::Yellow))
                .data(&wpm_dataset),
    ])
    .style(Style::default().bg(theme.bg()).fg(theme.fg()))
    .block(Block::default().style(Style::default().bg(theme.bg()).fg(theme.fg())))
    .x_axis(
            Axis::default()
                .style(Style::default().bg(theme.bg()).fg(Color::DarkGray))
                .labels(vec![
                        Span::styled("0",Style::default().fg(Color::DarkGray)),
                        Span::styled(
                                    (elapsed_time.as_secs() / 2).to_string(),
                                    Style::default().fg(Color::DarkGray),
                    ),
                        Span::styled(
                                    elapsed_time.as_secs().to_string(),
                                    Style::default().bg(theme.bg()).fg(theme.fg()),
                    ),
                ])
                .bounds([0.0, elapsed_time.as_secs_f64()]),
        )
    .y_axis()
}
