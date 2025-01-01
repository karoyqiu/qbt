#[derive(Debug)]
pub struct Error(pub anyhow::Error);

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:#}", self.0)
  }
}

impl std::error::Error for Error {}

impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(&format!("{:#}", self.0))
  }
}

impl specta::Type for Error {
  fn inline(_: &mut specta::TypeMap, _: specta::Generics) -> specta::datatype::DataType {
    specta::datatype::DataType::Any
  }
}

impl From<anyhow::Error> for Error {
  fn from(value: anyhow::Error) -> Self {
    Self(value)
  }
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait IntoResult<T> {
  fn into_result(self) -> Result<T>;
}

impl<T, E> IntoResult<T> for std::result::Result<T, E>
where
  E: Into<anyhow::Error>,
{
  fn into_result(self) -> Result<T> {
    self.map_err(|e| Error(e.into()))
  }
}

impl<T> IntoResult<T> for anyhow::Error {
  fn into_result(self) -> Result<T> {
    Err(Error(self))
  }
}

pub fn err<T>(msg: &'static str) -> Result<T> {
  Err(Error(anyhow::anyhow!(msg)))
}
