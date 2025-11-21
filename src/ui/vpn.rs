use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::vpn::{VpnProvider, VpnStatus};

pub fn render_vpn(frame: &mut Frame, vpn_status: &VpnStatus, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Length(26),
            Constraint::Min(0),
        ])
        .split(area);

    render_vpn_status(frame, vpn_status, chunks[0]);
    render_vpn_map(frame, vpn_status, chunks[1]);
    render_vpn_details(frame, vpn_status, chunks[2]);
}

pub fn render_vpn_status(frame: &mut Frame, vpn_status: &VpnStatus, area: Rect) {
    let status_text = if vpn_status.connected {
        "CONNECTED"
    } else {
        "DISCONNECTED"
    };

    let status_color = if vpn_status.connected {
        Color::Green
    } else {
        Color::Red
    };

    let status_indicator = if vpn_status.connected { "●" } else { "○" };

    let mut lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("  Status: "),
            Span::styled(
                format!("{} {}", status_indicator, status_text),
                Style::default()
                    .fg(status_color)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    if vpn_status.provider != VpnProvider::Unknown {
        lines.push(Line::from(vec![
            Span::raw("  Provider: "),
            Span::styled(
                vpn_status.provider.name(),
                Style::default().fg(if vpn_status.connected {
                    Color::Green
                } else {
                    Color::Yellow
                }),
            ),
        ]));
    }

    if let Some(ref server) = vpn_status.server {
        lines.push(Line::from(vec![
            Span::raw("  Server: "),
            Span::styled(server.clone(), Style::default().fg(Color::Cyan)),
        ]));
    }

    if let Some(ref country) = vpn_status.country {
        lines.push(Line::from(vec![
            Span::raw("  Country: "),
            Span::styled(country.clone(), Style::default().fg(Color::White)),
        ]));
    }

    let title = if vpn_status.provider != VpnProvider::Unknown {
        format!(" {} Status ", vpn_status.provider.name())
    } else {
        " VPN Status ".to_string()
    };

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(if vpn_status.connected {
                    Color::Green
                } else {
                    Color::Red
                })),
        )
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, area);
}

pub fn render_vpn_details(frame: &mut Frame, vpn_status: &VpnStatus, area: Rect) {
    let mut items = Vec::new();

    if vpn_status.connected {
        items.push(ListItem::new(Line::from(vec![
            Span::styled(
                "Provider:       ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                vpn_status.provider.name(),
                Style::default().fg(Color::Green),
            ),
        ])));

        if let Some(ref server) = vpn_status.server {
            items.push(ListItem::new(Line::from(vec![
                Span::styled(
                    "Server:         ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(server.clone()),
            ])));
        }

        if let Some(ref country) = vpn_status.country {
            items.push(ListItem::new(Line::from(vec![
                Span::styled(
                    "Country:        ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(country.clone()),
            ])));
        }

        if let Some(ref city) = vpn_status.city {
            items.push(ListItem::new(Line::from(vec![
                Span::styled(
                    "City:           ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(city.clone()),
            ])));
        }

        if let Some(ref ip) = vpn_status.ip {
            items.push(ListItem::new(Line::from(vec![
                Span::styled(
                    "IP Address:     ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(ip.clone()),
            ])));
        }

        if let Some(ref protocol) = vpn_status.protocol {
            items.push(ListItem::new(Line::from(vec![
                Span::styled(
                    "Protocol:       ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(protocol.clone()),
            ])));
        }

        if let Some(ref interface) = vpn_status.interface {
            items.push(ListItem::new(Line::from(vec![
                Span::styled(
                    "Interface:      ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(interface.clone()),
            ])));
        }

        if let Some(ref time) = vpn_status.connection_time {
            items.push(ListItem::new(Line::from(vec![
                Span::styled(
                    "Connected Time: ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(time.clone()),
            ])));
        }

        items.push(ListItem::new(Line::from("")));
        items.push(ListItem::new(Line::from(vec![Span::styled(
            "Raw Status:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )])));

        for line in vpn_status.raw_output.lines() {
            items.push(ListItem::new(Line::from(format!("  {}", line))));
        }
    } else {
        items.push(ListItem::new(Line::from("")));
        items.push(ListItem::new(Line::from(vec![Span::styled(
            "Not connected to ProtonVPN",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::ITALIC),
        )])));
        items.push(ListItem::new(Line::from("")));
        items.push(ListItem::new(Line::from(
            "To connect, use one of these commands:",
        )));
        items.push(ListItem::new(Line::from("")));
        items.push(ListItem::new(Line::from(vec![Span::styled(
            "  protonvpn-cli connect",
            Style::default().fg(Color::Cyan),
        )])));
        items.push(ListItem::new(Line::from(vec![Span::styled(
            "  protonvpn connect",
            Style::default().fg(Color::Cyan),
        )])));

        if !vpn_status.raw_output.is_empty() {
            items.push(ListItem::new(Line::from("")));
            items.push(ListItem::new(Line::from(vec![Span::styled(
                "Info:",
                Style::default().fg(Color::Yellow),
            )])));
            items.push(ListItem::new(Line::from(format!(
                "  {}",
                vpn_status.raw_output
            ))));
        }
    }

    let list = List::new(items).block(
        Block::default()
            .title(" Connection Details ")
            .borders(Borders::ALL),
    );

    frame.render_widget(list, area);
}

pub fn render_vpn_map(frame: &mut Frame, vpn_status: &VpnStatus, area: Rect) {
    let map = get_ascii_map(vpn_status, "?????"); // TODO: Get from app state

    let paragraph = Paragraph::new(map)
        .block(
            Block::default()
                .title(" Connection Map ")
                .borders(Borders::ALL),
        )
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}

pub fn get_ascii_map<'a>(vpn_status: &VpnStatus, _user_location: &str) -> Vec<Line<'a>> {
    let mut lines = Vec::new();

    let map_content = std::fs::read_to_string("map/map.txt").unwrap_or_else(|_| {
        // Fallback if file doesn't exist, //TODO change it to and questiong mark inside globe
        // thingy.
        "   Error: map/map.txt not found\n   Create a map file in the map directory".to_string()
    });

    let color = if vpn_status.connected {
        Color::Green
    } else {
        Color::Red
    };

    for map_line in map_content.lines() {
        lines.push(Line::from(vec![Span::styled(
            map_line.to_string(),
            Style::default().fg(color),
        )]));
    }

    if vpn_status.connected {
        let mut info = String::from("● Connected");

        if let Some(ref srv) = vpn_status.server {
            info.push_str(&format!(": {}", srv));
        }
        if let Some(ref country) = vpn_status.country {
            info.push_str(&format!(" | {}", country));
        }
        if let Some(ref city) = vpn_status.city {
            info.push_str(&format!(", {}", city));
        }

        lines.push(Line::from(vec![Span::styled(
            info,
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )]));
    } else {
        lines.push(Line::from(vec![Span::styled(
            "○ Not Connected",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )]));
    }

    lines
}
