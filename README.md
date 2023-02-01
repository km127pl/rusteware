Rusteware
=========

A warehouse inventory management software that helps you keep track of your inventory levels, orders, and generate reports. The software is built with Rust and uses a MySQL database to store data. With Rusteware, you can:

- Manage inventory levels for your products
- Track orders and keep a record of all past orders
- Generate reports for inventory levels and orders
- Easily access and update product and order information

Getting Started
---------------

1. Clone the repository:
   git clone https://github.com/km127pl/rusteware.git

2. Install the dependencies:
   cd rusteware
   cargo install

3. Set up the MySQL database:
   create a database and import the schema from the `server/schema.sql` file

4. Update the database configuration in the `server/config.toml` file

5. Start the server:
   cargo run

6. Use the API endpoints to access the data stored in the database.

API Endpoints
-------------

All of the API endpoints and more can be seen in the [Design Documentation](DESIGN.md)

Requirements
------------

- Rust v1.67 or higher
- MySQL v8.0.32  or MariaDB v11.0.0 or higher

License
-------

Rusteware is released under the [MIT License](https://opensource.org/licenses/MIT).
