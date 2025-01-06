use std::{
  fs::File,
  io::{BufReader, BufWriter},
  path::{Path, PathBuf},
};

use cookie_store::{Cookie, CookieStore, RawCookie};
use lazy_static::lazy_static;
use reqwest_cookie_store::CookieStoreMutex;
use serde::{Deserialize, Serialize};
use tauri::Manager;

use crate::{
  app_handle::get_app_handle,
  error::{Error, IntoResult, Result},
};

lazy_static! {
  static ref COOKIE_STORE_PATH: PathBuf = {
    let app = get_app_handle().expect("No app handle");
    let mut path = app.path().app_local_data_dir().expect("No local data dir");
    path.push("cookies.json");
    path
  };
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EditThisCookie {
  pub domain: String,
  pub expiration_date: Option<f64>,
  pub host_only: bool,
  pub http_only: bool,
  pub name: String,
  pub path: String,
  pub same_site: String,
  pub secure: bool,
  pub session: bool,
  pub store_id: String,
  pub value: String,
  pub id: i64,
}

#[derive(Default)]
pub struct CookieJar {
  store: CookieStoreMutex,
}

impl CookieJar {
  pub fn new() -> Self {
    let store = Self::open().unwrap_or_default();
    let store = CookieStoreMutex::new(store);
    CookieJar { store }
  }

  fn open() -> Result<CookieStore> {
    let reader = File::open::<&Path>(COOKIE_STORE_PATH.as_ref())
      .map(BufReader::new)
      .into_result()?;
    cookie_store::serde::json::load(reader).map_err(|_| Error(anyhow::anyhow!("Failed to load")))
  }
}

impl Drop for CookieJar {
  fn drop(&mut self) {
    let mut writer = File::create::<&Path>(COOKIE_STORE_PATH.as_ref())
      .map(BufWriter::new)
      .expect("Failed to open cookie store file");
    let store = self.store.lock().expect("Failed to lock cookie store");
    cookie_store::serde::json::save(&store, &mut writer).expect("Failed to save cookie store");
  }
}

impl reqwest::cookie::CookieStore for CookieJar {
  fn set_cookies(
    &self,
    cookie_headers: &mut dyn Iterator<Item = &tauri::http::HeaderValue>,
    url: &url::Url,
  ) {
    self.store.set_cookies(cookie_headers, url);
  }

  fn cookies(&self, url: &url::Url) -> Option<tauri::http::HeaderValue> {
    self.store.cookies(url)
  }
}
