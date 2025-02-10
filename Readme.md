# RESTful API with Actix-web and PostgreSQL

## Description
This project is a RESTful API built in Rust using the Actix-web framework and PostgreSQL as the database. It provides a secure and scalable platform for managing users, products, and orders. The API supports JWT authentication and enforces strict access controls to ensure data privacy and security.

---

## Features

### Users, Products, and Orders
- **User Management**: Users can create accounts and manage their profiles.
- **Product Management**: Users can create and list products.
- **Order Management**: Users can place orders for products created by other users.

### Security
- **JWT Authentication**: Utilizes access and refresh tokens for secure user sessions.
- **User Security**: Implements restricted actions to ensure privacy and data protection. Users cannot modify or access other users' accounts, products, or orders.

---

## Setup Instructions

### Prerequisites
- Rust (latest stable version)
- Cargo (Rust's package manager)

### Instructions
1. Start the Postgresql container:
	```bash
	docker compose up -d
2. Install sqlx and run migrations:
   ```bash
   cargo install sqlx-cli
   cargo sqlx migrate run
3. Build and run the project:
   ```bash
   cargo build --release
   cargo run --release

### Cleaning
1. Stop the Postgresql container:
   ```bash
   docker compose down

2. Cleaning the project:
   ```bash
   cargo clean