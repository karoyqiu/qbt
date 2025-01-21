use chrono::{Local, NaiveDate, NaiveDateTime, TimeZone};
use log::info;
use scraper::Html;
use url::Url;

use crate::{
  error::{err, IntoResult, Result},
  scrape::{crawlers::web::get_html, Actress, TranslatedText, VideoInfo, VideoInfoBuilder},
};

use super::CrawlerCDP;

pub trait Crawler {
  /** 网站名称 */
  fn name(&self) -> &'static str;

  /** 语言 */
  fn language(&self) -> &'static str {
    "ja"
  }

  /** 网站地址 */
  fn get_url(&self, code: &String) -> Result<String>;

  /** 下一步地址 */
  fn get_next_url(&self, _code: &String, _url: &Url, _html: &String) -> Option<String> {
    None
  }

  /** 信息 */
  fn get_info(&self, code: &String, html: &str) -> Result<VideoInfo> {
    let doc = Html::parse_document(html);
    let title = self.get_title(&doc)?;

    self
      .get_info_builder(&doc)
      .code(code.clone())
      .title(TranslatedText::text(title))
      .build()
      .into_result()
  }

  /** 标题 */
  fn get_title(&self, _doc: &Html) -> Result<String> {
    err("No title")
  }

  /** 信息构建器 */
  fn get_info_builder(&self, doc: &Html) -> VideoInfoBuilder {
    let mut builder = VideoInfoBuilder::default();

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
  fn get_actresses(&self, _doc: &Html) -> Option<Vec<Actress>> {
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

  /** 额外的插图 */
  fn get_extra_fanart(&self, _doc: &Html) -> Option<Vec<String>> {
    None
  }

  fn cdp(&self) -> Option<Box<dyn CrawlerCDP + Sync + Send>> {
    None
  }
}

/// 刮削
pub async fn crawl<T>(crawler: &T, code: &String) -> Result<VideoInfo>
where
  T: Crawler + ?Sized,
{
  info!("Crawling {} for {}", crawler.name(), code);
  let url = crawler.get_url(code)?;
  let (mut html, mut url) = get_html(&url).await?;

  while let Some(next_url) = crawler.get_next_url(code, &url, &html) {
    let next_url = url.join(&next_url).into_result()?.to_string();
    (html, url) = get_html(&next_url).await?;
  }

  let mut info = crawler.get_info(code, &html)?;

  if let Some(poster) = info.poster {
    let poster = url.join(&poster).into_result()?;
    info.poster = Some(poster.to_string());
  }

  if let Some(cover) = info.cover {
    let cover = url.join(&cover).into_result()?;
    info.cover = Some(cover.to_string());
  }

  if let Some(actresses) = &mut info.actresses {
    for actress in actresses {
      if let Some(photo) = &actress.photo {
        actress.photo = Some(url.join(photo).into_result()?.to_string());
      }
    }
  }

  info!("Crawled {} for {}: {:?}", crawler.name(), code, info);
  Ok(info)
}

pub fn convert_date_string_to_epoch(text: &str, fmt: Option<&str>) -> Option<i64> {
  let date = NaiveDate::parse_from_str(text, fmt.unwrap_or("%Y-%m-%d"));

  date.ok().map(|d| {
    d.and_hms_opt(0, 0, 0).map(|ndt| {
      Local
        .from_local_datetime(&ndt)
        .single()
        .map(|dt| dt.timestamp())
    })?
  })?
}

pub fn convert_datetime_string_to_epoch(text: &str, fmt: Option<&str>) -> Option<i64> {
  let date = NaiveDateTime::parse_from_str(text, fmt.unwrap_or("%Y-%m-%d %H:%M:%S"));

  date.ok().map(|d| {
    Local
      .from_local_datetime(&d)
      .single()
      .map(|dt| dt.timestamp())
  })?
}

pub fn convert_duration_string_to_seconds(text: &str) -> Option<i64> {
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
    let date = convert_date_string_to_epoch("2025-01-01", None);
    assert_eq!(date, Some(1735660800));
  }

  #[test]
  fn test_convert_datetime_string_to_epoch() {
    let date = convert_datetime_string_to_epoch("2025-01-01 00:00:00", None);
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
