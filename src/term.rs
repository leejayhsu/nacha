#![allow(unused)]
use crate::ui;
use crate::{
    app::App,
    lib::{DetailEntry, NachaFile},
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

pub struct DetailEntryWithCounter {
    pub entry: DetailEntry,
    pub counter: u32,
}

impl DetailEntryWithCounter {
    pub fn new(entry: DetailEntry, counter: u32) -> DetailEntryWithCounter {
        DetailEntryWithCounter { entry, counter }
    }
}
fn get_entries(nacha_file: &NachaFile) -> Vec<DetailEntryWithCounter> {
    let mut entries = Vec::new();
    let mut count: u32 = 1;
    for batch in &nacha_file.batches {
        for entry in &batch.detail_entries {
            entries.push(DetailEntryWithCounter::new(entry.clone(), count));
            count += 1;
        }
    }
    return entries;
}
pub fn run(tick_rate: Duration, nacha_file: &mut NachaFile) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // run the terminal
    let app = App::new(nacha_file, get_entries(nacha_file));
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => app.on_key(c),
                    KeyCode::Left => app.on_left(),
                    KeyCode::Up => app.on_up(),
                    KeyCode::Right => app.on_right(),
                    KeyCode::Down => app.on_down(),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
        if app.should_quit {
            return Ok(());
        }
    }
}
