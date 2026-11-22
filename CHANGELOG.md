# Changelog

## 0.0.8 (unreleased)

The first release on crates.io since 0.0.2 (January 2022): it gathers the work done on `dev` since then.
Read the breaking changes before upgrading: each one says what to change in your templates (*gabarits*) or in your `codegenr.toml`.

### Breaking changes

- **A partial template (*gabarit partiel*) no longer reads the context of its caller through `../`**, as in handlebars.js: handlebars goes from 5.1.2 to 6.4.4.
  The caller gives the partial what it needs as a hash parameter:

  ```handlebars
  {{!-- before: `_property.hbs` reads `../required` --}}
  {{#each properties}}{{> property}}{{/each}}

  {{!-- after: `_property.hbs` reads `parent_required` --}}
  {{#each properties}}{{> property parent_required=../required}}{{/each}}
  ```

  `@key`, `@index`, `@first` and `@last` still reach a partial called from an `#each`.
- **Downloading needs your approval.** A `source`, or a `$ref`, given as an url must be under an url of the new `allowed_sources` option of the section (`--allowed-sources` for `gen`).
  A `$ref` to a file must stay under the current folder, the folder of `source`, or a folder of `allowed_sources`.
  When a document is refused, the error gives the line to add, e.g. `allowed_sources = ['https://petstore3.swagger.io/api/v3/']`.
- **The paths of `### FILE` and `### CLEAN` must stay in the `output` folder**: an absolute path, or one holding `..`, is an error.
  A template that wrote into a sibling folder takes their parent folder as `output`.
- **Removed helpers.**
  - `http_get`, `gitignore_io` and `jsonnet`: a template can no longer reach the network, nor run another template language.
  - Since 0.0.2, the case and trim helpers of codegenr gave way to those of `handlebars_misc_helpers`, and some block helpers to plain ones:

    | 0.0.2 | 0.0.8 |
    | --- | --- |
    | `pascal_case`, `snake_case`, `camel_case`, `upper_case`, `lower_case` | `to_pascal_case`, `to_snake_case`, `to_camel_case`, `to_upper_case`, `to_lower_case` |
    | `trim`, `trim_start`, `trim_end` with a character to remove | `trim_char`, `trim_char_start`, `trim_char_end` (`trim` and its variants now remove the whitespace) |
    | `split_get_first`, `split_get_last` | `split`, then `@first` or `@last` in an `#each` |
    | `{{#if_equals a b}}`, `{{#if_not_equals a b}}` | `{{#if (eq a b)}}`, `{{#if (ne a b)}}` |
    | `{{#if_empty x}}`, `{{#if_not_empty x}}` | `{{#if (is_empty x)}}`, `{{#unless (is_empty x)}}` |
    | `{{#if_array_contains array value}}` | `{{#if (in value array)}}` |
    | `json` | `json_to_str`, `str_to_json`, `json_query` |
- **YAML follows YAML 1.2** (`serde_yaml` 0.9): `True` and `False` are booleans, they were strings.
- **Hidden files are skipped**: a `.#main.hbs` lock of Emacs, or a `.history` folder, is no longer taken for a template or a custom helper.
- **Rust 1.91 or later** is needed to build codegenr.
- **For the library users**: `resolver::resolve_refs` takes an `AllowedSources`, built from the source and the `allowed_sources` of the section.

### Added

- **`--watch`** (`-w`), for the template developer: codegenr keeps running.
  - A section is rendered again when a file of its template or custom helper folders changes.
  - It is resolved again first when its source changes, or a file one of its `$ref` leads to; a source or a `$ref` given as an url is not watched.
  - A change of `codegenr.toml` starts everything again; a broken one is waited for.
- **`-v`, `-vv` and `-q`** set the logs: debug, trace, or only the warnings and the errors.
- **Helpers**: `to_ascii` (`Échec` gives `Echec`, to name an identifier after a French text), `prefix_lines` (a text on several lines in a code comment), `split`, `in`, `is_empty`, `contains`, `end_with`.
- **Loaders**: graphql SDL, toml, xml and csv sources, reshaped into json.
- **One line per section** tells how many files were written and how many were unchanged.

### Changed

- A generated file whose content is the same is not written again: it keeps its date, and the build tools do not redo their work.
- When a section fails, the others still run, and `codegenr` now exits with an error: it used to exit with a success.
- `### FILE` writes its file on its closing line: a write error fails the generation, and a `### FILE` never closed is an error, its file not written.
- `custom_helpers` and `global_parameters` are optional in `codegenr.toml`.
- Clearer errors: an unreadable source names the document and gives the error of each format tried; the line numbers of the instruction errors start at 1; an empty `source` is an error, no longer a crash.
- `handlebars` is pinned to exactly 6.4.4, as several 6.x releases changed what a partial sees of its caller.

### Fixed

- A source starting with a UTF-8 BOM is read: json refused it, and yaml silently kept it in the first key.
- An absolute Windows path, like `C:\specs\api.yaml`, as a `source` or a `$ref`, is read as a file: its drive was taken for the scheme of an url.
  Only `http` and `https` are urls now.

### Under the hood

- HTTPS goes through rustls and the certificates of the system: no more OpenSSL.
- The command line moves to clap 4, and the xml loader to roxmltree, with the same output.
- The dependency advisories are fixed, `cargo deny` checks them in the CI, along with `cargo fmt` and `cargo clippy` without warnings.
- Edition 2024.
