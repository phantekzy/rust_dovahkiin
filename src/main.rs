use std::io;

use crossterm::{
    execute,
    terminal::{self, EnterAlternateScreen, enable_raw_mode},
};
use ratatui::{Terminal, backend, prelude::CrosstermBackend};

mod system;
mod ui;

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backebd = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
}
