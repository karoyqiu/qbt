use std::{sync::Arc, thread::sleep, time::Duration};

use headless_chrome::Tab;
use log::debug;

use crate::{
  error::{err, IntoResult, Result},
  scrape::{Actress, TranslatedText, VideoInfoBuilder},
};

use super::{
  crawler::{convert_date_string_to_epoch, convert_duration_string_to_seconds},
  crawler_cdp::CrawlerCDP,
};

#[derive(Default)]
pub struct Fc2ppvdbCDP;

impl CrawlerCDP for Fc2ppvdbCDP {
  fn get_name(&self) -> &'static str {
    "fc2ppvdb.com"
  }

  fn get_url(&self, code: &String) -> Result<String> {
    let number = code
      .replace("FC2-", "")
      .replace("FC2PPV", "")
      .replace("FC2-PPV-", "")
      .replace("-", "");
    Ok(format!("https://fc2ppvdb.com/articles/{}", number))
  }

  fn get_title(&self, tab: &Arc<Tab>) -> Result<String> {
    let h2 = tab.find_element("h2 > a")?;
    h2.get_inner_text().into_result()
  }

  fn get_info_builder(&self, tab: &Arc<Tab>) -> Result<VideoInfoBuilder> {
    let mut builder = VideoInfoBuilder::default();
    builder.poster(self.get_poster(tab).unwrap_or_default());

    let h2 = tab.find_element("h2")?;
    let result = h2.call_js_fn("function() { return this.parentElement; }", vec![], false)?;
    debug!("Parent element: {:?}", result);

    // builder
    //   .poster(self.get_poster(&tab))
    //   .cover(self.get_cover(&tab))
    //   .outline(self.get_outline(&tab))
    //   .actresses(self.get_actresses(&tab))
    //   .tags(self.get_tags(&tab))
    //   .series(self.get_series(&tab))
    //   .studio(self.get_studio(&tab))
    //   .publisher(self.get_publisher(&tab))
    //   .director(self.get_director(&tab))
    //   .duration(self.get_duration(&tab))
    //   .release_date(self.get_release_date(&tab))
    //   .extra_fanart(self.get_extra_fanart(&tab));

    Ok(builder)
  }

  fn get_poster(&self, tab: &Arc<Tab>) -> Result<Option<String>> {
    let img = tab.find_element("main img")?;
    img.get_attribute_value("src").into_result()
  }
}
