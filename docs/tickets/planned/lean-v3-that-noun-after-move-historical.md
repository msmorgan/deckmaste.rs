---
needs: [lean-v3-this-way-lookback-grounding]
---
# A definite noun phrase ("that creature") reads the moved object in a `thisWay` lookback

Standard constraints apply. Pre-existing; found reviewing
[lean-v3-this-way-lookback-grounding](../wip/lean-v3-this-way-lookback-grounding.md).

## Problem

"Destroy target creature. If that creature dies this way, ..." is refused with `anaphor (word
creature)`. `it` works. The ADR's historical read of a moved object covers only a mention written
inside a move: `docs/decisions/semantics-v3.md:213-217` ("A historical movement pattern such as
'that creature dies this way' reads a previously introduced participant against the movement's
written origin zone when selecting the movement patient in a `DidClause`"). In code that is the
`.move` arm of the checker (`lean/Semantics/Check/Phrase.lean:3215-3221`, `historicalMoveFrom`, set
only when `drs.actionPattern` and the move's origin is the battlefield) together with the `DidClause`
step that marks `actionPattern := true` and `historicalReferentIds` (`:1086-1100`). A definite noun
phrase ("that creature") in the subject position of the lookback is not resolved through that read,
so the noun-word anaphor finds no referent after the destroy moved it.

## Witnesses (data/derived/cards.jsonl)

*Saw in Half*, *Kalitas, Bloodchief of Ghet*, *Kaervek's Purge* ("If that creature dies this way");
*Tuktuk Scrapper*, *Afflicted Deserter // Werewolf Ransacker* ("If that artifact is put into a
graveyard this way"). Six matches in the derived data for "If that/the <noun> dies / is put into a
graveyard this way"; check each wording.

## Change

Extend the historical read so a definite noun phrase in the `did` subject resolves the earlier
participant against the moved object, as `it` does; update ADR §(`semantics-v3.md:213`) to say the
read covers definite noun phrases, not only mentions written inside a move. Confirm the refusing
path by evaluating before the change (read from code, not run).

## Tests

- `okThatCreatureDiesThisWay`: `did (that creature) die thisWay` after a `destroy` checks clean
  (the pin the owner named).
- Cards: *Saw in Half*, *Kalitas*, *Kaervek's Purge* spelled with "that creature" check clean.
- A refusal remains where nothing moved the object (`that creature` after a non-moving action).
