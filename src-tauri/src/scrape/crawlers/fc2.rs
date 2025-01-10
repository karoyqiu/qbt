use lazy_static::lazy_static;
use regex::Regex;
use scraper::Html;
use serde::Deserialize;

use crate::error::{Error, IntoResult, Result};

use super::{
  crawler::convert_date_string_to_epoch,
  web::{get_selector, remove_first},
  Crawler,
};

lazy_static! {
  static ref YMD_RE: Regex = Regex::new(r"\d+/\d+/\d+").unwrap();
}

#[derive(Debug, Deserialize)]
struct NameOnly {
  name: String,
}

#[derive(Default)]
pub struct Fc2;

impl Crawler for Fc2 {
  fn get_name(&self) -> &'static str {
    "fc2.com"
  }

  fn get_url(&self, code: &String) -> Result<String> {
    let number = code
      .replace("FC2-", "")
      .replace("FC2PPV", "")
      .replace("FC2-PPV-", "")
      .replace("-", "");
    Ok(format!(
      "https://adult.contents.fc2.com/article/{}/",
      number
    ))
  }

  fn get_title(&self, doc: &Html) -> Result<String> {
    let selector = get_selector("script[type=\"application/ld+json\"]");
    let script = doc
      .select(&selector)
      .next()
      .ok_or_else(|| Error::new("No ld+json"))?;
    let text: String = script.text().collect();
    let name: NameOnly = serde_json::from_str(&text).into_result()?;
    Ok(name.name)
  }

  fn get_poster(&self, doc: &Html) -> Option<String> {
    get_child_text(doc, "div.items_article_MainitemThumb", "img", Some("src"))
  }

  fn get_cover(&self, doc: &Html) -> Option<String> {
    get_child_text(doc, "ul.items_article_SampleImagesArea", "a", Some("href"))
  }

  fn get_tags(&self, doc: &Html) -> Option<Vec<String>> {
    let a = get_selector("a.tag.tagTag");
    let mut tags = vec![];

    for elem in doc.select(&a) {
      let text: String = elem.text().collect();
      tags.push(text);
    }

    if tags.is_empty() {
      None
    } else {
      Some(tags)
    }
  }

  fn get_studio(&self, doc: &Html) -> Option<String> {
    get_child_text(
      doc,
      "div.items_article_headerInfo > ul",
      "li:last-of-type > a",
      None,
    )
  }

  fn get_release_date(&self, doc: &Html) -> Option<i64> {
    let text = get_child_text(doc, "div.items_article_Releasedate", "p", None)?;
    let m = YMD_RE.find(&text)?;
    convert_date_string_to_epoch(m.as_str(), Some("%Y/%m/%d"))
  }

  fn get_extra_fanart(&self, doc: &Html) -> Option<Vec<String>> {
    let ul = get_selector("ul.items_article_SampleImagesArea");
    let a = get_selector("a");
    let mut arts = vec![];

    let ul = doc.select(&ul).next()?;

    for elem in ul.select(&a) {
      let href = elem.attr("href")?;
      arts.push(href.to_string());
    }

    remove_first(arts)
  }
}

fn get_child_text(
  doc: &Html,
  parent_selector: &'static str,
  child_selector: &'static str,
  attr: Option<&str>,
) -> Option<String> {
  let parent_selector = get_selector(parent_selector);
  let child_selector = get_selector(child_selector);
  let parent = doc.select(&parent_selector).next()?;
  let child = parent.select(&child_selector).next()?;

  if let Some(attr) = attr {
    child.attr(attr).map(String::from)
  } else {
    Some(child.text().collect())
  }
}
