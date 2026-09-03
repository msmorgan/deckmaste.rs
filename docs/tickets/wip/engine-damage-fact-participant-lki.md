---
needs: []
---
**A damage fact's participants are bare ids, so a history read cannot name a
creature that has since left the battlefield.** Found by
`core-regions-witness-fixtures`; it is the blocker on Predator Ooze. Standard
constraints apply.

## The gap

`FactView::of`'s `GameEvent::DamageDealt` arm records `source` and `patient`
through the `part` helper, which yields `Part::Obj(id)` for any card-backed
participant — no last-known-information snapshot, unlike the past-form
`ZoneChange` arm, which carries `Part::Gone(snapshot)`. Once that object has
moved, `GameState::part_matches` finds `self.objects.get(id)` empty and falls
back to matching only `Predicate::Any`, so an identity filter over the
participant is false.

"Whenever a creature dealt damage by this creature this turn dies" is exactly
the case where the damaged creature is gone at read time: the candidate is the
dying creature's snapshot, the pattern is
`Where(Happened(Damage(source: Ref(This), to: Ref(It)), ThisTurn))`, and the
recipient half can never match. [CR#608.2i] is explicit that a look-back read
does not require the object to still be where it was.

The source half already works (the Ooze is alive), which localizes the defect
to the recorded participant rather than to `Where`, `Happened`, or the
candidate binding.

## Acceptance

`crates/deckmaste_engine/tests/region_witnesses.rs`'s
`predator_ooze_counts_only_creatures_it_damaged_this_turn` loses its
`#[ignore]` and passes unchanged, including its control (a creature that dies
the same turn undamaged adds no counter).

## Landing record

### The diagnosis above is stale; the defect was one binding further in

The ticket's "The gap" section describes the fact side, and the fact side was
already fixed before this round started. `FactView::of` does build a live
damage fact with `Part::Obj` participants, but nothing reads that live view as
history: `record_history_fact` stores `FactView::of(..).into_lki(self)`, and
`Part::into_lki` snapshots every live card participant. Instrumenting the
`EventFilter::Damage` arm on the failing fixture showed the recorded damage
fact already carrying `patient = Part::Gone(<the bear>)` and
`source = Part::Gone(<the Ooze>)`, with the SOURCE half matching and only the
recipient half false. So `part_matches` never reached its stale-id fallback
and the ticket's localization ("not the candidate binding") was inverted.

The real defect was the CANDIDATE register. `filter_matches_snapshot_with_activation`'s
`Predicate::Where` arm entered the per-candidate region through
`enter_candidate_region(cond, &frame, snapshot.object)` — passing only the
dying creature's stale id, discarding the snapshot it was holding.
`pack_objects` then packed that stale id into
`ReferenceProduct { current: None, lki: None }`: a product that names nothing.
The recipient filter `Ref(It)` compares that product against the fact
participant by identity (`product.current == snapshot.object ||
product.lki.object == snapshot.object`), and a nameless product loses both
comparisons — so the read was false for the same reason the ticket gives, one
binding earlier.

This is disclosure of a stale premise, not a resolved ruling contradiction: no
recorded ruling says the fact side is unfixed, and the ADR's own law 11
("`eval_reference` is an indexed read plus provenance dispatch returning one
product (live id, optional LKI)") is what the fix restores for a departed
candidate. No STOP was taken.

### The change

`GameState::enter_candidate_region_snapshot` — the gone-candidate counterpart
of `enter_candidate_region`, binding the candidate as the `LkiSnapshot` the
caller already holds. It is the existing last-known-information idiom, not a
second mechanism: byte for byte the product shape `enter_region_with` already
builds for `Provenance::EventObject` and `event_patient_value` builds for a
departed event patient (`current` from a live lookup, `lki: Some(snapshot)`).
The one caller is the snapshot matcher's `Where` arm; the four live-candidate
callers (`target.rs` ×2, `resolve/count.rs`, `resolve/query.rs`) are untouched,
and a candidate that is still live keeps its live identity, so no action-side
read changes.

Rules basis, verified through the `mtg-rules` skill rather than recalled:
[CR#608.2i] — a look-back read's subjects "don't need to be currently in the
zone they were in at the time of that previous game state or action, nor do
they need to currently meet the criteria described in the action, as long as
they did so at the specified time", provided the effect takes no action on
them. Predator Ooze's trigger reads history about the dying creature and acts
only on itself, so it is squarely inside that exception; [CR#608.2h], which
[CR#608.2i] excepts, is the base rule that sends a departed object's reads to
last-known information.

### Numbers

- Tests: **1 added**, **1 ignore closed**, **0 removed**, **0 re-spelled**,
  **0 newly ignored**.
  - Added: `activation::tests::a_departed_candidate_binds_its_snapshot_where_its_stale_id_names_nothing`
    — pins both halves directly (the stale id packs to a product with neither
    `current` nor `lki`; the snapshot entry keeps the identity), so a
    regression is caught without a whole game.
  - Ignore closed: `predator_ooze_counts_only_creatures_it_damaged_this_turn`
    in `crates/deckmaste_engine/tests/region_witnesses.rs`. Its stale BLOCKED
    paragraph is replaced by what the fixture now proves; both assertions and
    the control are unchanged in substance.
  - Engine ignores now 7 (was 8): 5 in `region_witnesses` (unrelated grammar
    and lowering blockers), plus `reconfigure_suppresses_creature` and
    `bears_vs_bolts_50k_game_stats`. Every one still names its blocker.
- Citation lock: **untouched** — [CR#608.2i] was already registered, so no
  `cite bless` ran and there was no prune to repair. 17,983 citations checked,
  0 stale (the count is trunk-wide and moved with the pre-integrate refresh).
- Coverage lock, construction count, `english-v2-coverage.lock`: untouched.
  No core, lowering, grammar, or plugin-data change, so `plugins/wizards` was
  not regenerated; the corpus-gated engine tests (`layer`, `sba`) and every
  `deckmaste_plugin` binary ran rather than being cfg-skipped, which is the
  positive evidence that the workspace's corpus is live.

### Gate artifacts

Every line below is from the re-run AFTER `kata refresh` pulled
`workbench-slot-folds` onto the feature; the pre-refresh run agreed.

- `cargo test -p deckmaste_core -p deckmaste_lowering` — six binaries, each
  `test result: ok`, `0 failed`, `0 ignored`: 50, 738, 3, 4, 0, 0 passed.
- `cargo test -p deckmaste_engine` — twenty binaries, each `test result: ok`,
  `0 failed`. Ignored counts: lib `738 passed; 0 failed; 1 ignored`;
  `region_witnesses` `14 passed; 0 failed; 5 ignored`; the stats binary
  `4 passed; 0 failed; 1 ignored`; every other binary `0 ignored`.
- `cargo test -p deckmaste_plugin` — fourteen binaries, each `test result:
  ok`, `0 failed`, `0 ignored`.
- `cargo clippy -p deckmaste_core -p deckmaste_lowering -p deckmaste_engine -p
  deckmaste_plugin --all-targets` — `Finished`, no warning.
- `cargo xtask cite check --list-noncompliant` — `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check` — `checked 17983 citations against cr.txt
  (eff. 2026-08-07); 0 stale`.
- `cargo xtask idris-check plugins/canon --differential` — `68 agreed sound,
  0 agreed unsound, 12 skipped (no certifier verdict); differential OK: 0
  disagreements`.
- `jj diff --git | cargo xtask cite audit --diff` — 3 sites, all [CR#608.2i],
  each rule text read against its claim.

### Deviations and additions

- **The fixture's control cast was corrected, not weakened.** With the engine
  fix in place the first assertion passed and the test then failed inside the
  control on `the fixture spell is castable`. Cause is fixture-harness, not
  engine: `Priority.legal` is snapshotted when the window opens and
  `Priority::resolve` re-validates against that list, while the test's helper
  `into_hand` moves a card behind the engine's back — and it prefers the
  LIBRARY, so calling it a second time drew a SECOND Witness Doom that no open
  window had ever offered. The first call's return value was being discarded.
  The fix binds that first (pre-`to_phase`) Doom and casts it; every
  assertion, the target choice, and the asserted outcome are byte-identical.
  No engine behaviour was relaxed to accommodate it, and every other fixture
  in the file already seeds its hand before the run to combat.
- **One test added beyond the ticket's letter** (the activation unit test
  above). Justification: the ticket's acceptance is a whole-game witness, which
  proves the behaviour but localizes nothing; the unit test pins the exact
  register contract the fix establishes.
- No construction added or deleted; no test deleted.

### Residue

None. The fix is total over the one caller that had a snapshot to pass; the
four live-candidate call sites need nothing. No new ticket minted.
