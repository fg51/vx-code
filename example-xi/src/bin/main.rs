use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process;

use log::{error, info, warn};

use xi_core_lib::XiCore;
use xi_rpc::RpcLoop;

use example_xi as lib;

const XI_LOG_DIR: &str = "xi-core";
const XI_LOG_FILE: &str = "xi-core.log";

fn main() {
    if let Err(e) = lib::run() {
        eprintln!("{:?}", e);
        process::exit(1);
    }
    //let mut state = XiCore::new();
    //let stdin = io::stdin();
    //let stdout = io::stdout();
    //let mut rpc_looper = RpcLoop::new(stdout);

    //let mut raw_peer = rpc_looper.get_raw_peer();
    //// let config = match setup_config(&raw_peer) {
    //match lib::setup_config(&raw_peer) {
    //    Ok(config) => config,
    //    Err(err) => {
    //        eprintln!("failed to load the configuration: {}", err);
    //        process::exit(1);
    //    }
    //}

    //let flags = get_flags();

    //let logfile_config = generate_logfile_config(&flags);

    //let logging_path_result = generate_logging_path(logfile_config);

    //let logging_path = logging_path_result
    //    .as_ref()
    //    .map(|p: &PathBuf| -> &Path { p.as_path() })
    //    .ok();

    //if let Err(e) = setup_logging(logging_path) {
    //    eprintln!(
    //        "[ERROR] setup_logging returned error, logging not enabled: {:?}",
    //        e
    //    );
    //}
    //if let Err(e) = logging_path_result.as_ref() {
    //    warn!(
    //        "Unable to generate the logging path to pass to set up: {}",
    //        e
    //    )
    //}

    //match rpc_looper.mainloop(|| stdin.lock(), &mut state) {
    //    Ok(_) => (),
    //    Err(err) => {
    //        error!("xi-core exited with error:\n{:?}", err);
    //        process::exit(1);
    //    }
    //}
}
