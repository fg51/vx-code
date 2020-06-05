use std::env;
use std::io::{stdin, stdout, Write};

use anyhow::Result;
use env_logger as logger;
use log::info;

use clap::{crate_description, crate_version};
use clap::{App, Arg};

use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::{clear, cursor};

mod xi;

fn main() -> Result<()> {
    env::set_var("RUST_LOG", "debug");
    logger::init();
    info!("start");

    let app = App::new("vx")
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("file").help("The file to open"));
    let _file_path = app.get_matches().value_of("file");

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
