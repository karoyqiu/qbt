use std::sync::Arc;

use headless_chrome::Tab;

use crate::{
  error::{IntoResult, Result},
  scrape::{Actress, VideoInfoBuilder},
};

use super::{
  crawler::{convert_date_string_to_epoch, convert_duration_string_to_seconds},
  crawler_cdp::{get_parent_element, CrawlerCDP},
};

#[derive(Default)]
pub struct Fc2ppvdbCDP;

impl CrawlerCDP for Fc2ppvdbCDP {
  fn name(&self) -> &'static str {
    "fc2ppvdb.com cdp"
  }

  fn get_url(&self, code: &String) -> Result<String> {
    let number = code
      .replace("FC2-", "")
      .replace("FC2PPV", "")
      .replace("FC2-PPV-", "")
      .replace("-", "");
    Ok(format!("https://fc2ppvdb.com/articles/{}", number))
  }

  fn get_title(&self, tab: &Arc<Tab>) -> Result<String> {
    let h2 = tab.find_element("h2 > a")?;
    h2.get_inner_text().into_result()
  }

  fn get_info_builder(&self, tab: &Arc<Tab>) -> Result<VideoInfoBuilder> {
    let mut builder = VideoInfoBuilder::default();
    builder.poster(self.get_poster(tab).unwrap_or_default());

    let h2 = tab.find_element("h2")?;
    let parent = get_parent_element(&h2)?;

    for div in parent.find_elements("div")? {
      let text = div.get_inner_text()?;
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
        let mut tags = vec![];

        for elem in div.find_elements("a[href^='/tags/']")? {
          let tag = elem.get_inner_text()?;
          tags.push(tag);
        }

        if !tags.is_empty() {
          builder.tags(Some(tags));
        }
      }
    }

    // actresses

    let mut actresses = vec![];

    for elem in tab.find_elements("a[href^='/actresses/']")? {
      if let Ok(img) = elem.find_element("img") {
        let actress = elem
          .get_attribute_value("title")
          .unwrap_or_default()
          .unwrap_or_default();

        if !actress.is_empty() {
          let photo = img
            .get_attribute_value("src")
            .unwrap_or_default()
            .unwrap_or_default();
          actresses.push(Actress::new(actress, Some(photo)));
        }
      }
    }

    if !actresses.is_empty() {
      builder.actresses(Some(actresses));
    }

    Ok(builder)
  }

  fn get_poster(&self, tab: &Arc<Tab>) -> Result<Option<String>> {
    let img = tab.find_element("main img")?;
    img.get_attribute_value("src").into_result()
  }
}
