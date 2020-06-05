use std::io::Result;
use std::io::{stdin, stdout, Write};

use env_logger as logger;
use log::info;

use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() -> Result<()> {
    logger::init();
    info!("start");

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode()?;

    for event in stdin.events() {
        if event? == Event::Key(Key::Ctrl('c')) {
            info!("recieved: ctrl + c");
            return Ok(());
        }
    }

    Ok(())
}
