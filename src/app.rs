use crate::lib::{Currency, DetailEntry, NachaFile};
use tui::{
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Cell, Row, Table, TableState},
};
pub struct App<'a> {
    // pub title: &'a str,
    pub should_quit: bool,
    // pub tabs: TabsState<'a>,
    // pub show_chart: bool,
    // pub progress: f64,
    // pub sparkline: Signal<RandomSignal>,
    // pub tasks: StatefulList<&'a str>,
    pub entries: StatefulTable<DetailEntry>,
    // pub logs: StatefulList<(&'a str, &'a str)>,
    // pub signals: Signals,
    // pub barchart: Vec<(&'a str, u64)>,
    // pub servers: Vec<Server<'a>>,
    // pub enhanced_graphics: bool,
    pub nacha_file: &'a mut NachaFile,
}

impl<'a> App<'a> {
    pub fn new(nacha_file: &'a mut NachaFile, entries: Vec<DetailEntry>) -> App<'a> {
        // let e = nacha_file.get_entries();
        // let header_cells = vec![
        //     Cell::from(Span::styled(
        //         format!("{}", "TXN Code"),
        //         Style::default()
        //             .add_modifier(Modifier::BOLD)
        //             .fg(Color::Cyan),
        //     )),
        //     Cell::from(Span::styled(
        //         format!("{}", "Individual Name"),
        //         Style::default()
        //             .add_modifier(Modifier::BOLD)
        //             .fg(Color::Cyan),
        //     )),
        //     Cell::from(Span::styled(
        //         format!("{}", "DFI Acct Num"),
        //         Style::default()
        //             .add_modifier(Modifier::BOLD)
        //             .fg(Color::Cyan),
        //     )),
        //     Cell::from(Span::styled(
        //         format!("{}", "Trace Num"),
        //         Style::default()
        //             .add_modifier(Modifier::BOLD)
        //             .fg(Color::Cyan),
        //     )),
        //     Cell::from(Span::styled(
        //         format!("{:>13}", "Amount"),
        //         Style::default()
        //             .add_modifier(Modifier::BOLD)
        //             .fg(Color::Cyan),
        //     )),
        // ];
        // let mut table_stuff = vec![Row::new(header_cells)];
        // let items: Vec<Row> = entries
        //     .iter()
        //     .map(|e| {
        //         let code = &e.transaction_code[..];
        //         let color = match code {
        //             "22" | "32" | "42" | "52" => Color::Green,
        //             "27" | "37" | "47" => Color::Red,
        //             _ => Color::Reset,
        //         };
        //         let cells = vec![
        //             Cell::from(Span::styled(
        //                 format!("{}", e.transaction_code),
        //                 Style::default().fg(color),
        //             )),
        //             Cell::from(Span::styled(
        //                 format!("{}", e.individual_name),
        //                 Style::default().fg(Color::Reset),
        //             )),
        //             Cell::from(Span::styled(
        //                 format!("{}", e.dfi_account_number),
        //                 Style::default().fg(Color::Reset),
        //             )),
        //             Cell::from(Span::styled(
        //                 format!("{}", e.trace_number),
        //                 Style::default().fg(Color::Reset),
        //             )),
        //             Cell::from(Span::styled(
        //                 format!("{:>13}", e.amount.pretty_dollars_cents()),
        //                 Style::default().fg(color),
        //             )),
        //         ];
        //         Row::new(cells)
        //     })
        //     .collect();
        // table_stuff.extend(items);
        App {
            should_quit: false,
            nacha_file: nacha_file,
            entries: StatefulTable::with_items(entries),
        }
    }
    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            // 't' => {
            //     self.show_chart = !self.show_chart;
            // }
            _ => {}
        }
    }

    pub fn on_up(&mut self) {
        self.entries.previous();
    }

    pub fn on_down(&mut self) {
        self.entries.next();
    }

    // pub fn on_right(&mut self) {
    //     self.entries.next();
    // }

    // pub fn on_left(&mut self) {
    //     self.entries.previous();
    // }
}

pub struct StatefulTable<T> {
    pub state: TableState,
    pub items: Vec<T>,
}

impl<T> StatefulTable<T> {
    pub fn with_items(items: Vec<T>) -> StatefulTable<T> {
        StatefulTable {
            state: TableState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
