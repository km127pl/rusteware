use crate::config::{Config, load_config};
use crate::database::{db_connect, Product};

pub mod config;
mod database;

fn main() {
    let conf: Config = load_config("config.toml");
    println!("DEBUG: Running on version {}", conf.rusteware.version);

    let db = db_connect(conf);
    // let conn = get_conn(&db);

    println!("DEBUG: Connected to database");

    // get all products
    let products: Vec<Product> = database::get_all_products(&db);

    for product in products {
        println!("DEBUG: Product: {:?}", product);
    }
}
