use crate::lib::NachaFile;

pub struct App {
    // pub title: &'a str,
    pub should_quit: bool,
    // pub tabs: TabsState<'a>,
    // pub show_chart: bool,
    // pub progress: f64,
    // pub sparkline: Signal<RandomSignal>,
    // pub tasks: StatefulList<&'a str>,
    // pub logs: StatefulList<(&'a str, &'a str)>,
    // pub signals: Signals,
    // pub barchart: Vec<(&'a str, u64)>,
    // pub servers: Vec<Server<'a>>,
    // pub enhanced_graphics: bool,
    pub nacha_file: NachaFile,
}

impl App {
    pub fn new(nacha_file: NachaFile) -> App {
        App {
            should_quit: false,
            nacha_file: nacha_file,
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
}
