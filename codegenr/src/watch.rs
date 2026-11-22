//! `--watch`, for the template developer: renders a section again when one of its templates or custom helpers changes.
//! The sources are resolved once: a change of a source, of a file its `$ref` lead to, or of `codegenr.toml` needs a new run.

use crate::{Options, OptionsMap, render_section, resolve_section};
use notify::{Config, Event, EventKind, EventKindMask, RecommendedWatcher, RecursiveMode, Watcher, event::ModifyKind};
use serde_json::Value;
use std::{
  collections::BTreeMap,
  path::{Path, PathBuf},
  rc::Rc,
  sync::mpsc::{self, Receiver},
  time::{Duration, Instant},
};
use thiserror::Error;
use tracing::{error, info, warn};

/// The events closer than this make one change: an editor often writes a file in several steps.
const QUIET_TIME: Duration = Duration::from_millis(200);

/// Out of `CodegenrError`, which stays the same with or without the `bin` feature.
#[derive(Error, Debug)]
pub enum WatchError {
  #[error("Error while watching the templates: `{0}`.")]
  Notify(#[from] notify::Error),
  #[error("Nothing to watch: no section could be resolved, or no template folder nor custom helper could be found.")]
  NothingToWatch,
  #[error("The watch stopped: the file watcher is gone.")]
  Stopped,
}

struct WatchedSection {
  name: String,
  options: Options,
  json: Rc<Value>,
  /// Its template folders and custom helpers, canonical to compare with the paths of the events.
  roots: Vec<PathBuf>,
}

/// What changed since the last rendering.
#[derive(Debug, Default, PartialEq)]
struct Changes {
  paths: Vec<PathBuf>,
  /// Events were lost, a `git checkout` of many files can fill the queue of the system: every section renders again.
  lost: bool,
}

/// Runs every section once, then renders again the ones whose templates or custom helpers change, until Ctrl-C.
pub fn watch_codegenr(options_map: OptionsMap) -> Result<(), WatchError> {
  let sections = resolve_sections(options_map);
  let paths = paths_to_watch(&sections);
  if paths.is_empty() {
    return Err(WatchError::NothingToWatch);
  }

  // Watching before the first rendering, so that a template saved meanwhile is not missed;
  // without the access events, as rendering reads the templates and would trigger itself.
  let (sender, receiver) = mpsc::channel();
  let mut watcher = RecommendedWatcher::new(sender, Config::default().with_event_kinds(EventKindMask::CORE))?;
  for (path, mode) in paths {
    watcher.watch(&path, mode)?;
  }

  for section in &sections {
    info!("Rendering the `{}` section.", section.name);
    render(section);
  }
  info!(
    "Watching the templates and custom helpers of {} section(s), Ctrl-C to stop.",
    sections.len()
  );

  loop {
    let changes = next_changes(&receiver, |path| sections.iter().any(|s| concerns(&s.roots, path)))?;
    if changes.lost {
      warn!("Some file changes were lost: rendering every section again.");
    }
    for section in sections
      .iter()
      .filter(|s| changes.lost || changes.paths.iter().any(|path| concerns(&s.roots, path)))
    {
      info!("Rendering the `{}` section again.", section.name);
      render(section);
    }
  }
}

/// The documents read to resolve the sources are freed on return: the watch never reads them again.
fn resolve_sections(options_map: OptionsMap) -> Vec<WatchedSection> {
  let mut original_cache = Default::default();
  let mut sections = Vec::new();
  for (name, options) in options_map {
    info!("Resolving the source of the `{name}` section.");
    let json = match resolve_section(&options, &mut original_cache) {
      Ok(json) => json,
      Err(e) => {
        error!("Error while resolving the source of the `{name}` section, which is not watched: `{e}`.");
        continue;
      }
    };
    let mut roots = Vec::new();
    for path in options.templates.iter().chain(&options.custom_helpers) {
      match std::fs::canonicalize(path) {
        Ok(root) => roots.push(root),
        Err(e) => warn!("`{path}` of the `{name}` section cannot be found, it is not watched: `{e}`."),
      }
    }
    sections.push(WatchedSection {
      name,
      options,
      json,
      roots,
    });
  }
  sections
}

/// An error is logged, not returned: the next save may fix it.
fn render(section: &WatchedSection) {
  if let Err(e) = render_section(&section.options, &section.json) {
    error!("Error while rendering the `{}` section: `{e}`.", section.name);
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

/// Blocks until a first relevant change, then gathers the next ones until `QUIET_TIME` passes without any:
/// the events on other files neither start nor prolong that wait.
fn next_changes(receiver: &Receiver<notify::Result<Event>>, is_relevant: impl Fn(&Path) -> bool) -> Result<Changes, WatchError> {
  let mut changes = Changes::default();
  let mut deadline: Option<Instant> = None;
  loop {
    let event = match deadline {
      None => receiver.recv().map_err(|_| WatchError::Stopped)?,
      Some(deadline) => match receiver.recv_timeout(deadline.saturating_duration_since(Instant::now())) {
        Ok(event) => event,
        // The quiet time passed, or the watcher is gone: the changes gathered are rendered first.
        Err(_) => return Ok(changes),
      },
    };
    tracing::debug!("File event: {event:?}");
    let relevant = match event {
      Ok(event) if event.need_rescan() => {
        changes.lost = true;
        true
      }
      Ok(event) if is_change(&event.kind) => {
        let gathered = changes.paths.len();
        changes.paths.extend(event.paths.into_iter().filter(|path| is_relevant(path)));
        changes.paths.len() > gathered
      }
      Ok(_) => false,
      Err(e) => {
        warn!("Error while watching the templates: `{e}`.");
        false
      }
    };
    if relevant {
      deadline = Some(Instant::now() + QUIET_TIME);
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

/// A template, a custom helper, or a folder (created, moved or removed: its path has no extension), under one of the roots of a section;
/// not under a hidden file or folder of an editor or a tool, which the rendering skips too.
/// A generated file does not trigger a rendering, unless it is a template or a helper itself:
/// a section generating templates into a watched folder then renders forever, a limit we accept.
fn concerns(roots: &[PathBuf], changed: &Path) -> bool {
  let is_template_helper_or_folder = matches!(changed.extension().and_then(|e| e.to_str()), None | Some("hbs" | "rhai"));
  let is_hidden = |inside: &Path| {
    inside
      .components()
      .any(|c| c.as_os_str().to_str().is_some_and(|s| s.starts_with('.')))
  };
  is_template_helper_or_folder
    && roots
      .iter()
      .any(|root| changed.strip_prefix(root).is_ok_and(|inside| !is_hidden(inside)))
}

#[cfg(test)]
mod tests {
  use super::*;
  use notify::event::{CreateKind, Flag};
  use test_case::test_case;

  #[test_case(&["/p/templates"], "/p/templates/main.hbs", true ; "template")]
  #[test_case(&["/p/templates"], "/p/templates/sub/_partial.hbs", true ; "partial in a subfolder")]
  #[test_case(&["/p/templates"], "/p/templates/partials", true ; "folder moved in or out")]
  #[test_case(&["/p/helpers"], "/p/helpers/upper.rhai", true ; "helper")]
  #[test_case(&["/p/helpers/upper.rhai"], "/p/helpers/upper.rhai", true ; "helper given as a file")]
  #[test_case(&["/p/helpers/upper.rhai"], "/p/helpers/lower.rhai", false ; "other helper beside a helper given as a file")]
  #[test_case(&["/p/templates"], "/p/templates/api.generated.rs", false ; "generated file in a template folder")]
  #[test_case(&["/p/templates"], "/p/templates/.#main.hbs", false ; "lock file of an editor")]
  #[test_case(&["/p/templates"], "/p/templates/.history/main_20261122.hbs", false ; "file in a hidden folder")]
  #[test_case(&["/p/templates"], "/p/templates2/main.hbs", false ; "folder whose name starts the same")]
  #[test_case(&["/p/templates"], "/p/other/main.hbs", false ; "template of another section")]
  fn a_change_concerns_the_templates_and_helpers_of_a_section(roots: &[&str], changed: &str, expected: bool) {
    let roots: Vec<PathBuf> = roots.iter().map(PathBuf::from).collect();
    assert_eq!(concerns(&roots, Path::new(changed)), expected);
  }

  fn created(path: &str) -> notify::Result<Event> {
    Ok(Event::new(EventKind::Create(CreateKind::File)).add_path(PathBuf::from(path)))
  }

  #[test]
  fn only_the_relevant_files_make_a_change() -> anyhow::Result<()> {
    let (sender, receiver) = mpsc::channel();
    sender.send(created("/p/target/build.log"))?;
    sender.send(created("/p/templates/main.hbs"))?;
    sender.send(created("/p/target/other.log"))?;

    let changes = next_changes(&receiver, |path| path.starts_with("/p/templates"))?;

    let expected = Changes {
      paths: vec![PathBuf::from("/p/templates/main.hbs")],
      lost: false,
    };
    assert_eq!(changes, expected);
    Ok(())
  }

  /// Before, the quiet time started again on each event: a tool writing all the time in a watched folder delayed the rendering forever.
  #[test]
  fn other_files_do_not_delay_the_rendering() -> anyhow::Result<()> {
    let (sender, receiver) = mpsc::channel();
    sender.send(created("/p/templates/main.hbs"))?;
    // Another file written every 20 ms, for 3 s, unless the test is over before.
    std::thread::spawn(move || {
      for _ in 0..150 {
        if sender.send(created("/p/target/build.log")).is_err() {
          break;
        }
        std::thread::sleep(Duration::from_millis(20));
      }
    });

    let start = Instant::now();
    let changes = next_changes(&receiver, |path| path.starts_with("/p/templates"))?;

    assert_eq!(changes.paths, [PathBuf::from("/p/templates/main.hbs")]);
    assert!(start.elapsed() < Duration::from_millis(1500), "{:?}", start.elapsed());
    Ok(())
  }

  #[test]
  fn lost_events_render_every_section() -> anyhow::Result<()> {
    let (sender, receiver) = mpsc::channel();
    sender.send(Ok(Event::new(EventKind::Other).set_flag(Flag::Rescan)))?;

    let changes = next_changes(&receiver, |_| false)?;

    assert!(changes.lost);
    Ok(())
  }

  #[test]
  fn nothing_to_watch_is_an_error() {
    let missing_source = Options {
      source: "_samples/missing.yaml".into(),
      output: "out".into(),
      templates: vec!["_samples/render/templates".into()],
      intermediate: None,
      custom_helpers: vec![],
      global_parameters: Default::default(),
      allowed_sources: vec![],
    };
    for options_map in [OptionsMap::new(), OptionsMap::from([("missing".into(), missing_source)])] {
      let error = watch_codegenr(options_map).expect_err("Nothing can be watched.");
      assert!(matches!(error, WatchError::NothingToWatch), "{error}");
    }
  }
}
