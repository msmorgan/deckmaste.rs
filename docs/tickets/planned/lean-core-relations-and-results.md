---
needs: [lean-core-bindings-and-composition]
---
# Unify relation queries, numeric folds, and result collections

Replace phrase-specific reads with typed queries over explicit domains,
using the binding and member-binder structures from the prerequisite. Apply
the [campaign contract](lean-core-bindings-and-composition.md#campaign-contract).
These decisions were agreed on 2026-09-06.

## Relations and face state

Share `NounPhrase.possessorOf`, `attachHost`, and `designated` through relation
queries with declared endpoint kinds, directions, and cardinality. Ownership
and control yield players; an attachment host may be an object or player;
designation queries follow the relation declared for that designation.
Macros supply currently implicit source attachments. A selection and the
requirement for a singular result remain separate; do not impose one blanket
plurality rule on all relations.

Fold `Predicate.isAttached`, `attachedBy`, and `attachedTo` through the same
attachment relation. Bare attachment tests existence; a specified counterpart
can impose a quantified selection. Preserve "enchanted by at least two
Auras", both relation directions, and player hosts where the declaration
admits them. An omitted attachment word must not determine direction.

Surrounding constructions control publication of sources and projected
results. Preserve explicit counterpart targets, controller-sacrifice's access
to both controller and object, and attachment-host type evidence through later
movement. Do not make every projection introduce another ambiguous "it".
Endpoint restrictions come from the relation, not from label-specific checks.

Replace `Predicate.isTransformed` with a macro over a generic current-face
predicate. It describes a double-faced permanent with its back face up,
not a history of transformation. The read concerns the whole object, not a
back-face-up component of a merged or melded object [CR#701.27g]. Copying
back-face characteristics alone is insufficient. Keep front/back separate
from the physical face-up/face-down `Status` axis. Mutagen Connoisseur is the
existing bench witness. Retain `turnOver` as the lower face-switching operation
used by the transform macro; this ticket does not invent a historical flag.

## Numeric folds and results

- Make `countOf` a sum of one per member and projected totals/extrema folds
  over property reads from the explicit member binder. Keep selection
  separate from the measure and preserve numeric regimes.
- Represent typed result collections with enough information for individual
  results, coin faces, caller outcomes, ballot alternatives, and stored dice.
  The current single `OutcomeSort` marker cannot express those collections.
- Express `applyResultsTable` as result-based conditional dispatch, preserving
  range validity, the observed result, and each branch's incoming scope.
- Express `Condition.anyResultIs` and `rolledDoubles` as collection queries;
  the latter compares the two results rather than merely requiring a roll
  marker. `flipFace` and `flipCalled` project different information: a coin
  face and a caller's win/loss are not interchangeable, and some flips have
  no winner [CR#705.2]. Express `voteLead` through tally comparison over the
  ballot alternatives, preserving strict versus tied modes. The corresponding
  `coinsShowing`, `greatestStoredMatch`, and `votesFor` reads use the shared
  selection/aggregation vocabulary.
- Keep ignoring selected outcomes as an explicit operation. An ignored roll
  is treated as never having happened, not merely removed from a displayed
  collection [CR#706.6]. Selection of ignored outcomes is reusable.
- Fold `shiftResult` through numeric result updates, retaining natural versus
  modified results and an explicit choice when direction is unspecified.
  Storage retains both die kind and result. Expand `rerollStored` by selecting
  stored entries, rolling their recorded die kinds, and replacing those
  entries, preserving linkage between storage and readback abilities
  [CR#706.8a..706.8c]. Typed storage, rolling, and replacement must actually
  exist before retiring the specialized form.

Do not introduce a five-case `resultOp` bucket. Query macros can disappear;
ignoring, storage, and rolling still need their semantic operations.

## Completion evidence

Re-spell affected relation, aggregate, randomness, and face-state cards and
pins. Include counted attachments, a player attachment host, retained mixed
card-type evidence after movement, result cardinality, tied votes, callerless
flips, and stored dice of differing kinds. Record which specialized forms
were eliminated and which operations remain, then run `lean/scripts/build`.
Standard constraints apply.
