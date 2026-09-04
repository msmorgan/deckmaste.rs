---
needs: []
---
**Core: `Reference::Source` is not a reference.** Law 4 of
[Core is explicit regions](../../decisions/core-explicit-regions.md) lists
`Source` among the bindings that leave core, and `Reference::Reg(RefId)` as the
only binding read. `Source` survives at
`crates/deckmaste_core/src/reference.rs:146`, lowered identically from
semantics (`crates/deckmaste_semantics/src/reference.rs:244` →
`crates/deckmaste_lowering/src/reference.rs:84`).

It has exactly one meaning and one real consumer: inside
`Condition::Matches(Source, filter)` it is an existential over the marked-damage
records of the object under evaluation —
`crates/deckmaste_engine/src/condition.rs:64-72` reads `frame.source(self)` and
tests `filter` against each mark's deal-time `source_abilities`. Nothing else
denotes it:

- `crates/deckmaste_engine/src/target.rs:686` returns `None` for it (not a
  register derivation);
- `crates/deckmaste_engine/src/resolve/query.rs:483-486` fizzles it with
  "Source is only meaningful inside Matches(Source, …)";
- `crates/deckmaste_core/src/cost.rs:173` answers `true` to the boundness
  question trivially;
- `crates/deckmaste_plugin/src/idris_emit.rs:609-612` refuses it as a gap.

The production author is plugin data, not Rust:
`plugins/builtin/rules/sba/lethal-damage.ron:14` spells the lethal-damage SBA's
deathtouch clause as `Matches(Source, Has(Deathtouch))` ([CR#704.5h]), and
`plugins/builtin/macros/identity/Source.ron` is its identity macro. The
`Reference::Source` occurrences in `crates/deckmaste_engine/src/sba.rs` are
inside `#[cfg(test)] mod tests` (`sba.rs:420`) and are tests of that rule, not
consumers of the variant.

**Pinned shape: an explicit damage-history query condition,
`Condition::DealtDamageBy(Reference, Predicate)`, whose subject is a register
read — NOT a region param.** The binding is set-valued over stored deal-time
records and never denotes an object, so law 2's provenance list has no entry it
could take and `eval_reference` would have nothing to return; law 4 keeps pure
history reads as expressions at the read site, and `Count::Damage(Reference)`
in `crates/deckmaste_core/src/count.rs` is the existing house idiom for exactly
that query over the same marks.

Scope:

- Add `Condition::DealtDamageBy(Reference, Predicate)` to
  `crates/deckmaste_core/src/condition.rs`; move the existential at
  `engine/src/condition.rs:64-72` onto it, reading the subject register instead
  of `frame.source`.
- Delete `Reference::Source` from core, the `None`/fizzle/boundness arms above,
  and the `idris_emit.rs` gap branch — the new condition emits, so the refusal
  has nothing to refuse.
- Re-spell `plugins/builtin/rules/sba/lethal-damage.ron` as
  `DealtDamageBy(This, Has(Deathtouch))` and retire
  `plugins/builtin/macros/identity/Source.ron`.
- Semantics keeps `Reference::Source` (law 4: "semantics keeps its anaphors").
  The translation therefore lands in lowering's `Condition::Matches` arm, not
  its `Reference` arm — a `Reference` → `Reference` arm cannot produce a
  `Condition`.

Out of scope: the deal-time capture of `source_abilities` itself
(`crates/deckmaste_engine/src/object.rs:263`), which is unchanged.

Acceptance:

- `Reference::Source` is absent from `deckmaste_core`, `deckmaste_lowering`'s
  core-facing arms, `deckmaste_engine`, and `deckmaste_plugin`.
- The Idris emitter has no `Source` gap branch, and
  `cargo xtask idris-check <plugin> --differential` is unchanged in verdict.
- The deathtouch SBA tests in `sba.rs` are re-spelled against
  `DealtDamageBy`, not deleted, and still assert destruction of a creature
  struck by a deathtouch source that has since left ([CR#704.5h]).
- `cargo test --workspace` green.

Standard constraints apply. Effort: **S**.
