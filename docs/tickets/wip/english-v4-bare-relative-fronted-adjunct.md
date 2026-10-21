---
needs: []
---
# A bare relative clause opens with its subject

`BareRelative` (`english.ron:1150`) accepts any finite Clause with an object
Gap, including one that begins with a preposed adjunct. In Spark Double (*…
with an additional +1/+1 counter on it if it's a creature, it enters with an
additional loyalty counter on it …*), *counter* gets a bare relative *[if
it's a creature, it enters _]*: the fronted *if*-clause opens the relative,
and transitive *enter* supplies the object Gap.

## Owner's ruling

A bare relative clause (no *that*, *which* or *who*) cannot open with a
preposed adjunct before its subject. CGEL: "Bare relatives always have the
subject in initial position" (Ch. 12 §3.5.5, p. 1055).

The round-1 ruling cited §3.2.2. That section is the *the result of which*
percolation type, on p. 1040; the passage is §3.5.5.

**No new features in `english.ron`.** If the fix cannot meet that, STOP and
return to the owner.

## Caveat 2 (owner-approved): this does not fall out of the comma-adjunct fix

A bare relative contains a Clause, and comma-fronted adjuncts stay on
`Clause` under `english-v4-comma-adjunct-island`. So that ticket does not fix
this one, and neither ticket may claim it does.

## Change

Use one of these:
- an equation on an existing feature that marks the relative's Clause as
  subject-initial;
- a distinct category for the relative's clause, whose productions are only
  the subject-predicate ones.

The fronted productions (`FrontAdverbial`, `Adjunct:Clause+…/Before/…`,
`PreposedTemporalAdjunct`, `PreposedPrepositionAndTemporal`) then never head a
bare relative's Clause.
- Name no word, and add no feature.
- *That* and *wh* relatives are unaffected. CGEL's *that only the day before I
  had sent* keeps its Reading.

## Expected reductions

About 4,960 Surplus over 2 units: Spark Double 4,956 and Mirrorwood
Treefolk 4. No unit is emptied.

## Done when

- No Reading has a bare relative whose Clause begins with a preposed adjunct.
- Add a judgment for Spark Double, with no RelativeClause beginning *if it's a
  creature*.
- No new feature in `english.ron`.
- 0 faces lost, and no unit gains a Reading.

Standard constraints apply.

1. **Census delta**: measured at landing.
2. **Judgment set**: measured at landing.
