use std::env;
use std::io::Result;
use std::io::{stdin, stdout, Write};

use env_logger as logger;
use log::info;

use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::{clear, cursor};

fn main() -> Result<()> {
    env::set_var("RUST_LOG", "debug");
    logger::init();
    info!("start");

    let stdin = stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode()?);

    write!(stdout, "{}", clear::All)?;
    write!(stdout, "{}", cursor::Goto(1, 1))?;
    write!(stdout, "Hello World")?;
    stdout.flush()?;

    for event in stdin.events() {
        if event? == Event::Key(Key::Ctrl('c')) {
            info!("recieved: ctrl + c");
            return Ok(());
        }
    }

    Ok(())
}
