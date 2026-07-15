use std::{
    fs,
    path::Path,
};
use serde_json;
use serde::Deserialize;

/// JSON Config struct as env settings
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub ip: String,
    pub port: String,
    pub client_path: String,
    pub auto_sync: bool,
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
        
        self.port = port.to_string();
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
        
        self.client_path = path;
        Ok(())
    }
    
    pub fn set_auto_sync(&mut self, auto_sync: String) -> Result<(), String> {
        if auto_sync.trim().is_empty() {
            return Err("[!] Auto sync cannot be empty".to_string());
        }

        let true_list: &[&str] = &["true", "1", "on"];
        let false_list: &[&str] = &["false", "0", "off"];
        
        if true_list.contains(&auto_sync.as_str()) {
            self.auto_sync = true;
            Ok(())
        } else if false_list.contains(&auto_sync.as_str()) {
            self.auto_sync = false;
            Ok(())
        } else {
            Err("[!] Auto sync might be 'true|false|0|1|on|off'".to_string())
        }
    }

    pub fn new(ip: String, port: String, client_path: String, auto_sync: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut temp_config: Config = Self {
            ip: String::new(),
            port: String::new(),
            client_path: String::new(),
            auto_sync: false,
        };
        
        temp_config.set_ip(ip)?;
        temp_config.set_port(port)?;
        temp_config.set_client_path(client_path)?;
        temp_config.set_auto_sync(auto_sync)?;
        Ok(temp_config)
    }
    
    
    /// Read env file
    pub fn load_from_file(filename: String) -> Result<Self, Box<dyn std::error::Error>> {
        let res: String = fs::read_to_string(&filename)?;
        let temp_json: serde_json::Value = serde_json::from_str(&res)?;
        match Config::new(
            temp_json["ip"].as_str().unwrap().to_string(),
            temp_json["port"].to_string(),
            temp_json["client_path"].as_str().unwrap().to_string(),
            temp_json["auto_sync"].to_string(),
        ) {
            Ok(config) => {
                println!("[*] {:?}", config);
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