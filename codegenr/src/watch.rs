//! `--watch`, for the template developer: renders a section again when one of its templates or custom helpers changes.
//! The sources are resolved once: a change of a source, of a file its `$ref` lead to, or of `codegenr.toml` needs a new run.

use crate::{Options, OptionsMap, errors::CodegenrError, render_section, resolve_section};
use notify::{Config, EventKind, EventKindMask, RecommendedWatcher, RecursiveMode, Watcher, event::ModifyKind};
use serde_json::Value;
use std::{
  collections::BTreeMap,
  path::{Path, PathBuf},
  rc::Rc,
  sync::mpsc::{self, Receiver, RecvTimeoutError},
  time::Duration,
};
use tracing::{error, info, warn};

/// The events closer than this make one change: an editor often writes a file in several steps.
const QUIET_TIME: Duration = Duration::from_millis(200);

struct WatchedSection {
  name: String,
  options: Options,
  json: Rc<Value>,
  /// Its template folders and custom helpers, canonical to compare with the paths of the events.
  roots: Vec<PathBuf>,
}

/// Runs every section once, then renders again the ones whose templates or custom helpers change, until Ctrl-C.
#[allow(clippy::result_large_err)]
pub fn watch_codegenr(options_map: OptionsMap) -> Result<(), CodegenrError> {
  let mut original_cache = Default::default();
  let mut sections = Vec::new();
  for (name, options) in options_map {
    info!("Running code generation section `{name}`");
    match resolve_section(&options, &mut original_cache) {
      Ok(json) => {
        render(&name, &options, &json);
        let roots = options
          .templates
          .iter()
          .chain(&options.custom_helpers)
          .filter_map(|path| std::fs::canonicalize(path).ok())
          .collect();
        sections.push(WatchedSection {
          name,
          options,
          json,
          roots,
        });
      }
      Err(e) => error!("Error while resolving the source of the `{name}` section, which is not watched: `{e}`."),
    }
  }

  let (sender, receiver) = mpsc::channel();
  // Without the access events: rendering reads the templates, it would trigger itself.
  let mut watcher = RecommendedWatcher::new(sender, Config::default().with_event_kinds(EventKindMask::CORE))?;
  for (path, mode) in paths_to_watch(&sections) {
    watcher.watch(&path, mode)?;
  }
  info!(
    "Watching the templates and custom helpers of {} section(s), Ctrl-C to stop.",
    sections.len()
  );

  loop {
    let changed = next_changes(&receiver)?;
    for section in sections.iter().filter(|s| changed.iter().any(|path| concerns(&s.roots, path))) {
      info!("Rendering the `{}` section again.", section.name);
      render(&section.name, &section.options, &section.json);
    }
  }
}

/// An error is logged, not returned: the next save may fix it.
fn render(name: &str, options: &Options, json: &Value) {
  if let Err(e) = render_section(options, json) {
    error!("Error while rendering the `{name}` section: `{e}`.");
  }
}

/// A folder is watched with its subfolders; for a single helper file, its folder alone,
/// as many editors save a file by replacing it, which a watch on the file itself would lose.
fn paths_to_watch(sections: &[WatchedSection]) -> BTreeMap<PathBuf, RecursiveMode> {
  let mut paths = BTreeMap::new();
  for root in sections.iter().flat_map(|s| &s.roots) {
    let (path, mode) = match root.is_dir() {
      true => (root.as_path(), RecursiveMode::Recursive),
      false => (root.parent().unwrap_or(root), RecursiveMode::NonRecursive),
    };
    let watched = paths.entry(path.to_path_buf()).or_insert(mode);
    if mode == RecursiveMode::Recursive {
      *watched = mode;
    }
  }
  paths
}

/// Blocks until a first change, then gathers the next ones until `QUIET_TIME` passes without any.
#[allow(clippy::result_large_err)]
fn next_changes(receiver: &Receiver<notify::Result<notify::Event>>) -> Result<Vec<PathBuf>, CodegenrError> {
  let mut changed = Vec::new();
  loop {
    let event = match changed.is_empty() {
      true => receiver.recv().map_err(|_| CodegenrError::WatchStopped)?,
      false => match receiver.recv_timeout(QUIET_TIME) {
        Ok(event) => event,
        Err(RecvTimeoutError::Timeout) => return Ok(changed),
        Err(RecvTimeoutError::Disconnected) => return Err(CodegenrError::WatchStopped),
      },
    };
    match event {
      Ok(event) if is_change(&event.kind) => changed.extend(event.paths),
      Ok(_) => {}
      Err(e) => warn!("Error while watching the templates: `{e}`."),
    }
  }
}

/// A content, a creation, a removal or a renaming, not a metadata change.
fn is_change(kind: &EventKind) -> bool {
  match kind {
    EventKind::Create(_) | EventKind::Remove(_) => true,
    EventKind::Modify(modify) => !matches!(modify, ModifyKind::Metadata(_)),
    _ => false,
  }
}

/// A template or a custom helper under one of the roots of a section; never a generated file,
/// so writing the output in a watched folder cannot trigger a new rendering.
fn concerns(roots: &[PathBuf], changed: &Path) -> bool {
  let is_template_or_helper = matches!(changed.extension().and_then(|e| e.to_str()), Some("hbs" | "rhai"));
  is_template_or_helper && roots.iter().any(|root| changed.starts_with(root))
}

#[cfg(test)]
mod tests {
  use super::*;
  use test_case::test_case;

  #[test_case(&["/p/templates"], "/p/templates/main.hbs", true ; "template")]
  #[test_case(&["/p/templates"], "/p/templates/sub/_partial.hbs", true ; "partial in a subfolder")]
  #[test_case(&["/p/helpers"], "/p/helpers/upper.rhai", true ; "helper")]
  #[test_case(&["/p/helpers/upper.rhai"], "/p/helpers/upper.rhai", true ; "helper given as a file")]
  #[test_case(&["/p/helpers/upper.rhai"], "/p/helpers/lower.rhai", false ; "other helper beside a helper given as a file")]
  #[test_case(&["/p/templates"], "/p/templates/api.generated.rs", false ; "generated file in a template folder")]
  #[test_case(&["/p/templates"], "/p/templates2/main.hbs", false ; "folder whose name starts the same")]
  #[test_case(&["/p/templates"], "/p/other/main.hbs", false ; "template of another section")]
  fn a_change_concerns_the_templates_and_helpers_of_a_section(roots: &[&str], changed: &str, expected: bool) {
    let roots: Vec<PathBuf> = roots.iter().map(PathBuf::from).collect();
    assert_eq!(concerns(&roots, Path::new(changed)), expected);
  }
}
