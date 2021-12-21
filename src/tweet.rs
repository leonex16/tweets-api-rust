use actix_web::web::{Data, Json, Path};
use actix_web::{web, HttpResponse};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use diesel::QueryDsl;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use diesel::result::Error;

use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use crate::like::{Like};
use crate::responses::Response;
use crate::{DBPooledConnection, DBPool};
pub type Tweets = Response<Tweet>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
  id: String,
  pub create_at: DateTime<Utc>,
  pub message: String,
  pub likes: Vec::<Like>
}

impl Tweet {
  pub fn new(message: String) -> Self {
    Self {
      id: Uuid::new_v4().to_string(),
      create_at: Utc::now(),
      message: message,
      likes: Vec::<Like>::new()
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetRequest {
  pub message: Option<String>
}

impl TweetRequest {
  pub fn to_tweet(&self) -> Option<Tweet> {
    match &self.message {
      Some(message) => Some(Tweet::new(message.to_string())),
      None => None
    }
  }
}

#[get("/tweets")]
pub async fn list() -> HttpResponse {
  
  let tweets = Tweets { results: vec![] };

  HttpResponse::Ok()
    .content_type(APPLICATION_JSON)
    .json(tweets)
}

#[post("/tweets")]
pub async fn create(tweet_req: Json<TweetRequest>) -> HttpResponse {
  HttpResponse::Created()
    .content_type(APPLICATION_JSON)
    .json(tweet_req.to_tweet())
}

#[get("/tweets/{id}")]
pub async fn get(path: Path<(String,)>) -> HttpResponse {
  let found_tweet: Option<Tweet> = None;

  match found_tweet {
    Some(tweet) => HttpResponse::Ok()
    .content_type(APPLICATION_JSON)
    .json(tweet),
    None => HttpResponse::NoContent()
    .content_type(APPLICATION_JSON)
    .await
    .unwrap()
  }
}

#[delete("/tweets/{id}")]
pub async fn delete(path: Path<(String,)>) -> HttpResponse {
  HttpResponse::NoContent()
  .content_type(APPLICATION_JSON)
  .await
  .unwrap() 
}

// ==================================================================
fn list_tweets(total_tweets: i64, conn: &DBPooledConnection) -> Result<Tweets, Error> {
  use crate::schema::tweets::dsl::*;

  let _tweets = match tweets
    .order(created_at.desc())
    .limit(total_tweets)
    .load::<TwwetsDB>()
    {

  }

  
}