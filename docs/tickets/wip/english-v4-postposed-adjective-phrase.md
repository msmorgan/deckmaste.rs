---
needs: []
---
# A postpositive Adjective Phrase postposed out of the Object (Betor, Ancestor's Voice)

Owner ruling (cost-audit, known-wrong unit): Betor, Ancestor's Voice has no
correct Reading. Mint a ticket so that *equal to the amount …* modifies
*number*.

## The defect

Betor, Ancestor's Voice: *At the beginning of your end step, put a number of
+1/+1 counters on up to one other target creature you control equal to the
amount of life you gained this turn.*

- The Adjective Phrase *equal to the amount of life you gained this turn*
  measures the number of counters. It belongs to the Object *a number of +1/+1
  counters*, but it follows the Complement *on up to one other target creature
  you control*.
- Today it attaches only in three ways:
  - as a Modifier of *creature*;
  - inside the Bare Relative Clause, as an Adjunct of *control*;
  - as a Depictive Adjunct of *put*.
- None of these is the intended analysis.
- In the census the unit has 2,444 Readings, and none of them is correct.

The same shape recurs across the corpus. 43 units have *a number of … on/from
… equal to …*, for example:

- Lattice Library;
- Bone Devourer;
- Thought Sponge;
- Olag, Ludevic's Hubris;
- Dakkon, Shadow Slayer;
- Bala Ged Thief and Mire's Toll (*a number of cards from their hand equal to
  …*);
- Smoldering Egg.

Four of these units have no Reading today: Amzu, Swarm's Hunger; Conformer
Shuriken; Willowdusk, Essence Seer; Tyvar the Bellicose.

## Authority

- CGEL, Ch. 5 §14.2, p. 445: an Adjective Phrase with its own post-head
  dependents is postpositive (*members dissatisfied with the board's
  decision*).
- CGEL, Ch. 5 §15, p. 455: clausal post-head dependents are often postposed,
  occupying "a special position within the NP".
- CGEL, Ch. 12 §4, p. 1066 [22ii]: *Kim lent a book to Ed which contained all
  the information he needed*.
  - A dependent of the Object is postposed past the verb's PP Complement.
  - This is most likely when it carries more information than the material it
    crosses.
  - The grammar already models the clausal case as `PostposedRelative`
    (Postposed Relative Clause in `docs/contexts/oracle-english/CONTEXT.md`).
    The heavy postpositive Adjective Phrase here is the same configuration.

## Change

- Admit a heavy postpositive Adjective Phrase that modifies the Object Noun
  Phrase in postposed position, after the verb's selected Complement. This is
  the `PostposedRelative` pattern, applied to `AdjectivePhrase`.
- Restrict it to an Adjective Phrase with its own Complement (`heavy`), out of
  the Object only.
- Name it per the glossary: Postposed, never "extraposed".
- If a new term is needed, add it to `docs/contexts/oracle-english/CONTEXT.md`
  with the `domain-modeling` skill.

## Acceptance set

All figures are from the cost-audit evaluation data and the census, measured on
`loxuknun`.

- **Betor:** gains an endorsed Reading. It was known-wrong in the evaluation,
  where the closest available attachment, the Depictive of *put*, was accepted
  provisionally.
- **The 43-unit family:** each unit gains the postposed Reading.
- **The four unread units:** they read, unless another gap blocks them. In that
  case, record the gap.
- **Expected change:** Readings rise, by about one family of trees per affected
  Segment. This ticket adds the correct analysis. The wrong attachments to
  *creature* and *control* are genuine syntax, and their ranking belongs to the
  cost redesign, not to admission.

## Done when

- Betor has a `+` Judgment pinning `AdjectivePhrase "equal to the amount of
  life you gained this turn"` as Modifier of *number*, and it holds.
- No face is lost.
- Any unit outside the 43-unit family that gains Readings is listed and
  justified.

## Constraint: no new features in `english.ron`

Owner's ruling (round 2): this defect comes from a rule that is too broad, and
it is fixed by tightening or deleting that rule. The fix adds **zero new
features to `english.ron`**. It uses existing features, lexicon or catalog
frames, or an engine mechanism, and names no word in the grammar. If it
cannot be done that way, STOP and return to the owner.

Standard constraints apply.

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
