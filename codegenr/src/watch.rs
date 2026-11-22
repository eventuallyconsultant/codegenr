//! `--watch`, for the template developer: a section is rendered again when a file of its template or custom helper folders changes,
//! and resolved again first when its source, or a file one of its `$ref` leads to, changes; a source or a `$ref` given as an url is not watched.
//! A change of `codegenr.toml` starts everything again, as a new run.

use crate::{Options, OptionsMap, OriginalDocumentsHash, loaders::DocumentPath, opt::Command, render_section, resolve_section};
use notify::{Config, Event, EventKind, EventKindMask, RecommendedWatcher, RecursiveMode, Watcher, event::ModifyKind};
use serde_json::Value;
use std::{
  collections::{BTreeMap, btree_map::Entry},
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
  #[error("Error while watching the files: `{0}`.")]
  Notify(#[from] notify::Error),
  #[error("{0}")]
  Configuration(anyhow::Error),
  #[error("Nothing to watch: no template folder, custom helper nor source file could be found.")]
  NothingToWatch,
  #[error("The watch stopped: the file watcher is gone.")]
  Stopped,
}

/// Runs every section once, then watches them until Ctrl-C; a change of `codegenr.toml` starts again from scratch.
pub fn watch_codegenr(command: Command) -> Result<(), WatchError> {
  let configuration = match &command {
    Command::FromFile { file } => Some(canonical(Path::new(file))),
    Command::FromLine { .. } => None,
  };
  let mut first_run = true;
  loop {
    match OptionsMap::try_from(command.clone()) {
      Ok(options_map) => watch_sections(options_map, configuration.as_deref())?,
      Err(e) if first_run => return Err(WatchError::Configuration(e)),
      // Waits for `codegenr.toml` to be fixed.
      Err(e) => {
        error!("{e}");
        watch_sections(OptionsMap::new(), configuration.as_deref())?;
      }
    }
    first_run = false;
  }
}

struct WatchedSection {
  name: String,
  options: Options,
  /// `None` while the source cannot be resolved.
  json: Option<Rc<Value>>,
  /// Its template folders and custom helpers, canonical to compare with the paths of the events.
  roots: Vec<PathBuf>,
  /// The files its source and `$ref` lead to, canonical too.
  sources: Vec<PathBuf>,
}

impl WatchedSection {
  fn new(name: String, options: Options, original_cache: &mut OriginalDocumentsHash) -> Self {
    let mut roots = Vec::new();
    for path in options.templates.iter().chain(&options.custom_helpers) {
      match std::fs::canonicalize(path) {
        Ok(root) => roots.push(root),
        Err(e) => warn!("`{path}` of the `{name}` section cannot be found, it is not watched: `{e}`."),
      }
    }
    let mut section = Self {
      name,
      options,
      json: None,
      roots,
      sources: Vec::new(),
    };
    section.resolve(original_cache);
    section
  }

  /// Resolves the source again, and lists the files it leads to: a fixed source renders again, a new `$ref` gets watched.
  fn resolve(&mut self, original_cache: &mut OriginalDocumentsHash) {
    info!("Resolving the source of the `{}` section.", self.name);
    let mut read = Vec::new();
    self.json = resolve_section(&self.options, original_cache, &mut read)
      .inspect_err(|e| error!("Error while resolving the source of the `{}` section: `{e}`.", self.name))
      .ok();
    self.sources = read
      .iter()
      .filter_map(|document| match document {
        DocumentPath::FileName(path) => Some(canonical(Path::new(path))),
        DocumentPath::Url(url) => {
          info!("`{url}`, read by the `{}` section, is an url: it is not watched.", self.name);
          None
        }
        DocumentPath::None => None,
      })
      .collect();
  }

  /// An error is logged, not returned: the next save may fix it.
  fn render(&self) {
    let Some(json) = &self.json else {
      return;
    };
    info!("Rendering the `{}` section.", self.name);
    if let Err(e) = render_section(&self.options, json) {
      error!("Error while rendering the `{}` section: `{e}`.", self.name);
    }
  }
}

/// What a batch of changes asks for.
#[derive(Debug, PartialEq)]
enum Plan {
  /// `codegenr.toml` changed, or events were lost: everything again, as a new run.
  Restart,
  /// By index, the sections to resolve again then render, and the ones to render only.
  Sections { resolve: Vec<usize>, render: Vec<usize> },
}

/// One run of a configuration, until `codegenr.toml` changes or events are lost.
fn watch_sections(options_map: OptionsMap, configuration: Option<&Path>) -> Result<(), WatchError> {
  // The documents read by several sections are read once; freed once all are resolved.
  let mut original_cache = Default::default();
  let mut sections: Vec<WatchedSection> = options_map
    .into_iter()
    .map(|(name, options)| WatchedSection::new(name, options, &mut original_cache))
    .collect();
  drop(original_cache);
  let mut watched = paths_to_watch(&sections, configuration);
  if watched.is_empty() {
    return Err(WatchError::NothingToWatch);
  }

  // Watching before the first rendering, so that a file saved meanwhile is not missed;
  // without the access events, as rendering reads the templates and would trigger itself.
  let (sender, receiver) = mpsc::channel();
  let mut watcher = RecommendedWatcher::new(sender, Config::default().with_event_kinds(EventKindMask::CORE))?;
  for (path, mode) in &watched {
    watch(&mut watcher, path, *mode);
  }
  sections.iter().for_each(WatchedSection::render);
  info!("Watching {} section(s), Ctrl-C to stop.", sections.len());

  loop {
    let changes = next_changes(&receiver, |path| is_watched(path, configuration, &sections))?;
    match plan(&changes, configuration, &sections) {
      Plan::Restart => {
        match changes.lost {
          true => warn!("Some file changes were lost: starting again from scratch."),
          false => info!("`codegenr.toml` changed: starting again from scratch."),
        }
        return Ok(());
      }
      Plan::Sections { resolve, render } => {
        for &i in &resolve {
          sections[i].resolve(&mut Default::default());
          sections[i].render();
        }
        for &i in &render {
          sections[i].render();
        }
        // A new `$ref` gets its folder watched; a folder stays watched once it was.
        for (path, mode) in paths_to_watch(&sections, configuration) {
          if let Entry::Vacant(entry) = watched.entry(path) {
            watch(&mut watcher, entry.key(), mode);
            entry.insert(mode);
          }
        }
      }
    }
  }
}

/// A folder that cannot be watched, the one of a source not created yet for example, does not stop the others.
fn watch(watcher: &mut RecommendedWatcher, path: &Path, mode: RecursiveMode) {
  if let Err(e) = watcher.watch(path, mode) {
    warn!("`{}` cannot be watched: `{e}`.", path.display());
  }
}

fn plan(changes: &Changes, configuration: Option<&Path>, sections: &[WatchedSection]) -> Plan {
  if changes.lost || configuration.is_some_and(|configuration| changes.paths.iter().any(|path| path == configuration)) {
    return Plan::Restart;
  }
  let (mut resolve, mut render) = (Vec::new(), Vec::new());
  for (i, section) in sections.iter().enumerate() {
    if changes.paths.iter().any(|path| section.sources.contains(path)) {
      resolve.push(i);
    } else if changes.paths.iter().any(|path| concerns(&section.roots, path)) {
      render.push(i);
    }
  }
  Plan::Sections { resolve, render }
}

fn is_watched(path: &Path, configuration: Option<&Path>, sections: &[WatchedSection]) -> bool {
  configuration == Some(path)
    || sections
      .iter()
      .any(|section| section.sources.iter().any(|source| source == path) || concerns(&section.roots, path))
}

/// The template folders with their subfolders; a file through its folder alone, a custom helper, a source or `codegenr.toml`,
/// as many editors save a file by replacing it, which a watch on the file itself would lose.
fn paths_to_watch(sections: &[WatchedSection], configuration: Option<&Path>) -> BTreeMap<PathBuf, RecursiveMode> {
  let mut paths = BTreeMap::new();
  let files = sections
    .iter()
    .flat_map(|section| section.roots.iter().filter(|root| !root.is_dir()).chain(&section.sources))
    .map(PathBuf::as_path)
    .chain(configuration);
  for file in files {
    paths
      .entry(file.parent().unwrap_or(file).to_path_buf())
      .or_insert(RecursiveMode::NonRecursive);
  }
  for folder in sections.iter().flat_map(|section| &section.roots).filter(|root| root.is_dir()) {
    paths.insert(folder.clone(), RecursiveMode::Recursive);
  }
  paths
}

/// The canonical path, to compare with the paths of the events; for a missing file, the one of its folder, then its name.
fn canonical(path: &Path) -> PathBuf {
  std::fs::canonicalize(path).unwrap_or_else(|_| {
    let folder = match path.parent() {
      Some(parent) if !parent.as_os_str().is_empty() => parent,
      _ => Path::new("."),
    };
    match (std::fs::canonicalize(folder), path.file_name()) {
      (Ok(folder), Some(name)) => folder.join(name),
      _ => path.to_path_buf(),
    }
  })
}

/// What changed since the last rendering.
#[derive(Debug, Default, PartialEq)]
struct Changes {
  paths: Vec<PathBuf>,
  /// Events were lost, a `git checkout` of many files can fill the queue of the system.
  lost: bool,
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
        // The quiet time passed, or the watcher is gone: the changes gathered are handled first.
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
        warn!("Error while watching the files: `{e}`.");
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

  fn section(roots: &[&str], sources: &[&str]) -> WatchedSection {
    WatchedSection {
      name: "api".into(),
      options: Options {
        source: "api.yaml".into(),
        output: "out".into(),
        templates: vec![],
        intermediate: None,
        custom_helpers: vec![],
        global_parameters: Default::default(),
        allowed_sources: vec![],
      },
      json: None,
      roots: roots.iter().map(PathBuf::from).collect(),
      sources: sources.iter().map(PathBuf::from).collect(),
    }
  }

  #[test_case(&["/p/codegenr.toml"], false, Plan::Restart ; "codegenr toml changed")]
  #[test_case(&[], true, Plan::Restart ; "events lost")]
  #[test_case(&["/p/specs/common.yaml"], false, Plan::Sections { resolve: vec![0], render: vec![] } ; "file of a ref changed")]
  #[test_case(&["/p/specs/common.yaml", "/p/templates/main.hbs"], false, Plan::Sections { resolve: vec![0], render: vec![] } ; "source and template changed")]
  #[test_case(&["/p/templates/main.hbs"], false, Plan::Sections { resolve: vec![], render: vec![0] } ; "template changed")]
  #[test_case(&["/p/helpers/upper.rhai"], false, Plan::Sections { resolve: vec![], render: vec![1] } ; "helper of another section changed")]
  fn changes_plan_what_to_do_again(paths: &[&str], lost: bool, expected: Plan) {
    let sections = [
      section(&["/p/templates"], &["/p/specs/api.yaml", "/p/specs/common.yaml"]),
      section(&["/p/helpers"], &["/p/specs/other.yaml"]),
    ];
    let changes = Changes {
      paths: paths.iter().map(PathBuf::from).collect(),
      lost,
    };
    assert_eq!(plan(&changes, Some(Path::new("/p/codegenr.toml")), &sections), expected);
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
  fn lost_events_are_noticed() -> anyhow::Result<()> {
    let (sender, receiver) = mpsc::channel();
    sender.send(Ok(Event::new(EventKind::Other).set_flag(Flag::Rescan)))?;

    let changes = next_changes(&receiver, |_| false)?;

    assert!(changes.lost);
    Ok(())
  }

  /// A source that cannot be read is still watched, so `gen` with a missing source waits for it to exist.
  #[test]
  fn nothing_to_watch_is_an_error() {
    let error = watch_sections(OptionsMap::new(), None).expect_err("Nothing can be watched.");
    assert!(matches!(error, WatchError::NothingToWatch), "{error}");
  }

  #[test]
  fn a_missing_file_is_compared_through_its_folder() -> anyhow::Result<()> {
    let tmp = tempfile::TempDir::with_prefix("WATCH_tests")?;
    let missing = tmp.path().join("missing.yaml");
    assert_eq!(canonical(&missing), std::fs::canonicalize(tmp.path())?.join("missing.yaml"));
    Ok(())
  }
}
