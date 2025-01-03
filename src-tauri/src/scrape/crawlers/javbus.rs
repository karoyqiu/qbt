use lazy_static::lazy_static;
use log::{debug, info, trace};
use scraper::{ElementRef, Html, Selector};

use crate::{
  error::{err, IntoResult, Result},
  scrape::{
    code::get_code_prefix, crawlers::web::post_html, TranslatedText, VideoInfo, VideoInfoBuilder,
  },
};

use super::web::{get_html, get_selector, H3_SELECTOR};

lazy_static! {
  static ref ACTRESS_SELECTOR: Selector = Selector::parse("div.star-name").unwrap();
}

pub async fn crawl(code: &String) -> Result<VideoInfo> {
  info!("Crawling JavBus for {}", code);
  let url = format!("https://www.javbus.com/{}", code);
  let mut html = get_html(&url).await?;
  trace!("HTML: {}", html);

  if html.contains("地區年齡檢測") {
    html = post_html(&url, &[("submit", "確認")]).await?;
    trace!("HTML again: {}", html);
  }

  let doc = Html::parse_document(&html);

  let mut builder = VideoInfoBuilder::default();
  builder.code(code.clone()).actresses(get_actresses(&doc)?);

  let title = get_title(&doc)?;
  builder.title(TranslatedText {
    text: title.replace(code, "").trim().to_string(),
    translated: None,
  });

  Ok(builder.build().into_result()?)
}

fn get_title(doc: &Html) -> Result<String> {
  let h3 = get_selector("h3")?;

  if let Some(elem) = doc.select(&h3).next() {
    Ok(elem.text().collect())
  } else {
    err("Title not found")
  }
}

fn get_actresses(doc: &Html) -> Result<Option<Vec<String>>> {
  let selector = get_selector("div.star-name")?;
  let a = get_selector("a")?;
  let mut actresses = vec![];

  for elem in doc.select(&selector) {
    if let Some(link) = elem.select(&a).next() {
      actresses.push(link.text().collect());
    }
  }

  if actresses.is_empty() {
    Ok(None)
  } else {
    Ok(Some(actresses))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  static HTML: &str = include_str!("javbus.html");

  #[test]
  fn test_get_title() {
    let doc = Html::parse_document(HTML);
    assert_eq!(
      get_title(&doc).unwrap(),
      "PPX-023 涼森れむ 中出しBEST 8時間"
    );
  }
}
