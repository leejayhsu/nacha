use crate::app::{App, StatefulTable};
use crate::lib::{Addendum, Currency, DetailEntry};
use crate::term::DetailEntryWithCounter;
use std::cmp::Ordering;
use thousands::Separable;
use tui::text::Text;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, Wrap},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(7),
                Constraint::Min(2),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());
    draw_file_metadata(f, chunks[0], app);
    draw_file_contents(f, chunks[1], app);
    draw_shortcut_help(f, chunks[2]);
}

fn draw_shortcut_help<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let text = vec![Spans::from(vec![
        Span::styled("j/<down>", Style::default().fg(Color::Cyan)),
        Span::from(": next"),
        Span::raw("  "),
        Span::styled("k/<up>", Style::default().fg(Color::Cyan)),
        Span::raw(": prev"),
        Span::raw("  "),
        Span::styled("l/<right>", Style::default().fg(Color::Cyan)),
        Span::raw(": jump next"),
        Span::raw("  "),
        Span::styled("h/<left>", Style::default().fg(Color::Cyan)),
        Span::raw(": jump prev"),
        Span::raw("  "),
        Span::styled("o", Style::default().fg(Color::Cyan)),
        Span::raw(": toggle addenda"),
        Span::raw("  "),
        Span::styled("q", Style::default().fg(Color::Cyan)),
        Span::raw(": quit"),
    ])];
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Keyboard Shortcuts",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
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
                "{}",
                match app.nacha_file.file_header.file_creation_date {
                    Some(d) => d.to_string(),
                    None => "no date provided".to_string(),
                }
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
                "{}",
                match app.nacha_file.file_header.file_creation_time {
                    Some(t) => format!("{}", t.format("%H:%M")),
                    None => "no time provided".to_string(),
                }
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
            Span::styled(
                format!(
                    "{:>16}",
                    app.nacha_file
                        .file_control
                        .total_debit
                        .pretty_dollars_cents(),
                ),
                Style::default().fg(Color::Red),
            ),
        ]),
        Spans::from(vec![
            Span::styled(
                "total credit        : ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(
                    "{:>16}",
                    app.nacha_file
                        .file_control
                        .total_credit
                        .pretty_dollars_cents()
                ),
                Style::default().fg(Color::Green),
            ),
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
    let entries = &app.entries;
    let header_cells = make_header();
    let header = Row::new(header_cells);

    let items: Vec<Row> = app
        .entries
        .items
        .iter()
        .map(|e| {
            let cells = parse_entry_into_cells(e);
            Row::new(cells)
        })
        .collect();

    let table = Table::new(items)
        .block(
            Block::default()
                .title(Span::styled(
                    "File Contents",
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ")
        .header(header)
        .widths(&[
            Constraint::Ratio(6, 100),
            Constraint::Ratio(6, 100),
            Constraint::Ratio(20, 100),
            Constraint::Ratio(12, 100),
            Constraint::Ratio(15, 100),
            Constraint::Ratio(10, 100),
            Constraint::Ratio(6, 100),
        ]);
    f.render_stateful_widget(table, area, &mut app.entries.state);

    if app.show_popup {
        let block = Block::default().title("Popup").borders(Borders::ALL);
        let area = centered_rect(95, 50, f.size());
        let text = vec![Spans::from(vec![Span::raw(
            "display stateful table of addenda here",
        )])];
        // todo: add counter to addenda items
        let i = match app.entries.state.selected() {
            Some(i) => {
                let addenda_items: Vec<Row> = app
                    .addenda_popup
                    .items
                    .iter()
                    .map(|e| {
                        let cells = parse_addendum_into_cells(e);
                        Row::new(cells)
                    })
                    .collect();

                let addenda_table = Table::new(addenda_items)
                    .block(
                        Block::default()
                            .title(Span::styled(
                                "Addenda Info",
                                Style::default()
                                    .fg(Color::Magenta)
                                    .add_modifier(Modifier::BOLD),
                            ))
                            .borders(Borders::ALL),
                    )
                    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                    .highlight_symbol("> ")
                    .header(Row::new(make_addenda_header()))
                    .widths(&[
                        Constraint::Ratio(8, 100),
                        Constraint::Ratio(65, 100),
                        Constraint::Ratio(10, 100),
                        Constraint::Ratio(10, 100),
                    ]);
                f.render_widget(Clear, area); // this clears out the background
                f.render_stateful_widget(addenda_table, area, &mut app.addenda_popup.state);
            }
            None => {}
        };
    }
}

fn parse_addendum_into_cells(a: &Addendum) -> Vec<Cell<'static>> {
    let cells = vec![
        Cell::from(Span::styled(
            format!("{}", a.addenda_type_code),
            Style::default().fg(Color::Reset),
        )),
        Cell::from(Span::styled(
            format!("{}", a.payment_related_info),
            Style::default().fg(Color::Reset),
        )),
        Cell::from(Span::styled(
            format!("{}", a.addenda_sequence_number),
            Style::default().fg(Color::Reset),
        )),
        Cell::from(Span::styled(
            format!("{}", a.entry_detail_sequence_number),
            Style::default().fg(Color::Reset),
        )),
    ];
    return cells;
}

fn parse_entry_into_cells(e: &DetailEntryWithCounter) -> Vec<Cell<'static>> {
    let code = &e.entry.transaction_code[..];
    let color = match code {
        "22" | "32" | "42" | "52" => Color::Green,
        "27" | "37" | "47" => Color::Red,
        _ => Color::Reset,
    };
    let cells = vec![
        Cell::from(Span::styled(
            format!("{}", e.counter),
            Style::default().fg(Color::Reset),
        )),
        Cell::from(Span::styled(
            format!("{}", e.entry.transaction_code),
            Style::default().fg(color),
        )),
        Cell::from(Span::styled(
            format!("{}", e.entry.individual_name),
            Style::default().fg(Color::Reset),
        )),
        Cell::from(Span::styled(
            format!("{}", e.entry.dfi_account_number),
            Style::default().fg(Color::Reset),
        )),
        Cell::from(Span::styled(
            format!("{}", e.entry.trace_number),
            Style::default().fg(Color::Reset),
        )),
        Cell::from(Span::styled(
            format!("{:>13}", e.entry.amount.pretty_dollars_cents()),
            Style::default().fg(color),
        )),
        Cell::from(Span::styled(
            format!(
                "{:^8}",
                match e.entry.addenda.len().cmp(&0) {
                    Ordering::Less => "".to_string(),
                    Ordering::Equal => "".to_string(),
                    Ordering::Greater => e.entry.addenda.len().to_string(),
                }
            ),
            Style::default().fg(Color::Reset),
        )),
    ];
    return cells;
}

fn make_addenda_header() -> Vec<Cell<'static>> {
    vec![
        Cell::from(Span::styled(
            format!("{}", "Type Code"),
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan),
        )),
        Cell::from(Span::styled(
            format!("{}", "Payment Related Information"),
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan),
        )),
        Cell::from(Span::styled(
            format!("{}", "Seq Num"),
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan),
        )),
        Cell::from(Span::styled(
            format!("{}", "Entry Seq Num"),
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan),
        )),
    ]
}

fn make_header() -> Vec<Cell<'static>> {
    vec![
        Cell::from(Span::styled(
            format!("{}", "Entry #"),
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan),
        )),
        Cell::from(Span::styled(
            format!("{}", "TXN Code"),
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan),
        )),
        Cell::from(Span::styled(
            format!("{}", "Individual Name"),
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan),
        )),
        Cell::from(Span::styled(
            format!("{}", "DFI Acct #"),
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan),
        )),
        Cell::from(Span::styled(
            format!("{}", "Trace #"),
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan),
        )),
        Cell::from(Span::styled(
            format!("{:>13}", "Amount"),
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan),
        )),
        Cell::from(Span::styled(
            format!("{}", "Addenda?"),
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan),
        )),
    ]
}

// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
