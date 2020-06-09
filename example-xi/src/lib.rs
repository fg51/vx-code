use dirs::config_dir;

use xi_core_lib::client::Client;
use xi_rpc::Peer;
use xi_rpc::RpcLoop;

mod error;
use crate::error::Result;

mod channels;
//use channels::ClientToClientWriter;
use channels::start_xi_core;

pub fn run() {
    let (client_to_xi_writer, xi_to_client_reader) = start_xi_core();
    let front_event_loop = RpcLoop::new(client_to_xi_writer);

    // front_event_loop.mainloop(|| core_to_client_reader, &mut event_handler)?;

    let raw_peer = front_event_loop.get_raw_peer();
    let client = Client::new(raw_peer.box_clone());
    client.
    //let child = thread::spawn(move || {
    //    let layout = TermionLayout::new();

    //    let styles: Rc<RefCell<Box<dyn Styles>>> =
    //        Rc::new(RefCell::new(Box::new(TermionStyles::new())));

    //    let mut event_handler = EventController::new(Box::new(layout), styles.clone());
    //    front_event_loop
    //        .mainloop(|| core_to_client_reader, &mut event_handler)
    //        .unwrap();
    //});
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
