use std::io::{self, BufRead, Read, Write};

use serde_json::Value;
use xi_core_lib::XiCore;
use xi_rpc::RpcLoop;

pub fn run_xi() -> (Writer, Reader, ClientToClientWriter) {
    let mut core = XiCore::new();

    let (to_core_tx, to_core_rx) = channel();
    let client_to_core_writer = Writer(to_core_tx);
    let client_to_core_reader = Reader(to_core_rx);

    let (from_core_tx, from_core_rx) = channel();
    let core_to_client_writer = Writer(from_core_tx.clone());
    let core_to_client_reader = Reader(from_core_rx);

    let client_to_client_writer = ClientToClientWriter(Writer(from_core_tx));

    let mut core_event_loop = RpcLoop::new(core_to_client_writer);
    thread::spawn(move || core_event_loop.mainloop(|| client_to_core_reader, &mut core));

    (
        client_to_core_writer,
        core_to_client_reader,
        client_to_client_writer,
    )
}
