---
needs: []
---
# Shared adjuncts of bare imperative chains: the clause-level tree only

Source: the post-S7 re-scan (Argon, 2026-10-24), HIGH-1, Option A, which the
owner approved after `english-v4-rescan-duplicate-trees` landed Option B. No
corpus unit endorses a wide-scope trailing adjunct that only the Verb Phrase
coordination route builds, so that route is the duplicate.

## Change

- The `coordinated` and `uncoordinated` checks see a coordination through
  transparent post-adjunct heads, using a minimal structural `Summary` bit
  documented in `crates/deckmaste_syntax/README.md`. Imperative's
  `uncoordinated` then means what it says (english-lexical-analysis §2: a
  chain of bare imperatives is coordinated imperative Clauses).
- Remove the `ClauseType: Imperative` route. CoordinatedClauseAdjunct builds
  the one wide-scope tree.
- Replace the synthetic HIGH-1 witness in `trailing_adjunct_height.txt` with a
  Threaten-family card. Assert that its narrow *until end of turn* reading is
  admitted.

## Acceptance

Census unit by unit with no face lost; judgments hold.

The coordination bit also lets one coordination of Clauses take several
shared trailing Adjuncts, as a Verb Phrase coordination already could. It also
restores the wide-scope tree over a coordination whose first conjunct has its
own fronted dependent (*[At the beginning of your upkeep, put …, then
sacrifice …] unless …*), which the `ClauseType` route had removed. Those trees
account for the units whose Reading count rose.

Census delta (rowozrtp): faces read 32,769 of 32,828 before and after;
Readings 338,531 → 337,675 (113 units down by 2,601, 112 up by 1,745); lost
faces: none.
Judgment set (rowozrtp): all hold, 3,219 enforced and 317 pending.
