use utoipa::{
    openapi::security::{Http, HttpAuthScheme, SecurityScheme},
    Modify, OpenApi,
};

use crate::{
    dtos::{orders::*, products::*, users::*, *},
    error::*,
    routes::{auth, orders, products, user},
    utils::status::Status,
};

/// Security scheme modifier for JWT Bearer authentication
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
            )
        }
    }
}

/// Main OpenAPI documentation for the eAPI
#[derive(OpenApi)]
#[openapi(
    paths(
        // Auth routes
        auth::login,
        auth::register,
        auth::logout,
        auth::refresh,

        // Product routes
        products::get_all,
        products::get_by_id,
        products::create,
        products::delete,

        // User routes
        user::get_me,
        user::get_by_id,
        user::get_all,
        user::delete,
        user::add_sold,

        // User sub-routes
        user::products::get_my_products,
        user::products::get_user_products,
        user::orders::get_my_orders,

        // Order routes
        orders::create,
        orders::get_by_id,
        orders::delete,
        orders::validate,
    ),
    components(
        schemas(
            // User DTOs
            RegisterUserDto,
            LoginUserDto,
            FilterUserDto,
            FilterForeignUserDto,
            UserResponseDto,
            ForeignUserResponseDto,
            UserListResponseDto,
            LoginResponseDto,
            AddSoldDto,
            // Product DTOs
            CreateProductDto,
            ProductDto,
            FilterProductDto,
            ProductResponseDto,
            FilterProductResponseDto,
            ProductListResponseDto,
            FilterProductListResponseDto,
            // Order DTOs
            CreateOrderDto,
            OrderDto,
            FilterOrderDto,
            OrderResponseDto,
            FilterOrderResponseDto,
            OrderListResponseDto,
            FilterOrderListResponseDto,
            // Common DTOs
            RequestQueryDto,
            // Error responses
            ErrorResponse,
            Response,
            // Status enum
            Status,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Authentication", description = "User authentication endpoints"),
        (name = "Users", description = "User management endpoints"),
        (name = "Products", description = "Product management endpoints"),
        (name = "Orders", description = "Order management endpoints"),
    ),
    info(
        title = "eAPI",
        description = "A comprehensive e-commerce API built with Rust and Actix Web",
        version = "1.0.0"
    ),
)]
pub struct ApiDoc;
