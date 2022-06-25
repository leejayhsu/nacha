use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::{
        Axis, BarChart, Block, Borders, Cell, Chart, Dataset, Gauge, LineGauge, List, ListItem,
        Paragraph, Row, Sparkline, Table, Tabs, Wrap,
    },
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(8),
                Constraint::Min(8),
                Constraint::Length(8),
            ]
            .as_ref(),
        )
        .split(f.size());
    draw_text(f, chunks[0], app);
}

fn draw_text<B>(f: &mut Frame<B>, area: Rect, app: &mut App)
where
    B: Backend,
{
    let text = vec![
        Spans::from(vec![
            Span::styled(
                "date created : ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::from(format!(
                "{:#?}",
                app.nacha_file.file_header.file_creation_date
            )),
        ]),
        Spans::from(vec![
            Span::styled(
                "time created : ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::from(format!(
                "{:#?}",
                app.nacha_file.file_header.file_creation_time
            )),
        ]),
        Spans::from(vec![
            Span::styled(
                "origin       : ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::from(format!(
                "{} ({})",
                app.nacha_file.file_header.immediate_origin_name,
                app.nacha_file.file_header.immediate_origin,
            )),
        ]),
        Spans::from(vec![
            Span::styled(
                "destination  : ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::from(format!(
                "{} ({})",
                app.nacha_file.file_header.immediate_destination_name,
                app.nacha_file.file_header.immediate_destination,
            )),
        ]),
    ];
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "File Header",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}
