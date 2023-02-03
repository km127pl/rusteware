use crate::config::{Config, load_config};
use crate::database::{create_product, db_connect, get_conn};

pub mod config;
mod database;

fn main() {
    let conf: Config = load_config("config.toml");
    println!("DEBUG: Running on version {}", conf.rusteware.version);

    let db = db_connect(conf);
    let conn = get_conn(&db);

    println!("DEBUG: Connected to database");

    println!("DEBUG: Creating test product");

    create_product(&db, "Test Product", "This is a test product", 10);

    let mut prod = database::get_product_by_id(&db, 1);

    println!("DEBUG: Product: {:?}", prod);


}
