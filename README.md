# CodeGenR

![codegenr graphical explanation](_assets/codegenr.svg)

## Installation

Install Rust : https://www.rust-lang.org/tools/install.
And then install `codegenr`

```
cargo install codegenr
```

## Documentation

`codegenr` commmand is mostly a launcher for `codegenr-lib`. So most of the documentation can be found in [codegenr_lib documentation](https://docs.rs/codegenr-lib/latest/codegenr_lib/).

The defaults Handlebars helpers are documented [here](https://docs.rs/codegenr-lib/latest/codegenr_lib/helpers/index.html).

## Contribute

You can also open the repository in GitPod with this button
[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/eventuallyconsultant/codegenr)

In the command line you can :

- `cargo test --workspace` to launch all the unit tests
- `cargo install --path codegenr` to install the `codegenr` command line tool from source
- `cargo doc --open` to compile and open the documentation (that you can otherwise find here)

## Some command lines

- `cargo doc --open` compile and open the documentation
- `cargo install --path codegenr` installs codegenr command line from sources

## Legacy

This tool is based on the design of a precedent one written in `C#` : [CodegenUP](https://github.com/BeezUP/dotnet-codegen).

## Roadmap :

- [x] Load a yaml or json to serde::json
- [x] Resolve `$ref` tags
- [x] Pass all the resulting document to an handlebar template
- [x] Implement some default helper (and make some doc tests about them)
- [x] A plugin system
- [x] Migrate C# custom helpers to this new plugin system
- [x] A this point, we could use `codegenr` in place of `CodegenUP`, just by calling some commands
- [x] Rename `codegenr-cli` to `codegenr` & `codegenr` to `codegenr-lib`
- [x] Publish on `crates.io`
- [x] Be able to have a `codegenr.toml` on a workspace root to describe all the templates to execute around the workspace
- [ ] Better Errors (typed ones)
- [ ] Resolved Json Cache optimisation
- [ ] Verbose/Tracing? mode
- [ ] Better examples
- [ ] Smol strings optimisation ?
- [ ] all tests passing on windows too
- [ ] Watch mode for the file changes
- [ ] Make a VSCode extension about all of this to make it live / super user friendly for `everyone`

- [ ] Allow multiple swagger2 documents merging
- [ ] Allow multiple swagger3 documents merging
- [ ] Transform the json to an [OpenApi Generator](https://openapi-generator.tech/) model, and be able to use all the `OpenApi Generator` templates ?
