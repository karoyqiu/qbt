use std::{
  fs::File,
  io::{BufRead, BufReader, BufWriter},
  path::{Path, PathBuf},
};

use cookie::SameSite;
use derive_builder::Builder;
use lazy_static::lazy_static;
use reqwest_cookie_store::{CookieStore, CookieStoreMutex, RawCookie};
use serde::{Deserialize, Serialize};
use tauri::Manager;
use url::Url;

use crate::{
  app_handle::get_app_handle,
  error::{Error, IntoResult, Result},
};

lazy_static! {
  static ref COOKIE_STORE_PATHS: (PathBuf, PathBuf) = {
    let app = get_app_handle().expect("No app handle");
    let mut path = app.path().app_local_data_dir().expect("No local data dir");
    let main = path.join("cookies.json");
    path.push("cookies-merge.json");
    (main, path)
  };
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
struct EditThisCookie {
  pub domain: String,
  pub expiration_date: Option<f64>,
  pub http_only: bool,
  pub name: String,
  pub path: String,
  pub same_site: String,
  pub secure: bool,
  pub value: String,
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
    let mut store = CookieStore::new(None);
    Self::open_store(&mut store, &COOKIE_STORE_PATHS.0)?;

    if let Ok(_) = Self::open_store(&mut store, &COOKIE_STORE_PATHS.1) {
      let _ = std::fs::remove_file(&COOKIE_STORE_PATHS.1);
    }

    Ok(store)
  }

  fn open_store<P>(store: &mut CookieStore, path: P) -> Result<()>
  where
    P: AsRef<Path>,
  {
    let reader = File::open(path).map(BufReader::new).into_result()?;
    let edit_this_cookies: Vec<EditThisCookie> = serde_json::from_reader(reader).into_result()?;

    for etc in edit_this_cookies {
      let url = Url::parse(&format!("https://{}{}", etc.domain, etc.path)).into_result()?;
      let mut builder = RawCookie::build((etc.name, etc.value))
        .domain(etc.domain)
        .expires(
          etc
            .expiration_date
            .map(|exp| cookie::time::OffsetDateTime::from_unix_timestamp(exp as i64).unwrap()),
        )
        .http_only(etc.http_only)
        .path(etc.path)
        .secure(etc.secure);

      match etc.same_site.as_str() {
        "strict" => builder = builder.same_site(SameSite::Strict),
        "lax" => builder = builder.same_site(SameSite::Lax),
        "no_restriction" => builder = builder.same_site(SameSite::None),
        _ => {}
      }

      let cookie = builder.build();
      store.insert_raw(&cookie, &url).into_result()?;
    }

    Ok(())
  }

  pub fn save(&self) -> Result<()> {
    let writer = File::create(&COOKIE_STORE_PATHS.0)
      .map(BufWriter::new)
      .into_result()?;
    let store = self.store.lock().expect("Failed to lock cookie store");
    let mut etcs = vec![];

    for cookie in store.iter_unexpired() {
      let mut builder = EditThisCookieBuilder::default();
      builder
        .domain(cookie.domain().unwrap_or_default())
        .http_only(cookie.http_only().unwrap_or_default())
        .name(cookie.name())
        .path(cookie.path().unwrap_or("/"))
        .secure(cookie.secure().unwrap_or_default())
        .value(cookie.value());

      match cookie.same_site() {
        Some(SameSite::Strict) => {
          builder.same_site("strict");
        }
        Some(SameSite::Lax) => {
          builder.same_site("lax");
        }
        Some(SameSite::None) => {
          builder.same_site("no_restriction");
        }
        None => {}
      }

      etcs.push(builder.build().into_result()?);
    }

    #[cfg(debug_assertions)]
    return serde_json::to_writer_pretty(writer, &etcs).into_result();

    #[cfg(not(debug_assertions))]
    serde_json::to_writer(writer, &etcs).into_result()
  }
}

impl Drop for CookieJar {
  fn drop(&mut self) {
    self.save().unwrap();
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
