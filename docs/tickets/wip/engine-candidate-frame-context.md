---
needs: []
---
**Engine: three matcher seams need a full carrier `Frame`, not just a
watcher — and nobody has checked whether they're reachable.**

Distinct from `engine-frameless-carrier-threading`, which only needs an
`ObjectSource`. These three need resolved targets and bindings:

- `target::matches_with`'s catch-all `Predicate::Ref(r)` arm — any `Reference`
  beyond `This`/`You`/`AttachHostOf(This)`, such as `Target(n)` or
  `ThatObject`, panics even when a watcher *is* present.
- `target::const_count` — a non-literal bound (`CountOf`, `StatOf`, `X`)
  inside a `Stat`/`TargetCount` predicate can't be evaluated.
- `resolve/targets.rs`'s `const_target_count` — the same restriction on
  target quantities, and its doc comment says it "mirrors `const_count`".

**Why the owners were wrong.** The first two named `engine-filter-breadth`,
the third `engine-target-distinctness`. Neither ticket documents a
literal-only carve-out; `engine-target-distinctness`' "never a panic" language
is scoped to *malformed* authoring (out-of-range sibling index, inverted
bounds), not *dynamic* bounds.

**Audit before building.** The code comments assert these shapes are
"vanishingly rare" and that "no canon target quantity is dynamic", but no
corpus check backs that up, and a source comment is not a scoped decision. So
the first task is to establish reachability: can any live caller
(`candidates`/`candidates_with` in `activate.rs` and `replace.rs`,
`target_set_distinct_ok` and the resolution re-check in `resolve/targets.rs`)
reach a filter with a non-`This`/`You`/`AttachHostOf` `Reference` or a
non-literal bound today?

If reachable, thread the resolving `Frame` through. If genuinely unreachable
in the current corpus, downgrade these to a documented invariant — a
`debug_assert` and a comment — rather than leaving a production-reachable
panic. Either outcome is a valid close; guessing is not.

Effort: **M**.
