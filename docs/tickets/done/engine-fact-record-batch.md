---
needs: [core-eventfilter-master-forms]
---
**Fact-record growth: amounts, before/after totals, batch ids, and fact-backed
product groups; the written `Simultaneous` semantics.** This is engine work
wearing grammar clothes — sagas, "this way" anaphora, and batch application
all need fact-model changes. The fixtures are named NOW so the grammar tickets
that consume these facts cannot silently "complete" without the engine half.

Related: [[engine-sagas]], [[core-saga-chapters]] (chapter abilities consume
the before/after fields minted here), [[damage-provenance]].

## Fact-record fields

- `CounterPlaced` occurrences carry `amount`, `before`, `after` totals, so
  `Condition::Crossed { value, threshold }` reads "was less than N and became
  at least N" verbatim [CR#714.2b]. Walk-through fixture: a
  counter-doubling 0→2 lore jump from ONE batch fact — chapter I
  (`Crossed(AtLeast(1))`: before 0 < 1 ≤ 2 after) fires, chapter II fires,
  chapter III stays silent.
- **Batch ids:** simultaneous member facts share a batch id; a batch is ONE
  `Occurrence` for `OneOrMore` triggers [CR#603.3b].
- The event-amount channel (today `game_event_amount`) extends to every
  amount-bearing fact kind (damage, life gained/lost, draws, counters) — an
  amount read against a fact that carries none is unreachable because the caps
  gate already rejected it at load.

## `Simultaneous([Effect])` — the written spec

- **One snapshot:** all member effects read game state as of one
  pre-application view.
- **One timestamp, one batch:** application emits member facts sharing a batch
  id.
- **Replacements apply per member fact independently** (each event is
  replaceable on its own; ordering per the affected object/player's controller
  as usual [CR#616.1]); replacing one member does not unapply the others.
- **SBAs run after the whole batch**, never between members.
- **`Fight` is NOT `Simultaneous` sugar.** It stays a primitive verb with
  native semantics [CR#701.14a..701.14d]: both-or-neither legality
  [CR#701.14b], self-fight dealing its power twice to itself [CR#701.14c],
  non-combat damage [CR#701.14d]. Fight is its own CR event family — a cause
  verb replacements can intercept.
- Until this wiring lands, the elaborator restricts `Simultaneous` to the
  exchange-family macros' bodies (a load cap with a fixture, mirroring the
  matcher caps in [[engine-eventfilter-bridge]]).

## Fact-backed product groups

Product antecedents (the things a clause "just did") are backed by ENACTED
FACTS at runtime: the elaborator marks each product antecedent with its
producing clause; the engine populates the group from the facts that clause
actually enacted — never from the gathered input set. "For each nontoken
creature destroyed this way" over a destroy-all that only destroyed some
members (indestructible survivors, replaced destructions) is exactly the set
of that clause's actual destroy-caused `ZoneChange` facts, by construction.

## Done

- Fact record carries `amount`/`before`/`after`/`batch`; batch application
  implemented per the spec above.
- Fixtures green: (a) the 0→2 lore-counter batch walk; (b) an
  exchange-control card through `Simultaneous`; (c) a wither/infect
  interaction — counters applied as part of the batch's damage facts, SBAs
  after the batch; (d) a Blood-Money-shaped "destroyed this way" card whose
  product group excludes an indestructible survivor; (e) a "cards milled this
  way" read.
- The `Simultaneous` load cap active with its reject fixture.

## Verification

- `cargo test -p deckmaste_engine` and `cargo test --workspace` green.
- `cargo xtask validate` clean on hand-authored plugins.
- `cargo xtask cite check` — 0 stale, `--list-noncompliant` empty.
