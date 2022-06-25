use crate::app::App;
use crate::lib::DetailEntry;
use thousands::Separable;
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
        .constraints([Constraint::Length(7), Constraint::Min(2)].as_ref())
        .split(f.size());
    draw_file_metadata(f, chunks[0], app);
    draw_file_contents(f, chunks[1], app)
}

fn draw_file_metadata<B>(f: &mut Frame<B>, area: Rect, app: &mut App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .direction(Direction::Horizontal)
        .split(area);

    // #######################################
    // ########  draw file header  ###########
    // #######################################
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
    f.render_widget(paragraph, chunks[0]);

    // ########################################
    // ########  draw file control  ###########
    // ########################################
    let text = vec![
        Spans::from(vec![
            Span::styled(
                "batch count         : ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::from(format!("{}", app.nacha_file.file_control.batch_count)),
        ]),
        Spans::from(vec![
            Span::styled(
                "block count         : ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::from(format!("{}", app.nacha_file.file_control.block_count)),
        ]),
        Spans::from(vec![
            Span::styled(
                "entry/addenda count : ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::from(format!(
                "{}",
                app.nacha_file.file_control.entry_and_addenda_count,
            )),
        ]),
        Spans::from(vec![
            Span::styled(
                "total debit         : ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::from(format!(
                "{}",
                (app.nacha_file.file_control.total_debit as f32 / 100.0).separate_with_commas(),
            )),
        ]),
        Spans::from(vec![
            Span::styled(
                "total credit        : ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::from(format!(
                "{}",
                (app.nacha_file.file_control.total_credit as f32 / 100.0).separate_with_commas(),
            )),
        ]),
    ];
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "File Control",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[1]);
}

fn draw_file_contents<B>(f: &mut Frame<B>, area: Rect, app: &mut App)
where
    B: Backend,
{
    // let text = vec![Spans::from(
    //     app.nacha_file.batches[0].detail_entries[0].as_json(),
    // )];

    (1..10).filter(|x| x % 2 == 0).collect::<Vec<u32>>();

    // let mut entries: Vec<DetailEntry> = Vec::new();
    // // let slice = &app.nacha_file.batches[..];
    // for batch in std::iter::IntoIterator::into_iter(app.nacha_file.batches) {
    //     for entry in batch.detail_entries {
    //         entries.push(entry);
    //     }
    // }
    let entries = app.nacha_file.get_entries();
    // let amounts: Vec<i32> = entries.iter().map(|x| x.amount).collect();
    // let names: Vec<String> = entries.iter().map(|x| x.individual_name).collect();
    // let trace_numbers: Vec<String> = entries.iter().map(|x| x.trace_number).collect();
    // let transaction_codes: Vec<String> = entries.iter().map(|x| x.transaction_code).collect();
    // let dfi_account_numbers: Vec<String> = entries.iter().map(|x| x.dfi_account_number).collect();

    let items: Vec<Row> = entries
        .iter()
        .map(|e| {
            let cells = vec![
                Cell::from(Span::raw(format!("{}", e.transaction_code))),
                Cell::from(Span::raw(format!("{}", e.individual_name))),
                Cell::from(Span::raw(format!("{}", e.dfi_account_number))),
                Cell::from(Span::raw(format!("{}", e.trace_number))),
                Cell::from(Span::raw(format!("{}", e.amount.separate_with_commas()))),
            ];
            Row::new(cells)
        })
        .collect();
    let table = Table::new(items)
        .block(Block::default().title("Colors").borders(Borders::ALL))
        .widths(&[
            Constraint::Ratio(1, 6),
            Constraint::Ratio(1, 6),
            Constraint::Ratio(1, 6),
            Constraint::Ratio(1, 6),
            Constraint::Ratio(1, 6),
        ]);
    f.render_widget(table, area);
}
