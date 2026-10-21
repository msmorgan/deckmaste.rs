---
needs: []
---
# Comma-set post-adjuncts are Clause Supplements, not Verb Phrase modifiers

`english.ron:1278` declares
`Adjunct(host: "VerbPhrase", adjunct: "PrepositionPhrase", side: After,
host_kind: Predicate, delimiter: Some(','))`. It lets a trailing *, except …*,
*, if …*, *, unless …* or *, where …* attach to any Verb Phrase on the right
spine, including one buried inside a nominal:
- Mockingbird's *mana spent to cast this creature, except it's a Bird …*
  attaches *except* to *spend* or *cast*;
- Spark Double's *a creature or planeswalker you control, except it enters …*
  attaches it to *control* inside the bare relative clause.

## Owner's rulings

**Round 1:** comma-set adjuncts (*except*, *if*, *unless*, *where*,
*including*, *then*) are clause-level adjuncts or supplements. They cannot
close any of the following:
- a nominal modifier;
- a relative clause;
- a non-finite clause (*to cast this creature, except …*);
- an NP complement or object.

That ruling rests on:
- `docs/oracle-style-guide.md` §3, Commas and the serial comma;
- `docs/oracle-style-guide.md` §12, Copies: "Keep all copy exceptions inside
  the same sentence" [CR#707.9];
- CGEL, Ch. 15 §5.1, p. 1351 (supplements and their anchors);
- CGEL, Ch. 12 §4, p. 1058 (an integrated relative is prosodically bound to its
  antecedent).

**Round 2, superseding the method:** fix it structurally, not with a policing
feature.
- The comma-delimited post-adjuncts are finite Clause Supplements, not
  Verb Phrase or Relative Clause modifiers.
- Move them off `VerbPhrase`, and keep comma-fronted adjuncts on `Clause`
  above `RelativeClause`.
- **No new features in `english.ron`:**
  - no `Delimited` or `Fronted` feature;
  - no `Exclude` threaded through `NominalPostmodifier`, `RelativeClause` or
    `ToInf`.
  - If the fix cannot meet that, STOP and return to the owner.

## Change

- Delete the comma-delimited `VerbPhrase` post-adjunct Schema (line 1278).
- Let comma-delimited post-adjuncts attach to a finite `Clause`.
  - The `Clause` Supplement Schema at line 1282 already does this for
    *where*.
  - Use existing features (`VerbForm`, `ClauseMarking`), so a *to*-infinitival
    `Clause` and the Clause inside a relative clause cannot host one.
- Keep the comma-fronted Schemas on `Clause` (lines 1279 and 1286).

### Caveat 1 (owner-approved): the structural fix is a hypothesis

The `VerbPhrase` Schema may also carry legitimate comma adjuncts:
- on a non-finite Verb Phrase that is a clause's predicate;
- on one conjunct of a coordinated Verb Phrase (*… , then …*).

Before landing:
- measure which units read only through it;
- move those uses to a Clause host or an existing production.

If any face would be lost, STOP.

### First: Stalwart Successor's missing analysis

*Whenever one or more counters are put on a creature you control, if it's the
first time counters have been put on that creature this turn, put a +1/+1
counter on that creature.*

- Today all 5 Readings are wrong. Each one makes *if it's the first* a
  comma-set adjunct of *control* inside the bare relative.
- The intended analysis is missing: the *if*-clause as a preposed adjunct of
  *put* [CR#603.4].
  - *If it's the first time counters have been put on that creature this turn,
    put …* has 0 Derivations.
  - *… this ability has resolved this turn* has 5.
  - So it is the relative-time nominal *the first time* with a passive-perfect
    relative clause that fails.
- Supply that analysis first, with no new features. Otherwise this ticket
  empties the unit, against the owner's requirement of 0 faces lost.

## Out of scope

Comma-set adjuncts inside a finite Clause complement of a preposition, such
as the intervening *if* inside a *whenever* Clause (`trigger_clauses.txt:6`;
[CR#603.4]). That case needs its own ruling.

## Expected reductions

From the cost study's gate simulation, converted to Surplus per Segment:
- **Nominal-internal and NP-complement cases:** about 32,500 Surplus over 185
  units.
  - Mockingbird 20,979, Spark Double 4,911, Rootha 792, Activated Sleeper 565,
    Agility Bobblehead 515.
- **Non-finite clauses:** about 15,400 over 5 units.
  - Mockingbird 15,437.
  - *This ability costs {X} less to activate, where X is …*: Belt of Giant
    Strength, Baru, Survey Mechan, Agatha of the Vile Cauldron.
- The two overlap in Mockingbird.
- These figures assume the relative-clause case too. That half is
  `english-v4-bare-relative-fronted-adjunct`, which this ticket does not fix.

## Done when

- No Reading has a comma-set post-adjunct hosted by a Verb Phrase, or by a
  non-finite or relative Clause.
- These judgments still hold:
  - every entry in `enter_as_copy_except_clause.txt` and
    `except_matrix_complements.txt`;
  - every entry in `where_variable_clause.txt`.
  - Their *as Adjunct of enter* / *of block* assertions name the head of the
    finite Clause host. Rewrite any assertion that the Clause host changes,
    and say which.
- Add judgments:
  - Mockingbird, with no Reading attaching *except* to *spend* or *cast*;
  - Spark Double, with no Reading attaching *except* to *control*;
  - Stalwart Successor, with *if it's the first time …* as Adjunct of *put*.
- No new feature in `english.ron`.
- 0 faces lost, and no unit gains a Reading.

Standard constraints apply.

1. **Census delta**: measured at landing.
2. **Judgment set**: measured at landing.
