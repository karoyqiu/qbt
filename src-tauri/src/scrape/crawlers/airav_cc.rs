use scraper::{ElementRef, Html};
use serde::Deserialize;
use url::Url;

use crate::{
  error::{err, Result},
  scrape::{Actress, TranslatedText},
};

use super::{
  crawler::{convert_datetime_string_to_epoch, Crawler},
  web::get_selector,
};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VideoObject {
  thumbnail_url: String,
}

#[derive(Default)]
pub struct AiravCc;

// 现在貌似绕不过去 cloudflare 的检测
impl Crawler for AiravCc {
  fn get_name(&self) -> &'static str {
    "airav.io"
  }

  fn get_url(&self, code: &String) -> Result<String> {
    Ok(format!("https://airav.io/search_result?kw={}", code))
  }

  fn get_next_url(&self, url: &Url, html: &String) -> Option<String> {
    if !url.path().contains("search_result") {
      return None;
    }

    let html = Html::parse_document(html);
    let one_video = get_selector("div.col.oneVideo");
    let a = get_selector("a");
    let h5 = get_selector("h5");
    let mut url = None;

    for elem in html.select(&one_video) {
      if let Some(h5) = elem.select(&h5).next() {
        let text: String = h5.text().collect();

        if !text.contains("克破") && !text.contains("无码破解") && !text.contains("無碼破解")
        {
          if let Some(a) = elem.select(&a).next() {
            if let Some(href) = a.value().attr("href") {
              url = Some(href.to_string());
              break;
            }
          }
        }
      }
    }

    url
  }

  fn get_title(&self, doc: &Html) -> Result<String> {
    let selector = get_selector("h1");

    if let Some(elem) = doc.select(&selector).next() {
      Ok(elem.text().collect())
    } else {
      err("Title not found")
    }
  }

  fn get_cover(&self, doc: &Html) -> Option<String> {
    let selector = get_selector("script[type=\"application/ld+json\"]");
    let script = doc.select(&selector).next()?;
    let text: String = script.text().collect();
    let video_object: VideoObject = serde_json::from_str(&text).ok()?;
    Some(video_object.thumbnail_url)
  }

  fn get_outline(&self, doc: &Html) -> Option<TranslatedText> {
    let selector = get_selector("div.video-info > p");
    let elem = doc.select(&selector).next()?;
    Some(TranslatedText {
      text: elem.text().collect(),
      translated: None,
    })
  }

  fn get_actresses(&self, doc: &Html) -> Option<Vec<Actress>> {
    let actresses = get_info_list_items(doc, "女優");
    let actresses = actresses.map(|a| {
      a.iter()
        .map(|a| Actress {
          name: a.clone(),
          photo: None,
        })
        .collect::<Vec<_>>()
    });
    actresses
  }

  fn get_tags(&self, doc: &Html) -> Option<Vec<String>> {
    get_info_list_items(doc, "標籤")
  }

  fn get_series(&self, doc: &Html) -> Option<String> {
    let mut series = get_info_list_items(doc, "系列")?;
    series.pop()
  }

  fn get_studio(&self, doc: &Html) -> Option<String> {
    let mut studios = get_info_list_items(doc, "廠商")?;
    studios.pop()
  }

  fn get_release_date(&self, doc: &Html) -> Option<i64> {
    let fa = get_selector("i.fa.fa-clock");
    let i = doc.select(&fa).next()?;
    let parent = i.parent()?;
    let parent = ElementRef::wrap(parent)?;
    let text: String = parent.text().collect();
    convert_datetime_string_to_epoch(&text, None)
  }
}

fn get_info_list_items(doc: &Html, label: &str) -> Option<Vec<String>> {
  let li = get_selector("li");
  let a = get_selector("a");
  let mut items = vec![];

  for elem in doc.select(&li) {
    let text: String = elem.text().collect();

    if text.starts_with(label) {
      for a in elem.select(&a) {
        items.push(a.text().collect());
      }
    }
  }

  if items.is_empty() {
    None
  } else {
    Some(items)
  }
}
