mod assets;
mod utils;
use worker::*;
use bip39::Mnemonic;
use rand::Rng;
use std::result::Result;
use chrono::Utc;
use crate::MAX_FILE_SIZE;
use crate::Haste;

use self::utils::*;
use self::assets::*;
use serde_json::*;

pub async fn get(_request: Request, context: RouteContext<()>) -> worker::Result<Response> {
  let mut asset = context.param("id")
    .map(String::as_str)
    .unwrap_or("index.html"); // asset or falling back to index.html
  let mut headers = Headers::new();

  if ASSETS.contains_key(asset) {
    headers.set("Content-Type", get_mime_from_path(asset)).unwrap();
  } else {
    headers.set("Content-Type", "text/html").unwrap();
    asset = "index.html"; // in case if it's the document request, we should fall back to index.html because after it will request a file itself, e.g. http://localhost:8080/filename -> at first it'll return index.html, but then it has to return http://localhost:8080/filename -> filename (application/json)
  }

  match ASSETS.get(asset).unwrap() {
    AssetType::Bytes(asset_raw) => {
      Ok(Response::from_bytes(asset_raw.to_vec())?.with_headers(headers))
    }
    AssetType::Str(asset_raw) => {
      Ok(Response::ok(asset_raw.to_owned())?.with_headers(headers))
    }
  }
}

pub async fn post(mut request: Request, context: RouteContext<()>) -> worker::Result<Response> {
  let value = request.text().await.unwrap();

  if value.as_bytes().len() >= MAX_FILE_SIZE {
    return Response::error("File is too large", 413);
  }

  let file = json!({
    "data": value,
    "timestamp": Utc::now().timestamp()
  }).to_string();

  let mut rng = rand::thread_rng(); // Generating random mnemonic string e.g. "put crack ocean"
  let mut bytes = vec![0u8; 16];
  rng.fill(&mut bytes[..]);
  let mnemonic = Mnemonic::from_entropy(&bytes[..]).unwrap();
  let mut filename = String::new();

  for (_, word) in mnemonic.word_iter().take(3).enumerate() {
    filename.push_str(word);
  };

  match put_value(filename.as_str(), file, &context).await {
    Ok(_) => {
      let mut headers = Headers::new();
      headers.set("Content-Type", "application/json").unwrap();
      let response = json!({"key": filename}).to_string();
      Ok(Response::ok(response)?.with_headers(headers))
    }
    Err(err) => {
      Response::error(err.to_string(), 500)
    }
  }
}

async fn get_file(asset: String, context: &RouteContext<()>) -> Result<Option<(String, String )>, String> {
  let asset_no_ext = asset
    .split(".")
    .next()
    .unwrap();

  match get_value(asset_no_ext, &context).await {
    Ok(value) => {
      match value {
        Some(body) => {
          let file: Haste = serde_json::from_str(body.as_str()).unwrap();
          Ok(Some((file.data, asset_no_ext.to_string())))
        },
        _ => Ok(None)
      }
    },
    Err(err) => {
      Err(err.to_string())
    }
  }
}

pub async fn get_document(request: Request, context: RouteContext<()>) -> worker::Result<Response> {
  let asset = context.param("id").map(String::as_str).unwrap_or("aboud.md");
  let mut headers = Headers::new();

  if asset == "about" {
    headers.set("Content-Type", "application/json").unwrap();
    return Ok(Response::ok(json!({"data": ABOUT_MD, "key": "about"}).to_string())?.with_headers(headers));
  }

  match get_file(asset.to_string(), &context).await {
    Ok(result) => {
      match result {
        Some(body) => {
          let response: String;

          if request.path().starts_with("/documents/") {
            headers.set("Content-Type", "application/json").unwrap();
            response = json!({"data": body.0, "key": body.1}).to_string();
          } else { // if starts with /raw/
            headers.set("Content-Type", "text/plain").unwrap();
            response = body.0;
          };

          Ok(Response::ok(response)?.with_headers(headers))
        }
        _ => Response::error("Not found", 404)
      }
    },
    Err(err) => {
      Response::error(err, 500)
    }
  }
}