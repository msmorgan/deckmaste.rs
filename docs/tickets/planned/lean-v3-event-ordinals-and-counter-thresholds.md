---
needs: [lean-v3-ast-merges-by-rules-meaning]
---
# "The first time each turn" is a history test; "the Nth counter" is a counter threshold

Standard constraints apply. Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-lean.md`
#5, #21. Verified against head 2026-10-21. Latent in Lean; mirrors in `plugins_v2` and
`deckmaste_semantics_v2/src/phrase.rs` as named.

## 1. Moonlit Meditation: the ordinal belongs on the replaced event, not a Usage Limit

*Moonlit Meditation* (`Cards/Choice.lean:1109-1114`) is `replace (would (by_ you) (createObject …))
(instead (may (create groupSize …))) none .repeatedly (some oncePerTurn)`. The Usage Limit doc
(`Abilities.lean:510-511`) itself says "An event's ordinal … is not a usage limit". A replacement
effect watches "for a particular event" [CR#614.1]; the CR is silent on the ordinal, so the ruling
governs (2025-07-25): tokens created before it came under your control mean "the replacement effect
won't apply to any tokens you create for the rest of the turn". A declined first creation also
uses up "first".

Change: `replace` takes a `WouldClause` (`Abilities.lean:786-790`, `would | wouldAny | causedBy`),
which has no ordinal arm; `nthOccurrence` (`:473`) is a `GameEvent` only. Add an ordinal form to
`WouldClause` (the first matching event this turn, counted over the turn's history), use it for
Moonlit, and drop the `oncePerTurn` cap. Keep `triggeredOnlyOnce`/`oncePerTurn` for cards that
print "This ability triggers only once each turn". *Shimmering Glasskite* is already fixed
(`Cards/Keyword.lean:1071-1074`, `nthOccurrence (.nth 1) … (some .turn)`).

`docs/tickets/done/workbench-anaphora-d-creation-and-replacement.md:241,326` records the cap as the
landed shape; this ticket supersedes it. Pins: a token made before the Aura was cast means the
first creation after it is not replaced; declining the first replacement leaves the second
creation unreplaced.

## 2. "When the Nth counter is put on" fires on crossing N

*Midnight Clock* (`Cards/Trigger.lean:511-512`, `nthOccurrence (.nth 12) (countersPut hourCounter
.one thisArtifact)`) and *Political Triumph* (`:515-516`, `.nth 4`) count occurrences. [CR#122.7]:
such an ability triggers when counters are put on "such that the object had fewer than N counters
on it before … and N or more counters on it after". The readings diverge after removals, after
+1/+1 and -1/-1 annihilation [CR#122.3], and on a batch placement.
`docs/tickets/done/workbench-amount-comparison-and-quantity.md:495-519` applied the occurrence
reading to these headers; this ticket supersedes it for counter thresholds.

Change: after the wip [lean-v3-ast-merges-by-rules-meaning](../wip/lean-v3-ast-merges-by-rules-meaning.md)
§4 lands (it deletes `CounterBatch` and adds `Quantity.theLast`, already a threshold test: before
> 0, after = 0), give `countersPut` the put-side analogue, a threshold quantity meaning before < N ≤
after, rather than a separate `countersReach` event. Re-encode every header the done ticket
converted. Keep `nthOccurrence` for ordinal casts, draws and turn events. Move the
counter-threshold uses of `plugins_v2/builtin/macros/events/nthOccurrence.ron` to the same form. Pins: twelve counters put one at a
time with one removed in between still fires on reaching twelve; a batch of three from ten fires
once.

## Verification

`./lean/scripts/build`; `cargo xtask lean check`; `cargo test -p xtask -p deckmaste_semantics_v2`
and clippy for both; the CR citation gates.
