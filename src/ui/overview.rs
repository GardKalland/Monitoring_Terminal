use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols,
    text::{Line, Span},
    widgets::{Axis, Block, Borders, Chart, Dataset, Gauge, GraphType, List, ListItem},
};

use crate::{
    app::App,
    system::{SystemInfo, format_bytes},
};

pub fn render_overview(frame: &mut Frame, app: &App, system_info: &SystemInfo, area: Rect) {
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
