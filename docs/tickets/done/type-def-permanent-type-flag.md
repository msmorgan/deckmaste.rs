---
needs: []
---
**Rename `TypeDef.permanent` to `permanent_type` everywhere.** The old name
sounds as though the Card Type is itself a Permanent or that it partitions
Cards into Spell and Permanent kinds. The field actually records whether the
declared Card Type is one of the six Permanent Types in [CR#110.4]. See
[`Permanent Type`](../../contexts/game-model/CONTEXT.md).

Scope (ruling 2026-09-04): the Idris workbench, core, engine, and lowering's
core-facing output only; `deckmaste_semantics` and `idris/src/Semantics.idr`
are deletion-bound and untouched; their side converges at cutover when the RON
re-emit path points at `Experimental`.

Keep the flat `TypeDef` shape and its `confers` collection. Do not introduce a
two-member `Spell | Permanent` enum and do not encode combat or attackability
metadata in this flag: those capabilities remain positive conferrals. In
particular, Land is a Permanent Type and appears on Permanent Cards, but a land
card is not a Permanent Spell ([CR#110.4a..110.4b]). Any cast-resolution helper
must use the renamed fact with that distinction intact.

The workbench already spells this `Experimental.Words.permanentType`; core and
its consumers move to that spelling rather than the other way round. Migrate
`deckmaste_core`, the engine, lowering's core-facing output, the builtin-v2
type RON, generated fixtures, emitters, tests, and documentation in one ticket.

Overlaps `workbench-supplements-out`, which drops six card types from the
workbench `CardType`, and the `workbench-levelers`/`workbench-prototype` card
wrappers; whichever runs second reconciles the names.

Acceptance includes a Land witness, confirms the authored builtin-v2 RON
remains flat, and keeps `cd idris && ./scripts/build` green at its module
count.

## As landed

`TypeDef.permanent` renamed to `permanent_type` on `deckmaste_core::TypeDef`
(struct field, `Type::permanent()` → `Type::permanent_type()`,
`Type::def()`/`TypeRef::named`/`impl From<Type> for TypeRef`) and on every
site that constructs or reads a `deckmaste_core::TypeDef` value:

- `deckmaste_lowering/src/type.rs`: `Lower for deckmaste_semantics::TypeDef`
  writes `permanent_type` on the `deckmaste_core::TypeDef` it produces, still
  reading the semantics side's unrenamed `self.permanent` — exactly the
  "lowering's core-facing output only" scope line.
- `deckmaste_engine`: `cast.rs`, `copy.rs`, `layer.rs`, `legal.rs`,
  `resolve/mod.rs` (`is_permanent_spell`, its doc, and its unit tests),
  `state.rs` (doc comments), `tests/replace_registry.rs`,
  `tests/damage_result_rules.rs`, `tests/full_game.rs` (a raw
  `StrategyEvaluator::from_ron` RON fixture — deserializes straight into core
  types, no macro/semantics layer).
- `deckmaste_plugin`: `src/plugin.rs` (a `deckmaste_core::TypeDef`-typed unit
  test, renamed alongside its cross-referencing doc comment), `src/validate.rs`
  (one `TypeDef` lint fixture), `tests/builtin.rs`.
- `deckmaste_tui/src/game.rs`: doc comment + `.permanent_type` field read.
- `deckmaste_noncanon/strategies/{sped_red,stompy}.ron`: raw
  `StrategyEvaluator` RON, same direct-core-deserialization reasoning as
  `full_game.rs`.
- `plugins/builtin_v2/macros/stubs/types/*.ron` (all 10) and
  `crates/deckmaste_construction_core/tests/builtin_v2_types.rs` (RON body
  string literals + the local `ExpectedType.permanent` test field, renamed to
  match).

**Deliberately left unrenamed** (a distinct, off-limits `TypeDef`, per the
ticket's semantics/`Semantics.idr` carve-out): `deckmaste_semantics::TypeDef`
and its own `permanent` field/`Type::permanent()` method, plus every RON
literal that deserializes through the semantics-level macro/card/predicate
pipeline rather than straight into `deckmaste_core` — `plugins/builtin/macros/
cardtype/*.ron` (v1 type declarations), the `deckmaste_semantics`-scoped
embedded RON in `deckmaste_plugin/src/macros.rs` and
`deckmaste_plugin/src/plugin.rs:1239` (a `ConferralRule` fixture),
`deckmaste_migrations/src/graduate.rs` and `deckmaste_plugin/src/validate.rs`'s
`FOO_1_1`/`FOO_2_2` (both parse as `deckmaste_semantics::Card`), and
`deckmaste_lowering/src/minimal.rs`/`tests/diagnostics.rs`/
`tests/linked_memory.rs` (all `deckmaste_semantics::TypeDef` fixtures). Traced
each occurrence to its actual deserialization target before deciding — see the
gate-scope note below.

The Idris workbench (`idris/src/Experimental/Words.idr`) already spelled this
`permanentType`; no change needed there.

Two Land witnesses added (ticket's explicit acceptance item):
`deckmaste_core::type::tests::land_is_a_permanent_type` (`Type::Land.
permanent_type()` and `.def().permanent_type` are both true, [CR#110.4]) and
`deckmaste_engine::resolve::mod::tests::
land_reads_permanent_type_true_though_a_real_land_never_reaches_this_helper`
(a Land-flagged spell fixture through `is_permanent_spell`, documenting that
the real [CR#110.4b] Land/Permanent-Spell carve-out is enforced upstream by
lands never being cast in the first place, [CR#305.1] — `is_permanent_spell`
itself needed no Land special case).

Nothing undone: every consumer of the field compiles and the gate grep
(`plugins/builtin_v2 crates/deckmaste_core/src`) is empty.

## Landing record

- `cargo check --workspace --all-targets`: `Finished` dev profile, 0 errors.
- `cargo fmt --check`: exit 0.
- `cargo test --workspace`: every crate `test result: ok`, 0 failed across the
  whole run (captured to a file and grepped for `FAILED|panicked|error\[`,
  none found — the piped/backgrounded first attempt was discarded per the
  exit-code-lies trap).
- `cd idris && ./scripts/build`: 44/44 modules, no `Warning` lines.
- `cargo xtask cite check --list-noncompliant`: `0 non-compliant`.
- `cargo xtask cite check`: `checked 18172 citations against cr.txt (eff.
  2026-08-07); 0 stale`.
- `jj diff --git | cargo xtask cite audit --diff`: 7 sites audited (the two
  new Land-witness doc comments/messages), each rule text read against its
  claim — one citation was wrong on first pass ([CR#110.4a], "permanent card",
  used for the "Land is a Permanent Type" claim) and corrected to the bare
  [CR#110.4] ("there are six permanent types"); no `bless` needed, the
  corrected citation was already registered.
- Gate grep (`plugins/builtin_v2 crates/deckmaste_core/src`, `permanent:` /
  `.permanent\b`): empty.
- `plugins/wizards` (generated): checked for `permanent:` — no hits, no
  regeneration needed (its card-type macros come from builtin's sibling
  prelude, which is the untouched v1/semantics spelling).

Assurance counts: restored 0; re-spelled ~9 (2 renamed test functions —
`def_is_structural_name_and_permanent_type_empty_confers`,
`is_permanent_spell_reads_the_permanent_type_flag`,
`builtin_loads_ten_canonical_types_with_permanent_type_flags` — plus every
other test whose fixture literal/RON body carried the renamed field, all still
asserting the same subject); ignored 0; added 2 (the Land witnesses above);
removed 0.

Deviations and additions: the two Land-witness tests are additions the ticket
explicitly asked for ("Acceptance includes a Land witness"); no other test or
construction was added or deleted beyond the mechanical rename. No STOP was
taken — every ambiguous RON/fixture site was traced to its actual
deserialization target (`deckmaste_core::TypeDef` vs `deckmaste_semantics::
TypeDef`) rather than guessed from surrounding scope wording, and the citation
mismatch above was caught and fixed by the audit gate itself, not disclosed
and left standing.
