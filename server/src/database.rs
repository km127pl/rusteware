use mysql::*;
use mysql::prelude::*;

use crate::config::Config;

#[derive(Debug)]
pub struct Product {
    pub(crate) id: i32,
    pub name: String,
    pub description: String,
    pub stock: i32
}

#[derive(Debug)]
pub struct Order {
   pub(crate) id: i32,
   pub product_id: i32,
   pub quantity: i32,
   pub date: String,
   pub product: Product
}

pub fn db_connect(config: Config) -> Pool {
    println!("DEBUG: Connecting to database: mysql://{}:{}@{}:{}/{}", config.database.user, config.database.password, config.database.host, config.database.port, config.database.name);
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

    let result = conn.exec_iter("SELECT * FROM products WHERE id = :id", params! {
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

// create a new product
pub fn create_product(db: &Pool, name: &str, description: &str, stock: i32) {
    let mut conn = get_conn(db);
    conn.exec_iter("INSERT INTO products (name, description, stock) VALUES (:name, :description, :stock)", params! {
        "name" => name,
        "description" => description,
        "stock" => stock
    }).expect("Failed to create product");
}

// get last inserted id
pub fn get_last_insert_id(db: &Pool) -> i32 {
    let mut conn = get_conn(db);
    let result = conn.exec_iter("SELECT `id` FROM `products` ORDER BY id DESC LIMIT 1", ()).unwrap();
    let mut _id: i32 = 0;

    for row in result {
        let id : i32 = from_row(row.unwrap());
        _id = id;
    }

    return _id;
}


// update a product
pub fn update_product(db: &Pool, id: i32, name: &str, description: &str, stock: i32) {
    let mut conn = get_conn(db);

    conn.exec_iter("UPDATE products SET name = :name, description = :description, stock = :stock WHERE id = :id", params! {
        "id" => id,
        "name" => name,
        "description" => description,
        "stock" => stock
    }).expect("Failed to update product");
}

// delete a product
pub fn delete_product(db: &Pool, id: i32) {
    let mut conn = get_conn(db);
    conn.exec_iter("DELETE FROM products WHERE id = :id", params! {
        "id" => id
    }).expect("Failed to delete product");
}

// get all products
pub fn get_all_products(db: &Pool) -> Vec<Product> {
    let mut conn = get_conn(db);
    let mut products: Vec<Product> = Vec::new();

    let result = conn.exec_iter("SELECT * FROM products", ()).unwrap();

    for row in result {
        let (id, name, description, stock) = from_row(row.unwrap());
        products.push(Product {
            id: id,
            name: name,
            description: description,
            stock: stock
        });
    }

    return products;
}