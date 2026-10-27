---
needs: [plugins-v3-builtin-primitives-and-templates]
---
# Pins name referents by structure, not by literal id

`lean-v3-proofs-and-docs-alignment` removed `nextId ± k` arithmetic from public pins. About 239
lines in `lean/Semantics/Proofs/`, `Check.lean` and `Cards/` still assert literal ids
(`ids = [n]`, `findById? n`, `(·.id) = … n`). Per file: GetsPt 32, Damage 20, MovementScopes 16,
Counters 15, CallerScope 15, DrtCards 14, SelectionEvidence 12, Choice 12, Turn 11,
ControllerSacrifice 10, ReferenceScopes 9, EventPatterns 9, others 7 or fewer. An id that reads a
fixture's own `DRS.ofList` / `ofNumbered` input is fine. An id that names a referent a step
minted is as brittle as an offset: rewrite it with `headerFreshReferents`, `firstReach`,
`resolveCandidates` or a filter on the referent's structure, as the alignment ticket's C4 did.
Standard constraints apply.
