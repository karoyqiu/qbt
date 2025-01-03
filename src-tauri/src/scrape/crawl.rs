use std::collections::HashMap;

use lazy_static::lazy_static;
use log::{debug, warn};
use regex::Regex;

use crate::error::{err, Result};

use super::{code::is_uncensored, crawlers::officials, VideoInfo};

lazy_static! {
  static ref EU_RE: Regex = Regex::new(r"[^.]+\.\d{2}\.\d{2}\.\d{2}").unwrap();
  static ref DMM_RE: Regex = Regex::new(r"\D{2,}00\d{3,}").unwrap();
  static ref FALENO_RE: Regex = Regex::new(r"F[A-Z]{2}SS").unwrap();
  static ref FANTASTICA_RE: Regex = Regex::new(r"FA[A-Z]{2}-?\d+").unwrap();
  static ref FC2_WEBSITES: Vec<&'static str> = vec![
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
    "airav_cc",
    "iqqtv",
    "javbus",
    "freejavbt",
    "jav321",
    "dmm",
    "javlibrary",
    "7mmtv",
    "hdouban",
    "javdb",
    "avsex",
    "lulubar",
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
  static ref TITLE_ZH_WEBSITE: Vec<&'static str> = vec!["airav_cc", "iqqtv", "avsex", "lulubar"];
  static ref TITLE_WEBSITE_EXCLUDE: Vec<&'static str> = vec![""];
  static ref OUTLINE_WEBSITE: Vec<&'static str> = vec!["theporndb", "dmm", "jav321"];
  static ref OUTLINE_ZH_WEBSITE: Vec<&'static str> = vec!["airav_cc", "avsex", "iqqtv", "lulubar"];
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
  static ref EXTRAFANART_WEBSITE_EXCLUDE: Vec<&'static str> = vec![
    "airav",
    "airav_cc",
    "avsex",
    "avsox",
    "iqqtv",
    "javlibrary",
    "lulubar",
  ];
  static ref TRAILER_WEBSITE: Vec<&'static str> = vec!["freejavbt", "mgstage", "dmm"];
  static ref TRAILER_WEBSITE_EXCLUDE: Vec<&'static str> = vec!["7mmtv", "lulubar"];
  static ref TAG_WEBSITE: Vec<&'static str> = vec!["javbus", "freejavbt"];
  static ref TAG_WEBSITE_EXCLUDE: Vec<&'static str> = vec![""];
  static ref RELEASE_WEBSITE: Vec<&'static str> = vec!["javbus", "freejavbt", "7mmtv"];
  static ref RELEASE_WEBSITE_EXCLUDE: Vec<&'static str> = vec!["fc2club", "fc2hub"];
  static ref DURATION_WEBSITE: Vec<&'static str> = vec!["javbus", "freejavbt"];
  static ref RUNTIME_WEBSITE_EXCLUDE: Vec<&'static str> =
    vec!["airav", "airav_cc", "fc2", "fc2club", "fc2hub", "lulubar"];
  static ref SCORE_WEBSITE: Vec<&'static str> = vec!["jav321", "javlibrary", "javdb"];
  static ref SCORE_WEBSITE_EXCLUDE: Vec<&'static str> = vec![
    "airav", "airav_cc", "avsex", "avsox", "7mmtv", "fc2", "fc2hub", "iqqtv", "javbus", "xcity",
    "lulubar",
  ];
  static ref DIRECTOR_WEBSITE: Vec<&'static str> = vec!["javbus", "freejavbt"];
  static ref DIRECTOR_WEBSITE_EXCLUDE: Vec<&'static str> = vec![
    "airav", "airav_cc", "avsex", "avsox", "fc2", "fc2hub", "iqqtv", "jav321", "mgstage",
    "lulubar",
  ];
  static ref SERIES_WEBSITE: Vec<&'static str> = vec!["javbus", "freejavbt"];
  static ref SERIES_WEBSITE_EXCLUDE: Vec<&'static str> = vec![
    "airav",
    "airav_cc",
    "avsex",
    "iqqtv",
    "7mmtv",
    "javlibrary",
    "lulubar",
  ];
  static ref STUDIO_WEBSITE: Vec<&'static str> = vec!["javbus", "freejavbt"];
  static ref STUDIO_WEBSITE_EXCLUDE: Vec<&'static str> = vec!["avsex"];
  static ref PUBLISHER_WEBSITE: Vec<&'static str> = vec!["javbus"];
  static ref PUBLISHER_WEBSITE_EXCLUDE: Vec<&'static str> =
    vec!["airav", "airav_cc", "avsex", "iqqtv", "lulubar"];
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
      vec![
        "airav",
        "airav_cc",
        "avsex",
        "avsox",
        "iqqtv",
        "javlibrary",
        "lulubar",
      ],
    );
    m.insert("trailer", vec!["7mmtv", "lulubar"]);
    m.insert("tag", vec![]);
    m.insert("release", vec!["fc2club", "fc2hub"]);
    m.insert(
      "recorded",
      vec!["airav", "airav_cc", "fc2", "fc2club", "fc2hub", "lulubar"],
    );
    m.insert(
      "score",
      vec![
        "airav", "airav_cc", "avsex", "avsox", "7mmtv", "fc2", "fc2hub", "iqqtv", "javbus",
        "xcity", "lulubar",
      ],
    );
    m.insert(
      "director",
      vec![
        "airav", "airav_cc", "avsex", "avsox", "fc2", "fc2hub", "iqqtv", "jav321", "mgstage",
        "lulubar",
      ],
    );
    m.insert(
      "series",
      vec![
        "airav",
        "airav_cc",
        "avsex",
        "iqqtv",
        "7mmtv",
        "javlibrary",
        "lulubar",
      ],
    );
    m.insert("studio", vec!["avsex"]);
    m.insert(
      "publisher",
      vec!["airav", "airav_cc", "avsex", "iqqtv", "lulubar"],
    );
    m
  };
}

pub async fn crawl(code: &String) -> Result<()> {
  // TODO: 先判断是不是国产，避免浪费时间

  if code.starts_with("FC2") {
    // FC2: FC2-111111
    crawl_websites(code, &FC2_WEBSITES).await?;
  } else if code.starts_with("KIN8") {
    // kin8
    crawl_website(code, "kin8").await?;
  } else if code.starts_with("DLID") {
    // 同人
    crawl_website(code, "getchu").await?;
  } else if code.contains("GETCHU") {
    // 里番
    crawl_website(code, "getchu_dmm").await?;
  } else if code.starts_with("Mywife") {
    // Mywife No.1111
    crawl_website(code, "mywife").await?;
  } else if EU_RE.is_match(code) {
    // 欧美: sexart.15.06.14
    crawl_websites(code, &EU_WEBSITES).await?;
  } else if is_uncensored(code) {
    // 无码
    crawl_websites(code, &UNCENSORED_WEBSITES).await?;
  }
  if code.starts_with("SIRO") {
    // 素人
    // TODO: 259LUXU-1111
    crawl_websites(code, &AMATEUR_WEBSITES).await?;
  } else if DMM_RE.is_match(code) && !code.contains("-") && !code.contains("_") {
    // DMM: 00ID-111
    crawl_websites(code, &DMM_WEBSITES).await?;
  } else {
    // 有碼
    crawl_websites(code, &CENSORED_WEBSITES).await?;
  }

  Ok(())
}

async fn crawl_website(code: &String, website: &str) -> Result<()> {
  Ok(())
}

/// 获取一组网站的数据：按照设置的网站组，请求各字段数据，并返回最终的数据
async fn crawl_websites(code: &String, websites: &Vec<&'static str>) -> Result<VideoInfo> {
  // 获取使用的网站
  let mut titles = get_websites(&TITLE_WEBSITE, websites, code, "title");
  titles.insert(0, "official");
  let title_zh = get_websites(&TITLE_ZH_WEBSITE, websites, code, "title_zh");
  let outlines = get_websites(&OUTLINE_WEBSITE, websites, code, "outline");
  let outline_zh = get_websites(&OUTLINE_ZH_WEBSITE, websites, code, "outline_zh");
  let actresses = get_websites(&ACTRESS_WEBSITE, websites, code, "actress");
  let thumbs = get_websites(&THUMB_WEBSITE, websites, code, "thumb");
  let posters = get_websites(&POSTER_WEBSITE, websites, code, "poster");
  let extrafanarts = get_websites(&EXTRAFANART_WEBSITE, websites, code, "extrafanart");
  let trailers = get_websites(&TRAILER_WEBSITE, websites, code, "trailer");
  let tags = get_websites(&TAG_WEBSITE, websites, code, "tag");
  let releases = get_websites(&RELEASE_WEBSITE, websites, code, "release");
  let durations = get_websites(&DURATION_WEBSITE, websites, code, "duration");
  let scores = get_websites(&SCORE_WEBSITE, websites, code, "score");
  let directors = get_websites(&DIRECTOR_WEBSITE, websites, code, "director");
  let series = get_websites(&SERIES_WEBSITE, websites, code, "series");
  let studios = get_websites(&STUDIO_WEBSITE, websites, code, "studio");
  let publishers = get_websites(&PUBLISHER_WEBSITE, websites, code, "publisher");

  let requested_fields = [
    ("title", "标题", titles),
    ("title_zh", "中文标题", title_zh),
    ("outline", "简介", outlines),
    ("outline_zh", "中文简介", outline_zh),
    ("actress", "演员", actresses),
    ("cover", "封面", thumbs),
    ("poster", "海报", posters),
    ("extrafanart", "剧照", extrafanarts),
    ("tag", "标签", tags),
    ("release", "发行日期", releases),
    ("duration", "时长", durations),
    ("score", "评分", scores),
    ("director", "导演", directors),
    ("series", "系列", series),
    ("studio", "片商", studios),
    ("publisher", "发行商", publishers),
    ("trailer", "预告片", trailers),
  ];

  let mut info = VideoInfo::default();
  let mut cache = HashMap::new();

  for (field, name, websites) in requested_fields {
    let crawled = call_crawlers(code, &websites, &mut cache).await?;
    info.apply(crawled);
  }

  debug!("Video info: {:?}", info);
  Ok(info)
}

fn get_websites(
  field_websites: &Vec<&'static str>,
  video_websites: &Vec<&'static str>,
  code: &String,
  field: &str,
) -> Vec<&'static str> {
  // 取交集
  let mut websites = intersect(field_websites, video_websites);

  // 取剩余未相交网站， trailer 不取未相交网站，title 默认取未相交网站
  if field == "title" || WHOLE_FIELDS.contains(&field) {
    if field != "trailer" {
      let diff = sub(video_websites, field_websites);
      websites = [websites, diff].concat();
    }
  }

  // 根据字段排除一些不含这些字段的网站
  if let Some(excluded) = FIELD_EXCLUDES.get(field) {
    websites = sub(&websites, excluded);
  }

  // TODO: 素人番号检查

  if FALENO_RE.is_match(&code) {
    // faleno.jp 番号检查
    adjust_websites(&mut websites, field, "faleno");
  } else if code.starts_with("DLDSS") || code.starts_with("DHLA") {
    // dahlia-av.jp 番号检查 dldss177 dhla009
    adjust_websites(&mut websites, field, "dahlia");
  } else if FANTASTICA_RE.is_match(&code)
    || code.starts_with("CLASS")
    || code.starts_with("FADRV")
    || code.starts_with("FAPRO")
    || code.starts_with("FAKWM")
    || code.starts_with("PDS")
  {
    // fantastica 番号检查 FAVI、FAAP、FAPL、FAKG、FAHO、FAVA、FAKY、FAMI、FAIT、FAKA、FAMO、FASO、FAIH、FASH、FAKS、FAAN
    adjust_websites(&mut websites, field, "fantastica");
  }

  websites
}

fn intersect(a: &Vec<&'static str>, b: &Vec<&'static str>) -> Vec<&'static str> {
  a.iter()
    .filter(|website| b.contains(website))
    .map(|website| *website)
    .collect()
}

fn sub(a: &Vec<&'static str>, b: &Vec<&'static str>) -> Vec<&'static str> {
  a.iter()
    .filter(|website| !b.contains(website))
    .map(|website| *website)
    .collect()
}

fn adjust_websites(websites: &mut Vec<&'static str>, field: &str, website: &'static str) {
  if !websites.contains(&&website) {
    websites.push(&website);
  }

  if [
    "title",
    "outline",
    "thumb",
    "poster",
    "trailer",
    "extrafanart",
  ]
  .contains(&field)
  {
    if let Some(index) = websites.iter().position(|&x| x == website) {
      websites.remove(index);
    }

    websites.insert(0, &website);
  } else if ["tag", "score", "director", "series"].contains(&field) {
    if let Some(index) = websites.iter().position(|&x| x == website) {
      websites.remove(index);
    }
  }
}

/// 按照设置的网站顺序获取各个字段信息
async fn call_crawlers(
  code: &String,
  websites: &Vec<&'static str>,
  cache: &mut HashMap<&'static str, VideoInfo>,
) -> Result<VideoInfo> {
  let mut info = VideoInfo::default();

  for &website in websites {
    if let Some(cached) = cache.get(website) {
      info = cached.clone();
      break;
    }

    let result = match website {
      // 官方网站
      "official" => officials::crawl(code).await,
      _ => err("Unknown website"),
    };

    match result {
      Ok(crawled) => {
        cache.insert(website, crawled.clone());
        info = crawled;
        break;
      }
      Err(e) => {
        warn!("Crawl {} failed: {}", website, e);
      }
    }
  }

  Ok(info)
}
