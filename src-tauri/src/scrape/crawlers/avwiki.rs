use scraper::Html;
use url::Url;

use crate::{
  error::{Error, Result},
  scrape::Actress,
};

use super::{
  crawler::Crawler,
  web::{get_selector, optional},
};

#[derive(Default)]
pub struct AvWiki;

impl Crawler for AvWiki {
  fn name(&self) -> &'static str {
    "av-wiki.net"
  }

  fn get_url(&self, code: &String) -> Result<String> {
    Ok(format!("https://av-wiki.net/?s={}&post_type=product", code))
  }

  fn get_next_url(&self, _code: &String, url: &Url, html: &String) -> Option<String> {
    if url.path() != "/" {
      return None;
    }

    let doc = Html::parse_document(&html);
    let article = get_selector("article");
    let article = doc.select(&article).next()?;

    let read_more = get_selector("div.read-more > a");
    let a = article.select(&read_more).next()?;
    let href = a.attr("href")?;

    Some(href.to_string())
  }

  fn get_title(&self, doc: &Html) -> Result<String> {
    let h1 = get_selector("h1");
    let h1 = doc
      .select(&h1)
      .next()
      .ok_or(Error::new("No such element"))?;
    let title: String = h1.text().collect();
    Ok(title)
  }

  fn get_actresses(&self, doc: &Html) -> Option<Vec<Actress>> {
    let a = get_selector("dl > dd > a");
    let mut actresses = vec![];

    for a in doc.select(&a) {
      let href = a.attr("href")?;

      if href.contains("/av-actress/") && !href.contains("/unknown/") {
        let name: String = a.text().collect();
        actresses.push(Actress::name(name));
      }
    }

    optional(actresses)
  }
}
