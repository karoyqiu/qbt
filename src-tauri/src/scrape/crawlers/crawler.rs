use chrono::{Local, NaiveDate, TimeZone};
use log::info;
use scraper::Html;

use crate::{
  error::{IntoResult, Result},
  scrape::{crawlers::web::get_html, TranslatedText, VideoInfo, VideoInfoBuilder},
};

pub trait Crawler {
  /** 网站名称 */
  fn get_name(&self) -> &'static str;

  /** 网站地址 */
  fn get_url(&self, code: &String) -> Result<String>;

  /** 标题 */
  fn get_title(&self, doc: &Html) -> Result<String>;

  /** 信息构建器 */
  fn get_info_builder(&self, doc: &Html) -> VideoInfoBuilder {
    let mut builder = VideoInfoBuilder::default();

    builder
      .poster(self.get_poster(&doc))
      .cover(self.get_cover(&doc))
      .outline(self.get_outline(&doc))
      .actresses(self.get_actresses(&doc))
      .actress_photos(self.get_actress_photos(&doc))
      .tags(self.get_tags(&doc))
      .series(self.get_series(&doc))
      .studio(self.get_studio(&doc))
      .publisher(self.get_publisher(&doc))
      .director(self.get_director(&doc))
      .duration(self.get_duration(&doc))
      .release_date(self.get_release_date(&doc));

    builder
  }

  /** 海报 */
  fn get_poster(&self, _doc: &Html) -> Option<String> {
    None
  }

  /** 封面 */
  fn get_cover(&self, _doc: &Html) -> Option<String> {
    None
  }

  /** 简介 */
  fn get_outline(&self, _doc: &Html) -> Option<TranslatedText> {
    None
  }

  /** 演员列表 */
  fn get_actresses(&self, _doc: &Html) -> Option<Vec<String>> {
    None
  }

  /** 演员头像列表 */
  fn get_actress_photos(&self, _doc: &Html) -> Option<Vec<String>> {
    None
  }

  /** 标签列表 */
  fn get_tags(&self, _doc: &Html) -> Option<Vec<String>> {
    None
  }

  /** 系列 */
  fn get_series(&self, _doc: &Html) -> Option<String> {
    None
  }

  /** 片商 */
  fn get_studio(&self, _doc: &Html) -> Option<String> {
    None
  }

  /** 发行商 */
  fn get_publisher(&self, _doc: &Html) -> Option<String> {
    None
  }

  /** 导演 */
  fn get_director(&self, _doc: &Html) -> Option<String> {
    None
  }

  /** 时长（秒） */
  fn get_duration(&self, _doc: &Html) -> Option<i64> {
    None
  }

  /** 发布日期（Unix epoch） */
  fn get_release_date(&self, _doc: &Html) -> Option<i64> {
    None
  }
}

pub(crate) fn convert_date_string_to_epoch(text: &str) -> Option<i64> {
  let date = NaiveDate::parse_from_str(text, "%Y-%m-%d");

  date.ok().map(|d| {
    d.and_hms_opt(0, 0, 0).map(|ndt| {
      Local
        .from_local_datetime(&ndt)
        .single()
        .map(|dt| dt.timestamp())
    })?
  })?
}

pub(crate) fn convert_duration_string_to_seconds(text: &str) -> Option<i64> {
  let parts: Vec<&str> = text.split(':').collect();
  match parts.len() {
    2 => {
      let minutes = parts[0].parse::<i64>().unwrap_or(0);
      let seconds = parts[1].parse::<i64>().unwrap_or(0);
      Some(minutes * 60 + seconds)
    }
    3 => {
      let hours = parts[0].parse::<i64>().unwrap_or(0);
      let minutes = parts[1].parse::<i64>().unwrap_or(0);
      let seconds = parts[2].parse::<i64>().unwrap_or(0);
      Some(hours * 3600 + minutes * 60 + seconds)
    }
    _ => None,
  }
}

#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;

  use super::*;

  #[test]
  fn test_convert_date_string_to_epoch() {
    let date = convert_date_string_to_epoch("2025-01-01");
    assert_eq!(date, Some(1735660800));
  }

  #[test]
  fn test_convert_duration_string_to_seconds() {
    let duration = convert_duration_string_to_seconds("01:00:00");
    assert_eq!(duration, Some(3600));

    let duration = convert_duration_string_to_seconds("00:01:00");
    assert_eq!(duration, Some(60));

    let duration = convert_duration_string_to_seconds("00:00:01");
    assert_eq!(duration, Some(1));

    let duration = convert_duration_string_to_seconds("01:00");
    assert_eq!(duration, Some(60));

    let duration = convert_duration_string_to_seconds("00:01");
    assert_eq!(duration, Some(1));

    let duration = convert_duration_string_to_seconds("00:00");
    assert_eq!(duration, Some(0));
  }
}

pub async fn crawl<T>(crawler: &T, code: &String) -> Result<VideoInfo>
where
  T: Crawler + ?Sized,
{
  info!("Crawling {} for {}", crawler.get_name(), code);
  let url = crawler.get_url(code)?;
  let html = get_html(&url).await?;
  let doc = Html::parse_document(&html);
  let title = crawler.get_title(&doc)?;

  let info = crawler
    .get_info_builder(&doc)
    .code(code.clone())
    .title(TranslatedText {
      text: title,
      translated: None,
    })
    .build()
    .into_result()?;

  info!("Crawled {} for {}: {:?}", crawler.get_name(), code, info);
  Ok(info)
}
