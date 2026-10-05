---
needs: []
---
**Helpers that still declare a required parameter after a defaulted one.**
Residue of `plugins-v2-keyword-helper-additions` (2026-10-05), which
reordered only helpers whose builtin caller was forced into named form
(`activated`, `damage`, `deonticRule`, `verbedEvent`, predicate
`attachment`). Standard constraints apply.

A positional call can never omit a defaulted binder that precedes a
required one, so these defaults only serve named calls: `abilityGrantFrom`,
`attacksWith`, `choice`, `compare`, `doForEachKind`, `insertPart`,
`moveCounters`, `nthOccurrence`, `oneEachOf`, `paysCost`, `replacement`,
`rollsDice`, `spell` (every card writes `spell(timing: None, instruction:
…)`), `tappedForMana`, `zoneChange` (Eumidian Terrabotanist and Storm Fleet
Spy omit `from` by name). Also: `returnToBattlefield(subject, riders, agent,
from)` is already required-first, but unearth names `agent` and `from` to
skip `riders`; no order serves unearth and persist/undying/earthbend both
positionally. `doIfDone(body, if_did, if_not)`: fading names `if_not`; it is
"if you can't", not the `may` shape `mayOrElse` covers.

Each reorder must leave `cargo xtask expansions` and every card term
byte-identical, with every positional caller rewritten.
