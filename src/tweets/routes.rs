use actix_web::{web, Error, HttpResponse};
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};
use super::{TweetPayload, Tweet};

const DB_NAME: &str = "testdb";
const COLL_NAME: &str = "tweets";

pub async fn create(client: web::Data<Client>, payload: web::Json<TweetPayload>) -> HttpResponse {
  let collection = client.database(DB_NAME).collection(COLL_NAME);
  let result = collection.insert_one(payload.into_inner(), None).await;
  match result {
      Ok(_) => HttpResponse::Ok().body("user added"),
      Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
  }
}

// async fn index(client: web::Data<Client>) -> HttpResponse {
//   let collection: Collection<Tweet> = client.database(DB_NAME).collection(COLL_NAME);
//   let result = collection.find(doc! {}, None).await;
//   match result {
//       Ok(Some(tweet)) => HttpResponse::Ok().body(tweet),
//       Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//   }
// }

// pub async fn show(id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
//   let tweet = web::block(move || {
//     let conn = pool.get()?;
//     find_by_id(id.into_inner(), &conn)
//   })
//   .await?
//   .map_err(actix_web::error::ErrorInternalServerError)?;

//   Ok(HttpResponse::Ok().json(tweet))
// }

// pub async fn update(
//   id: web::Path<i32>,
//   payload: web::Json<TweetPayload>,
//   pool: web::Data<DbPool>,
// ) -> Result<HttpResponse, Error> {
//   let tweet = web::block(move || {
//     let conn = pool.get()?;
//     update_tweet(id.into_inner(), payload.message.clone(), &conn)
//   })
//   .await?
//   .map_err(actix_web::error::ErrorInternalServerError)?;

//   Ok(HttpResponse::Ok().json(tweet))
// }

// pub async fn destroy(id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
//   let result = web::block(move || {
//     let conn = pool.get()?;
//     delete_tweet(id.into_inner(), &conn)
//   })
//   .await?
//   .map(|tweet| HttpResponse::Ok().json(tweet))
//   .map_err(actix_web::error::ErrorInternalServerError)?;

//   Ok(result)
// }