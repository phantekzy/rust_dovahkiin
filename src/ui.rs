use crate::system::SysStats;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Gauge, Paragraph},
};

pub fn render(f: &mut Frame, stats: &SysStats) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            // CPU
            Constraint::Length(3),
            // RAM
            Constraint::Length(3),
            // INFO
            Constraint::Min(0),
        ])
        .split(f.size());

    // CPU BAR
    let cpu_gauge = Gauge::Default()
        .block(Block::default().title(" CPU USAGE ").borders(Borders::ALL))
        .gauge_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan))
        .percent(stats.cpu_usage as u16);
    f.render_widget(cpu_gauge, chunks[0]);

    // 2. RAM Bar
    let mem_percent = (stats.mem_used as f64 / stats.mem_total as f64 * 100.0) as u16;
    let mem_gauge = Gauge::default()
        .block(Block::default().title(" RAM Usage ").borders(Borders::ALL))
        .gauge_style(ratatui::style::Style::default().fg(ratatui::style::Color::Magenta))
        .percent(mem_percent);
    f.render_widget(mem_gauge, chunks[1])
}
