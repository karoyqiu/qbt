use std::{ffi::OsStr, sync::Arc, time::Duration};

use cookie::SameSite;
use headless_chrome::{
  protocol::cdp::{
    Network::{CookieParam, CookieSameSite},
    Runtime::{RemoteObjectSubtype, RemoteObjectType},
    DOM::RequestNode,
  },
  Browser, Element, LaunchOptionsBuilder, Tab,
};
use log::{debug, error, info};
use url::Url;

use crate::{
  error::{err, IntoResult, Result},
  scrape::{
    crawlers::{load_cookies, web::get_proxy},
    Actress, TranslatedText, VideoInfo, VideoInfoBuilder,
  },
};

pub trait CrawlerCDP {
  /** 网站名称 */
  fn name(&self) -> &'static str;

  /** 网站地址 */
  fn get_url(&self, code: &String) -> Result<String>;

  /** 下一步地址 */
  fn goto_next_url(&self, _url: &Url, _tab: &Arc<Tab>) -> Result<bool> {
    Ok(false)
  }

  /** 标题 */
  fn get_title(&self, tab: &Arc<Tab>) -> Result<String>;

  /** 信息构建器 */
  fn get_info_builder(&self, tab: &Arc<Tab>) -> Result<VideoInfoBuilder> {
    let mut builder = VideoInfoBuilder::default();

    builder
      .poster(self.get_poster(&tab).unwrap_or_default())
      .cover(self.get_cover(&tab).unwrap_or_default())
      .outline(self.get_outline(&tab).unwrap_or_default())
      .actresses(self.get_actresses(&tab).unwrap_or_default())
      .tags(self.get_tags(&tab).unwrap_or_default())
      .series(self.get_series(&tab).unwrap_or_default())
      .studio(self.get_studio(&tab).unwrap_or_default())
      .publisher(self.get_publisher(&tab).unwrap_or_default())
      .director(self.get_director(&tab).unwrap_or_default())
      .duration(self.get_duration(&tab).unwrap_or_default())
      .release_date(self.get_release_date(&tab).unwrap_or_default())
      .extra_fanart(self.get_extra_fanart(&tab).unwrap_or_default());

    Ok(builder)
  }

  /** 海报 */
  fn get_poster(&self, _tab: &Arc<Tab>) -> Result<Option<String>> {
    Ok(None)
  }

  /** 封面 */
  fn get_cover(&self, _tab: &Arc<Tab>) -> Result<Option<String>> {
    Ok(None)
  }

  /** 简介 */
  fn get_outline(&self, _tab: &Arc<Tab>) -> Result<Option<TranslatedText>> {
    Ok(None)
  }

  /** 演员列表 */
  fn get_actresses(&self, _tab: &Arc<Tab>) -> Result<Option<Vec<Actress>>> {
    Ok(None)
  }

  /** 标签列表 */
  fn get_tags(&self, _tab: &Arc<Tab>) -> Result<Option<Vec<String>>> {
    Ok(None)
  }

  /** 系列 */
  fn get_series(&self, _tab: &Arc<Tab>) -> Result<Option<String>> {
    Ok(None)
  }

  /** 片商 */
  fn get_studio(&self, _tab: &Arc<Tab>) -> Result<Option<String>> {
    Ok(None)
  }

  /** 发行商 */
  fn get_publisher(&self, _tab: &Arc<Tab>) -> Result<Option<String>> {
    Ok(None)
  }

  /** 导演 */
  fn get_director(&self, _tab: &Arc<Tab>) -> Result<Option<String>> {
    Ok(None)
  }

  /** 时长（秒） */
  fn get_duration(&self, _tab: &Arc<Tab>) -> Result<Option<i64>> {
    Ok(None)
  }

  /** 发布日期（Unix epoch） */
  fn get_release_date(&self, _tab: &Arc<Tab>) -> Result<Option<i64>> {
    Ok(None)
  }

  /** 额外的插图 */
  fn get_extra_fanart(&self, _tab: &Arc<Tab>) -> Result<Option<Vec<String>>> {
    Ok(None)
  }
}

/// 刮削
pub async fn crawl_cdp<T>(crawler: &T, code: &String) -> Result<VideoInfo>
where
  T: CrawlerCDP + ?Sized,
{
  info!("Crawling {} for {}", crawler.name(), code);

  let mut builder = LaunchOptionsBuilder::default();
  builder
    .disable_default_args(true)
    .enable_gpu(true)
    .headless(false)
    .idle_browser_timeout(Duration::from_secs(180))
    .ignore_certificate_errors(false)
    .window_size(Some((1920, 1080)))
    .args(vec![
      OsStr::new("--headless=new"),
      // OsStr::new("--disable-background-timer-throttling"),
      // OsStr::new("--disable-backgrounding-occluded-windows"),
      // OsStr::new("--disable-blink-features=AutomationControlled"),
      // OsStr::new("--disable-component-extensions-with-background-pages"),
      // OsStr::new("--disable-renderer-backgrounding"),
      // OsStr::new("--hide-crash-restore-bubble"),
      // OsStr::new("--no-default-browser-check"),
      // OsStr::new("--start-maximized"),
    ]);

  let proxy = get_proxy()?;

  if let Some(proxy) = &proxy {
    builder.proxy_server(Some(proxy));
  }

  let options = builder.build().into_result()?;

  let browser = Browser::new(options)?;
  let tab = browser.new_tab()?;
  tab.set_default_timeout(Duration::from_secs(60))
    .set_slow_motion_multiplier(1.0)
    .set_user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0", None, None)?;

  let result = crawl_with_browser(crawler, code, &tab);

  if result.is_err() {
    error!("Fuck");
    take_screenshot(&tab, "fuck")?;
    return Err(result.unwrap_err());
  }

  let (url, mut info) = result.unwrap();

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

fn crawl_with_browser<T>(crawler: &T, code: &String, tab: &Arc<Tab>) -> Result<(Url, VideoInfo)>
where
  T: CrawlerCDP + ?Sized,
{
  let url = crawler.get_url(code)?;
  debug!("Navigating to {}", url);
  tab.navigate_to(&url)?.wait_until_navigated()?;

  debug!("Setting cookies and reloading");
  set_cookies(&tab, &Url::parse(&url).into_result()?)?;
  tab.navigate_to(&url)?.wait_until_navigated()?;

  let mut url = Url::parse(&tab.get_target_info()?.url).into_result()?;
  debug!("Current url: {}", url);

  while crawler.goto_next_url(&url, &tab)? {
    //tab.wait_until_navigated()?;
    std::thread::sleep(Duration::from_secs(3));
    url = Url::parse(&tab.get_target_info()?.url).into_result()?;
    debug!("Current url: {}", url);
  }

  let info = {
    debug!("Url: {}", url);

    let title = crawler.get_title(&tab)?;
    crawler
      .get_info_builder(&tab)?
      .code(code.clone())
      .title(TranslatedText::text(title))
      .build()
      .into_result()?
  };

  Ok((url, info))
}

fn set_cookies(tab: &Arc<Tab>, url: &Url) -> Result<()> {
  let jar = load_cookies()?;

  let cookies: Vec<_> = jar
    .matches(&url)
    .into_iter()
    .map(|c| CookieParam {
      name: c.name().to_string(),
      value: c.value().to_string(),
      url: None,
      domain: c.domain().map(String::from),
      path: c.path().map(String::from),
      secure: c.secure(),
      http_only: c.http_only(),
      same_site: c.same_site().map(same_site_to_cookie_same_site),
      expires: c
        .expires()
        .map(|e| e.datetime().map(|d| d.unix_timestamp() as f64))
        .unwrap_or_default(),
      priority: None,
      same_party: None,
      source_scheme: None,
      source_port: None,
      partition_key: None,
    })
    .collect();

  tab.set_cookies(cookies)?;

  Ok(())
}

fn same_site_to_cookie_same_site(value: SameSite) -> CookieSameSite {
  match value {
    SameSite::Strict => CookieSameSite::Strict,
    SameSite::Lax => CookieSameSite::Lax,
    SameSite::None => CookieSameSite::None,
  }
}

pub fn get_parent_element<'a>(elem: &'a Element<'a>) -> Result<Element<'a>> {
  let tab = elem.parent;
  let result = elem.call_js_fn("function() { return this.parentElement; }", vec![], false)?;
  debug!("Parent element: {:?}", result);

  if result.Type != RemoteObjectType::Object
    || result.subtype != Some(RemoteObjectSubtype::Node)
    || result.object_id.is_none()
  {
    return err("Parent not found");
  }

  let result = tab.call_method(RequestNode {
    object_id: result.object_id.unwrap(),
  })?;

  Element::new(tab, result.node_id).into_result()
}

pub fn take_screenshot(tab: &Arc<Tab>, name: &str) -> Result<()> {
  let png = tab.capture_screenshot(
    headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Png,
    None,
    None,
    true,
  )?;

  let mut temp = std::env::temp_dir();
  temp.push(format!("{}.png", name));
  std::fs::write(temp, png).into_result()?;
  Ok(())
}
