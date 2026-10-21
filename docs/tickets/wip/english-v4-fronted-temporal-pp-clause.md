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

## *At* measurement (`snxspnqn`)

Measured by deleting `PreposedPrepositionAndTemporal` from the fixed grammar
(scratch only) and comparing the full census. After the fix, 30 units still
use it, 483 Readings by product (159 Surplus), every one with a Noun Phrase
Complement:
- *At …, this turn*, the PP complete: 5 units, 256 Readings (35 Surplus):
  Full Throttle (*At the beginning of each combat this turn,*), Feral
  Encounter, Vivien's Stampede, Pygmy Hippo (*at the beginning of your next
  main phase this turn,*) and Paradox Haze (*At the beginning of enchanted
  player's first upkeep each turn,*). Each also keeps the Reading with the
  temporal inside the PP.
- *After the second main phase this turn,* (World at War): 2 Readings.
- A genitive split, the PP ending on a possessor and the temporal taking its
  head noun (*[At this turn's] [next end of combat],*; *[During target
  player's] [next turn],*): 15 units, 157 Readings (99 Surplus). Six are *at*
  (Triton Tactics 50, Gaze of the Gorgon, Venomous Breath, Glyph of Doom,
  Faramir, Suppress); nine are *during*, *until* (Taunt, Gideon Jura, Rowan
  Kenrith, Imaginary Threats, Oracle en-Vec, Teferi's Reproach, Firestorm
  Phoenix, Casey & Raph, Nuka-Nuke Launcher).
- *For each … that entered this turn,*, the temporal lifted out of the
  relative clause: 8 units, 63 Readings (Hurkyl, Renewed Solidarity, Living
  Laser, Redoubled Stormsinger, Ocelot Pride, Murmuration, Giant Albatross,
  Tobias).
- North Star (*For one spell this turn,*), the endorsed use: 5 Readings.

Reported to the owner; the fix is not extended to them.

Standard constraints apply.

1. **Census delta** (`snxspnqn`, full corpus): faces read 28,709 → 28,709;
   total Readings 327,929 → 320,502 and Surplus 175,304 → 171,425 (908 units
   fewer, none more). Lost faces: none.
2. **Judgment set** (`snxspnqn`): 2,978 enforced, 195 pending; all hold.
