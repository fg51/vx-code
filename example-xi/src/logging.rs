use std::path::Path;
//use std::{env, fmt, fs, io, panic, thread};
use std::{fs, io};

use log::info;

pub fn setup(logging_path: &Path) -> Result<(), fern::InitError> {
    let level_filter = match std::env::var("XI_LOG") {
        Ok(level) => match level.to_lowercase().as_ref() {
            "trace" => log::LevelFilter::Trace,
            "debug" => log::LevelFilter::Debug,
            _ => log::LevelFilter::Info,
        },
        // Default to info
        Err(_) => log::LevelFilter::Info,
    };

    create_log_directory(logging_path)?;

    let fern_dispatch = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message,
            ))
        })
        .level(level_filter)
        .chain(fern::log_file(logging_path)?);

    // Start fern
    fern_dispatch.apply()?;
    info!("Logging with fern is set up to level {}", level_filter);
    info!("Writing logs to: {}", logging_path.display());

    Ok(())
}

fn create_log_directory(path_with_file: &Path) -> io::Result<()> {
    let log_dir = path_with_file.parent().ok_or_else(|| io::Error::new(
        io::ErrorKind::InvalidInput,
        format!(
            "Unable to get the parent of the following Path: {}, Your path should contain a file name",
            path_with_file.display(),
        ),
    ))?;
    fs::create_dir_all(log_dir)?;
    Ok(())
}
