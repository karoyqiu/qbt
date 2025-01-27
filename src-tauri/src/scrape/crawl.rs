use std::collections::HashMap;

use lazy_static::lazy_static;
use log::{debug, warn};
use regex::Regex;
use translators::Translator;

use crate::error::{err, Result};

use super::{
  code::is_uncensored,
  crawlers::{self, get_translator, Crawler},
  VideoInfo,
};

lazy_static! {
  static ref EU_RE: Regex = Regex::new(r"[^.]+\.\d{2}\.\d{2}\.\d{2}").unwrap();
  static ref DMM_RE: Regex = Regex::new(r"\D{2,}00\d{3,}").unwrap();
  static ref FALENO_RE: Regex = Regex::new(r"F[A-Z]{2}SS").unwrap();
  static ref FANTASTICA_RE: Regex = Regex::new(r"FA[A-Z]{2}-?\d+").unwrap();
  static ref FC2_WEBSITES: Vec<&'static str> = vec![
    "fc2ppvdb",
    "fc2ppvdb_cdp",
    "fc2",
    "fc2club",
    "fc2hub",
    "freejavbt",
    "7mmtv",
    "hdouban",
    "javdb",
    "avsox",
    "airav",
  ];
  static ref EU_WEBSITES: Vec<&'static str> = vec!["theporndb", "javdb", "javbus", "hdouban"];
  static ref AMATEUR_WEBSITES: Vec<&'static str> = vec![
    "mgstage",
    "avsex",
    "jav321",
    "freejavbt",
    "7mmtv",
    "javbus",
    "javdb",
  ];
  static ref UNCENSORED_WEBSITES: Vec<&'static str> = vec![
    "iqqtv",
    "javbus",
    "freejavbt",
    "jav321",
    "avsox",
    "7mmtv",
    "hdouban",
    "javdb",
    "airav",
  ];
  static ref CENSORED_WEBSITES: Vec<&'static str> = vec![
    "iqqtv",
    "avsex",
    "javbus",
    "lulubar",
    "freejavbt",
    "jav321",
    "dmm",
    "javlibrary",
    "7mmtv",
    "hdouban",
    "javdb",
    "airav",
    "xcity",
    "avsox",
  ];
  static ref DMM_WEBSITES: Vec<&'static str> = vec!["dmm"];
  static ref WHOLE_FIELDS: Vec<&'static str> =
    vec!["outline", "actress", "thumb", "release", "tag"];
  static ref TITLE_WEBSITE: Vec<&'static str> = vec![
    "theporndb",
    "mgstage",
    "dmm",
    "javbus",
    "jav321",
    "javlibrary",
  ];
  static ref TITLE_ZH_WEBSITE: Vec<&'static str> = vec!["iqqtv", "avsex", "lulubar"];
  static ref TITLE_WEBSITE_EXCLUDE: Vec<&'static str> = vec![""];
  static ref OUTLINE_WEBSITE: Vec<&'static str> = vec!["theporndb", "dmm", "jav321"];
  static ref OUTLINE_ZH_WEBSITE: Vec<&'static str> = vec!["avsex", "iqqtv", "lulubar"];
  static ref OUTLINE_WEBSITE_EXCLUDE: Vec<&'static str> = vec![
    "avsox",
    "fc2club",
    "javbus",
    "javdb",
    "javlibrary",
    "freejavbt",
    "hdouban",
  ];
  static ref ACTRESS_WEBSITE: Vec<&'static str> =
    vec!["theporndb", "javbus", "javlibrary", "javdb"];
  static ref ACTOR_WEBSITE_EXCLUDE: Vec<&'static str> = vec![""];
  static ref THUMB_WEBSITE: Vec<&'static str> = vec!["theporndb", "javbus"];
  static ref THUMB_WEBSITE_EXCLUDE: Vec<&'static str> = vec!["javdb"];
  static ref POSTER_WEBSITE: Vec<&'static str> = vec!["theporndb", "avsex", "javbus"];
  static ref POSTER_WEBSITE_EXCLUDE: Vec<&'static str> = vec![
    "airav",
    "fc2club",
    "fc2hub",
    "iqqtv",
    "7mmtv",
    "javlibrary",
    "lulubar",
  ];
  static ref EXTRAFANART_WEBSITE: Vec<&'static str> = vec!["javbus", "freejavbt"];
  static ref EXTRAFANART_WEBSITE_EXCLUDE: Vec<&'static str> =
    vec!["airav", "avsex", "avsox", "iqqtv", "javlibrary", "lulubar",];
  static ref TRAILER_WEBSITE: Vec<&'static str> = vec!["freejavbt", "mgstage", "dmm"];
  static ref TRAILER_WEBSITE_EXCLUDE: Vec<&'static str> = vec!["7mmtv", "lulubar"];
  static ref TAG_WEBSITE: Vec<&'static str> = vec!["javbus", "freejavbt"];
  static ref TAG_WEBSITE_EXCLUDE: Vec<&'static str> = vec![""];
  static ref RELEASE_WEBSITE: Vec<&'static str> = vec!["javbus", "freejavbt", "7mmtv"];
  static ref RELEASE_WEBSITE_EXCLUDE: Vec<&'static str> = vec!["fc2club", "fc2hub"];
  static ref DURATION_WEBSITE: Vec<&'static str> = vec!["javbus", "freejavbt"];
  static ref RUNTIME_WEBSITE_EXCLUDE: Vec<&'static str> =
    vec!["airav", "fc2", "fc2club", "fc2hub", "lulubar"];
  static ref SCORE_WEBSITE: Vec<&'static str> = vec!["jav321", "javlibrary", "javdb"];
  static ref SCORE_WEBSITE_EXCLUDE: Vec<&'static str> = vec![
    "airav", "avsex", "avsox", "7mmtv", "fc2", "fc2hub", "iqqtv", "javbus", "xcity", "lulubar",
  ];
  static ref DIRECTOR_WEBSITE: Vec<&'static str> = vec!["javbus", "freejavbt"];
  static ref DIRECTOR_WEBSITE_EXCLUDE: Vec<&'static str> =
    vec!["airav", "avsex", "avsox", "fc2", "fc2hub", "iqqtv", "jav321", "mgstage", "lulubar",];
  static ref SERIES_WEBSITE: Vec<&'static str> = vec!["javbus", "freejavbt"];
  static ref SERIES_WEBSITE_EXCLUDE: Vec<&'static str> =
    vec!["airav", "avsex", "iqqtv", "7mmtv", "javlibrary", "lulubar",];
  static ref STUDIO_WEBSITE: Vec<&'static str> = vec!["javbus", "freejavbt"];
  static ref STUDIO_WEBSITE_EXCLUDE: Vec<&'static str> = vec!["avsex"];
  static ref PUBLISHER_WEBSITE: Vec<&'static str> = vec!["javbus"];
  static ref PUBLISHER_WEBSITE_EXCLUDE: Vec<&'static str> =
    vec!["airav", "avsex", "iqqtv", "lulubar"];
  static ref WANTED_WEBSITE: Vec<&'static str> = vec!["javlibrary", "javdb"];
  static ref FIELD_EXCLUDES: HashMap<&'static str, Vec<&'static str>> = {
    let mut m = HashMap::new();
    m.insert("title", vec![]);
    m.insert(
      "outline",
      vec![
        "avsox",
        "fc2club",
        "javbus",
        "javdb",
        "javlibrary",
        "freejavbt",
        "hdouban",
      ],
    );
    m.insert("actress", vec![]);
    m.insert("thumb", vec!["javdb"]);
    m.insert(
      "poster",
      vec![
        "airav",
        "fc2club",
        "fc2hub",
        "iqqtv",
        "7mmtv",
        "javlibrary",
        "lulubar",
      ],
    );
    m.insert(
      "extrafanart",
      vec!["airav", "avsex", "avsox", "iqqtv", "javlibrary", "lulubar"],
    );
    m.insert("trailer", vec!["7mmtv", "lulubar"]);
    m.insert("tag", vec![]);
    m.insert("release", vec!["fc2club", "fc2hub"]);
    m.insert(
      "recorded",
      vec!["airav", "fc2", "fc2club", "fc2hub", "lulubar"],
    );
    m.insert(
      "score",
      vec![
        "airav", "avsex", "avsox", "7mmtv", "fc2", "fc2hub", "iqqtv", "javbus", "xcity", "lulubar",
      ],
    );
    m.insert(
      "director",
      vec![
        "airav", "avsex", "avsox", "fc2", "fc2hub", "iqqtv", "jav321", "mgstage", "lulubar",
      ],
    );
    m.insert(
      "series",
      vec!["airav", "avsex", "iqqtv", "7mmtv", "javlibrary", "lulubar"],
    );
    m.insert("studio", vec!["avsex"]);
    m.insert("publisher", vec!["airav", "avsex", "iqqtv", "lulubar"]);
    m
  };
  static ref CRAWLERS: HashMap<&'static str, Box<dyn Crawler + Sync + Send>> = {
    use super::crawlers::{Airav, AvWiki, Fc2, Fc2ppvdb, Iqqtv, JavBus, Officials, Prestige};

    let mut m: HashMap<&'static str, Box<dyn Crawler + Sync + Send>> = HashMap::new();
    m.insert("officials", Box::new(Officials::default()));
    m.insert("javbus", Box::new(JavBus::default()));
    m.insert("fc2", Box::new(Fc2::default()));
    m.insert("fc2ppvdb", Box::new(Fc2ppvdb::default()));
    m.insert("airav", Box::new(Airav::default()));
    m.insert("prestige", Box::new(Prestige::default()));
    m.insert("iqqtv", Box::new(Iqqtv::default()));
    m.insert("av-wiki", Box::new(AvWiki::default()));
    m
  };
}

/// 刮削
pub async fn crawl(code: &String) -> Result<VideoInfo> {
  debug!("Crawling {}", code);
  // TODO: 先判断是不是国产，避免浪费时间

  if code.starts_with("FC2") {
    // FC2: FC2-111111
    crawl_websites(code, &FC2_WEBSITES).await
  } else if code.starts_with("KIN8") {
    // kin8
    crawl_website(code, "kin8").await
  } else if code.starts_with("DLID") {
    // 同人
    crawl_website(code, "getchu").await
  } else if code.contains("GETCHU") {
    // 里番
    crawl_website(code, "getchu_dmm").await
  } else if code.starts_with("Mywife") {
    // Mywife No.1111
    crawl_website(code, "mywife").await
  } else if EU_RE.is_match(code) {
    // 欧美: sexart.15.06.14
    crawl_websites(code, &EU_WEBSITES).await
  } else if is_uncensored(code) {
    // 无码
    crawl_websites(code, &UNCENSORED_WEBSITES).await
  } else if code.starts_with("SIRO") {
    // 素人
    // TODO: 259LUXU-1111
    crawl_websites(code, &AMATEUR_WEBSITES).await
  } else if DMM_RE.is_match(code) && !code.contains("-") && !code.contains("_") {
    // DMM: 00ID-111
    crawl_websites(code, &DMM_WEBSITES).await
  } else {
    // 有碼
    crawl_websites(code, &CENSORED_WEBSITES).await
  }
}

async fn crawl_website(code: &String, website: &str) -> Result<VideoInfo> {
  if let Some(crawler) = CRAWLERS.get(website) {
    let result = crawl_one(crawler.as_ref(), code).await;

    if result.is_ok() {
      return result;
    }
  }

  err("Failed to crawl")
}

async fn crawl_one<T>(crawler: &T, code: &String) -> Result<VideoInfo>
where
  T: Crawler + ?Sized,
{
  let mut result = crawlers::crawl(crawler, code).await;

  if result.is_err() {
    if let Some(cdp) = crawler.cdp() {
      result = crawlers::crawl_cdp(cdp.as_ref(), code).await;
    } else {
      warn!("Failed to crawl: {:?}", result.as_ref().err());
    }
  }

  let result = result?;

  if crawler.language().starts_with("zh") {
    Ok(result)
  } else {
    translate_info(result).await
  }
}

async fn translate_info(mut info: VideoInfo) -> Result<VideoInfo> {
  let translator = get_translator()?;

  if info.title.translated.is_none() {
    info.title.translated = translator
      .translate_async(&info.title.text, "", "zh-CN")
      .await
      .ok();
  }

  if let Some(outline) = &mut info.outline {
    if outline.translated.is_none() {
      outline.translated = translator
        .translate_async(&outline.text, "", "zh-CN")
        .await
        .ok();
    }
  }

  Ok(info)
}

/// 获取一组网站的数据：按照设置的网站组，请求各字段数据，并返回最终的数据
async fn crawl_websites(code: &String, websites: &Vec<&'static str>) -> Result<VideoInfo> {
  debug!("Crawl websites: {:?}", websites);
  let mut info = crawl_officials(code).await.unwrap_or_default();

  for &website in websites {
    match crawl_website(code, website).await {
      Ok(result) => info.apply(result),
      Err(e) => warn!("Error: {:?}", e),
    }

    if info.is_good_enough() {
      break;
    }
  }

  // 单独拿一下演员列表
  if info.actresses.is_none() {
    if let Ok(wiki) = crawl_website(code, "av-wiki").await {
      if wiki.actresses.is_some() {
        info.actresses = wiki.actresses;
      }
    }
  }

  debug!("Video info: {:?}", info);
  Ok(info)
}

async fn crawl_officials(code: &String) -> Result<VideoInfo> {
  if let Ok(info) = crawl_website(code, "officials").await {
    Ok(info)
  } else {
    crawl_website(code, "prestige").await
  }
}
