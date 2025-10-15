# eAPI - Rust Web API with JWT Authentication

A secure web API built with Rust and Actix-web framework, featuring JWT authentication, PostgreSQL database integration, and robust error handling.

## Table of Contents
- [Quick Start](#quick-start)
- [API Documentation](#api-documentation)
- [Features](#features)
  - [Authentication System](#authentication-system)
  - [User Management](#user-management)
  - [Database Integration](#database-integration)

## Quick Start

1. **Clone the repository**:
   ```bash
   git clone https://github.com/iibabyy/eAPI.git
   cd myapi
   ```

2. **Build and run the application**:
   ```bash
   make
   ```
   
   > **Note**: If you want to run it in background, you can use:
   >
   > ```bash
   > make detach
   > ```

3. **Clear the application cache**:
   ```bash
   make fclean
   ```

## API Documentation

The API is self-documented using Swagger UI. Once the application is running, you can access the interactive documentation by navigating to:

[**http://localhost:8080/docs**](http://localhost:8080/docs)

From there, you can view all available endpoints, see their request/response models, and test them directly from your browser.

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

### Database Integration
- **PostgreSQL Integration**: Robust database support using SQLx
- **Transaction Support**: Database transactions for data integrity
- **Connection Pooling**: Efficient database connection management with deadpool
