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

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
