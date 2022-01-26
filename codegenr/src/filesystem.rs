use std::fs::create_dir_all;
use std::io::Write;
use std::{
  fs::File,
  path::{Path, PathBuf},
};

pub fn make_path_from_root(root_dir: impl AsRef<Path>, relative_path: &str) -> PathBuf {
  root_dir.as_ref().join(relative_path)
}

pub fn create_file(root_dir: impl AsRef<Path>, relative_path: &str) -> Result<(File, PathBuf), std::io::Error> {
  let file_path = make_path_from_root(root_dir, relative_path);
  if let Some(dir) = file_path.parent() {
    create_dir_all(dir)?;
  }
  let f = File::create(&file_path)?;
  Ok((f, file_path))
}

#[allow(dead_code)]
pub fn create_dir(root_dir: impl AsRef<Path>, relative_path: &str) -> Result<PathBuf, std::io::Error> {
  let dir_path = make_path_from_root(root_dir, relative_path);
  create_dir_all(&dir_path)?;
  Ok(dir_path)
}

pub fn save_file_content(root_dir: impl AsRef<Path>, relative_path: &str, content: &str) -> Result<(), std::io::Error> {
  let path = make_path_from_root(root_dir, relative_path);
  let mut f = File::create(path)?;
  f.write_all(content.as_bytes())
}
