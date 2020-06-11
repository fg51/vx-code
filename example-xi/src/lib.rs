//use std::io::BufRead;
use std::thread;

//use dirs::config_dir;

//use xi_core_lib::client::Client;
//use xi_core_lib::rpc::CoreNotification;
//use xi_core_lib::XiCore;
//use xi_rpc::Handler;
use xi_rpc::Peer;
use xi_rpc::RpcLoop;

use serde_json::json;

mod error;
use crate::error::Result;

mod channels;
//use channels::ClientToClientWriter;
use channels::start_xi_core;
//use channels::Reader;

use log::debug;

mod logging;

mod event_controller;
use event_controller::EventController;

// 1. run xi.
// 2. make xi channel, and client channel.
// 3. thread::sqwan (xi.tx.mainloop(client.rx, EventController);
// 4. thread::sqwan (client.tx.mainloop(xi.rx, EventController);
// 5. to xi: client_started.
// 6. to xi: new_view
pub fn run() -> Result<()> {
    setup_logger();

    let (writer_from_client_to_xi, reader_from_xi_to_client) = start_xi_core();
    let mut front_event_loop = RpcLoop::new(writer_from_client_to_xi);

    let raw_peer = front_event_loop.get_raw_peer();
    // setup_config(&raw_peer, &mut reader_from_xi_to_client)?;

    setup_config(&raw_peer)?;

    let child = thread::spawn(move || {
        let mut event_handler = EventController::new();
        front_event_loop
            .mainloop(|| reader_from_xi_to_client, &mut event_handler)
            .unwrap();
    });

    println!("run view-id");
    let view_id = raw_peer
        // .send_rpc_request("new_view", &json!({"file_path": "foo.md"}))
        .send_rpc_request("new_view", &json!({"file_path": "README.md"}))
        .expect("failed to create the new view");
    let view_id = view_id.as_str().unwrap().to_string();
    println!("run: new_view -> view_id: {}", view_id);

    child.join().unwrap();

    #[cfg(feature = "tracing")]
    trace::write_trace_dump_into("./trace.out");

    Ok(())
}

fn setup_config(tx_from_client_to_xi: &dyn Peer) -> Result<()> {
    debug!("setup config");
    // let config_dir = dirs::config_dir().ok_or_else(|| format_err!("config dir not found"))?;

    // let mut xi_config_dir = config_dir.clone();
    tx_from_client_to_xi.send_rpc_notification(
        "client_started",
        // &json!({ "config_dir": xi_config_dir.to_str().unwrap(), }),
        &json!({}),
    );

    Ok(())
}

fn setup_logger() {
    let logging_path = dirs::home_dir()
        .expect("failed to retrieve the home dir")
        .join(".local/share/vixy/vixi.log");

    logging::setup(&logging_path).expect("failed to set the logger")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
