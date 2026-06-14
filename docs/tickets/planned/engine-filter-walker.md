---
needs: [engine-derived-type-reads, engine-filter-breadth]
---
One generic `Filter` walker that owns the `AllOf`/`OneOf`/`Not`/`Expanded`/`Any`
combinator recursion once and takes a leaf-matcher closure, replacing the four
hand-recursed copies in `target::matches` (`target.rs:33`), `filter_matches_live`
(`trigger.rs:330`), `filter_matches_snapshot` (`trigger.rs:401`), and `matches_derived`
(`layer.rs:379`). The leaves legitimately differ (live / watcher-aware / snapshot /
derived-view) and stay as four closures; only the ~12 duplicated combinator arms
collapse. Also folds the `*_has_type` twins (`target.rs:99` / `trigger.rs:783`).
Preserve the existing `todo!` seams. Scheduled after the leaf behavior settles (hence
the deps) so the skeleton isn't churned mid-flight. Pure refactor.
