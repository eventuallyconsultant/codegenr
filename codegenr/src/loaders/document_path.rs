use path_dedot::ParseDot;
use serde_json::Value;
use std::path::Path;
use url::Url;

use super::{json_from_string, FormatHint, LoaderError};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum DocumentPath {
  /// Full url to a file : https://mywebsite/api.yaml
  Url(Url),
  /// File name or relative file name
  FileName(String),
  /// json or yaml out of thin silicon
  None,
}

impl DocumentPath {
  pub fn parse(ref_path: &str) -> Result<Self, LoaderError> {
    Ok(if ref_path.trim() == "" {
      Self::None
    } else {
      match Url::parse(ref_path) {
        Ok(url) => DocumentPath::Url(url),
        Err(_) => DocumentPath::FileName(ref_path.into()),
      }
    })
  }

  pub fn relate_from(self, refed_from: &Self) -> Result<Self, LoaderError> {
    use DocumentPath::*;
    Ok(match (refed_from, self) {
      (Url(_), Url(url)) => Url(url),
      (Url(url_from), FileName(path_to)) => {
        let mut url = url_from.clone();
        url
          .path_segments_mut()
          .map_err(|_| LoaderError::UrlCannotBeABase { url: url_from.clone() })?
          .pop();
        let path = url.path();
        let new_path = Path::new(path).join(&path_to);
        let new_path = new_path.parse_dot()?;
        let new_path = new_path.to_str().ok_or_else(|| LoaderError::UnableToAppendPath {
          to: path_to,
          from: url_from.to_string(),
        })?;
        url.set_path(new_path);
        Url(url)
      }
      (Url(_), None) => refed_from.clone(),
      (FileName(path_from), FileName(path_to)) => {
        let folder = Path::new(path_from)
          .parent()
          .ok_or_else(|| LoaderError::OriginPathShouldBeAFile { path: path_from.clone() })?;
        folder
          .join(&path_to)
          .parse_dot()?
          .to_str()
          .map(|s| FileName(s.to_owned()))
          .ok_or_else(|| LoaderError::UnableToAppendPath {
            to: path_to,
            from: path_from.clone(),
          })?
      }
      (FileName(_), Url(url)) => Url(url),
      (FileName(_path_from), None) => refed_from.clone(),
      (None, s) => s,
    })
  }

  pub(crate) fn guess_format(&self) -> FormatHint {
    let s = match self {
      DocumentPath::Url(url) => url.as_str(),
      DocumentPath::FileName(s) => s,
      DocumentPath::None => return FormatHint::NoIdea,
    };
    if s.ends_with(".json") {
      FormatHint::Json
    } else if s.ends_with(".yaml") || s.ends_with(".yml") {
      FormatHint::Yaml
    } else if s.ends_with(".toml") {
      FormatHint::Toml
    } else if s.ends_with(".graphql") || s.ends_with(".gql") {
      FormatHint::Graphql
    } else {
      FormatHint::NoIdea
    }
  }

  #[allow(clippy::result_large_err)]
  pub fn load_raw(&self) -> Result<Value, LoaderError> {
    let hint = self.guess_format();
    match self {
      DocumentPath::Url(url) => {
        let body = reqwest::blocking::get(url.clone())
          .map_err(|e| LoaderError::DownloadError(url.as_str().to_string(), e))?
          .text()
          .map_err(|e| LoaderError::DownloadError(url.as_str().to_string(), e))?;
        json_from_string(&body, hint)
      }
      DocumentPath::FileName(file_name) => {
        let content = std::fs::read_to_string(file_name).map_err(|e| LoaderError::Read(file_name.clone(), e))?;
        json_from_string(&content, hint)
      }
      DocumentPath::None => unreachable!("This is a non sense to try loading a 'None' document path."),
    }
  }
}
