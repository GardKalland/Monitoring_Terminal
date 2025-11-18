use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{
        Axis, Block, Borders, Cell, Chart, Dataset, Gauge, GraphType, List, ListItem, Paragraph,
        Row, Table, Tabs, Wrap,
    },
};

use crate::{
    app::{App, ProcessSort, Tab},
    system::{SystemInfo, format_bytes},
    vpn::{VpnProvider, VpnStatus}, // can have VPNAction here, but i couldnt use it with mine
                                   // vpn provider looool (i thihk)
};

pub fn render(frame: &mut Frame, app: &App, system_info: &SystemInfo, vpn_status: &VpnStatus) {
    if app.show_help {
        render_help(frame);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.area());

    render_tabs(frame, app, chunks[0]);

    match app.current_tab {
        Tab::Overview => render_overview(frame, app, system_info, chunks[1]),
        Tab::Processes => render_processes(frame, app, system_info, chunks[1]),
        Tab::SystemInfo => render_system_info(frame, system_info, chunks[1]),
        Tab::Vpn => render_vpn(frame, vpn_status, chunks[1]),
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

fn render_overview(frame: &mut Frame, app: &App, system_info: &SystemInfo, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Min(0),
        ])
        .split(area);

    render_cpu_memory_bars(frame, system_info, chunks[0]);
    render_cpu_graph(frame, app, chunks[1]);
    render_memory_graph(frame, app, chunks[2]);
    render_temperatures(frame, system_info, chunks[3]);
}

fn render_cpu_memory_bars(frame: &mut Frame, system_info: &SystemInfo, area: Rect) {
    let cpu_usage = system_info.get_cpu_usage();
    let mem_percent = system_info.get_memory_percentage();
    let (used_mem, total_mem) = system_info.get_memory_usage();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let cpu_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(" CPU Usage "))
        .gauge_style(
            Style::default()
                .fg(if cpu_usage > 80.0 {
                    Color::Red
                } else if cpu_usage > 50.0 {
                    Color::Yellow
                } else {
                    Color::Green
                })
                .bg(Color::Black),
        )
        .percent(cpu_usage as u16)
        .label(format!("{:.1}%", cpu_usage));

    let memory_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(format!(
            " Memory: {} / {} ",
            format_bytes(used_mem),
            format_bytes(total_mem)
        )))
        .gauge_style(
            Style::default()
                .fg(if mem_percent > 80.0 {
                    Color::Red
                } else if mem_percent > 50.0 {
                    Color::Yellow
                } else {
                    Color::Green
                })
                .bg(Color::Black),
        )
        .percent(mem_percent as u16)
        .label(format!("{:.1}%", mem_percent));

    frame.render_widget(cpu_gauge, chunks[0]);
    frame.render_widget(memory_gauge, chunks[1]);
}

fn render_cpu_graph(frame: &mut Frame, app: &App, area: Rect) {
    let data: Vec<(f64, f64)> = app
        .cpu_history
        .iter()
        .enumerate()
        .map(|(i, &val)| (i as f64, val as f64))
        .collect();

    let dataset = Dataset::default()
        .name("CPU %")
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Cyan))
        .data(&data);

    let max_y = app
        .cpu_history
        .iter()
        .fold(100.0f32, |max, &val| max.max(val));

    let chart = Chart::new(vec![dataset])
        .block(
            Block::default()
                .title(" CPU Usage History ")
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, app.history_size as f64]),
        )
        .y_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .labels(vec![Line::from("0"), Line::from("50"), Line::from("100")])
                .bounds([0.0, max_y as f64]),
        );

    frame.render_widget(chart, area);
}

fn render_memory_graph(frame: &mut Frame, app: &App, area: Rect) {
    let data: Vec<(f64, f64)> = app
        .memory_history
        .iter()
        .enumerate()
        .map(|(i, &val)| (i as f64, val))
        .collect();

    let dataset = Dataset::default()
        .name("Memory %")
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Magenta))
        .data(&data);

    let max_y = app
        .memory_history
        .iter()
        .fold(100.0f64, |max, &val| max.max(val));

    let chart = Chart::new(vec![dataset])
        .block(
            Block::default()
                .title(" Memory Usage History ")
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, app.history_size as f64]),
        )
        .y_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .labels(vec![Line::from("0"), Line::from("50"), Line::from("100")])
                .bounds([0.0, max_y]),
        );

    frame.render_widget(chart, area);
}

fn render_temperatures(frame: &mut Frame, system_info: &SystemInfo, area: Rect) {
    let temps = system_info.get_temperatures();

    let items: Vec<ListItem> = temps
        .iter()
        .map(|(label, temp)| {
            let color = if *temp > 80.0 {
                Color::Red
            } else if *temp > 60.0 {
                Color::Yellow
            } else {
                Color::Green
            };

            ListItem::new(Line::from(vec![
                Span::styled(format!("{:30}", label), Style::default().fg(Color::White)),
                Span::styled(format!("{:>6.1}°C", temp), Style::default().fg(color)),
            ]))
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(" Temperatures ")
            .borders(Borders::ALL),
    );

    frame.render_widget(list, area);
}

fn render_processes(frame: &mut Frame, app: &App, system_info: &SystemInfo, area: Rect) {
    let mut processes = system_info.get_processes();

    match app.process_sort {
        ProcessSort::Cpu => processes.sort_by(|a, b| {
            let cmp = b
                .cpu_usage
                .partial_cmp(&a.cpu_usage)
                .unwrap_or(std::cmp::Ordering::Equal);
            if app.sort_ascending {
                cmp.reverse()
            } else {
                cmp
            }
        }),
        ProcessSort::Memory => processes.sort_by(|a, b| {
            let cmp = b.memory.cmp(&a.memory);
            if app.sort_ascending {
                cmp.reverse()
            } else {
                cmp
            }
        }),
        ProcessSort::Name => processes.sort_by(|a, b| {
            let cmp = a.name.cmp(&b.name);
            if app.sort_ascending {
                cmp
            } else {
                cmp.reverse()
            }
        }),
        ProcessSort::Pid => processes.sort_by(|a, b| {
            let cmp = a.pid.cmp(&b.pid);
            if app.sort_ascending {
                cmp
            } else {
                cmp.reverse()
            }
        }),
    }

    let header_cells = ["PID", "Name", "CPU %", "Memory", "Status"]
        .iter()
        .map(|h| {
            Cell::from(*h).style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
        });

    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = processes.iter().skip(app.process_scroll).map(|proc| {
        let cells = vec![
            Cell::from(proc.pid.to_string()),
            Cell::from(proc.name.clone()),
            Cell::from(format!("{:.1}", proc.cpu_usage)),
            Cell::from(format_bytes(proc.memory)),
            Cell::from(proc.status.clone()),
        ];
        Row::new(cells).height(1)
    });

    let sort_field = match app.process_sort {
        ProcessSort::Cpu => "CPU",
        ProcessSort::Memory => "Memory",
        ProcessSort::Name => "Name",
        ProcessSort::Pid => "PID",
    };

    let sort_arrow = if app.sort_ascending { "↑" } else { "↓" };
    let sort_indicator = format!("{} {}", sort_field, sort_arrow);

    let table = Table::new(
        rows,
        [
            Constraint::Length(8),
            Constraint::Min(20),
            Constraint::Length(8),
            Constraint::Length(12),
            Constraint::Length(15),
        ],
    )
    .header(header)
    .block(Block::default().borders(Borders::ALL).title(format!(
        " Processes (sorted by: {}) - Total: {} ",
        sort_indicator,
        processes.len()
    )))
    .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    frame.render_widget(table, area);
}

fn render_system_info(frame: &mut Frame, system_info: &SystemInfo, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    render_general_info(frame, system_info, chunks[0]);
    render_disk_info(frame, system_info, chunks[1]);
}

fn render_general_info(frame: &mut Frame, system_info: &SystemInfo, area: Rect) {
    let info = system_info.get_system_info();

    let mut lines = Vec::new();
    let keys = [
        "OS",
        "Version",
        "Kernel",
        "Hostname",
        "CPU Brand",
        "CPUs",
        "Total Memory",
        "Uptime",
    ];

    for key in keys.iter() {
        if let Some(value) = info.get(*key) {
            lines.push(Line::from(vec![
                Span::styled(
                    format!("{:15}: ", key),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(value.clone(), Style::default().fg(Color::White)),
            ]));
        }
    }

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" System Information ")
                .borders(Borders::ALL),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

fn render_disk_info(frame: &mut Frame, system_info: &SystemInfo, area: Rect) {
    let disks = system_info.get_disk_info();

    let items: Vec<ListItem> = disks
        .iter()
        .map(|disk| {
            let used = disk.total_space - disk.available_space;
            let percent = if disk.total_space > 0 {
                (used as f64 / disk.total_space as f64) * 100.0
            } else {
                0.0
            };

            let bar_width = 20;
            let filled = ((percent / 100.0) * bar_width as f64) as usize;
            let bar = format!("[{}{}]", "=".repeat(filled), " ".repeat(bar_width - filled));

            ListItem::new(vec![
                Line::from(vec![Span::styled(
                    format!("{}", disk.mount_point),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )]),
                Line::from(vec![
                    Span::raw(format!("  {} ", bar)),
                    Span::raw(format!(
                        "{:.1}% ({} / {})",
                        percent,
                        format_bytes(used),
                        format_bytes(disk.total_space)
                    )),
                ]),
            ])
        })
        .collect();

    let list = List::new(items).block(Block::default().title(" Disk Usage ").borders(Borders::ALL));

    frame.render_widget(list, area);
}

fn render_vpn(frame: &mut Frame, vpn_status: &VpnStatus, area: Rect) {
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

fn render_vpn_status(frame: &mut Frame, vpn_status: &VpnStatus, area: Rect) {
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

fn render_vpn_details(frame: &mut Frame, vpn_status: &VpnStatus, area: Rect) {
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

fn render_vpn_map(frame: &mut Frame, vpn_status: &VpnStatus, area: Rect) {
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

fn get_ascii_map<'a>(vpn_status: &VpnStatus, _user_location: &str) -> Vec<Line<'a>> {
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

// This didnt work, maybe i will look at it again? who knows, i dont, thats for sure
// fn get_location_coordinates(country: &str) -> (usize, usize) {
//     match country.to_uppercase().as_str() {
//         // North America
//         s if s.contains("US") || s.contains("UNITED STATES") => (8, 12),
//         s if s.contains("CANADA") || s.contains("CA") => (4, 15),
//         s if s.contains("MEXICO") => (12, 10),
//
//         // Europe
//         s if s.contains("NORWAY") || s.contains("NO") => (2, 38), // Top of Europe
//         s if s.contains("SWEDEN") || s.contains("SE") => (3, 40),
//         s if s.contains("UK") || s.contains("UNITED KINGDOM") || s.contains("BRITAIN") => (6, 35),
//         s if s.contains("GERMANY") || s.contains("DE") => (7, 38),
//         s if s.contains("FRANCE") || s.contains("FR") => (8, 36),
//         s if s.contains("NETHERLANDS") || s.contains("NL") => (6, 37),
//         s if s.contains("SWITZERLAND") || s.contains("CH") => (8, 38),
//         s if s.contains("SPAIN") || s.contains("ES") => (10, 34),
//         s if s.contains("ITALY") || s.contains("IT") => (10, 39),
//         s if s.contains("POLAND") || s.contains("PL") => (7, 40),
//
//         // Asia
//         s if s.contains("JAPAN") || s.contains("JP") => (8, 68),
//         s if s.contains("CHINA") || s.contains("CN") => (10, 58),
//         s if s.contains("INDIA") || s.contains("IN") => (14, 52),
//         s if s.contains("SINGAPORE") || s.contains("SG") => (16, 60),
//         s if s.contains("HONG KONG") || s.contains("HK") => (13, 60),
//         s if s.contains("KOREA") || s.contains("KR") => (9, 66),
//         s if s.contains("TAIWAN") || s.contains("TW") => (12, 63),
//
//         // Middle East
//         s if s.contains("ISRAEL") || s.contains("IL") => (12, 44),
//
//         // Oceania
//         s if s.contains("AUSTRALIA") || s.contains("AU") => (20, 60),
//         s if s.contains("NEW ZEALAND") || s.contains("NZ") => (21, 70),
//
//         // South America
//         s if s.contains("BRAZIL") || s.contains("BR") => (16, 22),
//         s if s.contains("ARGENTINA") || s.contains("AR") => (20, 18),
//         s if s.contains("CHILE") || s.contains("CL") => (19, 12),
//
//         // Africa
//         s if s.contains("SOUTH AFRICA") || s.contains("ZA") => (19, 42),
//
//         // Default - middle of map
//         _ => (10, 40),
//     }
// }

fn render_help(frame: &mut Frame) {
    let area = frame.area();

    let help_text = vec![
        Line::from(vec![Span::styled(
            "Keyboard Shortcuts",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled("q / Ctrl+C       ", Style::default().fg(Color::Yellow)),
            Span::raw("Quit application"),
        ]),
        Line::from(vec![
            Span::styled("? / h            ", Style::default().fg(Color::Yellow)),
            Span::raw("Toggle help screen"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Tab / Right      ", Style::default().fg(Color::Yellow)),
            Span::raw("Next tab"),
        ]),
        Line::from(vec![
            Span::styled("Shift+Tab / Left ", Style::default().fg(Color::Yellow)),
            Span::raw("Previous tab"),
        ]),
        Line::from(vec![
            Span::styled("1 / 2 / 3 / 4    ", Style::default().fg(Color::Yellow)),
            Span::raw("Switch to specific tab"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("s                ", Style::default().fg(Color::Yellow)),
            Span::raw("Cycle process sort (CPU/Memory/Name/PID)"),
        ]),
        Line::from(vec![
            Span::styled("o                ", Style::default().fg(Color::Yellow)),
            Span::raw("Toggle sort order (ascending/descending)"),
        ]),
        Line::from(vec![
            Span::styled("Up/k, Down/j     ", Style::default().fg(Color::Yellow)),
            Span::raw("Scroll process list"),
        ]),
    ];

    let paragraph = Paragraph::new(help_text)
        .block(
            Block::default()
                .title(" Help ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    let centered_area = centered_rect(60, 60, area);
    frame.render_widget(paragraph, centered_area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
