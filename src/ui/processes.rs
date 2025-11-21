use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
};
use std::collections::HashMap;

use crate::{
    app::{App, ProcessSort},
    system::{ProcessCategory, ProcessInfo, SystemInfo, format_bytes},
};

pub fn render_processes(frame: &mut Frame, app: &App, system_info: &SystemInfo, area: Rect) {
    let processes = system_info.get_processes();

    if app.show_all_processes {
        render_all_processes(frame, app, system_info, area);
        if app.command_mode {
            render_command_prompt(frame, app, area);
        }
        return;
    }

    if app.category_expanded {
        render_expanded_category(frame, app, system_info, area);
        if app.command_mode {
            render_command_prompt(frame, app, area);
        }
        return;
    }

    let mut categorized: HashMap<ProcessCategory, Vec<_>> = HashMap::new();
    for process in processes.iter() {
        categorized
            .entry(process.category)
            .or_insert_with(Vec::new)
            .push(process.clone());
    }

    for processes_in_category in categorized.values_mut() {
        match app.process_sort {
            ProcessSort::Cpu => processes_in_category.sort_by(|a, b| {
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
            ProcessSort::Memory => processes_in_category.sort_by(|a, b| {
                let cmp = b.memory.cmp(&a.memory);
                if app.sort_ascending {
                    cmp.reverse()
                } else {
                    cmp
                }
            }),
            ProcessSort::Name => processes_in_category.sort_by(|a, b| {
                let cmp = a.name.cmp(&b.name);
                if app.sort_ascending {
                    cmp
                } else {
                    cmp.reverse()
                }
            }),
            ProcessSort::Pid => processes_in_category.sort_by(|a, b| {
                let cmp = a.pid.cmp(&b.pid);
                if app.sort_ascending {
                    cmp
                } else {
                    cmp.reverse()
                }
            }),
        }
    }

    let category_order = vec![
        ProcessCategory::System,
        ProcessCategory::Browser,
        ProcessCategory::Development,
        ProcessCategory::Terminal,
        ProcessCategory::Editor,
        ProcessCategory::Media,
        ProcessCategory::Background,
        ProcessCategory::User,
    ];

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(area);

    let mut category_areas = Vec::new();
    for chunk in chunks.iter() {
        let row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(*chunk);
        category_areas.push(row[0]);
        category_areas.push(row[1]);
    }

    let sort_field = match app.process_sort {
        ProcessSort::Cpu => "CPU",
        ProcessSort::Memory => "Memory",
        ProcessSort::Name => "Name",
        ProcessSort::Pid => "PID",
    };
    let sort_order = if app.sort_ascending {
        "Asc ↑"
    } else {
        "Desc ↓"
    };

    for (idx, category) in category_order.iter().enumerate() {
        if idx < category_areas.len() {
            let is_selected = idx == app.selected_category;
            render_category_box(
                frame,
                category_areas[idx],
                *category,
                categorized.get(category).unwrap_or(&Vec::new()),
                is_selected,
                sort_field,
                sort_order,
            );
        }
    }

    if app.command_mode {
        render_command_prompt(frame, app, area);
    }
}

fn render_category_box(
    frame: &mut Frame,
    area: Rect,
    category: ProcessCategory,
    processes: &[ProcessInfo],
    is_selected: bool,
    sort_field: &str,
    sort_order: &str,
) {
    let items: Vec<ListItem> = processes
        .iter()
        .take(10) // Limit to 10 processes per category in the overview
        .map(|proc| {
            ListItem::new(Line::from(vec![
                Span::styled(
                    format!("{:6} ", proc.pid),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(
                    format!("{:20} ", proc.name),
                    Style::default().fg(Color::White),
                ),
                Span::styled(
                    format!("{:5.1}%", proc.cpu_usage),
                    Style::default().fg(if proc.cpu_usage > 50.0 {
                        Color::Red
                    } else if proc.cpu_usage > 20.0 {
                        Color::Yellow
                    } else {
                        Color::Green
                    }),
                ),
            ]))
        })
        .collect();

    let border_style = if is_selected {
        Style::default()
            .fg(category.color())
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(category.color())
    };

    let title = if is_selected {
        format!(
            " {} ({}) | Sort: {} {} ",
            category.name(),
            processes.len(),
            sort_field,
            sort_order
        )
    } else {
        format!(" {} ({}) ", category.name(), processes.len())
    };

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(title)
            .title_style(
                Style::default()
                    .fg(category.color())
                    .add_modifier(Modifier::BOLD),
            ),
    );

    frame.render_widget(list, area);
}

fn render_expanded_category(frame: &mut Frame, app: &App, system_info: &SystemInfo, area: Rect) {
    let processes = system_info.get_processes();

    let category_order = vec![
        ProcessCategory::System,
        ProcessCategory::Browser,
        ProcessCategory::Development,
        ProcessCategory::Terminal,
        ProcessCategory::Editor,
        ProcessCategory::Media,
        ProcessCategory::Background,
        ProcessCategory::User,
    ];

    let selected_category = category_order[app.selected_category];

    let mut filtered: Vec<ProcessInfo> = processes
        .into_iter()
        .filter(|p| p.category == selected_category)
        .collect();

    match app.process_sort {
        ProcessSort::Cpu => filtered.sort_by(|a, b| {
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
        ProcessSort::Memory => filtered.sort_by(|a, b| {
            let cmp = b.memory.cmp(&a.memory);
            if app.sort_ascending {
                cmp.reverse()
            } else {
                cmp
            }
        }),
        ProcessSort::Name => filtered.sort_by(|a, b| {
            let cmp = a.name.cmp(&b.name);
            if app.sort_ascending {
                cmp
            } else {
                cmp.reverse()
            }
        }),
        ProcessSort::Pid => filtered.sort_by(|a, b| {
            let cmp = a.pid.cmp(&b.pid);
            if app.sort_ascending {
                cmp
            } else {
                cmp.reverse()
            }
        }),
    }

    let sort_field = match app.process_sort {
        ProcessSort::Cpu => "CPU",
        ProcessSort::Memory => "Memory",
        ProcessSort::Name => "Name",
        ProcessSort::Pid => "PID",
    };
    let sort_order = if app.sort_ascending {
        "Asc ↑"
    } else {
        "Desc ↓"
    };

    let items: Vec<ListItem> = filtered
        .iter()
        .skip(app.process_scroll)
        .map(|proc| {
            ListItem::new(Line::from(vec![
                Span::styled(format!("{:8} ", proc.pid), Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!("{:30} ", proc.name),
                    Style::default().fg(Color::White),
                ),
                Span::styled(
                    format!("{:8.1}% ", proc.cpu_usage),
                    Style::default().fg(if proc.cpu_usage > 50.0 {
                        Color::Red
                    } else if proc.cpu_usage > 20.0 {
                        Color::Yellow
                    } else {
                        Color::Green
                    }),
                ),
                Span::styled(
                    format_bytes(proc.memory),
                    Style::default().fg(Color::Magenta),
                ),
            ]))
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(
                Style::default()
                    .fg(selected_category.color())
                    .add_modifier(Modifier::BOLD),
            )
            .title(format!(
                " {} - {} Processes | Sort: {} {} | Press ESC to go back ",
                selected_category.name(),
                filtered.len(),
                sort_field,
                sort_order
            ))
            .title_style(
                Style::default()
                    .fg(selected_category.color())
                    .add_modifier(Modifier::BOLD),
            ),
    );

    frame.render_widget(list, area);
}

fn render_all_processes(frame: &mut Frame, app: &App, system_info: &SystemInfo, area: Rect) {
    let mut processes = system_info.get_processes();

    // Sort all processes
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

    let sort_field = match app.process_sort {
        ProcessSort::Cpu => "CPU",
        ProcessSort::Memory => "Memory",
        ProcessSort::Name => "Name",
        ProcessSort::Pid => "PID",
    };
    let sort_order = if app.sort_ascending {
        "Asc ↑"
    } else {
        "Desc ↓"
    };

    let items: Vec<ListItem> = processes
        .iter()
        .skip(app.process_scroll)
        .map(|proc| {
            ListItem::new(Line::from(vec![
                Span::styled(format!("{:8} ", proc.pid), Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!("{:30} ", proc.name),
                    Style::default().fg(Color::White),
                ),
                Span::styled(
                    format!("{:8.1}% ", proc.cpu_usage),
                    Style::default().fg(if proc.cpu_usage > 50.0 {
                        Color::Red
                    } else if proc.cpu_usage > 20.0 {
                        Color::Yellow
                    } else {
                        Color::Green
                    }),
                ),
                Span::styled(
                    format_bytes(proc.memory),
                    Style::default().fg(Color::Magenta),
                ),
            ]))
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .title(format!(
                " All Processes - {} Total | Sort: {} {} | Press ESC to go back ",
                processes.len(),
                sort_field,
                sort_order
            ))
            .title_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
    );

    frame.render_widget(list, area);
}

fn render_command_prompt(frame: &mut Frame, app: &App, area: Rect) {
    use ratatui::layout::Alignment;
    use ratatui::widgets::Paragraph;

    // Create a popup area at the bottom
    let popup_height = 3;
    let popup_area = Rect {
        x: area.x,
        y: area.y + area.height - popup_height,
        width: area.width,
        height: popup_height,
    };

    let prompt_text = vec![Line::from(vec![
        Span::styled("Command: ", Style::default().fg(Color::Yellow)),
        Span::styled(&app.command_buffer, Style::default().fg(Color::White)),
        Span::styled("_", Style::default().fg(Color::Green)),
    ])];

    let prompt = Paragraph::new(prompt_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .title(" Enter Command ")
                .title_alignment(Alignment::Center),
        )
        .alignment(Alignment::Left);

    frame.render_widget(prompt, popup_area);
}
