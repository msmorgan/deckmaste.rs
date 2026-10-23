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

The missing rule was adjective premodification inside NounPremodifierConjunct:
it previously admitted only a noun premodifier and its negative noun
premodifiers. Added an Adjunct schema for an attributive AdjectivePhrase
without a post-head Complement, following OSG §7, "Card types are common nouns
and attributive modifiers" and "Supertypes are lowercase modifiers". The
existing noun-premodifier Coordination now builds the requested serial
coordination. No engine or ranking changes were needed.

Added two positive judgments in `coordinated_noun_premodifiers.txt`: the short
nominal and Acclaimed Contender's full paragraph. Both assert the coordinated
Modifier of *card* and *legendary* as Modifier of *artifact* in one Reading.
No tests or judgments were rewritten or retired.

Exactly four units changed Derivation counts; every verdict remains Read:

| Face | Kind | Index | Before | After |
| --- | --- | ---: | ---: | ---: |
| Acclaimed Contender | Line | 0 | 95 | 135 |
| Definitely Not a Turtle | Line | 0 | 95 | 135 |
| Kamahl's Druidic Vow | Line | 0 | 14 | 23 |
| Lara Croft, Tomb Raider | Line | 1 | 210 | 294 |

The largest growth factor is 23/14 (about 1.64); no pathological growth.
Cheapest tiers are unchanged: singleton in 15,624 of 18,344 ambiguous units,
summed 21,723, largest 8. S/A/P/F is 227/21/9/91 → 227/21/9/93, measured on
single-group positive non-seed entries with 2–4,096 Readings and at least one
unendorsed Reading; the two new entries contribute F, which is not a gate.

Validation: `cargo xtask english-v4 strata --check`,
`cargo test -p deckmaste_english_v4`, and
`cargo clippy -p deckmaste_english_v4 --tests -- -D warnings` pass.
Citation checks report 0 noncompliant strings and 0 stale citations; the diff
audit selects 0 citation sites. No CR citation was added or changed.

Census delta (measured on `mylvrmvz` → `qslysprt`): faces read 28,801 → 28,801; total Readings 245,781 → 245,954; Surplus 106,817 → 106,844; lost faces: none.
Judgment set (`qslysprt`): holds; 3,054 enforced, 206 pending; both new positive assertions hold.
