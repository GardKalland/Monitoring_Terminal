mod help;
mod overview;
mod processes;
mod system_info;
mod vpn;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Tabs},
};

use crate::{
    app::{App, Tab},
    system::SystemInfo,
    vpn::VpnStatus,
};

pub fn render(frame: &mut Frame, app: &App, system_info: &SystemInfo, vpn_status: &VpnStatus) {
    if app.show_help {
        help::render_help(frame);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.area());

    render_tabs(frame, app, chunks[0]);

    match app.current_tab {
        Tab::Overview => overview::render_overview(frame, app, system_info, chunks[1]),
        Tab::Processes => processes::render_processes(frame, app, system_info, chunks[1]),
        Tab::SystemInfo => system_info::render_system_info(frame, system_info, chunks[1]),
        Tab::Vpn => vpn::render_vpn(frame, vpn_status, chunks[1]),
    }
}

fn render_tabs(frame: &mut Frame, app: &App, area: Rect) {
    let titles = vec![
        "Overview [1]",
        "Processes [2]",
        "System Info [3]",
        "VPN [4]",
    ];
    let selected = match app.current_tab {
        Tab::Overview => 0,
        Tab::Processes => 1,
        Tab::SystemInfo => 2,
        Tab::Vpn => 3,
    };

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" System Monitor ")
                .title_alignment(Alignment::Center),
        )
        .select(selected)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(tabs, area);
}
