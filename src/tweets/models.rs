use serde::{Deserialize, Serialize};
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};


#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Tweet {
  pub username: i32,
  pub message: String,
  pub created_at: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct TweetPayload {
  pub message: String,
}
