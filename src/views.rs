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
				theme,
			),
			chunks[1],
		);
		f.render_widget(result_view(&app.typing, Borders::TOP, theme), chunks[2]);
	}
}

pub fn chart_view<'a>(
	app: &App,
	wpm_dataset: &'a Vec<(f64, f64)>,
	acc_dataset: &'a Vec<(f64, f64)>,
	theme: &Theme,
) -> Chart<'a> {
	let elapsed_time = app.elapsed_time();
	let result = app.result();

	Chart::new(vec![
		Dataset::default()
			.name("wpm")
			.marker(symbols::Marker::Dot)
			.graph_type(GraphType::Line)
			.style(Style::default().bg(theme.bg()).fg(Color::Yellow))
			.data(&wpm_dataset),
		Dataset::default()
			.name("acc")
			.marker(symbols::Marker::Dot)
			.graph_type(GraphType::Line)
			.style(Style::default().bg(theme.bg()).fg(Color::DarkGray))
			.data(&acc_dataset),
	])
	.style(Style::default().bg(theme.bg()).fg(theme.fg()))
	.block(Block::default().style(Style::default().bg(theme.bg()).fg(theme.fg())))
	.x_axis(
		Axis::default()
			.style(Style::default().bg(theme.bg()).fg(Color::DarkGray))
			.labels(vec![
				Span::styled("0", Style::default().fg(Color::DarkGray)),
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
	.y_axis(
		Axis::default()
			.style(Style::default().bg(theme.bg()).fg(theme.fg()))
			.labels(vec![
				Span::styled("0", Style::default().fg(Color::DarkGray)),
				Span::styled(
					(result.wpm_max / 2.0).floor().to_string(),
					Style::default().bg(theme.bg()).fg(Color::DarkGray),
				),
				Span::styled(
					result.wpm_max.to_string(),
					Style::default().bg(theme.bg()).fg(Color::DarkGray),
				),
			])
			.bounds([0.0, result.wpm_max]),
	)
}

fn help_view<'a>(theme: &Theme, path: PathBuf) -> Paragraph<'a> {
	let file_path = Spans::from(Span::styled(
		path.into_os_string().into_string().unwrap(),
		Style::default().bg(theme.bg()).fg(Color::DarkGray),
	));
	let help = Spans::from(vec![
		Span::styled(
			"r",
			Style::default()
				.bg(theme.bg())
				.fg(Color::Yellow)
				.add_modifier(Modifier::BOLD),
		),
		Span::styled(
			" to restart",
			Style::default().bg(theme.bg()).fg(Color::DarkGray),
		),
		Span::styled(", ", Style::default().bg(theme.bg()).fg(Color::DarkGray)),
		Span::styled(
			"q",
			Style::default()
				.bg(theme.bg())
				.fg(Color::Red)
				.add_modifier(Modifier::BOLD),
		),
		Span::styled(
			" to quit",
			Style::default().bg(theme.bg()).fg(Color::DarkGray),
		),
		Span::styled(", ", Style::default().bg(theme.bg()).fg(Color::DarkGray)),
		Span::styled(
			"left, right",
			Style::default()
				.bg(theme.bg())
				.fg(Color::Green)
				.add_modifier(Modifier::BOLD),
		),
		Span::styled(
			" to select a time",
			Style::default().bg(theme.bg()).fg(Color::DarkGray),
		),
	]);
	Paragraph::new(vec![help, file_path])
		.style(Style::default().bg(theme.bg()).fg(theme.fg()))
		.block(
			Block::default()
				.borders(Borders::TOP)
				.style(Style::default().bg(theme.bg()).fg(theme.fg())),
		)
		.alignment(Alignment::Left)
}

fn remaining_time_view<'a>(typing: &Typing, theme: &Theme) -> Paragraph<'a> {
	let time = Spans::from(Span::styled(
		typing.get_remaining_time().to_string(),
		Style::default()
			.bg(theme.bg())
			.fg(Color::Green)
			.add_modifier(Modifier::BOLD),
	));
	Paragraph::new(vec![time])
		.style(Style::default().bg(theme.bg()).fg(theme.fg()))
		.alignment(Alignment::Left)
}

fn result_view<'a>(typing: &Typing, border: Borders, theme: &Theme) -> Paragraph<'a> {
	let result = Spans::from(vec![
		Span::styled(
			"wpm: ",
			Style::default().bg(Theme::bg(&theme)).fg(Color::DarkGray),
		),
		Span::styled(
			typing.wpm().to_string(),
			Style::default().bg(Theme::bg(&theme)).fg(Color::Yellow),
		),
		Span::styled(
			" acc: ",
			Style::default().bg(Theme::bg(&theme)).fg(Color::DarkGray),
		),
		Span::styled(
			typing.acc().to_string() + "%",
			Style::default().bg(Theme::bg(&theme)).fg(Color::Gray),
		),
		Span::styled(
			" key: ",
			Style::default().bg(Theme::bg(&theme)).fg(Color::DarkGray),
		),
		Span::styled(
			(typing.typed() + typing.typo()).to_string(),
			Style::default().bg(Theme::bg(&theme)).fg(Color::Gray),
		),
		Span::styled("/", Style::default().bg(Theme::bg(&theme)).fg(Color::Gray)),
		Span::styled(
			(typing.typo()).to_string(),
			Style::default().bg(Theme::bg(&theme)).fg(Color::Red),
		),
	]);
	Paragraph::new(vec![result])
		.style(Style::default().bg(Theme::bg(&theme)).fg(Theme::fg(&theme)))
		.block(
			Block::default()
				.borders(border)
				.style(Style::default().bg(Theme::bg(&theme)).fg(Theme::fg(&theme))),
		)
		.alignment(Alignment::Left)
}

fn time_view<'a>(app: &App, theme: &Theme) -> Paragraph<'a> {
	let times: Vec<Span> = app
		.selectable_time()
		.iter()
		.map(|t| {
			Span::styled(
				format!("{} ", t.as_secs().to_string()),
				if app.time == *t {
					Style::default()
						.bg(theme.bg())
						.fg(Color::Yellow)
						.add_modifier(Modifier::BOLD)
				} else {
					Style::default().bg(theme.bg()).fg(Color::DarkGray)
				},
			)
		})
		.collect();
	let result = Spans::from(times);
	Paragraph::new(vec![result])
		.alignment(Alignment::Left)
		.block(Block::default().style(Style::default().bg(theme.bg()).fg(theme.fg())))
}

fn lines<'a>(
	lines: Vec<Line>,
	current_line_index: usize,
	is_typing_error: bool,
	theme: &Theme,
) -> Paragraph<'a> {
	let text: Vec<Spans<'a>> = lines
		.iter()
		.map(|l| line(l.clone(), current_line_index, is_typing_error, theme))
		.collect();
	Paragraph::new(text)
		.style(Style::default().bg(theme.fg()).fg(theme.bg()))
		.block(Block::default().style(Style::default().bg(theme.bg()).fg(theme.fg())))
		.alignment(Alignment::Left)
}

fn line<'a>(
	line: Line,
	current_line_index: usize,
	is_typing_error: bool,
	theme: &Theme,
) -> Spans<'a> {
	if line.line_no() - 1 == current_line_index {
		let entered = Span::styled(
			line.entered_text().unwrap_or("".to_owned()),
			Style::default().bg(theme.bg()).fg(Color::Green),
		);
		let current = if is_typing_error {
			Span::styled(
				line.current_text()
					.map(String::from)
					.unwrap_or("".to_owned()),
				Style::default()
					.bg(Color::Red)
					.fg(Color::White)
					.add_modifier(Modifier::SLOW_BLINK),
			)
		} else {
			Span::styled(
				line.current_text()
					.map(String::from)
					.unwrap_or("".to_owned()),
				Style::default()
					.bg(Color::Green)
					.fg(Color::White)
					.add_modifier(Modifier::BOLD)
					.add_modifier(Modifier::SLOW_BLINK),
			)
		};
		let rest = Span::styled(
			line.rest_text().unwrap_or("".to_owned()),
			Style::default().bg(theme.bg()).fg(theme.fg()),
		);
		Spans::from(vec![entered, current, rest])
	} else if line.line_no() - 1 > current_line_index {
		let entered = Span::styled(
			line.entered_text().unwrap_or("".to_owned()),
			Style::default().bg(theme.bg()).fg(Color::Green),
		);
		let current = Span::styled(
			line.current_text()
				.map(String::from)
				.unwrap_or("".to_owned()),
			Style::default().bg(theme.bg()).fg(Color::DarkGray),
		);
		let rest = Span::styled(
			line.rest_text().unwrap_or("".to_owned()),
			Style::default().bg(theme.bg()).fg(Color::DarkGray),
		);
		Spans::from(vec![entered, current, rest])
	} else {
		let entered = Span::styled(
			line.entered_text().unwrap_or("".to_owned()),
			Style::default().bg(theme.bg()).fg(Color::Green),
		);
		let current = Span::styled(
			line.current_text()
				.map(String::from)
				.unwrap_or("".to_owned()),
			Style::default().bg(theme.bg()).fg(Color::Green),
		);
		let rest = Span::styled(
			line.rest_text().unwrap_or("".to_owned()),
			Style::default().bg(theme.bg()).fg(Color::DarkGray),
		);
		Spans::from(vec![entered, current, rest])
	}
}
