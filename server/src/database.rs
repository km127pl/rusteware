use serde_derive::Deserialize;
use mysql::*;
use mysql::prelude::*;

use crate::config::Config;
use mysql::*;

#[derive(Debug)]
pub struct Product {
    id: i32,
    name: String,
    description: String,
    stock: i32
}

#[derive(Debug)]
pub struct Order {
    id: i32,
    product_id: i32,
    quantity: i32,
    date: String,
    product: Product
}

pub fn db_connect(config: Config) -> Pool {
    let pool = Pool::new(&*format!(
        "mysql://{}:{}@{}:{}/{}",
        config.database.user,
        config.database.password,
        config.database.host,
        config.database.port,
        config.database.name)
    ).unwrap();

    return pool;
}

pub fn get_conn(pool: &Pool) -> PooledConn {
    let conn = pool.get_conn().unwrap();
    return conn;
}

pub fn get_product_by_id(db: &Pool, id: i32) -> Product {
    let mut conn = get_conn(db);
    let mut product = Product {
        id: 0,
        name: String::from(""),
        description: String::from(""),
        stock: 0
    };

    let mut result = conn.exec_iter("SELECT * FROM products WHERE id = :id", params! {
        "id" => id
    }).unwrap();

    for row in result {
        let (id, name, description, stock) = from_row(row.unwrap());
        product = Product {
            id: id,
            name: name,
            description: description,
            stock: stock
        };
    }

    return product;
}

