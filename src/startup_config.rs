use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub bind_ip_port: String,
    pub log_level: String,
    pub data_dir: String,
    pub protocol: String,
    pub server_fqdn: String,
    pub url: String,
}

pub fn load_config() -> Config {
    let config_file = "config.json";
    // Read the contents of the config.json file into a string
    let mut file = File::open(config_file).expect("Failed to open config.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read config.json");

    // Deserialize the JSON content into the Config struct
    let config: Config = serde_json::from_str(&contents).expect("Failed to parse config.json");
    config
}
