---
needs: []
---
New `crates/deckmaste_tui` binary crate (ratatui + crossterm), wired as the
workspace's primary run target so `cargo run` from the root launches it. Build a
two-player Goblins-vs-Elves `GameState` from committed `plugins/canon` +
`plugins/builtin` (no data download — load like the engine tests do), and run a
minimal `step()` / `submit_decision()` loop behind a plain-text fallback UI: the
skeleton the render, input, and decision tickets build on. Part of `tui-client`.
