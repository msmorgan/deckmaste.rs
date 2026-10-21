---
needs: []
---
# *In addition to* is a Predicate adjunct, never a Nominal postmodifier

`prepositions.ron` gives *in* + Nominal (the bare idiom nominal of *in
addition to*, CGEL, Ch. 7 §3.1, p. 620) `hosts: NominalAndPredicate`. So
*in addition to its other types* also postmodifies any Nominal on the right
spine:
- *Bird*, *Phyrexian* or *copy* in *except it's a Bird in addition to its
  other types* (Mockingbird, Activated Sleeper);
- *flying* in token text.

Owner's ruling (round 2, caveat 6): *in addition to* only seemed to need a
ranking preference because its host categories are too broad. This is a frame
correction to `hosts: Predicate`, ticketed as a measured defect fix.

## Corpus check: no legitimate Nominal-hosted use

All 335 occurrences of *in addition* in the corpus attach to a predicate:
- *its other …* or *their other …* (types, colors, costs): 313;
- *paying its other costs* or *paying their other costs*: 18;
- *his other types*: 4;
- one other, Spy Kit's *has all names … in addition to its name*.

The study's gate, which removed every Nominal-hosted *in addition to*,
emptied no unit.

## Change

- The *in* + Nominal frame also carries *cards in hand* (129 occurrences, all
  Nominal-hosted). So changing that frame's hosts would break *cards in hand*.
- Make the correction where *in addition to* is declared, so that *in hand*
  keeps its Nominal host. For example:
  - give *in addition* its own Predicate-hosted frame in `prepositions.ron`,
    as a compound preposition beside *instead of* and *up to*;
  - or split the *in* + Nominal frame by its Complement-taking nominal through
    existing frame data.
- **No new features in `english.ron`**, and no word named in the grammar. If
  the correction cannot be made in the lexicon alone, STOP and return to the
  owner.

## Expected reduction

About 19,150 Surplus (20,260 Readings by product) over 282 units, none
emptied:
- Mockingbird 15,960;
- Activated Sleeper 667;
- Dawnhand Dissident 149;
- Deceptive Frostkite 148;
- The Jolly Balloon Man 144;
- Mindlink Mech 126.

## Done when

- No Reading has *in addition to …* as a Modifier of a Nominal.
- `+PrepositionPhrase in addition to its other types` still holds.
- *cards in hand* keeps its Nominal host.
- Add judgments: Activated Sleeper or Deceptive Frostkite, asserting *in
  addition to its other types* as Adjunct of *be*.
- No new feature in `english.ron`.
- 0 faces lost, and no unit gains a Reading.

Standard constraints apply.

1. **Census delta**: measured at landing.
2. **Judgment set**: measured at landing.
