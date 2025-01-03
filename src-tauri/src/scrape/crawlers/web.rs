use crate::error::{err, IntoResult, Result};

pub async fn get_html(url: &str) -> Result<String> {
  let mut builder = reqwest::Client::builder().cookie_store(true);

  // TODO: Proxy

  let client = builder.build().into_result()?;
  let mut req = client.get(url);

  if url.contains("getchu") {
    req = req.header("Referer", "http://www.getchu.com/top.html");
  } else if url.contains("xcity") {
    req = req.header(
      "Referer",
      "https://xcity.jp/result_published/?genre=%2Fresult_published%2F&q=2&sg=main&num=60",
    );
  } else if url.contains("javbus") {
    // javbus封面图需携带refer，refer似乎没有做强校验，但须符合格式要求，否则403
    req = req.header("Referer", "https://www.javbus.com/");
  } else if url.contains("giga") && !url.contains("cookie_set.php") {
    // 搜索时需要携带refer，获取cookies时不能携带
    req = req.header("Referer", "https://www.giga-web.jp/top.html");
  }

  let res = req.send().await.into_result()?;

  if res.status().is_success() {
    res.text().await.into_result()
  } else {
    err("Failed to get HTML")
  }
}
