---
needs: [core-regions-discourse]
---
**Real witness tests for the reference channel: hand-spelled semantic
fixtures, not the corpus.** The witness cards inherited by
`core-regions-substrate` from the absorbed engine tickets are, in eighteen
of nineteen cases, `.ron.todo` in the wizards corpus with the witnessed
clause as the `Unparsed` line. The corpus test that claims to cover them
(`plugin/tests/corpus_identity.rs`) strips those lines first, so fourteen
cards contribute no region-bearing ability and the gate is a single
aggregate region count. Standard constraints apply.

## Scope

For each witness in `docs/tickets/done/core-regions-substrate.md` (Angel of
Finality; Arbiter of Woe; Blasphemous Edict; Bloodtithe Collector; Burglar
Rat; Deadly Brew; Duress; Liliana, Dreadhorde General; Painful Quandary;
Perforating Artist; Pilfer; River's Rebuke; Tribute to Hunger; Tinybones,
Bauble Burglar; Fiery Annihilation; Run Away Together; Steel Hellkite;
Trygon Predator; Predator Ooze): spell the witnessed ability in semantics
RON by hand, lower it, and run it through the engine asserting the card
result the absorbed ticket named (per-player choice, target-relative
filter, trigger-relative target, whole-zone selection, cross-target and
event filter, the damaged-this-turn history predicate). One test per card,
named for it. Delete the corpus-stripping witness test. Where the semantic
grammar cannot yet spell a clause, the test is `#[ignore]` with the
blocking grammar gap named, never a passing test over a different ability.

## Gates

Nineteen named tests; each ignored one names its gap; engine and plugin
suites green with `plugins/wizards` regenerated and `crates/*/build.rs`
touched.
