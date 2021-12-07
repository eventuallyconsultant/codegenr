use std::fs::create_dir_all;
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
