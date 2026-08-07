---
needs: []
---
Adopt `thiserror` for the workspace's hand-written error types: pin it once in
`[workspace.dependencies]`, convert every manual `impl std::error::Error` and
its paired `Display` impl (13 today — regenerate the list at claim time with
`rg -l "impl std::error::Error" crates`), and use it for new error types going
forward.

Decisions (2026-08-06):

- `anyhow` stays where it is (binary-ish crates); thiserror covers the typed
  library errors. Messages must survive conversion byte-identical.
- `LoadError` (`macro_ron/src/frames.rs`) keeps its `source()` chain via
  `#[source]`. Its `PathBuf` fields aren't `Display`, so those messages keep a
  `.display()` shim or switch to `{path:?}`.
- Do it as one uniform pass, claimed only when no other feature is in flight —
  the diff is shallow but touches error enums across ~8 crates and conflicts
  easily with anything adding variants.
