---
needs: [english-v4-free-choice-any]
---
# One analysis for *another*, *a further*, *an additional* (R5.3)

From `english-v4-child-name-control-and-category-fission-audit`, Finding G
(the T list). The owner ruled R5.3 in round 9.

## Owner's ruling (round 9, R5.3)

"Approve as a follow-up ticket: unify SingularOrQuantified and
IndefiniteArticle (CGEL Ch. 5 §3.4, p. 353), preserving `another day` without
admitting `*a day`."

## Evidence

CGEL's class (Ch. 5 §3.4, p. 353) holds *another*, *a further* and *an
additional*. The grammar splits it across two DeterminativeDistribution
values:
- `SingularOrQuantified`: *another*, read by SingularOrQuantifiedDetermination
  (`english.ron:796`);
- `IndefiniteArticle`: *a*, read by DeterminedModifiedQuantity (`:798`).

AgreeingModifiedQuantity (`:799`) excludes *a* only to avoid a duplicate tree.
A Closed Word fold would lose *another day* (TemporalLocation; CGEL, Ch. 8 §6,
p. 698).

## Change

1. Give the class one analysis: one value, or a shared Production, whichever
   states CGEL's class directly. Remove the duplicate-tree Exclude on
   AgreeingModifiedQuantity if the unified analysis no longer needs it.
2. *another day* keeps its TemporalLocation Reading, and *a day* gets no
   TemporalLocation Reading. Pin both with judgments
   on real card text: a `+` entry for *another day*, and a `!` assertion on a
   card whose text puts *a day* where the wrong analysis could apply. If no
   card has one, there is no entry (judgments README, "What a negative entry
   may claim").
3. DeterminedModifiedQuantity also carries a FreeChoice Exclude. That Exclude
   is settled first, by `english-v4-gate-review` and
   `english-v4-free-choice-any`.

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

## Resolution (owner ruling, round 11)

Unification contradicted by its own source; Option A cleanup landed instead.
CGEL puts bare *a* in set (a), which selects a singular head (Ch. 5 §3.4,
p. 353, [67]); set (c) ([69–70]) holds *another* and the sequences *a
further*, *an additional*, *a good*. *A* and *another* part twice: only
*another* takes a bare quantified plural (*another three bodies*, never *a
three creatures*), and only *another* determines a temporal location Noun
Phrase (*another day*, never *a day*; Ch. 8 §6.3, p. 698). One value would
admit one of these or lose the other, and no shared Production can read both.
So both values stay as justified single holders, documented in `english.ron`
and `schema.rs`, and TemporalLocation's IndefiniteArticle Exclude joins the
kept gates (`docs/decisions/english-v4.md` §4.4, item 10). Option A:
DeterminedModifiedQuantity now requires a plural QuantityNumber, so it reads
only the set (c) case of a singular determinative over a quantified plural, and
AgreeingModifiedQuantity's duplicate-tree IndefiniteArticle Exclude is deleted
(it also ends the second tree of *another additional one card*). No card text
has a temporal *another* or *a* Noun Phrase, so there is no judgment entry.
Owner-directed rider (round 11, item 4): FloatedEachSubjectGap's
`Exclude("head.Inflection", "Irrealis")` is deleted; its plural Requirement
already keeps out the singular-only irrealis *were* (CGEL, Ch. 3 §1.5,
pp. 85–88).

Census delta (measured on ootlmplv → kslysolp): faces read 28,801 → 28,801, Readings 245,781 → 245,781, Surplus 106,817 → 106,817; lost faces: none.
Judgment set: all entries hold.
