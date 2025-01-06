use actix_web::{get, middleware::Logger, post, web::{self, Json, Path, Query}, App, HttpServer, Responder};
use serde::{ser::SerializeStruct, Deserialize, Serialize};
use uuid::Uuid;

type ActixResult<T> = Result<T, actix_web::Error>;

#[derive(Deserialize, Serialize)]
struct User {
    username: String,
    // friends: Vec<String>
}

// impl Serialize for User {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer {
        
//         let mut serialized_user = serializer.serialize_struct("User", 3)?;

//         serialized_user.serialize_field("username", &self.username)?;
//         // serialized_user.serialize_field("friends", &self.friends)?;
//         serialized_user.end()
//     }
// }

#[get("/{username}")]
async fn root(name: Path<String>) -> Json<String> {
    Json(format!("Hello {name} !"))
}

#[post("/{username}")]
async fn post_root(user: Path<String>, friend: Query<User>) -> ActixResult<String> {
    Ok(format!("username: {user}\nnew friend: {}", friend.into_inner().username))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // std::env::set_var("RUST_LOG", "actix_web=info");
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    // dotenv::dotenv().ok();

    env_logger::init();

    HttpServer::new(|| {
        App::new()
        .service(root)
        .service(post_root)
        .wrap(Logger::new("%a %r %s length: %{Content-Length}i"))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
