use actix_web::{get, post, web, App, HttpResponse, HttpServer, middleware};
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};

mod users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    users::models::create_username_index(&client).await;

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .wrap(middleware::Logger::default())
            .service(users::routes::add_user)
            .service(users::routes::get_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}