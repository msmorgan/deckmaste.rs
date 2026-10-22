---
needs: []
---
# Acclaimed Contender: the *[Knight, Aura, Equipment, or legendary artifact] card* analysis is missing

The owner approved this ticket in round 9 as a side finding, raised in
`english-v4-audit-rulings-pending`.

## Owner's ruling (round 9, side findings)

"Acclaimed Contender: the [Knight, Aura, Equipment, or legendary artifact]
card analysis is missing from the forest: find which rule keeps it out; adding
it is new coverage."

## The finding

The ranking-residuals work found that the analysis *[Knight, Aura, Equipment,
or legendary artifact] card* is not in Acclaimed Contender's forest. That is a
serial coordination of noun premodifiers, the last of which carries its own
premodifier, under one head *card*. It was one of three units whose Readings rose when serial coordination of unlike
attributes was allowed (`english-v4-attributive-participle-licence`).

## Change

1. Find which rule keeps the analysis out of the chart: the premodifier
   coordination (the Attributes Coordination schema and NounPremodifierConjunct),
   the conjunct Category, or a unary restriction.
2. Add the analysis as new coverage (`docs/decisions/english-v4.md` §4.4,
   item 3). State the addition generally, with a CGEL or
   `docs/oracle-style-guide.md` citation (§7, "Types, subtypes, and
   supertypes as nouns and modifiers").
3. Add a `+` judgment on Acclaimed Contender that asserts the analysis. Other
   analyses may stay beside it, and the analysis need not rank first.

## Acceptance (round 9: `docs/decisions/english-v4.md` §4.4, item 9)

- No face is lost.
- Every judgment's positive assertions hold.
- Nothing ungrammatical is admitted.
- `cargo xtask english-v4 strata --check` passes.
- No pathological forest explosion on real cards.

The landing reports the cheapest-tier figures and the judgment-set S/A/P/F,
but they are not gates (item 7). No Attachment Class, Production Cost tweak or
residual ticket is added to win a rank (item 8).

Standard constraints apply.

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
