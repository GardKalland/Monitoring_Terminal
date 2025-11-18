mod app;
mod events;
mod system;
mod ui;
mod vpn;

use anyhow::Result;
use crossterm::{
    event::Event,
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{
    io,
    time::{Duration, Instant},
};

use app::App;
use system::SystemInfo;
use vpn::VpnStatus;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let mut system_info = SystemInfo::new();
    let mut vpn_status = VpnStatus::new();

    let tick_rate = Duration::from_millis(500); //refresh rate. Dont come here and say its an magix number
    let mut last_tick = Instant::now();
    let vpn_check_rate = Duration::from_secs(5);
    let mut last_vpn_check = Instant::now();

    let result = run_app(
        &mut terminal,
        &mut app,
        &mut system_info,
        &mut vpn_status,
        tick_rate,
        &mut last_tick,
        vpn_check_rate,
        &mut last_vpn_check,
    );

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    system_info: &mut SystemInfo,
    vpn_status: &mut VpnStatus,
    tick_rate: Duration,
    last_tick: &mut Instant,
    vpn_check_rate: Duration,
    last_vpn_check: &mut Instant,
) -> Result<()> {
    // Initial VPN check
    *vpn_status = vpn::get_vpn_status();

    loop {
        terminal.draw(|f| ui::render(f, app, system_info, vpn_status))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if let Some(event) = events::poll_events(timeout)? {
            if let Event::Key(key) = event {
                events::handle_key_event(key, app);
            }
        }

        if last_tick.elapsed() >= tick_rate {
            match app.current_tab {
                app::Tab::Overview => {
                    system_info.refresh_light();
                }
                app::Tab::Processes => {
                    system_info.refresh_full();
                }
                app::Tab::SystemInfo => {
                    system_info.refresh_system_info();
                }
                app::Tab::Vpn => {
                    system_info.refresh_minimal();
                }
            }

            app.add_cpu_data(system_info.get_cpu_usage());
            app.add_memory_data(system_info.get_memory_percentage());
            *last_tick = Instant::now();
        }

        if last_vpn_check.elapsed() >= vpn_check_rate {
            *vpn_status = vpn::get_vpn_status();
            *last_vpn_check = Instant::now();
        }

        if app.should_quit {
            return Ok(());
        }
    }
}
