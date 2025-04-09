use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;
use log::debug;
use regex::Regex;
use scraper::{ElementRef, Html};
use url::Url;

use crate::{
  error::{Result, err},
  scrape::{Actress, TranslatedText, VideoInfoBuilder, code::get_code_prefix},
};

use super::{
  crawler::{Crawler, convert_date_string_to_epoch},
  web::{get_selector, optional, remove_first},
};

lazy_static! {
  static ref OFFICIAL_WEBSITES: HashMap<&'static str, &'static &'static str> = {
    let websites = &[
      ("https://s1s1s1.com", "SIVR|SSIS|SSNI|SNIS|SOE|ONED|ONE|ONSD|OFJE|SPS|TKSOE|SONE"),  // https://s1s1s1.com/search/list?keyword=soe
      ("https://moodyz.com", "MIDA|MDVR|MIDV|MIDE|MIDD|MIBD|MIMK|MIID|MIGD|MIFD|MIAE|MIAD|MIAA|MDL|MDJ|MDI|MDG|MDF|MDE|MDLD|MDED|MIZD|MIRD|MDJD|RMID|MDID|MDMD|MIMU|MDPD|MIVD|MDUD|MDGD|MDVD|MIAS|MIQD|MINT|RMPD|MDRD|TKMIDE|TKMIDD|KMIDE|TKMIGD|MDFD|RMWD|MIAB"),
      ("https://madonna-av.com", "JUVR|JUSD|JUQ|JUY|JUX|JUL|JUK|JUC|JUKD|JUSD|OBA|JUFD|ROEB|ROE|URE|MDON|JFB|OBE|JUMS"),
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
      //("https://www.prestige-av.com", "ABP|MBM|EZD|DOCP|ONEZ|YRH|ABW|ABS|CHN|MGT|TRE|EDD|ULT|CMI|MBD|DNW|SGA|RDD|DCX|EVO|RDT|PPT|GETS|SIM|KIL|TUS|DTT|GNAB|MAN|MAS|TBL|RTP|CTD|FIV|DIC|ESK|KBI|TEM|AMA|KFNE|TRD|HAR|YRZ|SRS|MZQ|ZZR|GZAP|TGAV|RIX|AKA|BGN|LXV|AFS|GOAL|GIRO|CPDE|NMP|MCT|ABC|INU|SHL|MBMS|PXH|NRS|FTN|PRDVR|FST|BLO|SHS|KUM|GSX|NDX|ATD|DLD|KBH|BCV|RAW|SOUD|JOB|CHS|YOK|BSD|FSB|NNN|HYK|SOR|HSP|JBS|XND|MEI|DAY|MMY|KZD|JAN|GYAN|TDT|TOK|DMS|FND|CDC|JCN|PVRBST|SDVR|DOCVR|FCP|ABF|PPX"),
    ];

    let mut map = HashMap::new();

    for (url, codes) in websites {
      for code in codes.split('|') {
        map.insert(code, url);
      }
    }

    map
  };

  static ref LAST_POSTER: Mutex<String> = Mutex::new(String::default());

  static ref DESC_RE: Regex = Regex::new(r"【公式】([^(]+)\(([^\)]+)").unwrap();
  static ref MINUTE_RE: Regex = Regex::new(r"([\d]+)分").unwrap();
}

#[derive(Default)]
pub struct Officials;

impl Crawler for Officials {
  fn name(&self) -> &'static str {
    "official website"
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
    url.push_str("/search/list?keyword=");
    url.push_str(&code.replace("-", ""));

    Ok(url)
  }

  fn get_next_url(&self, _code: &String, url: &Url, html: &String) -> Option<String> {
    if !url.path().contains("search") {
      return None;
    }

    let doc = Html::parse_document(html);
    let selector = get_selector("a.img.hover");
    let img = get_selector("img");
    let mut poster = LAST_POSTER.lock().unwrap();
    poster.clear();

    doc.select(&selector).next().map(|a| {
      if let Some(img) = a.select(&img).next() {
        if let Some(src) = img.attr("data-src") {
          *poster = src.to_string();
        }
      }

      a.attr("href").map(String::from)
    })?
  }

  fn get_title(&self, doc: &Html) -> Result<String> {
    let selector = get_selector("h2.p-workPage__title");

    if let Some(elem) = doc.select(&selector).next() {
      let title: String = elem.text().collect();
      Ok(title.trim().to_string())
    } else {
      err("Title not found")
    }
  }

  fn get_poster(&self, _doc: &Html) -> Option<String> {
    let mut poster = LAST_POSTER.lock().unwrap();

    if poster.is_empty() {
      None
    } else {
      let p = poster.clone();
      poster.clear();
      Some(p)
    }
  }

  fn get_cover(&self, doc: &Html) -> Option<String> {
    let img = get_selector("img.swiper-lazy");
    doc
      .select(&img)
      .next()
      .map(|e| e.attr("data-src").map(String::from))?
  }

  fn get_outline(&self, doc: &Html) -> Option<TranslatedText> {
    let p = get_selector("p.p-workPage__text");
    doc
      .select(&p)
      .next()
      .map(|e| TranslatedText::text::<String>(e.text().collect()))
  }

  fn get_actresses(&self, doc: &Html) -> Option<Vec<Actress>> {
    let a = get_selector(r#"a.c-tag.c-main-bg-hover.c-main-font.c-main-bd[href*="/actress/"]"#);
    let actresses: Vec<_> = doc
      .select(&a)
      .map(|e| Actress::name::<String>(e.text().collect()))
      .collect();

    optional(actresses)
  }

  fn get_info_builder(&self, doc: &Html) -> VideoInfoBuilder {
    let th = get_selector("div.th");
    let a = get_selector("a");

    let mut builder = VideoInfoBuilder::default();
    builder
      .poster(self.get_poster(doc))
      .cover(self.get_cover(doc))
      .outline(self.get_outline(doc))
      .actresses(self.get_actresses(doc))
      .extra_fanart(self.get_extra_fanart(doc));

    for th in doc.select(&th) {
      let text: String = th.text().collect();
      let td = next_sibling_element(&th).expect("No td");

      match text.as_str() {
        "ジャンル" => {
          let mut tags = vec![];

          for a in td.select(&a) {
            let text: String = a.text().collect();
            tags.push(text);
          }

          if !tags.is_empty() {
            builder.tags(Some(tags));
          }
        }

        "シリーズ" => {
          if let Some(a) = td.select(&a).next() {
            let text: String = a.text().collect();
            builder.series(Some(text));
          }
        }

        "レーベル" => {
          if let Some(a) = td.select(&a).next() {
            let text: String = a.text().collect();
            builder.publisher(Some(text));
          }
        }

        "監督" => {
          let text: String = td.text().collect();
          let text = text.trim().to_string();
          builder.director(Some(text));
        }

        "発売日" => {
          if let Some(a) = td.select(&a).next() {
            let text: String = a.text().collect();
            builder.release_date(convert_date_string_to_epoch(&text, Some("%Y年%m月%d日")));
          }
        }

        "収録時間" => {
          let text: String = td.text().collect();

          if let Some(captures) = MINUTE_RE.captures(&text) {
            if let Some(duration) = captures.get(1) {
              builder.duration(Some(duration.as_str().parse::<i64>().unwrap() * 60));
            }
          }
        }

        _ => {}
      }
    }

    {
      let meta = get_selector("meta[name=description]");

      if let Some(meta) = doc.select(&meta).next() {
        let content = meta.attr("content").unwrap_or_default();

        if let Some(captures) = DESC_RE.captures(content) {
          if let Some(studio) = captures.get(2) {
            builder.studio(Some(studio.as_str().to_string()));
          }
        }
      }
    }

    builder
  }

  fn get_extra_fanart(&self, doc: &Html) -> Option<Vec<String>> {
    let img = get_selector("img.swiper-lazy");
    let mut arts = vec![];

    for elem in doc.select(&img) {
      let src = elem.attr("data-src")?;
      arts.push(src.to_string());
    }

    remove_first(arts)
  }
}

fn next_sibling_element<'a>(elem: &ElementRef<'a>) -> Option<ElementRef<'a>> {
  let mut next = elem.next_sibling();

  while let Some(sibling) = next {
    if sibling.value().is_element() {
      return ElementRef::wrap(sibling);
    }

    next = sibling.next_sibling();
  }

  None
}
