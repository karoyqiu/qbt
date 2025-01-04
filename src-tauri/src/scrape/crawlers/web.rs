use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};

use lazy_static::lazy_static;
use log::{debug, trace};
use reqwest::{Client, ClientBuilder, Proxy, Response};
use scraper::Selector;
use serde::Serialize;
use tauri_plugin_store::StoreExt;

use crate::{
  app_handle::get_app_handle,
  error::{err, Error, IntoResult, Result},
};

lazy_static! {
  pub static ref DIV_SELECTOR: Selector = Selector::parse("div").unwrap();
  pub static ref A_SELECTOR: Selector = Selector::parse("A").unwrap();
  pub static ref H3_SELECTOR: Selector = Selector::parse("h3").unwrap();
  static ref SELECTORS: Mutex<HashMap<&'static str, Arc<Selector>>> = Mutex::new(HashMap::new());
}

#[derive(Debug, serde::Deserialize)]
struct StringValue {
  value: String,
}

fn apply_proxy(builder: ClientBuilder) -> Result<ClientBuilder> {
  let app = get_app_handle().ok_or(Error(anyhow::anyhow!("App handle not found")))?;
  let store = app.store("settings.json").into_result()?;
  let proxy = store.get("proxy");

  if let Some(proxy) = proxy {
    let proxy: StringValue = serde_json::from_value(proxy).into_result()?;
    let proxy = proxy.value;

    match proxy.as_str() {
      "<system>" | "" => Ok(builder),
      "<direct>" => Ok(builder.no_proxy()),
      _ => Ok(builder.proxy(Proxy::all(&proxy).into_result()?)),
    }
  } else {
    Ok(builder)
  }
}

/// 获取客户端
pub fn get_client() -> Result<Client> {
  apply_proxy(ClientBuilder::new().cookie_store(true))?
    .build()
    .into_result()
}

/// 获取HTML
pub async fn get_html(url: &str) -> Result<String> {
  debug!("Getting HTML from {}", url);
  let client = get_client()?;
  let mut req = client.get(url);

  if url.contains("getchu") {
    req = req.header("Referer", "http://www.getchu.com/top.html");
  } else if url.contains("xcity") {
    req = req.header(
      "Referer",
      "https://xcity.jp/result_published/?genre=%2Fresult_published%2F&q=2&sg=main&num=60",
    );
  } else if url.contains("javbus") {
    // javbus封面图需携带refer，refer似乎没有做强校验，但须符合格式要求，否则403
    req = req.header("Referer", "https://www.javbus.com/");
  } else if url.contains("giga") && !url.contains("cookie_set.php") {
    // 搜索时需要携带refer，获取cookies时不能携带
    req = req.header("Referer", "https://www.giga-web.jp/top.html");
  }

  let res = req.send().await.into_result()?;
  get_response_text(res).await
}

/// 提交 HTML
pub async fn post_html<F>(url: &str, form: &F) -> Result<String>
where
  F: Serialize + ?Sized,
{
  debug!("Posting HTML to {}", url);
  let client = get_client()?;
  let resp = client.post(url).form(form).send().await.into_result()?;
  get_response_text(resp).await
}

/// 获取响应文本
pub async fn get_response_text(res: Response) -> Result<String> {
  let status = res.status();
  let body = res.text().await.into_result()?;

  if status.is_success() {
    Ok(body)
  } else {
    trace!("Failed to get HTML: {}, {}", status, body);
    err("Failed to get HTML")
  }
}

pub fn get_selector(selector: &'static str) -> Arc<Selector> {
  let mut selectors = SELECTORS.lock().unwrap();

  if let Some(sel) = selectors.get(selector) {
    sel.clone()
  } else {
    let sel = Arc::new(Selector::parse(selector).unwrap());
    selectors.insert(selector, sel.clone());
    sel
  }
}
