---
needs: []
---
# A comma-set adjunct cannot close a nominal-internal or non-finite constituent

The delimited (comma-set) adjunct schemas (`Adjunct(..., delimiter: Some(','))`)
let a trailing *, except …*, *, if …*, *, unless …*, *, where …*,
*, including …* or *, then …* attach to any Verb Phrase on the right spine,
including one buried inside a nominal. Mockingbird's *mana spent to cast this
creature, except it's a Bird …* attaches *except* to *spend* or *cast*; Spark
Double's *a creature or planeswalker you control, except it enters …*
attaches it to *control* inside the bare relative clause.

## Owner's ruling

Comma-set adjuncts (*except*, *if*, *unless*, *where*, *including*, *then*)
are clause-level adjuncts or supplements. None can close any of the following:
- a nominal modifier;
- a relative clause;
- a non-finite clause (*to cast this creature, except …*);
- an NP complement or object.

The ruling rests on:
- `docs/oracle-style-guide.md` §3, Commas and the serial comma;
- `docs/oracle-style-guide.md` §12, Copies: "Keep all copy exceptions inside
  the same sentence" [CR#707.9];
- CGEL, Ch. 15 §5.1, p. 1351: supplements and their anchors;
- CGEL, Ch. 12 §4, p. 1058: an integrated relative is prosodically bound to
  its antecedent.

## Change

- Mark structurally a constituent that ends in a delimited adjunct, for example
  a Summary bit like `final_preposition`. Productions must not accept such a
  constituent as:
  - a Nominal postmodifier (participial, relative, prepositional or
    adjectival);
  - a relative clause;
  - a non-finite (`ToInf`, `Gerund`, `BareInf`, participial) adjunct or
    complement clause;
  - an NP complement of a preposition, or an object.
- Identify delimited adjuncts by their schema's delimiter, never by a list of
  words in the grammar.
- Out of scope: a finite Clause complement of a preposition (*except [it's a
  Bird … and it has flying]*, *whenever [X], if …*). The cost simulation also
  closed off the trigger-clause case (`trigger_clauses.txt:6`, the
  intervening *if* [CR#603.4]); that extension needs its own ruling.

## The unit that would lose all Readings: Stalwart Successor

*Whenever one or more counters are put on a creature you control, if it's the
first time counters have been put on that creature this turn, put a +1/+1
counter on that creature.*

- Today all 5 Readings are wrong. Each one makes *if it's the first* a
  comma-set adjunct of *control* inside the bare relative.
- The intended analysis is missing: the *if*-clause is a preposed adjunct of
  *put*, after the trigger [CR#603.4].
  - *If it's the first time counters have been put on that creature this turn,
    put …* has 0 Derivations.
  - *… this ability has resolved this turn* has 5.
  - So the relative-time nominal *the first time* + a passive-perfect relative
    clause is what fails.
- The owner requires that this unit keep its intended, higher attachment with
  0 faces lost. Supply that analysis first, in this ticket if it is small,
  otherwise as a prerequisite. Then land the island.

## Expected reductions

From the cost study's gate simulation, converted to Surplus per Segment:
- **Nominal-internal and NP-complement cases:** about 32,500 Surplus over 185
  units.
  - Mockingbird 20,979, Spark Double 4,911, Rootha 792, Activated Sleeper 565,
    Agility Bobblehead 515.
  - Also Rionya, Fire Dancer and Olag, Ludevic's Hubris.
- **Non-finite adjunct clauses:** about 15,400 Surplus over 5 units.
  - Mockingbird 15,437.
  - *This ability costs {X} less to activate, where X is …*: Belt of Giant
    Strength, Baru, Survey Mechan, Agatha of the Vile Cauldron.
- The two overlap in Mockingbird. Together with ruling #4 they take
  Mockingbird from 36,709 Readings to about 2,900, and Spark Double from 5,691
  to about 80.

## Done when

- No Reading has a comma-set adjunct ending any constituent listed in the
  ruling.
- These judgments still hold:
  - every entry in `enter_as_copy_except_clause.txt` and
    `except_matrix_complements.txt`;
  - the *where*-clause entries in `where_variable_clause.txt`.
- Add judgments:
  - Mockingbird with *except* as Adjunct of *enter*, and no Reading with
    *except* as Adjunct of *spend* or *cast*;
  - Spark Double, with no Reading with *except* as Adjunct of *control*;
  - Stalwart Successor, with *if it's the first time …* as Adjunct of *put*.
- 0 faces lost, and no unit gains a Reading.

Standard constraints apply.

1. **Census delta**: measured at landing.
2. **Judgment set**: measured at landing.
