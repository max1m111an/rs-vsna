use std::{
    fs,
    path::Path,
};
use serde_json;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub port: u16,
    pub max_clients: u8,
    pub ip: String,
    pub server_path: String,
}

impl Config {
    pub fn set_ip(&mut self, ip: String) -> Result<(), String> {
        if ip.trim().is_empty() {
            return Err("[!] IP address cannot be empty".to_string());
        }
        
        if ip.parse::<std::net::Ipv4Addr>().is_err() {
            return Err(format!("[!] '{}' is not a valid IP address", ip));
        }
        
        self.ip = ip;
        Ok(())
    }
        
    pub fn set_port(&mut self, port: String) -> Result<(), String> {
        let port = port.trim().trim_matches('"');
        
        if port.is_empty() {
            return Err("[!] Port cannot be empty".to_string());
        }
        
        let port_num = port.parse::<u16>()
            .map_err(|_| format!("[!] '{}' is not a valid port number (must be 1-65535)", port))?;
        
        if port_num == 0 {
            return Err("[!] Port 0 is reserved and cannot be used".to_string());
        }
        
        self.port = port_num;
        Ok(())
    }
    
    pub fn set_client_path(&mut self, path: String) -> Result<(), String> {
        if path.trim().is_empty() {
            return Err("[!] Client path cannot be empty".to_string());
        }
        
        let path_obj = Path::new(&path);
        if !path_obj.exists() {
            return Err(format!("[!] Path '{}' does not exist", path));
        }
        
        self.server_path = path;
        Ok(())
    }
    
    pub fn set_max_clients(&mut self, max_clients: String) -> Result<(), String> {
        let max_clients = max_clients.trim().trim_matches('"');
        
        if max_clients.is_empty() {
            return Err("[!] Max clients cannot be empty".to_string());
        }
        
        let max_clients = max_clients.parse::<u8>()
            .map_err(|_| format!("[!] '{}' is not a valid max clients number (must be 1-255)", max_clients))?;
        
        self.max_clients = max_clients;
        Ok(())
    }

    pub fn new(ip: String, port: String, client_path: String, max_clients: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut temp_config: Config = Self {
            ip: String::new(),
            port: 0,
            server_path: String::new(),
            max_clients: 0,
        };
        
        temp_config.set_ip(ip)?;
        temp_config.set_port(port)?;
        temp_config.set_client_path(client_path)?;
        temp_config.set_max_clients(max_clients)?;
        Ok(temp_config)
    }
    
    
    /// Read env file
    pub fn load_from_file(filename: String) -> Result<Self, Box<dyn std::error::Error>> {
        let res: String = fs::read_to_string(&filename)?;
        let temp_json: serde_json::Value = serde_json::from_str(&res)?;
        match Config::new(
            temp_json["ip"].as_str().unwrap().to_string(),
            temp_json["port"].to_string(),
            temp_json["server_path"].as_str().unwrap().to_string(),
            temp_json["max_clients"].to_string(),
        ) {
            Ok(config) => {
                Ok(config)
            }
            Err(e) => {
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("[!] Failed to parse config: {e}"))))
            }
        }
    }

    /// Get WS addr to connect
    pub fn socket(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}