---
needs: [ci-lean-gate, semantics-v2-crate]
---
**Deferred by the user (2026-09-06) until `semantics-v2-crate` lands:** the
emitter consumes the actual v2 card representation and macro expansions,
emitting fully expanded terms as untracked generated Lean (`Macros.lean`
plays no part in the gate). Do not build an adapter from the current
expanded RON or invent a Lean/Rust crosswalk; the existing Idris gate stays
in place meanwhile. Chain and rulings: the promoted
`docs/decisions/semantics-v2.md`.

**The Lean workbench, not the Idris mirror, is the soundness gate for card
data.** Today `cargo xtask idris-check <plugin>` re-emits every expanded
RON card as an `idris/src/Semantics.idr` term through
`crates/deckmaste_plugin/src/idris_emit.rs` (4,507 lines) and typechecks it
with `idris2`; a per-plugin `idris-check-baseline.ron` ratchets pass/gap, and
`--differential` cross-checks that verdict against `deckmaste_lowering`. The
ADR `docs/decisions/idris-is-a-soundness-gate.md` is what makes the CI
`idris` job load-bearing. Replace it:

- an emitter from the expanded card to a Lean `Card` value of
  `lean/Semantics/Card.lean` (a `.lean` file per plugin, or one generated
  module), and a gate that `Card.check` returns `[]` for each, evaluated by
  `decide` exactly as the pin suites are;
- the baseline ratchet and the differential mode re-homed on the Lean verdict;
- `idris_emit.rs`, `idris_check.rs` and `plugins/*/idris-check-baseline.ron`
  deleted once the Lean gate covers every card the Idris gate did, with the
  landing record listing every card whose verdict differs and why.

Decisions already made: the Lean model is the semantics the engine consumes
(numbers are `Int` with declared regimes; core deeds are a closed taxonomy;
conferrals come from the registries); a law reads a declared feature, never a
lexeme. `plugin-rider-split` proposed extracting `idris_emit.rs` into a crate;
this ticket supersedes that half of it.

## Ledger

Routed to `docs/tickets/planned/plugins-v2-canon.md` (lines added there
2026-09-06), because this ticket's own deletion condition — "once the Lean gate
covers every card the Idris gate did" — cannot be met yet: the Idris gate
checks the 80 v1 `plugins/canon` cards, and `plugins_v2/` holds only the two
`testing` fixtures until `plugins-v2-canon` lands.

- Delete `crates/deckmaste_plugin/src/idris_emit.rs`,
  `crates/xtask/src/idris_check.rs`, the `IdrisCheck` command and every
  `plugins/*/idris-check-baseline.ron`, and drop the `check canon Idris
  baseline` CI step.
- Re-home `idris-check --differential` onto `lean-check` + `deckmaste_lowering_v2`
  (blocked on `lowering-v2`, which does not exist; mint it as its own ticket if
  `lowering-v2` has not landed by then).

## Landing record

Change: `vzozuwxqpzkp` (`lean-check: the xtask gate, ratchet, and fixtures`),
plus the docs/CI commit above it.

### PROVE

- **The gate is executable and discriminating.** `cargo xtask lean-check`
  reads `plugins_v2/testing` through the v2 reader, emits both cards as fully
  expanded Lean terms, and `lake build --wfail Generated` proves
  `Card.check = []` for each by `decide`. Lightning Bolt's term is
  post-expansion through two macro levels (`DealsDamageToAnyTarget` →
  `Instruction.dealDamage` with `NounPhrase.described (DetPhrase.target …)`),
  so what the kernel proved is the card, not the macro call.
  The negative fixture proves the gate refuses: "Lawless Land" is a land face
  printed with a mana cost, `cardCostOk` in `lean/Semantics/Check/Card.lean`
  returns `.cardCost`, `decide` refutes the theorem, and the gate attributes
  the diagnostic to that card alone while its neighbour "Lawful Land" still
  proves.
- **No silent loss.** Nothing was retired. The Idris gate, its emitter, its
  baselines and its CI step are untouched and still cover the 80 v1
  `plugins/canon` cards; no card lost a checker.
- **The mapping cannot drift.** The Lean↔Rust name mapping moved out of
  `tests/lean_drift.rs` into `deckmaste_semantics_v2::lean_emit`, and the drift
  test now scans Lean names RAW, translates through the crate's
  `rust_variant`/`rust_field`, and additionally asserts
  `lean_variant(rust_variant(n)) == n` and `lean_field(rust_field(n)) == n` for
  all 1,788 constructor and field names the six syntax files declare (the test
  refuses a scan finding fewer than 800). A Lean spelling the
  emitter would get wrong therefore fails the drift test first.
- **No word-naming.** The emitter names no card, lexeme or constructor: it
  renders through `serde::Serialize`, so a Lean constructor's identity comes
  from the mirror's own declaration, never from a match arm listing names. The
  gate's only named data are the fixture card names in its own test.
- **`lean/scripts/build` is unchanged.** Verified in both states: the cold
  build with `lean/Generated/` absent, and after a gate run with it present —
  77 jobs, "Build completed successfully" both times. `Generated` is a
  `lean_lib` outside `defaultTargets`; lake does not refuse a declared library
  whose root file is missing, so no tracked placeholder was needed.

### DISCLOSE

- **Cards newly covered:** `plugins_v2/testing`'s two, "Grizzly Bears" and
  "Lightning Bolt", both `Pass`. That is every card `plugins_v2/` holds.
- **Deviations and additions.**
  1. **The one deviation the brief names.** The ticket's own text deletes
     `idris_emit.rs`, `idris_check.rs` and the baselines "once the Lean gate
     covers every card the Idris gate did". It does not yet: v2 has no canon.
     So this landing delivers the emitter, the gate, the ratchet and the CI
     wiring, keeps the Idris job, and routes the deletion and the differential
     re-home to `plugins-v2-canon` (see `## Ledger`).
  2. **Emission goes through `serde::Serialize` rather than hand-written
     per-type renderers.** The serde data model already carries what a Lean
     term needs — a variant's declaring type and name, a struct's fields in
     declaration order, `u32` and `i32` apart — so the renderer covers the
     whole mirror by construction instead of ~200 arms that could fall behind a
     new constructor. It also unwraps `macro_ron_derive`'s private
     `__TypeVariant` helper struct, which a `SupportsMacros` type lowers each
     struct variant through; a plainly serde-derived type writes the same
     variant directly, and both spell one Lean constructor applied to its
     fields.
  3. **Terms are fully qualified and ascribed** (`Semantics.Card.singleFaced`,
     `({ … } : Semantics.Characteristics)`) rather than relying on `open` and
     dot-notation, so an emitted fragment elaborates in any position without
     depending on the expected type being inferable.
  4. **The negative fixture lives at
     `crates/xtask/tests/fixtures/plugins_v2_lawless/`,** not as a
     `plugins_v2/` sibling: a deliberately unsound plugin under `plugins_v2/`
     would be picked up by the gate's own default plugin set and by any other
     v2 reader. It carries no tracked baseline; its test copies it to a
     temporary directory, writes a both-`Pass` baseline there, and asserts the
     refusal, then the bless, then the clean re-run.
  5. **The default plugin set is every `plugins_v2/` directory that has a
     `cards/` directory.** `plugins_v2/builtin` is declarations only, so it has
     nothing to prove and owns no baseline; it is loaded as the prelude every
     other plugin's declarations resolve against (§15).
  6. **The CI `lean` job gained a Rust toolchain and cache** (mirroring the
     `idris` job) and its `timeout-minutes` rose from 15 to 30, because the job
     now carries a Rust compile as well as the Lean build.
  7. **Four tracked documents pointed at
     `docs/tickets/planned/lean-card-soundness-gate.md`** and asserted the gate
     was not implemented: `docs/guided_tour.md`, `docs/keyword-policy.md`,
     `docs/decisions/lean-is-the-workbench.md` and
     `docs/decisions/semantics-v2.md` §9. All four were corrected to the landed
     state and the `done/` path.
- **STOPs:** none. The one the brief anticipated — lake refusing a `lean_lib`
  whose root is absent — did not occur; it was probed before any code was
  written and `lake build --wfail` succeeded with `Generated/` missing.
- **Glossary gaps:** none. The landing introduced no term
  `docs/contexts/oracle-english/CONTEXT.md` or
  `docs/contexts/game-model/CONTEXT.md` does not already define.

### Assurance counts

Restored 0, re-spelled 1, ignored 0, added 24, removed 0.

- Re-spelled: `lean_drift.rs`'s `the_name_mapping_is_the_documented_one` —
  same assertions and same subject, now spelled against the crate's
  `rust_variant`/`rust_field`.
- Added: 12 unit tests in `deckmaste_semantics_v2::lean_emit` (each numeric,
  option, list, string, struct and enum case, the three named escapes, the
  ident allocator, the module shape), 1 in `lean_drift.rs` (the round trip
  over every Lean name), 9 in `xtask::lean_check` (diagnostic parsing,
  attribution, the gate-defect refusal, the baseline format, and every
  direction of the ratchet), and 2 xtask integration tests that really invoke
  `lake`.
- No test was deleted, `#[ignore]`d, or weakened to a `matches!`.

### REPORT

- **Gate line** (`cargo xtask gate --changed`):
  `cargo test -p deckmaste_semantics_v2 -p xtask` — green:
  25 + 4 + 6 (semantics_v2 lib, `lean_drift`, `reader`) and
  488 + 13 + 1 + 1 + 2 + 2 (xtask lib, bin, `english_v2_determinism`,
  `flavor_words`, `lean_check`, `plugins_v2_declarations`), 1 pre-existing
  ignored.
- `cargo fmt --all`, and
  `cargo clippy -p deckmaste_semantics_v2 -p xtask --all-targets -- -D warnings`
  clean.
- `lean/scripts/build`: 77 jobs, successful.
- `cargo xtask cite check --list-noncompliant`: 0 non-compliant.
  `cargo xtask cite check`: 15,087 citations, 0 stale. One citation site added
  (`[CR#107.1]` in `lean_emit`'s `int_literal`), audited against the rule text.
- **Performance advisory.** `lean-check` wall time on `plugins_v2/testing`
  (2 cards), measured on `vzozuwxqpzkp`: 0.2 s with the generated library's
  oleans warm, 0.7 s with them removed and rebuilt, against a warm
  `Semantics`. Host load average 4.71, 24 cores, `lake`'s own worker count.
  The per-run floor is the `lake build` invocation, not the emission; emission
  of both cards is under a millisecond.
