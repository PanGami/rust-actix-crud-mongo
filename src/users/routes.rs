use actix_web::{get, post, web, App, HttpResponse, HttpServer, middleware};
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};
use super::models::User;

const DB_NAME: &str = "testdb";
const COLL_NAME: &str = "users";

pub async fn create(client: web::Data<Client>, payload: web::Json<User>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let result = collection.insert_one(payload.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_user(client: web::Data<Client>, username: web::Path<String>) -> HttpResponse {
    let username = username.into_inner();
    let collection: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);
    match collection
        .find_one(doc! { "username": &username }, None)
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => {
            HttpResponse::NotFound().body(format!("No user found with username {username}"))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}