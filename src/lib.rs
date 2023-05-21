use worker::*;
use serde::{Deserialize, Serialize};
use chrono::Utc;
mod haste;

pub static STORAGE_BINDING: &str = "HASTE_POSTS";
pub static MAX_FILE_SIZE: usize = 65536;
pub static TTL: i64 = 86400;

#[derive(Debug, Deserialize, Serialize)]
pub struct Haste {
  pub timestamp: u64,
  pub data: String,
}

#[event(fetch)]
pub async fn main(_req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
  Router::new()
    .get_async("/",                haste::get)
    .get_async("/:id",             haste::get)
    .get_async("/documents/",      haste::get)
    .get_async("/documents/:id",   haste::get_document)
    .get_async("/raw/:id",         haste::get_document) 
    .post_async("/documents",      haste::post)
    .run(_req, env).await
}

#[event(scheduled)]
pub async fn main(_: ScheduledEvent, env: Env, _: ScheduleContext) {
  let storage = env.kv(STORAGE_BINDING).unwrap();
  let hastes = storage.list().execute().await.unwrap();

  for name in hastes.keys {
    let haste = storage.get(name.name.as_str()).text().await.unwrap().unwrap();
    let file: Haste = serde_json::from_str(haste.as_str()).unwrap();
    let current_epoch = Utc::now().timestamp();
    
    if current_epoch >= i64::try_from(file.timestamp).unwrap() + TTL {
      storage.delete(name.name.as_str()).await.unwrap();
    }
  }
}