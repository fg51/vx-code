use std::io::{stdin, stdout, Write};

use termion::clear;
use termion::cursor;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let stdin = stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode()?);

    write!(stdout, "{}", clear::All)?;
    write!(stdout, "{}", cursor::Goto(1, 1))?;
    stdout.flush()?;

    for c in stdin.events() {
        let event = c.unwrap();
        match event {
            Event::Key(Key::Ctrl('c')) => break,
            _ => {}
        }
    }
    Ok(())
}
