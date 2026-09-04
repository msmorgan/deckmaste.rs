---
needs: []
---
**Core: build law 13's printer, and clear the transitional spellings it would
print.** Law 13 of
[Core is explicit regions](../../decisions/core-explicit-regions.md) —
"numeric ids, typed riders, a printer" — argues that because core is generated,
verbosity is free and a pretty-printer showing riders is THE debugging
affordance for numeric ids. No such printer exists: `deckmaste_core`'s only
`Display` impls are `KeywordAbility`
(`crates/deckmaste_core/src/keyword.rs:196`) and `ValidationError`
(`crates/deckmaste_core/src/region.rs:315`), so a `Region` or an `Instr` can
only be read as `Debug`, where a `RefId(3)` says nothing about what it binds.

Alongside it, three pieces of mechanical residue the region stages left behind.

**The `OneShotEffect` alias.** `pub type OneShotEffect = Instr;`
(`crates/deckmaste_core/src/effect.rs:216`) is documented as transitional —
"new core code should say `Instr`" — and is the dominant spelling across the
workspace, so the migration it was holding open never happened. Rename the uses
to `Instr` and delete the alias.

**The `Act` compat constructor.** `#[allow(non_snake_case)] pub fn Act`
(`crates/deckmaste_core/src/effect.rs:169-176`) exists to preserve the
pre-region `OneShotEffect::Act(action)` call spelling and is byte-for-byte
`Instr::act` (`effect.rs:200`). Delete it; the callers say `act`.

**`core::Sort`.** `crates/deckmaste_core/src/sort.rs` is exported at
`crates/deckmaste_core/src/lib.rs:247-248`. Its one consumer is the `Lower`
impl at `crates/deckmaste_lowering/src/sort.rs`, which is itself dead: nothing
lowers a `Sort` since `Reference::That(Sort)` left core under law 4. Every live
`Sort` in the workspace is `deckmaste_semantics::Sort`. Both files delete
together.

**Stale docs.** Some doc comments still describe retired machinery as current.
The mechanically decidable half is the broken intra-doc links — measured today,
`cargo doc -p deckmaste_core --no-deps` reports 7 unresolved links that are
not citation-bracket false positives: `Reference::It` ×3 (`count.rs:138`,
`filter.rs:214`, `selection.rs:104`), `Selection` ×2, `crate::layer`, and
`crate::Decision`. The
prose half, to read and correct by hand: `core/src/count.rs:147`,
`selection.rs:95-96`, `continuous.rs:284`, `action.rs:488` (names
`AdditionalCost`, deleted by law 9), `target_spec.rs:11`, `region.rs:82-87`
("Reserved by the discourse stage" on variants that are live), `region.rs:143-147`
("Stage 1 keeps…"), and engine `state.rs:457`, `resolve/query.rs:426-437`,
`activation.rs:31-32`.

**Leave `crates/deckmaste_core/src/reference.rs:76-88` alone** — it is not
stale. It is the doc comment of the live test
`discourse_anaphors_have_no_core_spelling`, which asserts law 4.

Scope: a `Display` for `Region` and `Instr` that shows the `DefId`/`RefId`
riders and each param's provenance; the alias rename plus deletion of the alias
and `Act`; deletion of `core::Sort` and `lowering/src/sort.rs`; the doc fixes
above.

Out of scope: the citation-bracket-versus-rustdoc collision itself, which is
[[ci-rustdoc-link-gate]] — until that lands the doc-link count carries no
signal in aggregate, which is why this ticket's acceptance is scoped to the
named links rather than to a total.

Acceptance:

- `OneShotEffect`, `pub fn Act`, `crates/deckmaste_core/src/sort.rs`, and
  `crates/deckmaste_lowering/src/sort.rs` are absent from the workspace.
- `cargo doc -p deckmaste_core --no-deps` reports none of the 7 unresolved
  links listed above.
- One printer test snapshotting a region with declared params and at least one
  instruction product, so the rider spelling is pinned rather than incidental.
- `cargo test --workspace` green.

**Mechanical and delegable** — the alias rename is a scripted sweep, and the
deletions and doc fixes are per-file. Only the printer's output shape is a
judgment call; pin it in the snapshot test.

Standard constraints apply. Effort: **M**.
