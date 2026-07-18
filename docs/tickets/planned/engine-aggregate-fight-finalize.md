---
needs: []
---
# engine-aggregate-fight-finalize — aggregate Fight batch can never finalize its fact

`FinalizeWatch::AnyContained(verb)` decides whether an aggregate keyword-action `Batch(n, verb)`
committed, by scanning for a `ZoneChange` with a matching cause-verb since `mark`
(`crates/deckmaste_engine/src/step/mod.rs:871-877`). It watches the ground-truth `ZoneChange`
(not the contained future's own `Act` fact, which `finalize_act` suppresses) so a
"whenever you Verb" trigger fires once per batch.

This assumes the verb's body performs a **zone change**. A `Fight` body moves nothing — it is
reciprocal damage ([CR#701.14a]) — so `AnyContained("Fight")` never matches, and an aggregate
`Batch(n, Fight)` (whose head exists, `crates/deckmaste_engine/src/resolve/effect.rs:1384-1396`)
could never finalize its aggregate `Act` fact.

Latent: no printed card currently forms an aggregate fight batch, so nothing exercises it today.
Fix options when a card needs it: give `FinalizeWatch` a verb-appropriate ground-truth signal
(fight → the reciprocal `DamageDealt`, honoring the 0-damage caveat of
[[engine-damage-zero-no-event]]), or a per-verb finalize predicate read off the body facet.
Do not paper over it with a Fight special-case in the watcher.

Found while grounding `engine-act-fight-patient`.
