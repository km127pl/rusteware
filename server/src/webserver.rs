use std::net::SocketAddr;
use crate::config::Config;
use warp::*;
use crate::database::{db_connect, get_all_products, get_product_by_id};

pub async fn start_webserver(config: &Config) {
    let db = db_connect(&config);
    // construct a socket address from config
    let addr = SocketAddr::from(([127, 0, 0, 1], config.webserver.port));

    // construct the warp routes

    // GET /api/products - get all products
    let products = path!("products")
        .and(warp::get())
        .map( move || {
            let products = get_all_products(&db);
            warp::reply::json(&products)
        })
        .with(warp::reply::with::header("Content-Type", "application/json"));

    println!("DEBUG: Webserver started on {}", addr);

    // serve the routes
    serve(products)
        .run(addr)
        .await;

}