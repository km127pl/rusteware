use crate::config::{Config, load_config};
use crate::database::{db_connect, Product};
use crate::webserver::start_webserver;
use std::thread;
use warp::Filter;

mod config;
mod database;
mod webserver;

#[tokio::main]
async fn main() {
    let conf: Config = load_config("config.toml");
    println!("DEBUG: Running on version {}", conf.rusteware.version);

    // start the webserver on another thread
    let server_future = Box::pin(async move {
        start_webserver(&conf).await;
    });

    thread::spawn(move || {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(server_future);
    });

    loop {
        thread::sleep(std::time::Duration::from_secs(1));
    }
}
