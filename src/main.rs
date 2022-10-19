use actix_web::{cookie::{time, Key}, middleware, web, App, HttpServer};
use actix_session::{config::PersistentSession, storage::RedisActorSessionStore, SessionMiddleware};
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};
use std::env;

mod users;
mod tweets;
mod session;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Loading .env into environment variable.
    dotenv::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let app_port = env::var("APP_PORT").expect("APP_PORT Must be set");
    let app_host = env::var("APP_HOST").expect("APP_PORT Must be set");
    let app_url =  format!("{}:{}", &app_host, &app_port);
    let secret_key = Key::generate();
    let client = Client::with_uri_str(uri).await.expect("failed to connect");

    users::models::create_username_index(&client).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::new("127.0.0.1:6379"),
                    secret_key.clone(),
                )
                .session_lifecycle(
                    PersistentSession::default().session_ttl(time::Duration::hours(1)),
                )
                .cookie_secure(false)
                .build(),
            )
            .wrap(middleware::Logger::default())
            .configure(config::app::config_path)
    })
    .bind(&app_url)?
    .run()
    .await
}