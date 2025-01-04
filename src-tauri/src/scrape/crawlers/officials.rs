use std::collections::HashMap;

use lazy_static::lazy_static;
use log::{debug, info};
use scraper::{ElementRef, Html, Selector};
use url::Url;

use crate::{
  error::{err, IntoResult, Result},
  scrape::{
    code::get_code_prefix, crawlers::web::DIV_SELECTOR, TranslatedText, VideoInfo, VideoInfoBuilder,
  },
};

use super::{
  crawler::Crawler,
  web::{get_html, get_selector},
};

lazy_static! {
  static ref OFFICIAL_WEBSITES: HashMap<&'static str, &'static &'static str> = {
    let websites = &[
      ("https://s1s1s1.com", "SIVR|SSIS|SSNI|SNIS|SOE|ONED|ONE|ONSD|OFJE|SPS|TKSOE"),  // https://s1s1s1.com/search/list?keyword=soe
      ("https://moodyz.com", "MDVR|MIDV|MIDE|MIDD|MIBD|MIMK|MIID|MIGD|MIFD|MIAE|MIAD|MIAA|MDL|MDJ|MDI|MDG|MDF|MDE|MDLD|MDED|MIZD|MIRD|MDJD|RMID|MDID|MDMD|MIMU|MDPD|MIVD|MDUD|MDGD|MDVD|MIAS|MIQD|MINT|RMPD|MDRD|TKMIDE|TKMIDD|KMIDE|TKMIGD|MDFD|RMWD|MIAB"),
      ("https://www.madonna-av.com", "JUVR|JUSD|JUQ|JUY|JUX|JUL|JUK|JUC|JUKD|JUSD|OBA|JUFD|ROEB|ROE|URE|MDON|JFB|OBE|JUMS"),
      ("https://www.wanz-factory.com", "WAVR|WAAA|BMW|WANZ"),
      ("https://ideapocket.com", "IPVR|IPX|IPZ|IPTD|IPSD|IDBD|SUPD|IPIT|AND|HPD|TKIPZ|IPZZ|COSD|ANPD|DAN|ALAD|KIPX"),
      ("https://kirakira-av.com", "KIVR|BLK|KIBD|KIFD|KIRD|KISD|SET"),
      ("https://www.av-e-body.com", "EBVR|EBOD|MKCK|EYAN"),
      ("https://bi-av.com", "CJVR|CJOD|BBI|BIB|CJOB|BEB|BID|BIST|BWB"),
      ("https://premium-beauty.com", "PRVR|PGD|PRED|PBD|PJD|PRTD|PXD|PID|PTV"),
      ("https://miman.jp", "MMVR|MMND|MMXD|AOM"),
      ("https://tameikegoro.jp", "MEVR|MEYD|MBYD|MDYD|MNYD"),
      ("https://fitch-av.com", "FCVR|JUFE|JUFD|JFB|JUNY|NYB|FINH|GCF|NIMA"),
      ("https://kawaiikawaii.jp", "KAVR|CAWD|KWBD|KAWD|KWSR|KWSD|KANE"),
      ("https://befreebe.com", "BF"),
      ("https://muku.tv", "MUCD|MUDR|MUKD|SMCD|MUKC"),
      ("https://attackers.net", "ATVR|RBK|RBD|SAME|SHKD|ATID|ADN|ATKD|JBD|SSPD|ATAD|AZSD"),
      ("https://mko-labo.net", "MVR|MISM|EMLB"),
      ("https://dasdas.jp", "DSVR|DASS|DAZD|DASD|PLA"),
      ("https://mvg.jp", "MVSD|MVBD"),
      ("https://av-opera.jp", "OPVR|OPBD|OPUD"),
      ("https://oppai-av.com", "PPVR|PPPE|PPBD|PPPD|PPSD|PPFD"),
      ("https://v-av.com", "VVVD|VICD|VIZD|VSPD"),
      ("https://to-satsu.com", "CLVR|STOL|CLUB"),
      ("https://bibian-av.com", "BBVR|BBAN"),
      ("https://honnaka.jp", "HNVR|HMN|HNDB|HND|KRND|HNKY|HNJC|HNSE"),
      ("https://rookie-av.jp", "RVR|RBB|RKI"),
      ("https://nanpa-japan.jp", "NJVR|NNPJ|NPJB"),
      ("https://hajimekikaku.com", "HJBB|HJMO|AVGL"),
      ("https://hhh-av.com", "HUNTB|HUNTA|HUNT|HUNBL|ROYD|TYSF"),
      ("https://www.prestige-av.com", "ABP|MBM|EZD|DOCP|ONEZ|YRH|ABW|ABS|CHN|MGT|TRE|EDD|ULT|CMI|MBD|DNW|SGA|RDD|DCX|EVO|RDT|PPT|GETS|SIM|KIL|TUS|DTT|GNAB|MAN|MAS|TBL|RTP|CTD|FIV|DIC|ESK|KBI|TEM|AMA|KFNE|TRD|HAR|YRZ|SRS|MZQ|ZZR|GZAP|TGAV|RIX|AKA|BGN|LXV|AFS|GOAL|GIRO|CPDE|NMP|MCT|ABC|INU|SHL|MBMS|PXH|NRS|FTN|PRDVR|FST|BLO|SHS|KUM|GSX|NDX|ATD|DLD|KBH|BCV|RAW|SOUD|JOB|CHS|YOK|BSD|FSB|NNN|HYK|SOR|HSP|JBS|XND|MEI|DAY|MMY|KZD|JAN|GYAN|TDT|TOK|DMS|FND|CDC|JCN|PVRBST|SDVR|DOCVR|FCP|ABF|PPX"),
    ];

    let mut map = HashMap::new();

    for (url, codes) in websites {
      for code in codes.split('|') {
        map.insert(code, url);
      }
    }

    map
  };

  static ref POSTER_LINK_SELECTOR: Selector = Selector::parse("a.img.hover").unwrap();
  static ref POSTER_IMG_SELECTOR: Selector = Selector::parse("img[data-src]").unwrap();
  static ref TITLE_SELECTOR: Selector = Selector::parse("h2.p-workPage__title").unwrap();
  static ref COVER_SELECTOR: Selector = Selector::parse("img.swiper-lazy").unwrap();
  static ref OUTLINE_SELECTOR: Selector = Selector::parse("p.p-workPage__text").unwrap();
  static ref ACTRESS_SELECTOR: Selector = Selector::parse("a.c-tag.c-main-bg-hover.c-main-font.c-main-bd").unwrap();

  static ref FIRST_DIV_SELECTOR: Selector = Selector::parse("div:first-of-type").unwrap();
  static ref DIV_A_SELECTOR: Selector = Selector::parse("div:first-of-type > a").unwrap();
  static ref DIV_DIV_A_SELECTOR: Selector = Selector::parse("div:first-of-type > div > a").unwrap();
  static ref DIV_DIV_P_SELECTOR: Selector = Selector::parse("div:first-of-type > div > p").unwrap();
}

#[derive(Default)]
pub struct Officials;

impl Crawler for Officials {
  fn get_name(&self) -> &'static str {
    "official"
  }

  fn get_url(&self, code: &String) -> Result<String> {
    let prefix = get_code_prefix(code);

    if prefix.is_none() {
      return err("Invalid code");
    }

    let prefix = prefix.unwrap();
    debug!("Code prefix: {}", prefix);
    let url = OFFICIAL_WEBSITES.get(prefix.as_str());

    if url.is_none() {
      return err("No official website found");
    }

    let mut url = String::from(**url.unwrap());

    if url == "https://www.prestige-av.com" {
      // TODO: Prestige
    }

    url.push_str("/search/list?keyword=");
    url.push_str(&code.replace("-", ""));

    Ok(url)
  }

  fn get_next_url(&self, code: &String, html: &String) -> Option<String> {
    let html = Html::parse_document(html);
    None
  }

  fn get_title(&self, doc: &Html) -> Result<String> {
    let selector = get_selector("h2.p-workPage__title");

    if let Some(elem) = doc.select(&selector).next() {
      Ok(elem.text().collect())
    } else {
      err("Title not found")
    }
  }
}

pub async fn crawl(code: &String) -> Result<VideoInfo> {
  info!("Crawling official website for {}", code);
  let prefix = get_code_prefix(code);

  if prefix.is_none() {
    return err("Invalid code");
  }

  let prefix = prefix.unwrap();
  debug!("Code prefix: {}", prefix);
  let url = OFFICIAL_WEBSITES.get(prefix.as_str());

  if url.is_none() {
    return err("No official website found");
  }

  let mut url = String::from(**url.unwrap());

  if url == "https://www.prestige-av.com" {
    // TODO: Prestige
  }

  url.push_str("/search/list?keyword=");
  url.push_str(&code.replace("-", ""));

  let (html, _) = get_html(&url).await?;
  let (href, poster) = {
    let doc = Html::parse_document(&html);
    get_poster(&doc, code)?
  };

  debug!("Video url: {}", href);
  debug!("Poster url: {}", poster);
  let (html, _) = get_html(&href).await?;
  let doc = Html::parse_document(&html);

  let mut builder = VideoInfoBuilder::default();
  builder
    .code(code.to_string())
    .poster(Some(poster))
    .cover(get_cover(&doc))
    .actresses(get_actresses(&doc));

  let title = get_title(&doc)?;
  builder.title(TranslatedText {
    text: title,
    translated: None,
  });

  for div in doc.select(&DIV_SELECTOR) {
    let text: String = div.text().collect();
    //let class = div.attr("class").unwrap_or_default();

    if text.contains("製作商") {
      builder.studio(get_text(&div, &FIRST_DIV_SELECTOR));
    } else if text.contains("発売日") {
      let release = get_text(&div, &DIV_DIV_A_SELECTOR);
      debug!("Release date: {:?}", release);
    } else if text.contains("シリーズ") {
      builder.series(get_text(&div, &DIV_A_SELECTOR));
    } else if text.contains("監督") {
      builder.director(get_text(&div, &DIV_DIV_P_SELECTOR));
    } else if text.contains("レーベル") {
      builder.publisher(get_text(&div, &DIV_A_SELECTOR));
    } else if text.contains("収録時間") {
      let time = get_text(&div, &DIV_DIV_P_SELECTOR);
      debug!("Record time: {:?}", time);
    } else if text.contains("ジャンル") {
      let tags: Vec<String> = div
        .select(&DIV_DIV_A_SELECTOR)
        .map(|a| a.text().collect())
        .collect();

      if !tags.is_empty() {
        builder.tags(Some(tags));
      }
    }
  }

  Ok(builder.build().into_result()?)
}

fn get_poster(doc: &Html, code: &String) -> Result<(String, String)> {
  for elem in doc.select(&POSTER_LINK_SELECTOR) {
    if let Some(href) = elem.value().attr("href") {
      if href.to_uppercase().contains(&code.replace("-", "")) {
        if let Some(img) = elem.select(&POSTER_IMG_SELECTOR).next() {
          if let Some(src) = img.value().attr("data-src") {
            return Ok((href.to_string(), src.to_string()));
          }
        }
      }
    }
  }

  err("Poster not found")
}

fn get_title(doc: &Html) -> Result<String> {
  if let Some(elem) = doc.select(&TITLE_SELECTOR).next() {
    Ok(elem.text().collect())
  } else {
    err("Title not found")
  }
}

fn get_cover(doc: &Html) -> Option<String> {
  if let Some(elem) = doc.select(&COVER_SELECTOR).next() {
    if let Some(src) = elem.value().attr("data-src") {
      return Some(src.to_string());
    }
  }

  None
}

fn get_actresses(doc: &Html) -> Option<Vec<String>> {
  let mut actresses = Vec::new();

  for elem in doc.select(&ACTRESS_SELECTOR) {
    if let Some(href) = elem.value().attr("href") {
      if href.contains("/actress/") {
        actresses.push(elem.text().collect());
      }
    }
  }

  if actresses.is_empty() {
    None
  } else {
    Some(actresses)
  }
}

fn get_text(parent: &ElementRef, selector: &Selector) -> Option<String> {
  if let Some(elem) = parent.select(selector).next() {
    let text: String = elem.text().collect();

    if !text.is_empty() {
      return Some(text);
    }
  }

  None
}
