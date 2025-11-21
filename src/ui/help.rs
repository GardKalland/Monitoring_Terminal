use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn render_help(frame: &mut Frame) {
    let area = frame.area();

    let help_text = vec![
        Line::from(vec![Span::styled(
            "Keyboard Shortcuts",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "General",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("q / Ctrl+C       ", Style::default().fg(Color::Yellow)),
            Span::raw("Quit application"),
        ]),
        Line::from(vec![
            Span::styled("?                ", Style::default().fg(Color::Yellow)),
            Span::raw("Toggle help screen"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Tab Navigation",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("Tab              ", Style::default().fg(Color::Yellow)),
            Span::raw("Next tab"),
        ]),
        Line::from(vec![
            Span::styled("Shift+Tab        ", Style::default().fg(Color::Yellow)),
            Span::raw("Previous tab"),
        ]),
        Line::from(vec![
            Span::styled("1 / 2 / 3 / 4    ", Style::default().fg(Color::Yellow)),
            Span::raw("Switch to specific tab"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Processes Tab - Box Selection Mode",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("h/j/k/l or Arrows", Style::default().fg(Color::Yellow)),
            Span::raw("Navigate between category boxes"),
        ]),
        Line::from(vec![
            Span::styled("Enter            ", Style::default().fg(Color::Yellow)),
            Span::raw("Expand selected category (show all processes)"),
        ]),
        Line::from(vec![
            Span::styled("s                ", Style::default().fg(Color::Yellow)),
            Span::raw("Cycle sort field (CPU/Memory/Name/PID)"),
        ]),
        Line::from(vec![
            Span::styled("o                ", Style::default().fg(Color::Yellow)),
            Span::raw("Toggle sort order (Asc/Desc)"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Processes Tab - Expanded Mode",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("k/j or Up/Down   ", Style::default().fg(Color::Yellow)),
            Span::raw("Scroll through process list"),
        ]),
        Line::from(vec![
            Span::styled("Esc              ", Style::default().fg(Color::Yellow)),
            Span::raw("Exit expanded mode (back to boxes)"),
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

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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
