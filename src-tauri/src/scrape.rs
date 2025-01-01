use lazy_static::lazy_static;
use log::debug;
use regex::Regex;
use tauri::AppHandle;

use crate::error::Result;

lazy_static! {
  /// xxx-CD1
  static ref XXX_CD1: Regex = Regex::new(r"[-_ .]CD\d{1,2}").unwrap();
  /// xxx_1, xxx.1, xxx.A, xxx A
  static ref XXX_1: Regex = Regex::new(r"[-_ .][A-Z0-9]\.$$").unwrap();
  static ref YMD1: Regex= Regex::new(r"\d{4}[-_.]\d{1,2}[-_.]\d{1,2}").unwrap();
  static ref YMD2: Regex= Regex::new(r"[-\[]\d{2}[-_.]\d{2}[-_.]\d{2}]?").unwrap();
  static ref MYWIFE_RE: Regex = Regex::new(r"NO\.\d*").unwrap();
  static ref CW3D2D_RE: Regex = Regex::new(r"CW3D2D?BD-?\d{2,}").unwrap();
  static ref MMR_RE: Regex = Regex::new(r"MMR-?[A-Z]{2,}-?\d+[A-Z]*").unwrap();
  static ref MD_RE: Regex = Regex::new(r"([^A-Z]|^)(MD[A-Z-]*\d{4,}(-\d)?)").unwrap();
  static ref OUMEI_RE: Regex = Regex::new(r"([A-Z0-9_]{2,})[-.]2?0?(\d{2}[-.]\d{2}[-.]\d{2})").unwrap();
  static ref XXX_AV_RE: Regex = Regex::new(r"XXX-AV-\d{4,}").unwrap();
  static ref MKY_RE: Regex = Regex::new(r"MKY-[A-Z]+-\d{3,}").unwrap();
  static ref FC2_RE: Regex = Regex::new(r"FC2-\d{5,}").unwrap();
  static ref HEYZO_RE: Regex = Regex::new(r"HEYZO-\d{3,}").unwrap();
  static ref H4610_RE: Regex = Regex::new(r"(H4610|C0930|H0930)-[A-Z]+\d{4,}").unwrap();
  static ref KIN8_RE: Regex = Regex::new(r"KIN8(TENGOKU)?-?\d{3,}").unwrap();
  static ref S2M_RE: Regex = Regex::new(r"S2M[BD]*-\d{3,}").unwrap();
  static ref MCB3D_RE: Regex = Regex::new(r"MCB3D[BD]*-\d{2,}").unwrap();
  static ref T28_RE: Regex = Regex::new(r"T28-?\d{3,}").unwrap();
  static ref TH101_RE: Regex = Regex::new(r"TH101-\d{3,}-\d{5,}").unwrap();
  static ref AZ_RE: Regex = Regex::new(r"([A-Z]{2,})00(\d{3})").unwrap();
  static ref NUM_AZ_RE: Regex = Regex::new(r"\d{2,}[A-Z]{2,}-\d{2,}[A-Z]?").unwrap();
  static ref AZ_NUM_RE: Regex = Regex::new(r"[A-Z]{2,}-\d{2,}").unwrap();
  static ref AZ_AZ_NUM_RE: Regex = Regex::new(r"[A-Z]+-[A-Z]\d+").unwrap();
  static ref NUM_NUM_RE: Regex = Regex::new(r"\d{2,}[-_]\d{2,}").unwrap();
  static ref NUM_AZ_RE2: Regex = Regex::new(r"\d{3,}-[A-Z]{3,}").unwrap();
  static ref N_RE: Regex = Regex::new(r"([^A-Z]|^)(N\d{4})(\D|$)").unwrap();
  static ref H_RE: Regex = Regex::new(r"H_\d{3,}([A-Z]{2,})(\d{2,})").unwrap();
  static ref AZ3_NUM2_RE: Regex = Regex::new(r"([A-Z]{3,}).*?(\d{2,})").unwrap();
  static ref AZ2_NUM3_RE: Regex = Regex::new(r"([A-Z]{2,}).*?(\d{3,})").unwrap();
}

/// 刮削
#[tauri::command]
#[specta::specta]
pub async fn scrape(_app: AppHandle, filename: String) -> Result<()> {
  let movie_code = get_movie_code(&filename);
  debug!("code: {:?}", movie_code);
  Ok(())
}

/// 获取番号
fn get_movie_code(name: &String) -> Option<String> {
  // 去除多余字符
  static USELESS_WORDS: &[&str] = &[
    "h_720",
    "2048论坛@fun2048.com",
    "1080p",
    "720p",
    "22-sht.me",
    "-HD",
    "bbs2048.org@",
    "hhd800.com@",
    "kfa55.com@",
    "icao.me@",
    "hhb_000",
    "[456k.me]",
    "[ThZu.Cc]",
  ];

  let mut name = name.to_uppercase();

  for word in USELESS_WORDS {
    name = name.replace(word, "");
  }

  // 替换cd_part、EP、-C
  name = name
    .replace("-C", ".")
    .replace(".PART", "-CD")
    .replace("-PART", "-CD")
    .replace(" EP.", ".EP")
    .replace("-CD-", "");

  // 去除分集
  name = XXX_CD1.replace_all(&name, "").to_string();
  name = XXX_1.replace_all(&name, "").to_string();
  name = name
    .replace(" ", "-")
    .trim_matches(&['-', '_', '.'])
    .to_string();

  // 去除时间
  name = YMD1.replace_all(&name, "").to_string();
  name = YMD2.replace_all(&name, "").to_string();

  // 转换番号
  name = name
    .replace("FC2-PPV", "FC2-")
    .replace("FC2PPV", "FC2-")
    .replace("GACHIPPV", "GACHI")
    .replace("--", "-");

  // 提取番号
  if let Some(mut code) = extract_movie_code(&name) {
    if code.starts_with("FC-") {
      code = code.replace("FC-", "FC2-");
    }

    code = code.trim_matches(&['-', '_', '.']).to_string();

    Some(code)
  } else {
    None
  }
}

fn extract_movie_code(filename: &str) -> Option<String> {
  if filename.contains("MYWIFE") && MYWIFE_RE.is_match(filename) {
    let temp_num = MYWIFE_RE.captures(filename)?.get(0)?.as_str();
    return Some(format!("Mywife No.{}", temp_num));
  } else if let Some(file_number) = CW3D2D_RE.find(filename) {
    return Some(file_number.as_str().to_string());
  } else if let Some(file_number) = MMR_RE.find(filename) {
    return Some(file_number.as_str().replace("MMR-", "MMR"));
  } else if let Some(caps) = MD_RE.captures(filename) {
    if !filename.contains("MDVR") {
      return Some(caps.get(2)?.as_str().to_string());
    }
  } else if let Some(result) = OUMEI_RE.captures(filename) {
    return Some(format!(
      "{}.{}",
      result.get(1)?.as_str(),
      result.get(2)?.as_str().replace("-", ".")
    ));
  } else if let Some(file_number) = XXX_AV_RE.find(filename) {
    return Some(file_number.as_str().to_string());
  } else if let Some(file_number) = MKY_RE.find(filename) {
    return Some(file_number.as_str().to_string());
  } else if filename.contains("FC2") {
    let filename = filename
      .replace("PPV", "")
      .replace('_', "-")
      .replace("--", "-");
    if let Some(file_number) = FC2_RE.find(&filename) {
      return Some(file_number.as_str().to_string());
    } else if let Some(file_number) = FC2_RE.find(&filename) {
      return Some(file_number.as_str().replace("FC2", "FC2-"));
    } else {
      return Some(filename);
    }
  } else if filename.contains("HEYZO") {
    let filename = filename.replace('_', "-").replace("--", "-");
    if let Some(file_number) = HEYZO_RE.find(&filename) {
      return Some(file_number.as_str().to_string());
    } else if let Some(file_number) = HEYZO_RE.find(&filename) {
      return Some(file_number.as_str().replace("HEYZO", "HEYZO-"));
    } else {
      return Some(filename);
    }
  } else if let Some(file_number) = H4610_RE.find(filename) {
    return Some(file_number.as_str().to_string());
  } else if let Some(file_number) = KIN8_RE.find(filename) {
    return Some(
      file_number
        .as_str()
        .replace("TENGOKU", "-")
        .replace("--", "-"),
    );
  } else if let Some(file_number) = S2M_RE.find(filename) {
    return Some(file_number.as_str().to_string());
  } else if let Some(file_number) = MCB3D_RE.find(filename) {
    return Some(file_number.as_str().to_string());
  } else if let Some(file_number) = T28_RE.find(filename) {
    return Some(file_number.as_str().replace("T2800", "T28-"));
  } else if let Some(file_number) = TH101_RE.find(filename) {
    return Some(file_number.as_str().to_lowercase());
  } else if let Some(caps) = AZ_RE.captures(filename) {
    return Some(format!(
      "{}-{}",
      caps.get(1)?.as_str(),
      caps.get(2)?.as_str()
    ));
  } else if let Some(file_number) = NUM_AZ_RE.find(filename) {
    return Some(file_number.as_str().to_string());
  } else if let Some(file_number) = AZ_NUM_RE.find(filename) {
    return Some(file_number.as_str().to_string());
  } else if let Some(file_number) = AZ_AZ_NUM_RE.find(filename) {
    return Some(file_number.as_str().to_string());
  } else if let Some(file_number) = NUM_NUM_RE.find(filename) {
    return Some(file_number.as_str().to_string());
  } else if let Some(file_number) = NUM_AZ_RE2.find(filename) {
    return Some(file_number.as_str().to_string());
  } else if let Some(caps) = N_RE.captures(filename) {
    return Some(caps.get(2)?.as_str().to_lowercase());
  } else if let Some(caps) = H_RE.captures(filename) {
    return Some(format!(
      "{}-{}",
      caps.get(1)?.as_str(),
      caps.get(2)?.as_str()
    ));
  } else if let Some(temp) = AZ3_NUM2_RE.captures(filename) {
    return Some(format!(
      "{}-{}",
      temp.get(1)?.as_str(),
      temp.get(2)?.as_str()
    ));
  } else if let Some(temp) = AZ2_NUM3_RE.captures(filename) {
    return Some(format!(
      "{}-{}",
      temp.get(1)?.as_str(),
      temp.get(2)?.as_str()
    ));
  } else {
    let temp_name = filename
      .replace("[", "")
      .replace("]", "")
      .replace("(", "")
      .replace(")", "")
      .replace("【", "")
      .replace("】", "")
      .replace("（", "")
      .replace("）", "")
      .trim()
      .to_string();
    return Some(temp_name);
  }

  None
}
