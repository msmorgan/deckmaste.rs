#!/usr/bin/env fish
# Formats the workspace: rustfmt on nightly (rustfmt.toml uses unstable
# options), then `cargo sort-derives` (alphabetical, the tool's default —
# the order this codebase uses). A cargo alias can only run a single cargo
# command, so this script is what "cargo fmt" means here.

cd (dirname (status filename))/..; or exit

cargo +nightly fmt --all $argv; or exit
cargo sort-derives --order Debug,Clone,Copy,Default,PartialEq,Eq,Hash
