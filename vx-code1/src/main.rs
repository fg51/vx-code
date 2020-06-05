use std::cell::RefCell;
use std::env;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::process::exit;
use std::rc::Rc;
use std::thread;

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

// use xi_rpc::{Peer, RpcLoop};
use xi_rpc::RpcLoop;

mod xi;

fn main() -> Result<()> {
    env::set_var("RUST_LOG", "debug");
    logger::init();
    info!("start");

    let app = App::new("vx")
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("file").help("The file to open"));
    let file_path = app.get_matches().value_of("file");

    let (client_to_core_writer, core_to_client_reader, client_to_client_writer) =
        xi::run_xi();
    let mut front_event_loop = RpcLoop::new(client_to_core_writer);
    let raw_peer = front_event_loop.get_raw_peer();

    let child = thread::spawn(move || {});

    let child = thread::spawn(move || {
        let layout = TermionLayout::new();

        let styles: Rc<RefCell<Box<dyn Styles>>> =
            Rc::new(RefCell::new(Box::new(TermionStyles::new())));

        let mut event_handler = EventController::new(Box::new(layout), styles.clone());
        front_event_loop
            .mainloop(|| core_to_client_reader, &mut event_handler)
            .unwrap();
    });

    let mut input_controller = InputController::new(
        Box::new(TermionKeyboard::from_reader(stdin())),
        client_to_client_writer,
        // &config,
    );

    match file_path {
        Some(file_path) => {
            if let Err(err) = input_controller.open_file(&raw_peer, file_path) {
                eprintln!("failed to open {}: {}", file_path, err);
                exit(1);
            }
        }
        None => {
            eprintln!("failed to open the file. need fail path.");
            exit(1);
        }
    }

    if let Err(err) = input_controller.start_keyboard_event_loop(&raw_peer) {
        eprintln!("an error occured: {}", err);
        exit(1);
    }

    child.join().unwrap();

    //write!(stdout, "{}", clear::All)?;
    //write!(stdout, "{}", cursor::Goto(1, 1))?;
    //write!(stdout, "Hello World")?;
    //stdout.flush()?;

    //for event in stdin.events() {
    //    if event? == Event::Key(Key::Ctrl('c')) {
    //        info!("recieved: ctrl + c");
    //        return Ok(());
    //    }
    //}

    Ok(())
}
