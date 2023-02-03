use crate::config::{Config, load_config};

pub mod config;

fn main() {
    let conf: Config = load_config("config.toml");
    println!("DEBUG: Running on version {}", conf.rusteware.version);

}
