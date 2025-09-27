# MyAPI - Rust Web API with JWT Authentication

A secure web API built with Rust and Actix-web framework, featuring JWT authentication, PostgreSQL database integration, and robust error handling.

## Table of Contents
- [MyAPI - Rust Web API with JWT Authentication](#myapi---rust-web-api-with-jwt-authentication)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
    - [Authentication System](#authentication-system)
    - [User Management](#user-management)
    - [Security](#security)
    - [Database Integration](#database-integration)
  - [Technology Stack](#technology-stack)
  - [Setup and Configuration](#setup-and-configuration)
    - [Environment Variables](#environment-variables)
    - [Running the Application](#running-the-application)
  - [API Endpoints](#api-endpoints)
    - [Authentication](#authentication)
    - [Protected Routes](#protected-routes)
  - [Security Notes](#security-notes)
  - [Development](#development)
    - [Running Tests](#running-tests)
    - [Building for Production](#building-for-production)

## Features

### Authentication System
- **JWT-based Authentication**: Secure authentication using JSON Web Tokens
- **Token Refresh**: Endpoint for refreshing authentication tokens
- **Token Revocation**: Support for invalidating tokens (logout)
- **Last Active Token Tracking**: Prevents token reuse after logout

### User Management
- **User Registration**: Create new user accounts
- **User Login**: Authenticate existing users
- **Password Security**: BCrypt hashing for secure password storage

### Security
- **Input Validation**: Request data validation using the validator crate
- **Error Handling**: Comprehensive error handling with descriptive messages
- **Cookie Security**: Secure session handling

### Database Integration
- **PostgreSQL Integration**: Robust database support using SQLx
- **Transaction Support**: Database transactions for data integrity
- **Connection Pooling**: Efficient database connection management with deadpool

## Technology Stack

- **Framework**: Actix-web 4.9.0
- **Database**: PostgreSQL (via SQLx 0.8.3)
- **Authentication**: jsonwebtoken 9.3.0
- **Password Hashing**: bcrypt 0.16.0
- **Validation**: validator 0.20.0
- **Serialization**: serde 1.0.217
- **UUID**: uuid 1.4.1
- **Date/Time**: chrono 0.4.39
- **Environment**: dotenvy 0.15.7

## Setup and Configuration

### Environment Variables
The application requires the following environment variables:

```
SECRET_KEY=your_secret_key
JWT_MAX_AGE=300  # Token expiration in seconds

POSTGRES_USER=db_user
POSTGRES_PASSWORD=db_password
POSTGRES_HOST=localhost
POSTGRES_PORT=5432
POSTGRES_DB=db_name

REDIS_HOST=localhost
REDIS_PORT=6379

LISTEN=8080  # API listening port
```

### Running the Application

1. **Install Rust and Cargo**:
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone the repository**:
   ```
   git clone https://github.com/yourusername/myapi.git
   cd myapi
   ```

3. **Set up environment variables**:
   Create a `.env` file with the required variables.

4. **Install PostgreSQL and Redis**:
   Ensure PostgreSQL and Redis are installed and running.

5. **Run database migrations**:
   ```
   cargo sqlx migrate run
   ```

6. **Build and run the application**:
   ```
   cargo run
   ```

## API Endpoints

### Authentication

- **POST /auth/register**: Register a new user
  ```json
  {
    "name": "User Name",
    "email": "user@example.com",
    "password": "password",
    "password_confirm": "password"
  }
  ```

- **POST /auth/login**: Login with existing credentials
  ```json
  {
    "email": "user@example.com",
    "password": "password"
  }
  ```

- **POST /auth/logout**: Logout and invalidate current token

- **POST /auth/refresh**: Refresh authentication token

### Protected Routes

All protected routes require a valid JWT token in the Authorization header:
```
Authorization: Bearer your_jwt_token
```

## Security Notes

- JWT tokens are signed with a secure key and include user ID, issue time, expiry time, and a unique token ID
- Passwords are securely hashed using BCrypt
- The API tracks the last active token ID to prevent token reuse after logout
- Token validation includes expiration checks

## Development

### Running Tests
```
cargo test
```

### Building for Production
```
cargo build --release
```
