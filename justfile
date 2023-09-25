
_default:
	@just --list --unsorted

test: 
  cargo test --workspace

doc:
  cargo doc --open

install:
  cargo install --path codegenr
