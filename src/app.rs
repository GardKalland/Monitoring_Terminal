use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    Overview,
    Processes,
    SystemInfo,
    Vpn,
}

impl Tab {
    pub fn next(&self) -> Self {
        match self {
            Tab::Overview => Tab::Processes,
            Tab::Processes => Tab::SystemInfo,
            Tab::SystemInfo => Tab::Vpn,
            Tab::Vpn => Tab::Overview,
        }
    }

    pub fn previous(&self) -> Self {
        match self {
            Tab::Overview => Tab::Vpn,
            Tab::Processes => Tab::Overview,
            Tab::SystemInfo => Tab::Processes,
            Tab::Vpn => Tab::SystemInfo,
        }
    }

    pub fn title(&self) -> &str {
        match self {
            Tab::Overview => "Overview",
            Tab::Processes => "Processes",
            Tab::SystemInfo => "System Info",
            Tab::Vpn => "VPN",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessSort {
    Cpu,
    Memory,
    Name,
    Pid,
}

pub struct App {
    pub should_quit: bool,
    pub show_help: bool,
    pub current_tab: Tab,
    pub process_sort: ProcessSort,
    pub sort_ascending: bool,
    pub process_scroll: usize,
    pub cpu_history: VecDeque<f32>,
    pub memory_history: VecDeque<f64>,
    pub history_size: usize,
    pub user_location: String,
}

impl Default for App {
    fn default() -> Self {
        let user_location = detect_user_location();

        Self {
            should_quit: false,
            show_help: false,
            current_tab: Tab::Overview,
            process_sort: ProcessSort::Cpu,
            sort_ascending: false,
            process_scroll: 0,
            cpu_history: VecDeque::with_capacity(100),
            memory_history: VecDeque::with_capacity(100),
            history_size: 100,
            user_location,
        }
    }
}

fn detect_user_location() -> String {
    if let Ok(config) = std::fs::read_to_string(".user_location") {
        let location = config.trim().to_string();
        if !location.is_empty() {
            return location;
        }
    }

    // Defaulting to ??? if it dont find ya
    "???".to_string()
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn next_tab(&mut self) {
        self.current_tab = self.current_tab.next();
        self.process_scroll = 0;
    }

    pub fn previous_tab(&mut self) {
        self.current_tab = self.current_tab.previous();
        self.process_scroll = 0;
    }

    pub fn cycle_process_sort(&mut self) {
        self.process_sort = match self.process_sort {
            ProcessSort::Cpu => ProcessSort::Memory,
            ProcessSort::Memory => ProcessSort::Name,
            ProcessSort::Name => ProcessSort::Pid,
            ProcessSort::Pid => ProcessSort::Cpu,
        };
    }

    pub fn toggle_sort_order(&mut self) {
        self.sort_ascending = !self.sort_ascending;
    }

    pub fn scroll_up(&mut self) {
        self.process_scroll = self.process_scroll.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        self.process_scroll = self.process_scroll.saturating_add(1);
    }

    pub fn add_cpu_data(&mut self, value: f32) {
        if self.cpu_history.len() >= self.history_size {
            self.cpu_history.pop_front();
        }
        self.cpu_history.push_back(value);
    }

    pub fn add_memory_data(&mut self, value: f64) {
        if self.memory_history.len() >= self.history_size {
            self.memory_history.pop_front();
        }
        self.memory_history.push_back(value);
    }
}
