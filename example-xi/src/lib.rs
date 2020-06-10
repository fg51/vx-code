use std::io::BufRead;

use dirs::config_dir;

use xi_core_lib::client::Client;
use xi_core_lib::rpc::CoreNotification;
use xi_core_lib::XiCore;
use xi_rpc::Handler;
use xi_rpc::Peer;
use xi_rpc::RpcLoop;

use serde_json::json;

mod error;
use crate::error::Result;

mod channels;
//use channels::ClientToClientWriter;
use channels::start_xi_core;
use channels::Reader;

use log::debug;

mod logging;

// 1. run xi.
// 2. connect xi channel.
// 3. to xi: client_started.
// 4. to xi: new_view
pub fn run() -> Result<()> {
    setup_logger();

    let (writer_from_client_to_xi, mut reader_from_xi_to_client) = start_xi_core();
    let front_event_loop = RpcLoop::new(writer_from_client_to_xi);

    // front_event_loop.mainloop(|| core_to_client_reader, &mut event_handler)?;

    // let raw_peer = front_event_loop.get_raw_peer();
    // raw_peer.send_rpc_notification(
    //     "client_started",
    //     //&json!({ "config_dir": xi_config_dir.to_str().unwrap(), }),
    //     &json!({}),
    // );

    let raw_peer = front_event_loop.get_raw_peer();
    setup_config(&raw_peer, &mut reader_from_xi_to_client)?;
    //     &mut reader_from_xi_to_client,

    println!("run view-id");
    let view_id = raw_peer
        // .send_rpc_request("new_view", &json!({"file_path": "foo.md"}))
        .send_rpc_request("new_view", &json!({}))
        .expect("failed to create the new view");
    let view_id = view_id.as_str().unwrap().to_string();
    println!("new_view -> view_id: {}", view_id);

    //let mut buf = String::new();
    //reader_from_xi_to_client.read_line(&mut buf)?;
    //println!("new_view -> view_id: {}", buf);

    // peer.send_rpc_notification("new_view", &json!({"file_path": "foo.md"}));
    // let mut buf = String::new();
    // reader_from_xi_to_client.read_line(&mut buf)?;
    // println!("{}", buf);

    //{
    // let client = Client::new(raw_peer.box_clone());
    // client.available_themes();
    // let mut buf = String::new();
    // rx_from_xi_to_client.read_line(&mut buf)?;
    // println!("recv from xi: {}", buf);
    //}

    //let child = thread::spawn(move || {
    //    let layout = TermionLayout::new();

    //    let styles: Rc<RefCell<Box<dyn Styles>>> =
    //        Rc::new(RefCell::new(Box::new(TermionStyles::new())));

    //    let mut event_handler = EventController::new(Box::new(layout), styles.clone());
    //    front_event_loop
    //        .mainloop(|| core_to_client_reader, &mut event_handler)
    //        .unwrap();
    //});

    //{
    //let fw = xi_core_lib::watcher::FileWatcher::new(&front_event_loop.get_raw_peer());
    //let fm = xi_core_lib::file::FileManager::new(fw);
    //}

    Ok(())
}

//// pub fn setup_config<T: Peer>(xi: &T) -> Result<()> {
//pub fn setup_config(xi: &mut ClientToClientWriter) -> Result<()> {
//    let config_dir = config_dir().unwrap();
//
//    let mut xi_config_dir = config_dir.clone();
//    xi_config_dir.push("xi");
//    xi.client_started(xi_config_dir.to_str().unwrap());
//
//    Ok(())
//}

// fn setup_config(core: &dyn Peer) -> Result<Config, Error> {
fn setup_config(
    tx_from_client_to_xi: &dyn Peer,
    rx_from_xi_to_client: &mut Reader,
) -> Result<()> {
    debug!("setup config");
    // let config_dir = dirs::config_dir().ok_or_else(|| format_err!("config dir not found"))?;

    // let mut xi_config_dir = config_dir.clone();
    tx_from_client_to_xi.send_rpc_notification(
        "client_started",
        // &json!({ "config_dir": xi_config_dir.to_str().unwrap(), }),
        &json!({}),
    );
    let mut buf = String::new();
    rx_from_xi_to_client.read_line(&mut buf)?;
    println!("recv from xi: {}", buf);
    // {"method":"available_languages","params":{"languages":[]}}

    let mut buf = String::new();
    rx_from_xi_to_client.read_line(&mut buf)?;
    println!("recv from xi: {}", buf);
    // recv from xi: {"method":"available_themes","params":{"themes":["InspiredGitHub","Solarized (dark)","Solarized (light)","base16-eighties.dark","base16-mocha.dark","base16-ocean.dark","base16-ocean.light"]}}

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
