mod config;
mod utils;

use std::{collections::HashMap, sync::Arc};
use tokio::{net::TcpListener, sync::RwLock};
use tracing::{error, info};
use local_ip_address::list_afinet_netifas;
use crate::{
    config::Config, utils::ws::{Clients, handle_connection}
};

/// Run WS server
async fn start_server(config: Config) {
    let addr: String = config.socket();
    let listener = TcpListener::bind(&addr).await;
    info!("WebSocket server is listening on {}", addr);

    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

    while let Ok((stream, addr)) = listener
        .as_ref()
        .unwrap()
        .accept()
        .await
    {
        tokio::spawn(handle_connection(stream, addr, clients.clone(), config.clone()));
    }
}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    if let Ok(config) = Config::new() {
        println!("{config:?}");
        println!("Server IPs:");
    
        match list_afinet_netifas() {
            Ok(interfaces) => {
                for (name, ip) in interfaces {
                    if ip.is_ipv4() && !ip.is_loopback() {
                        println!("- {} ({})", ip, name);
                    }
                }
            }
            Err(e) => {
                eprintln!("[!] Err get interfaces: {}", e);
            }
        }
        start_server(config).await;
    } else {
        error!("[!] Error with config");
    }
}
