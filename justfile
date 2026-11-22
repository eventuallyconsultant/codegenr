
_default:
	@just --list --unsorted

alias c := check
check:
	cargo check --workspace

alias t := test
test: 
  cargo test --workspace

doc:
  cargo doc --open

install:
  cargo install --path codegenr
