mod config;
mod utils;

use clap::Parser;
use std::{collections::HashMap,sync::Arc};
use tokio::{net::TcpListener, sync::RwLock};
use tracing::info;
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

/// Rust-VSNA server CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// IP address to bind to
    #[arg(short, long, default_value = "0.0.0.0")]
    ip: String,

    /// Port to bind to
    #[arg(short, long, default_value = "5555")]
    port: String,

    /// Server directory to sync
    #[arg(short, long, default_value = "")]
    dir: String,

    /// Max number of clients
    #[arg(short, long, default_value = "1")]
    max_clients: String,

    /// Config file path
    #[arg(short, long)]
    config: Option<String>,
}

fn print_all_net_interfaces() {
    println!("Available IPs:");
    if let Ok(interfaces) = list_afinet_netifas() {
        for (name, ip) in interfaces {
            if ip.is_ipv4() && !ip.is_loopback() {
                println!("- {} ({})", ip, name);
            }
        }
    } else {
        eprintln!("[!] Err get interfaces");
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args = Cli::parse();

    let ip: String = args.ip;
    let port: String = args.port;
    let dir: String = args.dir;
    let max_clients: String = args.max_clients;
    let config_file: Option<String> = args.config;

    let config = if let Some(config_path) = config_file {        
        if !std::path::Path::new(&config_path).exists() {
            eprintln!("[!] Config file '{}' does not exist!", config_path);
            return;
        }
        
        Config::load_from_file(config_path)
    } else {
        Config::new(ip, port, dir, max_clients)
    };
    
    match config {
        Ok(config) => {
            print_all_net_interfaces();
            println!("{config:?}");
            start_server(config).await;
        }
        Err(e) => {
            eprintln!("{e}");
        }
    }
}
