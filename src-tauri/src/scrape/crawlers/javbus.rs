use scraper::{ElementRef, Html};

use crate::{
  error::{err, Result},
  scrape::Actress,
};

use super::{
  crawler::{convert_date_string_to_epoch, Crawler},
  web::get_selector,
};

#[derive(Default)]
pub struct JavBus;

impl Crawler for JavBus {
  fn get_name(&self) -> &'static str {
    "javbus.com"
  }

  fn get_url(&self, code: &String) -> Result<String> {
    Ok(format!("https://www.javbus.com/{}", code))
  }

  fn get_title(&self, doc: &Html) -> Result<String> {
    let h3 = get_selector("h3");

    if let Some(elem) = doc.select(&h3).next() {
      Ok(elem.text().collect())
    } else {
      err("Title not found")
    }
  }

  fn get_cover(&self, doc: &Html) -> Option<String> {
    let selector = get_selector("a.bigImage");
    doc
      .select(&selector)
      .next()
      .map(|a| a.attr("href").map(String::from))?
  }

  fn get_actresses(&self, doc: &Html) -> Option<Vec<Actress>> {
    let star_name = get_selector("div.star-name");
    let img = get_selector("img");
    let mut actresses = vec![];

    for elem in doc.select(&star_name) {
      let name: String = elem.text().collect();
      let parent = ElementRef::wrap(elem.parent()?)?;
      let img = parent.select(&img).next()?;
      let src = img.attr("src");
      actresses.push(Actress::new(name, src));
    }

    if actresses.is_empty() {
      None
    } else {
      Some(actresses)
    }
  }

  fn get_tags(&self, doc: &Html) -> Option<Vec<String>> {
    let selector = get_selector("a[href*=\"/genre/\"]");
    let mut tags = vec![];

    for elem in doc.select(&selector) {
      let text: String = elem.text().collect();
      tags.push(text);
    }

    if tags.is_empty() {
      None
    } else {
      Some(tags)
    }
  }

  fn get_series(&self, doc: &Html) -> Option<String> {
    let selector = get_selector("a[href*=\"/series/\"]");
    doc
      .select(&selector)
      .next()
      .map(|e| e.text().collect::<String>())
  }

  fn get_studio(&self, doc: &Html) -> Option<String> {
    let selector = get_selector("a[href*=\"/studio/\"]");
    doc
      .select(&selector)
      .next()
      .map(|e| e.text().collect::<String>())
  }

  fn get_publisher(&self, doc: &Html) -> Option<String> {
    let selector = get_selector("a[href*=\"/label/\"]");
    doc
      .select(&selector)
      .next()
      .map(|e| e.text().collect::<String>())
  }

  fn get_director(&self, doc: &Html) -> Option<String> {
    let selector = get_selector("a[href*=\"/director/\"]");
    doc
      .select(&selector)
      .next()
      .map(|e| e.text().collect::<String>())
  }

  fn get_duration(&self, doc: &Html) -> Option<i64> {
    let selector = get_selector("span.header");

    for elem in doc.select(&selector) {
      let text: String = elem.text().collect();

      if text.contains("長度:") {
        let parent = ElementRef::wrap(elem.parent()?)?;
        let text: String = parent.text().collect();
        let text = text.replace("長度:", "").replace("分鐘", "");
        let text = text.trim();
        return Some(text.parse().unwrap());
      }
    }

    None
  }

  fn get_release_date(&self, doc: &Html) -> Option<i64> {
    let selector = get_selector("span.header");

    for elem in doc.select(&selector) {
      let text: String = elem.text().collect();

      if text.contains("發行日期:") {
        let parent = ElementRef::wrap(elem.parent()?)?;
        let text: String = parent.text().collect();
        let text = text.replace("發行日期:", "");
        let text = text.trim();
        return convert_date_string_to_epoch(text);
      }
    }

    None
  }
}

// #[cfg(test)]
// mod tests {
//   use super::*;

//   static HTML: &str = include_str!("javbus.html");

//   #[test]
//   fn test_get_title() {
//     let doc = Html::parse_document(HTML);
//     assert_eq!(
//       get_title(&doc).unwrap(),
//       "PPX-023 涼森れむ 中出しBEST 8時間"
//     );
//   }
// }
