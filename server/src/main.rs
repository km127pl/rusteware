use crate::config::load_config;

pub mod config;

fn main() {
    let conf = load_config("config.toml");
    println!("Hello, {:#?}", conf);
}
