# CodeGenR

A rewrite of the C# [CodegenUP](https://github.com/BeezUP/dotnet-codegen).

## Roadmap :

- [x] Load a yaml or json to serde::json

- [ ] Resolve `$ref` tags

- [ ] Pass all the resulting document to an handlebar template
- [ ] Implement some default helper (and make some doc tests about them)
- [ ] A plugin system
- [ ] Migrate csharp customs helpers to this new plugin system
- [ ] A this point, we could use `codegenr` in place of `CodegenUP`, just by calling some commands
- [ ] Publish on `crates.io`
- [ ] Be able to have a `codegenr.toml` on a workspace root to describe all the templates to execute around the workspace, and watch for the file changes
- [ ] Make a VSCode extension about all of this to make it live / super user friendly for `everyone`

- [ ] Allow multiple swagger2 documents merging
- [ ] Allow multiple swagger3 documents merging
- [ ] Transform the json to an [OpenApi Generator](https://openapi-generator.tech/) model, and be able to use all the `OpenApi Generator` templates ?

## Some command lines

- `cargo doc --open` to open the documentation for `codegenr`
