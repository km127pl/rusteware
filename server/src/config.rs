use serde_derive::Deserialize;
use std::fs;
use std::io::Read;
use std::process::exit;
use toml;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub rusteware: RustewareConfig,
    pub database: DatabaseConfig,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct RustewareConfig {
    pub version: String
}

// loads a config from a .toml file
pub fn load_config(filename: &str) -> Config {
    // read the file
    let mut file = fs::File::open(filename).unwrap();
    let mut toml_str = String::new();

    // parse the config
    file.read_to_string(&mut toml_str).unwrap();

    let config : Config = match toml::from_str(&*toml_str) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to parse config: `{}`", filename);
            exit(1);
        }
    };

    return config;
}