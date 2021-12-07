use std::fs::create_dir_all;
use std::{
  fs::File,
  path::{Path, PathBuf},
};

pub fn create_file(root_dir: &Path, relative_path: &str) -> Result<(File, PathBuf), std::io::Error> {
  let file_path = root_dir.join(relative_path);
  if let Some(dir) = file_path.parent() {
    create_dir_all(dir)?;
  }
  let f = File::create(&file_path)?;
  Ok((f, file_path))
}
