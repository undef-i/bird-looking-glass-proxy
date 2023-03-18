use serde_derive::Deserialize;
extern crate toml;

use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub bind_ip: String,
    pub bind_port: u16,
    pub access_list: Vec<String>,
    pub shared_secret: String,
    pub ipv4_source: String,
    pub ipv6_source: String,
    pub bird_socket: String,
    pub bird6_socket: String,
}
impl Config {
    pub fn new(file_path: &str) -> Self {
        return match toml::from_str(&match fs::read_to_string(file_path) {
            Ok(f) => f,
            Err(e) => panic!("Unable to Find the Config: {}", e),
        }) {
            Ok(f) => f,
            Err(e) => panic!("Unable to Read the Config: {}", e),
        };
    }
}
