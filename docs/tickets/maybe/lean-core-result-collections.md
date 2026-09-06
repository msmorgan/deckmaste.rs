---
needs: []
---
# Design typed result collections and their operations

Parked by the user on 2026-09-06. The result-table, dice, coin, vote, and stored
result proposals belong together because they need shared collection access.
Existing forms remain until that mechanism earns its complexity. Do not add a
five-case `resultOp` bucket or assume all `OutcomeSort` uses must be replaced.

## Proposed mechanism and consumers

- Represent typed collections with individual results, coin faces, caller
  outcomes, ballot alternatives, and stored die kinds/values. The current
  single result marker cannot supply these collection queries.
- Express `applyResultsTable` as result-based conditional dispatch, preserving
  range validity, the observed result, and each branch's incoming scope.
- Express `Condition.anyResultIs` and `rolledDoubles` as queries over results;
  doubles must compare the two results rather than merely require a roll
  marker. `flipFace` and `flipCalled` project different information: some
  flips have no winner [CR#705.2]. Express `voteLead` through tally comparison
  over ballot alternatives, preserving strict versus tied modes.
  `coinsShowing`, `greatestStoredMatch`, and `votesFor` are related aggregate
  consumers. Identify any actual dependency on a general numeric binder;
  do not impose one just because both proposals involve collections.
- Keep ignoring selected outcomes as a semantic operation. An ignored roll
  is treated as never having happened, not merely filtered from a collection
  [CR#706.6]. Its selection logic can still be shared.
- Assess `shiftResult` as a numeric update, retaining natural versus modified
  results and an explicit choice when direction is unspecified.
- Storage retains die kind and value. A possible `rerollStored` expansion
  selects stored entries, rolls their recorded die kinds, and replaces those
  entries. Preserve linkage between storage and readback abilities
  [CR#706.8a..706.8c]. Storage, rolling, and replacement must actually exist;
  moving the specialized constructor into a helper type is insufficient.

## Decision evidence

Show the smallest common collection/read/update interface and full expansions
of representative table, dice, coin, vote, and storage consumers. Preserve
cardinality, scope, ignored-occurrence meaning, tied votes, callerless flips,
and mixed stored die kinds. Separate eliminated query forms from retained
semantic operations and compare total vocabulary. Obtain a design ruling
before implementation. Standard constraints apply.
