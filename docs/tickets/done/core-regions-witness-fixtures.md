---
needs: [core-regions-discourse, core-regions-test-restoration]
---
**Real witness tests for the reference channel: hand-spelled semantic
fixtures, not the corpus.** The witness cards inherited by
`core-regions-substrate` from the absorbed engine tickets are, in eighteen
of nineteen cases, `.ron.todo` in the wizards corpus with the witnessed
clause as the `Unparsed` line. The corpus test that claims to cover them
(`plugin/tests/corpus_identity.rs`) strips those lines first, so fourteen
cards contribute no region-bearing ability and the gate is a single
aggregate region count. Standard constraints apply.

## Scope

For each witness in `docs/tickets/done/core-regions-substrate.md` (Angel of
Finality; Arbiter of Woe; Blasphemous Edict; Bloodtithe Collector; Burglar
Rat; Deadly Brew; Duress; Liliana, Dreadhorde General; Painful Quandary;
Perforating Artist; Pilfer; River's Rebuke; Tribute to Hunger; Tinybones,
Bauble Burglar; Fiery Annihilation; Run Away Together; Steel Hellkite;
Trygon Predator; Predator Ooze): spell the witnessed ability in semantics
RON by hand, lower it, and run it through the engine asserting the card
result the absorbed ticket named (per-player choice, target-relative
filter, trigger-relative target, whole-zone selection, cross-target and
event filter, the damaged-this-turn history predicate). One test per card,
named for it. Delete the corpus-stripping witness test. Where the semantic
grammar cannot yet spell a clause, the test is `#[ignore]` with the
blocking grammar gap named, never a passing test over a different ability.

## Gates

Nineteen named tests; each ignored one names its gap; engine and plugin
suites green with `plugins/wizards` regenerated and `crates/*/build.rs`
touched.

## Landing record

### Numbers

- Tests: **19 added** (one per witness, named for its card) in the new
  `crates/deckmaste_engine/tests/region_witnesses.rs` — 13 passing, 6
  `#[ignore]`d with the exact blocker in the attribute and an adjacent
  paragraph. **1 removed**: `corpus_identity.rs`'s
  `core_region_substrate_witness_cards_lower_with_valid_regions`, the
  corpus-stripping sweep this ticket exists to replace; its module doc now
  points at the successor file. **0 restored, 0 re-spelled** — nothing else
  covered these cards.
- Citation lock: 1568 rules blessed, two newly registered here
  ([CR#608.2a] and [CR#608.2e]); `cite check` 0 stale. The noncompliance
  sweep also flagged a bare rule reference in the landing record of
  `docs/tickets/done/workbench-facts-tables.md`, which was itself prose
  describing an earlier one — each round that quotes the offending text
  reproduces it. Reworded there rather than quoted again here.
- Coverage lock, construction count, `english-v2-coverage.lock`: untouched —
  this round adds tests only, no core, lowering, grammar, or plugin data
  change. `plugins/wizards` regenerated (30,688 cards) and the four
  `build.rs` files touched.

### Gate artifacts

- `cargo test -p deckmaste_core -p deckmaste_lowering` — `ok. 50/738/3/4/0/0
  passed; 0 failed; 0 ignored` across its six binaries.
- `cargo test -p deckmaste_engine` — every binary `ok`, `0 failed`; the new
  `region_witnesses` binary reads `13 passed; 0 failed; 6 ignored`. Two other
  binaries carry 1 pre-existing ignored test each.
- `cargo test -p deckmaste_plugin` — every binary `ok`, `0 failed`, `0
  ignored` (so the wizards corpus sweep ran rather than being cfg-skipped).
- `cargo clippy --workspace --all-targets` — no warning from any file in this
  diff; the only workspace warnings left are the 9 pre-existing
  `unexpected cfg condition value: parser-metrics` in `deckmaste_construction`
  (another session's area, disclosed in the brief).
- `cargo xtask idris-check plugins/canon --differential` — `68 agreed sound, 0
  agreed unsound, 12 skipped (no certifier verdict); differential OK: 0
  disagreements`.
- `jj diff --git | cargo xtask cite audit --diff` — 23 sites, each rule read
  against its claim. Two were rewritten as a result: `[CR#118.6]` says
  attempting to CAST an unpayable cost is legal (only paying it is not), so
  the module doc now says the cast never completes and cites `[CR#601.2h]`
  for it rather than claiming the card "can't be cast at all"; and Painful
  Quandary's "the actor is the caster" moved from `[CR#601.2i]` (when
  cast-triggers fire) to `[CR#601.2a]` (the caster becomes the spell's
  controller).

### Per-card outcome

| Card | Asserted, or the blocker |
|---|---|
| Angel of Finality | whole-zone selection + trigger-relative target: the whole target graveyard exiles, the other player's is untouched |
| Arbiter of Woe | per-player choice: both opponents discard from their own hand and lose 2; the controller draws and gains 2 |
| Blasphemous Edict | per-player choice at scale: each player sacrifices thirteen of their OWN creatures |
| Bloodtithe Collector | intervening-if history read: silent with no prior life loss, drains every opponent's hand after one |
| Burglar Rat | per-player choice: each of two opponents discards from their own hand; the controller discards nothing |
| Deadly Brew | IGNORED — no way to name the group an enclosed per-player region chose ("this way") |
| Duress | target-relative filter with split roles: the caster chooses, from the target's noncreature nonlands only |
| Liliana, Dreadhorde General | per-player choice behind a loyalty ability: each player sacrifices two of their own |
| Painful Quandary | IGNORED — a decision-bearing cost inside a body's `Pay` declares no register; `validate` refuses the card |
| Perforating Artist | IGNORED — a semantic `Cost` is a conjunction, so "sacrifices … or discards a card" has no spelling |
| Pilfer | target-relative filter: the offered set is exactly the target's three nonland cards, land excluded |
| River's Rebuke | target-relative filter over a group move: the target's nonlands bounce, their land and everyone else's board stay |
| Tribute to Hunger | target-relative sacrifice pool + the LKI read: life gained equals the departed creature's toughness |
| Tinybones, Bauble Burglar | per-player choice behind an activated ability: each opponent discards, the activator does not |
| Fiery Annihilation | IGNORED — target slots are enumerated before any is announced, so the Equipment slot is offered an empty set |
| Run Away Together | IGNORED — same seam, negated: the illegal same-controller pair is accepted |
| Steel Hellkite | cross-reference: announced X, the source, and the candidate's controller in one per-candidate history predicate; two near misses survive |
| Trygon Predator | trigger-relative target: only the damaged player's artifact and enchantment are offered |
| Predator Ooze | IGNORED — a damage fact's recipient is a bare id with no LKI snapshot, so it cannot be matched once the creature dies |

### Deviations and additions

- **Printed mana costs are replaced by `{0}`** in every fixture except Steel
  Hellkite's `{X}`, which its body reads. A card with NO mana cost has an
  unpayable cost and never finishes casting ([CR#202.1b,118.6,601.2h]), so
  some cost is required; which one is announcement-stage business, not this
  ticket's. Recorded in the module doc.
- **Cards whose witnessed ability is one of several** are spelled with that
  ability alone, and the omission is named in the test's doc comment:
  Liliana's −4 (not her other three), Tinybones's activated discard (not its
  stash trigger or play permission), Arbiter of Woe's ETB (not its additional
  cost), Blasphemous Edict's effect (not its alternative cost), Fiery
  Annihilation's two targeting sentences (not its death replacement).
- **Two fixtures were re-spelled around an engine limitation, not around the
  witnessed behaviour.** `Ref(ControllerOf(…))` inside a predicate trips a
  `debug_assert!(false)` in the frameless matcher, so Run Away Together's
  "different players" and Steel Hellkite's "whose controller was dealt
  damage" both use `Controls(Ref(…))` instead — an equally faithful reading
  ("the controller is a player who controls X"). Ticketed below; Steel
  Hellkite passes on the alternative spelling.
- **Scaffolding cards added** (all `{0}`, all named `Witness …` so they
  cannot collide with a real card): a 2/2 vanilla, a 1/4 vanilla, an
  artifact, an enchantment, two mana-valued artifacts, an Equipment, a
  destroy instant, a drain sorcery, a no-op cantrip. Each exists to make a
  witness's control case observable.
- **`corpus_identity.rs` gained an eight-line module-doc note** saying where
  the witnesses went, so the deletion is not a silent hole.
- **Zone-change reminting** ([CR#400.7]) means a destination is asserted by
  card name, never by the id the object had before it moved; the helper says
  so. Three assertions were written the wrong way first and corrected.

### STOPs

None. No ticket-vs-ruling contradiction arose: the ticket's own instruction
covers the six blocked cards ("`#[ignore]` with the blocking gap named, never
a passing test over a different ability"), and that is what they carry.

### Ledger residue routed

Six blockers were found by these fixtures; each is now a live ticket, and each
names the fixture that un-ignores when it closes:

- `critical/engine-cross-target-announce-enumeration` — Fiery Annihilation,
  Run Away Together.
- `critical/engine-damage-fact-participant-lki` — Predator Ooze.
- `critical/lowering-pay-cost-decision-registers` — Painful Quandary.
- `maybe/semantics-disjunctive-cost` — Perforating Artist; deferred to successor
  lowering on 2026-09-06, with the witness obligation retained.
- `maybe/semantics-produced-group-across-regions` — Deadly Brew; deferred to
  successor lowering on 2026-09-06, with the witness obligation retained.
- `planned/engine-derived-reference-in-filters` — the `debug_assert!(false)`
  the two re-spellings avoided; blocks nothing today.
