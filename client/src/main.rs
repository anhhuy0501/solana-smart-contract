use logger::init_logger;
use std::sync::Arc;
use swap_server;
use swap_server::server_builder::AccessControl;
use swap_server::server_builder::WebServerBuilder;

const SWAP_SERVICE_ENDPOINT: &str = "127.0.0.1:5050";

#[tokio::main]
async fn main() {
    init_logger(&"Swap server".to_string());
    let access_control = AccessControl::default();
    let server = WebServerBuilder::default()
        .with_entry_point(SWAP_SERVICE_ENDPOINT)
        .with_access_control(access_control)
        .build();
    server.serve().await
}
