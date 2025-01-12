use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
  time::Duration,
};

use lazy_static::lazy_static;
use log::{debug, trace};
use reqwest::{
  header::{ACCEPT, ACCEPT_LANGUAGE, CONNECTION, DNT, UPGRADE_INSECURE_REQUESTS},
  Client, ClientBuilder, Proxy, Response,
};
use scraper::Selector;
use tauri::http::{HeaderMap, HeaderName, HeaderValue};
use tauri_plugin_store::StoreExt;
use translators::GoogleTranslator;
use url::Url;

use crate::{
  app_handle::get_app_handle,
  error::{err, Error, IntoResult, Result},
};

use super::cookie_jar::CookieJar;

lazy_static! {
  static ref SELECTORS: Mutex<HashMap<&'static str, Arc<Selector>>> = Mutex::new(HashMap::new());
}

#[derive(Debug, serde::Deserialize)]
struct StringValue {
  value: String,
}

pub fn get_proxy() -> Result<Option<String>> {
  let app = get_app_handle().ok_or(Error(anyhow::anyhow!("App handle not found")))?;
  let store = app.store("settings.json").into_result()?;
  let proxy = store.get("proxy");

  if let Some(proxy) = proxy {
    let proxy: StringValue = serde_json::from_value(proxy).into_result()?;
    Ok(Some(proxy.value))
  } else {
    Ok(None)
  }
}

fn apply_proxy(builder: ClientBuilder) -> Result<ClientBuilder> {
  if let Some(proxy) = get_proxy()? {
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
  let mut headers = HeaderMap::new();
  headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
  headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("zh-CN,zh;q=0.9"));
  headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
  headers.insert(DNT, HeaderValue::from_static("1"));
  headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));
  headers.insert(
    HeaderName::from_static("sec-ch-ua"),
    HeaderValue::from_static(
      r#""Microsoft Edge";v="131", "Chromium";v="131", "Not_A Brand";v="24""#,
    ),
  );
  headers.insert(
    HeaderName::from_static("sec-ch-ua-mobile"),
    HeaderValue::from_static("?0"),
  );
  headers.insert(
    HeaderName::from_static("sec-ch-ua-platform"),
    HeaderValue::from_static(r#""Windows""#),
  );
  headers.insert(
    HeaderName::from_static("sec-fetch-dest"),
    HeaderValue::from_static("document"),
  );
  headers.insert(
    HeaderName::from_static("sec-fetch-mode"),
    HeaderValue::from_static("navigate"),
  );
  headers.insert(
    HeaderName::from_static("sec-fetch-site"),
    HeaderValue::from_static("none"),
  );
  headers.insert(
    HeaderName::from_static("sec-fetch-user"),
    HeaderValue::from_static("?1"),
  );

  let store = Arc::new(CookieJar::new());
  let client = apply_proxy(
    ClientBuilder::new()
      .timeout(Duration::from_secs(30))
      .cookie_provider(store)
      .default_headers(headers)
      .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0")
  )?
  .build()
  .into_result()?;

  Ok(client)
}

/// 获取响应
pub async fn get_response(url: &str) -> Result<Response> {
  debug!("Getting {}", url);
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

  req.send().await.into_result()
}

/// 获取 HTML
pub async fn get_html(url: &str) -> Result<(String, Url)> {
  let res = get_response(url).await?;
  get_response_text(res).await
}

// /// 提交 HTML
// pub async fn post_html<F>(url: &str, form: &F) -> Result<(String, Url)>
// where
//   F: Serialize + ?Sized,
// {
//   debug!("Posting HTML to {}", url);
//   let client = get_client()?;
//   let resp = client.post(url).form(form).send().await.into_result()?;
//   get_response_text(resp).await
// }

/// 获取响应文本
async fn get_response_text(res: Response) -> Result<(String, Url)> {
  let status = res.status();
  let url = res.url().clone();
  let body = res.text().await.into_result()?;

  if status.is_success() {
    //trace!("Got HTML: {} - {}", url, body);
    Ok((body, url))
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

pub fn get_translator() -> Result<GoogleTranslator> {
  let app = get_app_handle().ok_or(Error(anyhow::anyhow!("App handle not found")))?;
  let store = app.store("settings.json").into_result()?;
  let proxy = store.get("proxy");

  if let Some(proxy) = proxy {
    let proxy: StringValue = serde_json::from_value(proxy).into_result()?;
    let proxy = proxy.value;

    if !proxy.is_empty() && proxy != "<system>" && proxy != "<direct>" {
      return Ok(GoogleTranslator::builder().proxy_address(proxy).build());
    }
  }

  Ok(GoogleTranslator::builder().build())
}

pub fn remove_first<T>(mut v: Vec<T>) -> Option<Vec<T>> {
  if v.len() <= 1 {
    None
  } else {
    v.remove(0);

    if v.is_empty() {
      None
    } else {
      Some(v)
    }
  }
}
