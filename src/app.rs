use crate::lib::{Currency, DetailEntry, NachaFile};
use crate::term::DetailEntryWithCounter;
use tui::{
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Cell, Row, Table, TableState},
};
pub struct App<'a> {
    pub should_quit: bool,
    pub entries: StatefulTable<DetailEntryWithCounter>,
    pub entry_count: usize,
    pub nacha_file: &'a mut NachaFile,
}

impl<'a> App<'a> {
    pub fn new(nacha_file: &'a mut NachaFile, entries: Vec<DetailEntryWithCounter>) -> App<'a> {
        let count = entries.len();
        App {
            should_quit: false,
            nacha_file: nacha_file,
            entries: StatefulTable::with_items(entries),
            entry_count: count,
        }
    }
    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            'j' => {
                self.entries.next();
            }
            'k' => {
                self.entries.previous();
            }
            'h' => {
                self.entries.jump_previous();
            }
            'l' => {
                self.entries.jump_next();
            }
            _ => {}
        }
    }

    pub fn on_up(&mut self) {
        self.entries.previous();
    }

    pub fn on_down(&mut self) {
        self.entries.next();
    }

    pub fn on_right(&mut self) {
        self.entries.jump_next();
    }

    pub fn on_left(&mut self) {
        self.entries.jump_previous();
    }
}

pub struct StatefulTable<T> {
    pub state: TableState,
    pub items: Vec<T>,
    pub jump_size: usize,
}

impl<T> StatefulTable<T> {
    pub fn with_items(items: Vec<T>) -> StatefulTable<T> {
        let jump_size = items.len() / 10;
        StatefulTable {
            state: TableState::default(),
            items,
            jump_size,
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

    pub fn jump_next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i > self.items.len() - 1 - self.jump_size {
                    // self.items.len() - 1 - i
                    self.jump_size % (self.items.len() - i)
                } else {
                    i + self.jump_size
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn jump_previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i < self.jump_size {
                    self.items.len() - (self.jump_size - i)
                } else {
                    i - self.jump_size
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
