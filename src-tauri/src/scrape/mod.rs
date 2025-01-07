mod code;
mod crawl;
mod crawlers;

use std::ops::Add;

use derive_builder::Builder;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::AppHandle;

use crate::error::{err, Result};

use code::get_movie_code;

#[derive(Debug, Default, Clone, Serialize, Deserialize, Type)]
pub struct TranslatedText {
  pub text: String,
  pub translated: Option<String>,
}

impl TranslatedText {
  pub fn new<N, P>(text: N, translated: Option<P>) -> Self
  where
    N: ToString,
    P: ToString,
  {
    TranslatedText {
      text: text.to_string(),
      translated: translated.map(|s| s.to_string()),
    }
  }

  pub fn text<N>(text: N) -> Self
  where
    N: ToString,
  {
    TranslatedText {
      text: text.to_string(),
      translated: None,
    }
  }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Type)]
pub struct Actress {
  pub name: String,
  pub photo: Option<String>,
}

impl Actress {
  pub fn new<N, P>(name: N, photo: Option<P>) -> Self
  where
    N: ToString,
    P: ToString,
  {
    Actress {
      name: name.to_string(),
      photo: photo.map(|photo| photo.to_string()),
    }
  }

  pub fn name<N>(name: N) -> Self
  where
    N: ToString,
  {
    Actress {
      name: name.to_string(),
      photo: None,
    }
  }
}

/// 视频信息
#[derive(Debug, Default, Clone, Serialize, Deserialize, Type, Builder)]
#[builder(default)]
pub struct VideoInfo {
  /** 番号 */
  pub code: String,
  /** 标题 */
  pub title: TranslatedText,
  /** 海报 */
  pub poster: Option<String>,
  /** 封面 */
  pub cover: Option<String>,
  /** 简介 */
  pub outline: Option<TranslatedText>,
  /** 演员列表 */
  pub actresses: Option<Vec<Actress>>,
  /** 标签列表 */
  pub tags: Option<Vec<String>>,
  /** 系列 */
  pub series: Option<String>,
  /** 片商 */
  pub studio: Option<String>,
  /** 发行商 */
  pub publisher: Option<String>,
  /** 导演 */
  pub director: Option<String>,
  /** 时长（秒） */
  pub duration: Option<i64>,
  /** 发布日期（Unix epoch） */
  pub release_date: Option<i64>,
}

impl VideoInfo {
  pub(crate) fn apply(&mut self, other: VideoInfo) {
    if self.code.is_empty() {
      self.code = other.code;
    } else if self.code != other.code {
      return;
    }

    if !other.title.text.is_empty() {
      self.title.text = other.title.text;
    }

    if other.title.translated.is_some() {
      self.title.translated = other.title.translated;
    }

    if other.poster.is_some() {
      self.poster = other.poster;
    }

    if other.cover.is_some() {
      self.cover = other.cover;
    }

    if let Some(outline) = other.outline {
      if let Some(self_outline) = &mut self.outline {
        if !outline.text.is_empty() {
          self_outline.text = outline.text;
        }

        if outline.translated.is_some() {
          self_outline.translated = outline.translated;
        }
      } else {
        self.outline = Some(outline);
      }
    }

    if other.actresses.is_some() {
      self.actresses = other.actresses;
    }

    if other.tags.is_some() {
      self.tags = other.tags;
    }

    if other.series.is_some() {
      self.series = other.series;
    }

    if other.studio.is_some() {
      self.studio = other.studio;
    }

    if other.publisher.is_some() {
      self.publisher = other.publisher;
    }

    if other.director.is_some() {
      self.director = other.director;
    }

    if other.duration.is_some() {
      self.duration = other.duration;
    }

    if other.release_date.is_some() {
      self.release_date = other.release_date;
    }
  }
}

/// 刮削
#[tauri::command]
#[specta::specta]
pub async fn scrape(_app: AppHandle, filename: String) -> Result<VideoInfo> {
  info!("Scraping {}", filename);
  let code = get_movie_code(&filename);

  if let Some(code) = code {
    debug!("Movie code: {}", code);
    crawl::crawl(&code).await
  } else {
    err("No movie code found")
  }
}
