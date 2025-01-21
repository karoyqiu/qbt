use std::sync::Mutex;

use lazy_static::lazy_static;
use scraper::{ElementRef, Html};
use url::Url;

use crate::{
  error::{err, Result},
  scrape::{Actress, TranslatedText},
};

use super::{
  crawler::{convert_date_string_to_epoch, convert_duration_string_to_seconds, Crawler},
  web::get_selector,
};

lazy_static! {
  static ref LAST_DURATION: Mutex<i64> = Mutex::new(0);
}

#[derive(Default)]
pub struct Iqqtv;

impl Crawler for Iqqtv {
  fn name(&self) -> &'static str {
    "iqqtv"
  }

  fn language(&self) -> &'static str {
    "zh-CN"
  }

  fn get_url(&self, code: &String) -> Result<String> {
    Ok(format!(
      "https://iqq5.xyz/cn/search.php?kw_type=key&kw={}",
      code
    ))
  }

  fn get_next_url(&self, code: &String, url: &Url, html: &String) -> Option<String> {
    if !url.path().contains("search.php") {
      return None;
    }

    let html = Html::parse_document(html);
    let span = get_selector("span.title");

    for span in html.select(&span) {
      let text: String = span.text().collect();

      if text.contains(code) && is_meaningful(&text) {
        let a = get_selector("a");

        if let Some(a) = span.select(&a).next() {
          let href = a.attr("href")?.to_string();
          let _ = get_video_time(&span);
          return Some(href);
        }
      }
    }

    None
  }

  fn get_title(&self, doc: &Html) -> Result<String> {
    let selector = get_selector("h1.h4.b");

    if let Some(elem) = doc.select(&selector).next() {
      Ok(elem.text().collect())
    } else {
      err("Title not found")
    }
  }

  fn get_info_builder(&self, doc: &Html) -> crate::scrape::VideoInfoBuilder {
    let mut builder = crate::scrape::VideoInfoBuilder::default();

    builder
      .poster(self.get_poster(&doc))
      .cover(self.get_cover(&doc))
      .outline(self.get_outline(&doc))
      .actresses(self.get_actresses(&doc))
      .tags(self.get_tags(&doc))
      .series(self.get_series(&doc))
      .studio(self.get_studio(&doc))
      .publisher(self.get_publisher(&doc))
      .director(self.get_director(&doc))
      .duration(self.get_duration(&doc))
      .release_date(self.get_release_date(&doc))
      .extra_fanart(self.get_extra_fanart(&doc));

    builder
  }

  fn get_cover(&self, doc: &Html) -> Option<String> {
    let meta = get_selector("meta[property=\"og:image\"]");
    doc
      .select(&meta)
      .next()
      .map(|meta| meta.attr("content").map(String::from))?
  }

  fn get_outline(&self, doc: &Html) -> Option<TranslatedText> {
    let div = get_selector("div.intro");
    doc.select(&div).next().map(|div| {
      let text: String = div.text().collect();
      let text = text.replace("简介：", "").replace("簡介：", "");
      TranslatedText::text(text.trim())
    })
  }

  fn get_actresses(&self, doc: &Html) -> Option<Vec<Actress>> {
    let a = get_selector("div.tag-info > a[href*=actor]");
    let mut actresses = vec![];

    for a in doc.select(&a) {
      actresses.push(Actress::name(a.text().collect::<String>()));
    }

    if actresses.is_empty() {
      None
    } else {
      Some(actresses)
    }
  }

  fn get_tags(&self, doc: &Html) -> Option<Vec<String>> {
    let a = get_selector("div.tag-info > a[href*=tag]");
    let mut tags = vec![];

    for a in doc.select(&a) {
      tags.push(a.text().collect());
    }

    if tags.is_empty() {
      None
    } else {
      Some(tags)
    }
  }

  fn get_series(&self, doc: &Html) -> Option<String> {
    let a = get_selector("a[href*=series]");
    doc
      .select(&a)
      .next()
      .map(|div| div.text().collect::<String>())
  }

  fn get_studio(&self, doc: &Html) -> Option<String> {
    let div = get_selector("div.company");
    doc.select(&div).next().map(|div| {
      let text: String = div.text().collect();
      text.trim().to_string()
    })
  }

  fn get_duration(&self, _doc: &Html) -> Option<i64> {
    let mut lock = LAST_DURATION.lock().unwrap();
    let value = *lock;

    if value > 0 {
      *lock = 0;
      Some(value)
    } else {
      None
    }
  }

  fn get_release_date(&self, doc: &Html) -> Option<i64> {
    let div = get_selector("div.date");
    doc.select(&div).next().map(|div| {
      let text: String = div.text().collect();
      convert_date_string_to_epoch(text.trim(), None)
    })?
  }

  fn get_extra_fanart(&self, doc: &Html) -> Option<Vec<String>> {
    let img = get_selector("div.cover img");
    let mut arts = vec![];

    for img in doc.select(&img) {
      if let Some(src) = img.attr("data-src") {
        arts.push(src.to_string());
      }
    }

    if arts.is_empty() {
      None
    } else {
      Some(arts)
    }
  }
}

fn is_meaningful(text: &str) -> bool {
  static MEANINGLESS_WORDS: &[&str] = &["克破", "无码破解", "無碼破解", "无码流出", "無碼流出"];

  for meaningless in MEANINGLESS_WORDS {
    if text.contains(meaningless) {
      return false;
    }
  }

  true
}

fn get_video_time(elem: &ElementRef) -> Option<()> {
  let parent = elem.parent()?.parent()?;
  let parent = ElementRef::wrap(parent)?;
  let a = get_selector("span.video-time");
  let span = parent.select(&a).next()?;
  let text: String = span.text().collect();
  let video_time = convert_duration_string_to_seconds(&text)?;

  let mut lock = LAST_DURATION.lock().unwrap();
  *lock = video_time;

  None
}
