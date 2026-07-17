---
needs: []
---
Pin down the per-verb facet contract of the `Act` composite lane. SHIPPED: the
dual-facet `Composite { name, body }` now gives each verb its own implicit
semantics under one honest contract, and the six correctness holes the
2026-07-16 post-integration review found are closed (item 3 by a surface
reshape rather than the validator the ticket first sketched; item 6 by the
`Batch` count channel).

What landed, item by item:

1. **Post-commit verbs are no longer falsely replaceable.** `Act` carries a
   `committed: bool` phase marker (the mirror of `ZoneChange`'s
   `snapshot: Option`); `replace_registry::replaceable()` gates the FUTURE
   `committed: false` window as the one guardable/replaceable moment and
   refuses the committed PAST fact — so a mill/scry replacement can no longer
   fire "in addition" after the cards already moved. `FinalizeAct` records the
   committed fact observationally, only when the verb's characteristic change
   actually lands (redirect- and suppression-safe).

2. **Degenerate destroy fizzles.** A composite whose `move_from` is `None`
   (patient gone, zoneless, or already in the destination) or whose
   `from == to` returns no work — no body run, no `Act` ([CR#701.8a]).

3. **Atom↔body coherence — resolved by reshape, not a validator.** The nested
   `Act(Verb(..))` effect/pattern spelling was retired: `Action::Composite` is
   a struct variant `{ name, body }`, and each verb reads its own body facets
   at dispatch, keying its cause on the verb (not the move shape). Pattern
   twins are 8 hand-authored bare-verb `filter/` macros (`Destroy(pred)`,
   `Discard(who, what)`, `Mill/Draw/Scry/Surveil/Fateseal(who)`,
   `Fight(a, b)`) expanding to the `EventFilter::Act { verb, who, on, cause }`
   master form. No load-time `Composite` validator was built — the reshape plus
   the observational `FinalizeAct` (which won't commit a fact the body didn't
   realize) removes the silent-wrong-fact class without one.

4. **`Act(Mill)` no longer outlives a fully-suppressed batch.** Slice verbs
   ride the `Batch` aggregate (`Batch(Param(0), Act(Composite(..)))`) with
   per-contained-future replaceability; `BatchActHead.from = Some(Library)`
   gives Mill a source-scoped `CantHappen(from: Library)` that suppresses the
   WHOLE aggregate (fact included), while a destination-scoped Instead (Rest in
   Peace) bites only the per-card contained futures — tested in both directions
   ([CR#701.17c]).

5. **Draw/mill with an unresolvable `who` fizzles before emitting** — no
   phantom "a player drew" facts.

6. **Count channel for multi-draw/mill replacement — present.** Draw/mill are
   `Batch(Param(0), Act(..))` carrying `batch: Some(n)`, surfaced on the fact
   as `amount`, so a count-doubling replacement (Bruvac / Alhammarret's
   Archive) bites the aggregate before any card moves ([CR#121.2a]).

Beyond the six items, this feature also shipped, as one contract:

- **Lane-split matching** — the trigger/history lanes match the finalized
  name-fact (verb + who/on + cause); the Replacement lane ADDITIONALLY requires
  the event's realized zone facets to still match the verb's canonical shape (a
  divergence-only auto-guard sourced from the entailment table), giving stacked
  madness / Leyline-first its "applies once" behavior with zero authored clause
  ([CR#616.1,616.1f]).
- **Madness end-to-end** — `Cast(Reference, Option<Cost>)` alt-cost facet,
  unified `Delayed`/`Reflexive`, `Madness.ron`, and the full [CR#702.35a]
  matrix (Megrim under redirect, owner-chosen Leyline/madness order, stacked
  madness once, declined-cast graveyard move, alt-cost `{1}{R}` payment).
- **Conferred-ability gathering (Task 10b + follow-up)** —
  `derive::derived_abilities_of` folds two tails into the printed spine for the
  replacement gather (all three sweeps) and the trigger scan: predicate-scoped
  `ConferralRule` abilities, and (follow-up) the [CR#613] LAYER view's granted
  abilities beyond the printed base. The layer-tail fold is what makes
  Falkenrath Gorger's PRINTED static ("Each Vampire creature card you own that
  isn't on the battlefield has madness") actually function: the layer view
  scopes it correctly (active only while Gorger is on the battlefield; reaching
  only its controller's own off-battlefield Vampires via `Owner(Ref(You))`), so
  the granted madness self-replacement opens the [CR#616.1] discard window on
  exactly those cards. The card's own static is now the SOLE source (the earlier
  scope-less companion `ConferralRule`, which would have leaked madness to every
  off-battlefield Vampire in the game, was removed); its `SelectAll` scope was
  also corrected from the permanent-implying `Creature` filter macro to the
  zone-agnostic `Type("Creature")` atom (the macro's `∧ Permanent` gate
  contradicted `Not(InZone(Battlefield))`, so the grant had matched nothing).
- **Canon pair + idris parity** — Falkenrath Gorger and Anje's Ravager
  authored with render fidelity; `KeywordActionSpec` re-typed entity-keyed and
  `EventFilter::Act` given its `EventKind` lowering (see `idris-act-parity`).

Residual, tracked elsewhere (not regressions of this contract):
`engine-act-fight-patient` (the `Fight` fact still carries only the first
combatant, so a narrowed/second-slot fight pattern can't fire — an open design
fork); `idris-act-parity` item 1 (a `Composite` draw paired with `ThatMany`
still fails the idris typecheck — latent until a canon card needs it).
