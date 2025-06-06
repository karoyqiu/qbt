use scraper::{ElementRef, Html};

use crate::{
  error::{Result, err},
  scrape::{Actress, VideoInfoBuilder},
};

use super::{
  crawler::{Crawler, convert_date_string_to_epoch, convert_duration_string_to_seconds},
  fc2ppvdb_cdp::Fc2ppvdbCDP,
  web::get_selector,
};

#[derive(Default)]
pub struct Fc2ppvdb;

impl Crawler for Fc2ppvdb {
  fn name(&self) -> &'static str {
    "fc2ppvdb.com"
  }

  fn get_url(&self, code: &String) -> Result<String> {
    let number = code
      .replace("FC2-", "")
      .replace("FC2PPV", "")
      .replace("FC2-PPV-", "")
      .replace("-", "");
    Ok(format!("https://fc2ppvdb.com/articles/{}", number))
  }

  fn get_title(&self, doc: &Html) -> Result<String> {
    let h2a = get_selector("h2 > a");

    if let Some(a) = doc.select(&h2a).next() {
      let title: String = a.text().collect();
      Ok(title)
    } else {
      err("Title not found")
    }
  }

  fn get_info_builder(&self, doc: &Html) -> VideoInfoBuilder {
    let mut builder = VideoInfoBuilder::default();
    builder.poster(self.get_poster(&doc));

    {
      let h2 = get_selector("h2");

      if let Some(elem) = doc.select(&h2).next() {
        if let Some(parent) = elem.parent() {
          if let Some(parent) = ElementRef::wrap(parent) {
            let div = get_selector("div");

            for elem in parent.select(&div) {
              let text: String = elem.text().collect();
              let text = text.trim();

              if text.starts_with("販売者：") {
                builder.publisher(Some(text.replace("販売者：", "").trim().to_string()));
              } else if text.starts_with("販売日：") {
                let text = text.replace("販売日：", "");
                let text = text.trim();
                builder.release_date(convert_date_string_to_epoch(text, None));
              } else if text.starts_with("収録時間：") {
                let text = text.replace("収録時間：", "");
                let text = text.trim();
                builder.duration(convert_duration_string_to_seconds(text));
              } else if text.starts_with("タグ：") {
                let a = get_selector("a[href^='/tags/']");
                let mut tags = vec![];

                for elem in elem.select(&a) {
                  let tag: String = elem.text().collect();
                  tags.push(tag);
                }

                if !tags.is_empty() {
                  builder.tags(Some(tags));
                }
              }
            }
          }
        }
      }
    }

    // actresses
    {
      let mut actresses = vec![];
      let a_selector = get_selector("a[href^='/actresses/']");
      let img_selector = get_selector("img");

      for elem in doc.select(&a_selector) {
        if let Some(img) = elem.select(&img_selector).next() {
          let actress = elem.attr("title").unwrap_or_default();

          if !actress.is_empty() {
            let photo = img.value().attr("src").unwrap_or_default().trim();
            actresses.push(Actress::new(actress, Some(photo)));
          }
        }
      }

      if !actresses.is_empty() {
        builder.actresses(Some(actresses));
      }
    }

    builder
  }

  fn get_poster(&self, doc: &Html) -> Option<String> {
    let main_img = get_selector("main img");

    if let Some(img) = doc.select(&main_img).next() {
      let poster = img.value().attr("src").unwrap_or_default();
      Some(poster.to_string())
    } else {
      None
    }
  }

  fn cdp(&self) -> Option<Box<dyn super::CrawlerCDP + Sync + Send>> {
    Some(Box::new(Fc2ppvdbCDP::default()))
  }
}

#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use std::vec;

  use super::*;

  static HTML: &str = include_str!("fc2ppvdb.html");

  #[test]
  fn test_crawl() {
    let doc = Html::parse_document(HTML);
    let crawler = Fc2ppvdb::default();
    let title = crawler.get_title(&doc);
    assert!(title.is_ok());
    assert_eq!(
      title.unwrap(),
      "【無】清楚で美人なお姉さんをホテルに連れ込みプライベートSEX！お風呂でちゃっかりアナルをオ〇ス。お互いに気持ちいいことだけを追求したハメ撮りです！※特典高画質"
    );

    let info = crawler.get_info_builder(&doc).build();
    assert!(info.is_ok());

    let info = info.unwrap();
    assert_eq!(
      info.poster,
      Some("https://fc2ppvdb.com/storage/images/article/004/38/fc2ppv-4382449.jpg".to_string())
    );
    assert_eq!(info.publisher, Some("ハメ撮りランキング".to_string()));
    assert_eq!(
      info.actresses,
      Some(vec![Actress::new(
        "某高級外車ディーラーの美人受付嬢",
        Some("https://fc2ppvdb.com/storage/images/actress/4960.jpg")
      )])
    );
    assert_eq!(info.release_date, Some(1712678400));
    assert_eq!(info.duration, Some(46 * 60 + 7));
    assert_eq!(
      info.tags,
      Some(vec![
        "ハメ撮り".to_string(),
        "中出し".to_string(),
        "個人撮影".to_string(),
        "フェラ".to_string(),
        "アナル".to_string(),
        "無修正".to_string(),
        "美女".to_string(),
        "可愛い".to_string(),
      ])
    );
  }
}
