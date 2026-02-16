use std::io;

use crossterm::terminal::enable_raw_mode;

mod system;
mod ui;

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
}
