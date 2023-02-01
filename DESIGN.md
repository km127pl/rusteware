# Design Documentation for Rusteware

## Backend
- [Introduction](#introduction)
- [API](#application-user-interface)
  - [Endpoints](#api-endpoints)
- Design
  - [Database](#database-design)
    - [Tables](#table---products)
  - [Backend](#server-design)

### Introduction
This document outlines the design for a Warehouse Inventory Management Software which will be used to manage inventory levels, and generate reports for a warehouse. The software will have a backend server that will be written in Rust and will use a MySQL database to store data.

### Application-User Interface
The API for the Warehouse Inventory Management Software will be designed to provide access to the backend server and the database. The API will allow the client-side of the software to communicate with the server and retrieve or update information stored in the database.

#### API Endpoints
- `GET /products` This endpoint will retrieve a list of all the - products stored in the database.
- `GET /products/:id` This endpoint will retrieve the details of a - single product based on its id.
- `POST /products/:id` This endpoint will add a new product to the - database.
- `PUT /products/:id` This endpoint will update the details of an - existing product in the database.
- `DELETE /products/:id` This endpoint will delete a product from - the database based on its id.

#### Server Design
The backend server will be written in Rust and will use a MySQL database to store data. The server will handle all communication with the database and will provide an API for the client-side of the software to access the data stored in the database.

#### Database Design
The MySQL database will be used to store information about products. 

#### Table - products

- id: unique identifier for each product
- name: name of the product
- description: description of the product
- quantity: current quantity of the product in the warehouse
- unit_price: price of the product per unit

#### Table - orders

- id: unique identifier for each product
- product_id: the id of the product
- quantity: current quantity of the product in the warehouse
- date: date of order creation
- a reference to the product