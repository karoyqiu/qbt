use std::sync::Arc;

use headless_chrome::Tab;
use log::debug;

use crate::{
  error::{IntoResult, Result},
  scrape::{crawlers::crawler_cdp::take_screenshot, Actress, TranslatedText},
};

use super::{
  airav::VideoObject,
  crawler::convert_datetime_string_to_epoch,
  crawler_cdp::{get_parent_element, CrawlerCDP},
};

#[derive(Default)]
pub struct AiravCDP;

impl CrawlerCDP for AiravCDP {
  fn get_name(&self) -> &'static str {
    "airav.io cdp"
  }

  fn get_url(&self, code: &String) -> Result<String> {
    Ok(format!("https://airav.io/search_result?kw={}", code))
  }

  fn goto_next_url(&self, url: &url::Url, tab: &Arc<Tab>) -> Result<bool> {
    debug!("Path: {}", url.path());
    take_screenshot(&tab, "goto_next_url")?;

    if !url.path().contains("search_result") {
      debug!("False");
      return Ok(false);
    }

    let mut link = None;

    for elem in tab.find_elements("div.col.oneVideo")? {
      if let Ok(h5) = elem.find_element("h5") {
        let text = h5.get_inner_text()?;

        if !text.contains("克破") && !text.contains("无码破解") && !text.contains("無碼破解")
        {
          if let Ok(a) = elem.find_element("a") {
            #[cfg(debug_assertions)]
            {
              let href = a.get_attribute_value("href")?;
              debug!("Going to {:?}", href);
            }
            a.click()?;
            return Ok(true);
          }
        }

        if link.is_none() {
          link = Some(h5);
        }
      }
    }

    if let Some(link) = link {
      if let Ok(a) = link.find_element("a") {
        #[cfg(debug_assertions)]
        {
          let href = a.get_attribute_value("href")?;
          debug!("Going to {:?}", href);
        }
        a.click()?;
        return Ok(true);
      }
    }

    Ok(false)
  }

  fn get_title(&self, tab: &Arc<Tab>) -> Result<String> {
    let h1 = tab.find_element("h1")?;
    h1.get_inner_text().into_result()
  }

  fn get_cover(&self, tab: &Arc<Tab>) -> Result<Option<String>> {
    let script = tab.find_element("script[type=\"application/ld+json\"]")?;
    let text = script.get_inner_text()?;
    let mut video_object: VideoObject = serde_json::from_str(text.trim()).into_result()?;
    Ok(video_object.thumbnail_url.pop())
  }

  fn get_outline(&self, tab: &Arc<Tab>) -> Result<Option<TranslatedText>> {
    let p = tab.find_element("div.video-info > p")?;
    let text = p.get_inner_text()?;
    Ok(Some(TranslatedText::text(text)))
  }

  fn get_actresses(&self, tab: &Arc<Tab>) -> Result<Option<Vec<Actress>>> {
    let actresses = get_info_list_items(tab, "女優")?;
    let actresses = actresses.map(|a| a.iter().map(|a| Actress::name(a)).collect::<Vec<_>>());
    Ok(actresses)
  }

  fn get_tags(&self, tab: &Arc<Tab>) -> Result<Option<Vec<String>>> {
    get_info_list_items(tab, "標籤")
  }

  fn get_series(&self, tab: &Arc<Tab>) -> Result<Option<String>> {
    let mut series = get_info_list_items(tab, "系列")?.unwrap_or_default();
    Ok(series.pop())
  }

  fn get_studio(&self, tab: &Arc<Tab>) -> Result<Option<String>> {
    let mut studios = get_info_list_items(tab, "廠商")?.unwrap_or_default();
    Ok(studios.pop())
  }

  fn get_release_date(&self, tab: &Arc<Tab>) -> Result<Option<i64>> {
    let i = tab.find_element("i.fa.fa-clock")?;
    let parent = get_parent_element(&i)?;
    let text = parent.get_inner_text()?;
    Ok(convert_datetime_string_to_epoch(&text, None))
  }
}

fn get_info_list_items(tab: &Arc<Tab>, label: &str) -> Result<Option<Vec<String>>> {
  let div = tab.find_element("div.video-info")?;
  let mut items = vec![];

  for elem in div.find_elements("li")? {
    let text = elem.get_inner_text()?;

    if text.starts_with(label) {
      for a in elem.find_elements("a")? {
        let text = a.get_inner_text()?;

        if !text.contains(label) {
          items.push(a.get_inner_text()?);
        }
      }
    }
  }

  if items.is_empty() {
    Ok(None)
  } else {
    Ok(Some(items))
  }
}
