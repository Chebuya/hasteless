use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug)]
pub enum AssetType<'a> {
  Bytes(&'a [u8]),
  Str(&'a str),
}

pub static ABOUT_MD: &str = include_str!("../static/about.md");

lazy_static! {
  pub static ref ASSETS: HashMap<String, AssetType<'static>> = { // storing assets directly in worker's WASM binary
      let mut m = HashMap::new();
      m.insert(String::from("application.css"), AssetType::Str(include_str!("../static/application.css")));
      m.insert(String::from("application.min.js"), AssetType::Str(include_str!("../static/application.min.js")));
      m.insert(String::from("favicon.ico"), AssetType::Bytes(include_bytes!("../static/favicon.ico")));
      m.insert(String::from("function-icons.png"), AssetType::Bytes(include_bytes!("../static/function-icons.png")));
      m.insert(String::from("hover-dropdown-tip.png"), AssetType::Bytes(include_bytes!("../static/hover-dropdown-tip.png")));
      m.insert(String::from("index.html"), AssetType::Str(include_str!("../static/index.html")));
      m.insert(String::from("logo.png"), AssetType::Bytes(include_bytes!("../static/logo.png")));
      m.insert(String::from("robots.txt"), AssetType::Str(include_str!("../static/robots.txt")));
      m.insert(String::from("solarized_dark.css"), AssetType::Str(include_str!("../static/solarized_dark.css")));
      m
  };
}