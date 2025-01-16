use chrono::DateTime;
use log::debug;
use serde::Deserialize;

use crate::{
  error::{err, IntoResult, Result},
  scrape::{code::get_code_prefix, Actress, TranslatedText, VideoInfo, VideoInfoBuilder},
};

use super::crawler::Crawler;

#[derive(Deserialize)]
struct SearchResult {
  hits: HitOuter,
}

#[derive(Deserialize)]
struct HitOuter {
  hits: Vec<HitInner>,
}

#[derive(Deserialize)]
struct HitInner {
  #[serde(rename = "_source")]
  source: Source,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Source {
  product_uuid: String,
  delivery_item_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Product {
  title: String,
  body: String,
  play_time: i64,
  maker: Option<Name>,
  label: Option<Name>,
  series: Option<Name>,
  genre: Option<Vec<Name>>,
  directors: Option<Vec<Name>>,
  thumbnail: Path,
  sku: Vec<Sku>,
  package_image: Path,
  actress: Vec<Name>,
  media: Option<Vec<Path>>,
}

#[derive(Deserialize)]
struct Name {
  name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Sku {
  sales_start_at: String,
}

#[derive(Deserialize)]
struct Path {
  path: String,
}

#[derive(Default)]
pub struct Prestige;

impl Crawler for Prestige {
  fn get_name(&self) -> &'static str {
    "prestige-av.com"
  }

  fn get_url(&self, code: &String) -> Result<String> {
    static PREFIXES: &[&str] = &[
      "ABC", "ABF", "ABP", "ABS", "ABW", "AFS", "AKA", "AMA", "ATD", "BCV", "BGN", "BLO", "BSD",
      "CDC", "CHN", "CHS", "CMI", "CPDE", "CTD", "DAY", "DCX", "DIC", "DLD", "DMS", "DNW", "DOCP",
      "DOCVR", "DTT", "EDD", "ESK", "EVO", "EZD", "FCP", "FIV", "FND", "FSB", "FST", "FTN", "GETS",
      "GIRO", "GNAB", "GOAL", "GSX", "GYAN", "GZAP", "HAR", "HSP", "HYK", "INU", "JAN", "JBS",
      "JCN", "JOB", "KBH", "KBI", "KFNE", "KIL", "KUM", "KZD", "LXV", "MAN", "MAS", "MBD", "MBM",
      "MBMS", "MCT", "MEI", "MGT", "MMY", "MZQ", "NDX", "NMP", "NNN", "NRS", "ONEZ", "PPT", "PPX",
      "PRDVR", "PVRBST", "PXH", "RAW", "RDD", "RDT", "RIX", "RTP", "SDVR", "SGA", "SHL", "SHS",
      "SIM", "SOR", "SOUD", "SRS", "TBL", "TDT", "TEM", "TGAV", "TOK", "TRD", "TRE", "TUS", "ULT",
      "XND", "YOK", "YRH", "YRZ", "ZZR",
    ];
    let prefix = get_code_prefix(code).unwrap_or_default();

    if PREFIXES.binary_search(&prefix.as_str()).is_ok() {
      Ok(format!("https://www.prestige-av.com/api/search?isEnabledQuery=true&searchText={}&isEnableAggregation=false&release=false&reservation=false&soldOut=false&from=0&aggregationTermsSize=0&size=20",code))
    } else {
      err("Invalid code")
    }
  }

  fn get_next_url(&self, code: &String, _url: &url::Url, json: &String) -> Option<String> {
    let result = serde_json::from_str::<SearchResult>(json).ok()?;

    for hit in result.hits.hits {
      if hit.source.delivery_item_id.ends_with(code) {
        return Some(format!(
          "https://www.prestige-av.com/api/product/{}",
          hit.source.product_uuid
        ));
      }
    }

    None
  }

  fn get_info(&self, code: &String, html: &str) -> Result<VideoInfo> {
    debug!("Product: {}", html);
    let product: Product = serde_json::from_str(&html).into_result()?;
    let title = product.title.replace("【配信専用】", "");

    if title.is_empty() {
      return err("No title");
    }

    let mut builder = VideoInfoBuilder::default();
    builder
      .code(code.clone())
      .title(TranslatedText::text(title))
      .poster(Some(product.thumbnail.path))
      .cover(Some(product.package_image.path))
      .outline(Some(TranslatedText::text(product.body)))
      .series(product.series.map(|n| n.name))
      .studio(product.maker.map(|n| n.name))
      .publisher(product.label.map(|n| n.name))
      .duration(Some(product.play_time * 60));

    if !product.sku.is_empty() {
      if let Ok(date) = DateTime::parse_from_rfc3339(&product.sku[0].sales_start_at) {
        builder.release_date(Some(date.timestamp()));
      }
    }

    if !product.actress.is_empty() {
      let mut actresses = vec![];

      for actress in product.actress {
        actresses.push(Actress::name(actress.name));
      }

      builder.actresses(Some(actresses));
    }

    if let Some(genre) = product.genre {
      if !genre.is_empty() {
        let mut tags = vec![];

        for tag in genre {
          tags.push(tag.name);
        }

        builder.tags(Some(tags));
      }
    }

    if let Some(media) = product.media {
      if !media.is_empty() {
        let mut arts = vec![];

        for m in media {
          arts.push(format!("https://www.prestige-av.com/api/media/{}", m.path));
        }

        builder.extra_fanart(Some(arts));
      }
    }

    if let Some(mut directors) = product.directors {
      if !directors.is_empty() {
        let director = directors.remove(0);
        builder.director(Some(director.name));
      }
    }

    builder.build().into_result()
  }
}
