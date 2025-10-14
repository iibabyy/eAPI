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
  - [Quick Start](#quick-start)
  - [Security Notes](#security-notes)

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

## Quick Start

1. **Clone the repository**:
   ```
   git clone https://github.com/iibabyy/eAPI.git
   cd myapi
   ```

2. **Build and run the application**:
   ```
   make
   ```
   
   > **Note**: If you want to run it in the background, you can use:
   >
   > ```bash
   > make detach
   > ```

3. **Clean application cache**:
   ```
   make fclean
   ```

## Security Notes

- JWT tokens are signed with a secure key and include user ID, issue time, expiry time, and a unique token ID
- Passwords are securely hashed using BCrypt
- The API tracks the last active token ID to prevent token reuse after logout
- Token validation includes expiration checks