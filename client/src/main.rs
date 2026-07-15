mod config;
mod utils;

use clap::Parser;
use crate::{
    config::Config,
    utils::{
        client_connect::client_connect,
        file_handler::read_string,
    },
};

/// First CLI layer
async fn main_cli(config: &Config) {
    loop {
        println!("");
        println!("[0] Exit");
        println!("[1] Connect");

        let choice: &str = &read_string();
        
        match choice {
            "0" => break,
            "1" => client_connect(&config).await,
            _ => eprintln!("[!] Unknown command")
        }
    }
}

/// Rust-VSNA client CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// IP address to connect to
    #[arg(short, long, default_value = "0.0.0.0")]
    ip: String,

    /// Port to connect to
    #[arg(short, long, default_value = "8080")]
    port: String,

    /// Client directory to sync
    #[arg(short, long, default_value = "")]
    dir: String,

    /// Auto sync between client and server
    #[arg(short, long, default_value = "false")]
    auto_sync: String,

    /// Config file path
    #[arg(short, long)]
    config: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let ip: String = args.ip;
    let port: String = args.port;
    let dir: String = args.dir;
    let auto_sync: String = args.auto_sync;
    let config_file: Option<String> = args.config;

    let config = if let Some(config_path) = config_file {
        println!("[*] Loading config from {config_path}");
        
        if !std::path::Path::new(&config_path).exists() {
            eprintln!("[!] Config file '{}' does not exist!", config_path);
            return;
        }
        
        Config::load_from_file(config_path)
    } else {
        Config::new(ip, port, dir, auto_sync)
    };
    
    match config {
        Ok(config) => {
            println!("{config:?}");
            main_cli(&config).await;
        }
        Err(e) => {
            eprintln!("{e}");
        }
    }
}
