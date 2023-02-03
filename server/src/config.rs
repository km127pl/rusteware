

use serde_derive::Deserialize;
use std::fs;
use std::io::Read;
use std::process::exit;
use toml;

#[derive(Deserialize, Debug)]
pub struct Config {
    rusteware: RustewareConfig,
    database: DatabaseConfig,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    host: String,
    port: u16,
    name: String,
    user: String,
    password: String,
}

#[derive(Deserialize, Debug)]
pub struct RustewareConfig {
    version: String
}

// loads a config from a .toml file
pub fn load_config(filename: &str) -> Config {
    // read the file
    let mut file = fs::File::open(filename).unwrap();
    let mut toml_str = String::new();

    // parse the config
    file.read_to_string(&mut toml_str).unwrap();
    let config: Config = toml::from_str(&toml_str).unwrap();

    return config;
}