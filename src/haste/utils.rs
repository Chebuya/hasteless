use std::{ ffi::OsStr, path::Path, result::Result };
use worker::{ *, kv::{ KvStore, KvError } };

pub fn get_extension(path: &str) -> String {
  Path::new(path) // getting file extension
    .extension()
    .unwrap_or(OsStr::new("txt")) // if there's no extension — falling back to .txt
    .to_string_lossy()
    .into_owned()
}

pub fn get_mime(ext: &str) -> &'static str {
  let mime_type = match ext {
    "html" => "text/html",
    "css" => "text/css",
    "js" => "text/javascript",
    "json" => "application/json",
    "png" => "image/png",
    "jpg" => "image/jpeg",
    "jpeg" => "image/jpeg",
    "ico" => "image/x-icon",
    "wasm" => "application/wasm",
    _ => "text/plain",
  };

  mime_type
}

pub fn get_mime_from_path(path: &str) -> &'static str {
  get_mime(get_extension(path).as_str())
}

pub async fn get_storage(context: &RouteContext<()>) -> KvStore {
  context
    .kv(crate::STORAGE_BINDING)
    .expect(format!("No storage binding {}", crate::STORAGE_BINDING).as_str())
}

pub async fn get_value(key: &str, context: &RouteContext<()>) -> Result<Option<String>, KvError> {
  let storage = get_storage(context).await;
  storage.get(key).text().await
}

pub async fn put_value(key: &str, value: String, context: &RouteContext<()>) -> Result<(), KvError> {
  let storage = get_storage(context).await;
  match storage.put(key, value) {
    Ok(value) => {
      match value.execute().await {
        Ok(value) => Ok(value),
        Err(err) => Err(err)
      }
    }
    Err(err) => Err(err)
  }
}