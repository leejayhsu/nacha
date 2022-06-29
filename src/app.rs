use crate::lib::{Addendum, NachaFile};
use crate::term::DetailEntryWithCounter;
use tui::widgets::TableState;
pub struct App<'a> {
    pub should_quit: bool,
    pub entries: StatefulTable<DetailEntryWithCounter>,
    pub entry_count: usize,
    pub nacha_file: &'a mut NachaFile,
    pub show_popup: bool,
    pub addenda_popup: StatefulTable<Addendum>,
}

impl<'a> App<'a> {
    pub fn new(nacha_file: &'a mut NachaFile, entries: Vec<DetailEntryWithCounter>) -> App<'a> {
        let count = entries.len();
        App {
            should_quit: false,
            nacha_file: nacha_file,
            entries: StatefulTable::with_items(entries),
            entry_count: count,
            show_popup: false,
            addenda_popup: StatefulTable::new(),
        }
    }
    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            'j' => {
                if self.show_popup {
                    self.addenda_popup.next();
                } else {
                    self.entries.next();
                }
            }
            'k' => {
                if self.show_popup {
                    self.addenda_popup.previous();
                } else {
                    self.entries.previous();
                }
            }
            'h' => {
                if self.show_popup {
                    self.addenda_popup.jump_previous();
                } else {
                    self.entries.jump_previous();
                }
            }
            'l' => {
                if self.show_popup {
                    self.addenda_popup.jump_next();
                } else {
                    self.entries.jump_next();
                }
            }
            'o' => {
                match self.entries.state.selected() {
                    Some(i) => {
                        if self.entries.items[i].entry.has_addenda() {
                            self.show_popup = !self.show_popup;
                        }
                        if self.addenda_popup.has_items() && !self.show_popup {
                            self.addenda_popup.clear_items();
                        } else if !self.addenda_popup.has_items() && self.show_popup {
                            self.addenda_popup
                                .add_items(self.entries.items[i].entry.addenda.clone())
                        }
                    }
                    None => {}
                };
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
    pub fn new() -> StatefulTable<T> {
        let mut s = StatefulTable {
            state: TableState::default(),
            items: Vec::new(),
            jump_size: 1,
        };
        s.state.select(Some(0));
        return s;
    }

    /// used with popup
    pub fn clear_items(&mut self) {
        self.items = Vec::new();
    }

    pub fn has_items(&self) -> bool {
        self.items.len() > 0
    }

    pub fn add_items(&mut self, items: Vec<T>) {
        let mut jump_size = items.len() / 10;
        if jump_size == 0 {
            jump_size = 1;
        }
        self.items = items;
        self.jump_size = jump_size;
    }

    pub fn with_items(items: Vec<T>) -> StatefulTable<T> {
        let mut jump_size = items.len() / 10;
        if jump_size == 0 {
            jump_size = 1;
        }
        let mut s = StatefulTable {
            state: TableState::default(),
            items,
            jump_size,
        };
        s.state.select(Some(0));
        return s;
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
