CREATE DATABASE rusteware;

USE rusteware;

CREATE TABLE products (
  id INT AUTO_INCREMENT PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  description TEXT NOT NULL,
  stock INT NOT NULL
);

CREATE TABLE orders (
  id INT AUTO_INCREMENT PRIMARY KEY,
  product_id INT NOT NULL,
  quantity INT NOT NULL,
  date DATE NOT NULL,
  FOREIGN KEY (product_id) REFERENCES products(id)
);