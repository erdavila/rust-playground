#!/bin/bash
cargo +nightly fmt -- --config group_imports=StdExternalCrate --config imports_granularity=Module
cargo clippy --all-targets
