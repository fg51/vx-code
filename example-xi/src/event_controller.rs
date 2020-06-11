use log::{debug, info, warn};
use serde_json::{json, Value};
use xi_rpc::{RemoteError, RpcCall, RpcCtx};

pub struct EventController {}

impl EventController {
    pub fn new() -> Self {
        Self {}
    }
}

impl xi_rpc::Handler for EventController {
    type Notification = RpcCall;
    type Request = RpcCall;

    fn handle_notification(&mut self, _ctx: &RpcCtx, rpc: Self::Notification) {
        debug!("handle notification: {}", rpc.method.as_str());
        match rpc.method.as_str() {
            "add_status_item" => info!("{}", &rpc.params),
            "update_status_item" => info!("{:?}", &rpc.params),
            "plugin_started" => info!("{}: -> {}", &rpc.method, &rpc.params),
            "available_languages" => info!("{}", &rpc.method),
            "available_themes" => info!("{}", &rpc.method),
            "available_plugins" => info!("{}", &rpc.method),
            "config_changed" => info!("{}", &rpc.method),
            "def_style" => info!("{}", &rpc.params),
            "language_changed" => info!("{}", &rpc.method),
            "scroll_to" => info!("{}", &rpc.params),
            "update" => info!("{}", &rpc.params),
            "theme_changed" => info!("{}", &rpc.method),
            "set_path_for_view" => info!("{}", &rpc.params),
            "write_to_file" => info!("{}", &rpc.params),
            _ => warn!("unhandled notif \"{}\" -> {}", &rpc.method, &rpc.params),
        };
    }

    fn handle_request(
        &mut self,
        _ctx: &RpcCtx,
        rpc: Self::Request,
    ) -> Result<Value, RemoteError> {
        info!("[request] {} -> {:#?}", rpc.method, rpc.params);
        Ok(json!({}))
    }
}
