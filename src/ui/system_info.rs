use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::system::{SystemInfo, format_bytes};

pub fn render_system_info(frame: &mut Frame, system_info: &SystemInfo, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    render_general_info(frame, system_info, chunks[0]);
    render_disk_info(frame, system_info, chunks[1]);
}

pub fn render_general_info(frame: &mut Frame, system_info: &SystemInfo, area: Rect) {
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

pub fn render_disk_info(frame: &mut Frame, system_info: &SystemInfo, area: Rect) {
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
