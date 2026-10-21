---
needs: []
---
# A trigger's temporal NP stays inside its clause, not beside the trigger's comma

`PreposedPrepositionAndTemporal` (`english.ron:728`; *For one spell this turn,
…*) lets a fronted Preposition Phrase and a temporal Noun Phrase share one
comma. When the Preposition Phrase is a trigger whose complement is a Clause,
this gives every trailing *this turn* a second analysis:
- `[When you next cast a spell] [this turn], copy …`;
- `[When you next cast a spell this turn], copy …`.

The trigger condition runs to its comma: `docs/oracle-style-guide.md`
checklist item 4 and §8, Triggered abilities; [CR#603.1].

## Owner's ruling

Add the existing equation `Require("adjunct.ComplementKind", "NP")` to
`PreposedPrepositionAndTemporal`. It then matches *For one spell this turn,*
and not *When <Clause> this turn,*.

**No new features in `english.ron`.** If the fix cannot meet that, STOP and
return to the owner.

## Caveat 3 (owner-approved): *at* triggers

The equation still admits *At the beginning of …* triggers, whose
Preposition Phrase has an NP complement.
- Measure whether fronting *At …, this turn* is spurious in any corpus unit.
- If it is, report it to the owner rather than extending the fix.
- Do not name *at* in the grammar.

## Expected reductions

About 2,100 Surplus (4,822 Readings by product) over 425 units, with no unit
emptied:
- Magus Lucea Kane 604;
- Roving Actuator 84, Brass Infiniscope 72, Zevlor 68, Mindlink Mech 60;
- also Codie, Vociferous Codex; Thunderclap Drake; Loki Laufeyson; Saruman of
  Many Colors; Whiskervale Forerunner.

The simulation also gated *At* triggers, so the *at* share, if any, comes out
of this figure.

## Done when

- No Reading uses `PreposedPrepositionAndTemporal` with a Clause-complemented
  Preposition Phrase.
- *For one spell this turn, …* keeps its Reading.
- Add a judgment for Magus Lucea Kane's *When you next cast a spell … this
  turn, copy that spell or ability.*, with *this turn* inside the *when*
  Clause.
- The *at* measurement is recorded here.
- No new feature in `english.ron`.
- 0 faces lost, and no unit gains a Reading.

Standard constraints apply.

1. **Census delta**: measured at landing.
2. **Judgment set**: measured at landing.
