use crate::system::SysStats;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, List, ListItem},
};
pub fn render(f: &mut Frame, stats: &SysStats) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            //CPU
            Constraint::Length(3),
            //RAM
            Constraint::Length(3),
            // PROCESSES AND NET
            Constraint::Min(10),
        ])
        .split(f.area());

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            // Processes
            Constraint::Percentage(50),
            // Network
            Constraint::Percentage(50),
        ])
        .split(main_chunks[2]);

    // CPU USAGE
    f.render_widget(
        Gauge::default()
            .block(Block::default().title(" CPU ").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Cyan))
            .percent(stats.cpu_usage as u16),
        main_chunks[0],
    );

    // RAM USAGE
    let mem_p = (stats.mem_used as f64 / stats.mem_total as f64 * 100.0) as u16;
    f.render_widget(
        Gauge::default()
            .block(Block::default().title(" RAM ").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Magenta))
            .percent(mem_p),
        main_chunks[1],
    );

    // PROCESSES LIST
    let items: Vec<ListItem> = stats
        .processes
        .iter()
        .map(|(name, mem)| ListItem::new(format!("{:<15} | {} MB", name, mem)))
        .collect();

    let process_list = List::new(items)
        .Block(
            Block::default()
                .title(" Top Processes (Mem) ")
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White));
    f.render_widget(process_list, bottom_chunks[0]);
}
