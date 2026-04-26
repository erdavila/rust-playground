default: fmt test clippy

[no-cd]
fmt:
  cargo +nightly fmt -- --config group_imports=StdExternalCrate --config imports_granularity=Module

[no-cd]
test *ARGS:
  cargo test {{ARGS}}

[no-cd]
clippy:
  cargo clippy --all-targets
