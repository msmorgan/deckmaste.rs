---
needs: []
---
# A trigger's temporal NP stays inside its clause, not beside the trigger's comma

`PreposedPrepositionAndTemporal` (*For one spell this turn, …*) lets a
fronted Preposition Phrase and a temporal Noun Phrase share one comma. When
the Preposition Phrase is a trigger whose complement is a Clause, this
creates a second analysis of every trailing *this turn*:
- `[When you next cast a spell] [this turn], copy …`;
- `[When you next cast a spell this turn], copy …`.

## Owner's ruling

`PreposedPrepositionAndTemporal` must not apply to clause-complemented trigger
Preposition Phrases (*When*, *Whenever*, *At*). There, a trailing temporal
Noun Phrase (*this turn*) belongs inside the trigger clause rather than
sharing the trigger's comma. The trigger condition runs to its comma:
`docs/oracle-style-guide.md` checklist item 4 and §8, Triggered abilities;
[CR#603.1].

## Change

- Restrict the production's Preposition Phrase by the kind of its complement
  (a Clause), never by naming *when* or *whenever*.
- An *at*-trigger takes a Noun Phrase (*At the beginning of …*).
  - Measure it first. If no unit has an *at*-trigger whose following temporal
    NP is split this way, the complement-kind restriction is the whole fix.
  - Otherwise characterize the *at* case structurally, for example by its
    turn-part complement [CR#500.1], again without naming the word.

## Expected reductions

About 2,100 Surplus (4,822 Readings by product) over 425 units, with no unit
emptied in simulation:
- Magus Lucea Kane 604;
- Roving Actuator 84, Brass Infiniscope 72, Zevlor 68, Mindlink Mech 60;
- also Codie, Vociferous Codex; Thunderclap Drake; Loki Laufeyson; Saruman of
  Many Colors; Whiskervale Forerunner.

## Done when

- No Reading uses `PreposedPrepositionAndTemporal` with a Clause-complemented
  Preposition Phrase.
- *For one spell this turn, …*-type units keep their Readings.
- Add a judgment for Magus Lucea Kane's *When you next cast a spell … this
  turn, copy that spell or ability.*, with *this turn* inside the *when*
  Clause.
- 0 faces lost, and no unit gains a Reading.

Standard constraints apply.

1. **Census delta**: measured at landing.
2. **Judgment set**: measured at landing.
