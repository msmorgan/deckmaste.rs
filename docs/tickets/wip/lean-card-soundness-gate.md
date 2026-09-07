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

The Idris gate retires with Idris, which retires with v1 (user ruling,
2026-09-06). Its deletion and the differential's fate are recorded on
`docs/tickets/planned/semantics-v1-cutover.md`; nothing is deleted here and
the Idris CI job stays.

## Landing record

Change: `vzozuwxqpzkp` (`lean-check: the xtask gate, ratchet, and fixtures`),
plus the docs/CI commit above it and the review-round commit
`lean-check: the verdict rests on lake's status, not on silence`.

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
- **A verdict rests on positive evidence.** A card is `Pass` only when Lean is
  shown to have elaborated it: either its module produced the `.olean` `lake`
  writes, or Lean reported a diagnostic somewhere in that module (proof it
  elaborated the module and reported per-declaration failures). Beyond that,
  `lake`'s exit status is read, every parsed diagnostic must land on a card,
  and each generated build artifact is deleted before the run so a previous
  run's `.olean` can never stand in for this one's. A `lake` that fails without
  naming a card, a diagnostic in `Generated.lean` or in `Semantics/`, and a
  module that was neither built nor diagnosed are each a **gate defect** that
  stops the command — never a verdict about a card. See the STOP below for what
  this replaced.
- **A refuted card's reason names the law it broke.** Each card's block carries
  a `#guard_msgs`-guarded `#eval Semantics.Card.check <card>` beside its
  theorem. A card that checks matches the docstring and stays silent; a card
  that does not gets an `info` line carrying the exact refusal list, and that
  list — not `decide`'s card-independent preamble — is what the baseline
  records. "Lawless Land" ratchets as
  `Fail(reason: "[Semantics.Refusal.cardCost]")`.
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
     re-home to `semantics-v1-cutover` (see `## Ledger`).
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
     now carries a Rust compile as well as the Lean build. It runs
     `cargo xtask lean-check` and then `cargo test -p xtask --test lean_check`:
     the gate's FAILURE paths — a card that breaks a Lean law, a `lake` that
     fails without naming a card — live in tests that need a Lean toolchain, so
     they run nowhere else (the Rust jobs have no elan, and the integration
     test skips itself without `lake`).
  7. **`LeanCheckArgs` carries the `lake` program as a non-CLI field**
     (`#[arg(skip)]`, set through `LeanCheckArgs::with_lake`). It adds no
     command-line surface; it exists so the gate's own test can point the
     command at a stub that fails, which is the only way to show that a failed
     build stops the command rather than leaving every card at its seeded
     verdict.
  8. **Four tracked documents pointed at
     `docs/tickets/planned/lean-card-soundness-gate.md`** and asserted the gate
     was not implemented: `docs/guided_tour.md`, `docs/keyword-policy.md`,
     `docs/decisions/lean-is-the-workbench.md` and
     `docs/decisions/semantics-v2.md` §9. All four were corrected to the landed
     state and the `done/` path.
- **STOPs.** The one the brief anticipated — lake refusing a `lean_lib` whose
  root is absent — did not occur; it was probed before any code was written and
  `lake build --wfail` succeeded with `Generated/` missing.

  One STOP was raised by review and fixed in this workspace:

  **H1 — the gate could report every card sound on a build that never ran.**
  As first landed, `build` discarded `lake`'s exit status and `attribute`
  seeded every card `Pass`, downgrading only on a parsed
  `error: <path>.lean:<line>:<col>:` line whose path matched an emitted
  module. So any failure that named no card — an unknown target, an error in
  the generated root, a broken `Semantics/`, a toolchain that never reached the
  compiler — left every card at `Pass` and the command exited zero.
  Demonstrated with a stub `lake` printing `error: unknown target` and exiting
  1: "2/2 cards prove … baseline OK", exit 0. That is the one way a soundness
  gate must never fail, and it was a defect in the gate's own shape, not a
  missing case: absence of evidence was being read as evidence.

  Fixed by making the verdict rest on positive evidence (above) and proved by
  five new unit tests plus an end-to-end test that runs the command against a
  stub `lake`. The same stub through the CLI now gives:
  `Error: gate defect: `lake` exited nonzero but reported no error against any
  emitted card, so no card has a verdict.`
- **Glossary gaps:** none. The landing introduced no term
  `docs/contexts/oracle-english/CONTEXT.md` or
  `docs/contexts/game-model/CONTEXT.md` does not already define.

### Assurance counts

Restored 0, re-spelled 2, ignored 0, added 31, removed 0.

- Re-spelled: `lean_drift.rs`'s `the_name_mapping_is_the_documented_one` —
  same assertions and same subject, now spelled against the crate's
  `rust_variant`/`rust_field`. And `lean_check`'s
  `a_lean_diagnostic_is_parsed_to_its_file_line_and_head`, now
  `..._keeps_its_position_severity_and_whole_message`: same subject and same
  real-run output, asserting the whole message rather than only its head, since
  the head is no longer all the parser keeps.
- Added, first round (24): 12 unit tests in
  `deckmaste_semantics_v2::lean_emit` (each numeric, option, list, string,
  struct and enum case, the three named escapes, the ident allocator, the
  module shape), 1 in `lean_drift.rs` (the round trip over every Lean name),
  9 in `xtask::lean_check` (diagnostic parsing, attribution, the gate-defect
  refusal, the baseline format, and every direction of the ratchet), and 2
  xtask integration tests that really invoke `lake`.
- Added, review round (7): 5 unit tests in `xtask::lean_check` — a lake
  failure naming no card, a module neither built nor diagnosed, a diagnostic in
  a file no card owns, a card that fails to elaborate (the fallback reason),
  and the guarded eval's refusal list becoming the reason — and 2 integration
  tests, the stub-`lake` end-to-end refusal and a real run recovering after it.
- No test was deleted, `#[ignore]`d, or weakened to a `matches!`.

### REPORT

- **Gate line** (`cargo xtask gate --changed`):
  `cargo test -p deckmaste_semantics_v2 -p xtask` — green:
  25 + 4 + 6 (semantics_v2 lib, `lean_drift`, `reader`) and
  492 + 13 + 1 + 1 + 4 + 2 (xtask lib, bin, `english_v2_determinism`,
  `flavor_words`, `lean_check`, `plugins_v2_declarations`), 1 pre-existing
  ignored.
- `cargo fmt --all`, and
  `cargo clippy -p deckmaste_semantics_v2 -p xtask --all-targets -- -D warnings`
  clean.
- `lean/scripts/build`: 77 jobs, successful — checked both with `lean/Generated/`
  absent and with it present.
- `cargo xtask cite check --list-noncompliant`: 0 non-compliant.
  `cargo xtask cite check`: 15,088 citations, 0 stale. One citation site added
  (`[CR#107.1]` in `lean_emit`'s `int_literal`), audited against the rule text.
- **Performance advisory.** `lean-check` wall time on `plugins_v2/testing`
  (2 cards), measured on the review-round tip: **0.2 s**, three runs, host load
  average 1.76 on 24 cores, `lake`'s own worker count. Every run now deletes
  the generated library's build artifacts first (so a stale `.olean` cannot
  stand in as evidence), which means 0.2 s is the FULL cost — emit, a from-
  scratch `lake` build of the generated module against a warm `Semantics`,
  attribution and the ratchet. The guarded `#eval` added beside each theorem
  did not move that number at this size; on a corpus it costs one extra
  interpreted evaluation of `Card.check` per card, and `plugins-v2-canon`
  should re-measure. Emission itself is under a millisecond for both cards.
