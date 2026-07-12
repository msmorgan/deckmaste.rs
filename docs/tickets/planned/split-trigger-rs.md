---
needs: []
---
**Carve `crates/deckmaste_engine/src/trigger.rs` (~4.9k lines, the largest
file left after the resolve split) into a `trigger/` module directory, using
the `resolve/` split as the template.** Like pre-split `resolve.rs`, the bulk
is almost certainly the embedded `mod tests`; measure first, then split code
by subsystem seam and tests by the submodule that owns what they exercise.

## Template (what worked for `resolve/`)

1. `mv trigger.rs trigger/mod.rs` — verify the lib still builds before
   touching anything else.
2. Carve code into submodules along method-cluster seams, each holding its own
   `impl GameState` block (established crate convention — no traits, no
   forwarding shims). Helpers crossing submodule lines get `pub(super)`;
   parent-module free fns are *visible* to children but not *in scope* — add
   `use super::<fn>;` where needed. Anything referenced from outside the
   module by `crate::trigger::<item>` paths keeps its old path via
   `pub(crate) use` re-exports in `mod.rs`.
3. Split `mod tests` per owning submodule; shared test helpers go to a
   `#[cfg(test)] mod fixtures` sibling (`pub(super)` items), theme-specific
   helpers stay local to their test mod.
4. Gates: engine suite green with **exact `#[test]`-count parity** before vs
   after; workspace clippy clean; nightly fmt; `cargo xtask cite check` (CR
   citations move files); and a **comment-multiset audit** — diff the sorted
   set of comment lines before vs after so no authored comment is silently
   dropped.

## Gotchas hit during the resolve split (avoid re-learning)

- The tests mod had `use` statements interleaved *between* test fns, not just
  in the header. Any mechanical carve that assigns "lines between items" to
  the preceding item will glue those imports into the wrong bucket — collect
  ALL module-level `use` lines in the tests mod up front and treat them as
  header for every emitted test mod (then let `cargo fix` prune unused).
- Section-banner comments (`// ---- … ----`) sitting between test clusters
  are easy to orphan or delete with adjacent stray imports; the
  comment-multiset audit catches this.
- `cargo fix --lib/--tests --allow-dirty --allow-no-vcs` handles the
  unused-import blizzard from copying the full import block into every new
  file (`--allow-no-vcs` needed: jj feature workspaces have no `.git`).
