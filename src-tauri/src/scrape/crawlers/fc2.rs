use log::info;
use scraper::Html;

use crate::{
  error::{err, IntoResult, Result},
  scrape::{
    code::get_code_prefix, crawlers::web::get_html, TranslatedText, VideoInfo, VideoInfoBuilder,
  },
};

pub async fn crawl(code: &String) -> Result<VideoInfo> {
  info!("Crawling fc2 website for {}", code);
  let number = code
    .replace("FC2-", "")
    .replace("FC2PPV", "")
    .replace("FC2-PPV-", "")
    .replace("-", "");
  let url = format!("https://adult.contents.fc2.com/article/{}/", number);
  let (html, _) = get_html(&url).await?;
  //let doc = Html::parse_document(&html);
  let info = VideoInfo::default();
  Ok(info)
}
