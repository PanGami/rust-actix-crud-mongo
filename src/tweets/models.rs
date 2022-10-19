use serde::{Deserialize, Serialize};
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};


#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Tweet {
  pub username: String,
  pub message: String,
}
