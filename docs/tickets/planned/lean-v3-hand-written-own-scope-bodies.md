---
needs: []
---
# Audit hand-written `ownScope` bodies in the proof suites

Proof suites still write `.ownScope (some (.action …))` bodies by hand. Some are deliberate
scope or refusal fixtures; others may be replicas of production Definitions, the defect
`lean-v3-proofs-and-docs-alignment` §C2 fixed for the named suites. Classify each site as a
fixture (say so in its docstring) or a replica (point it at the production declaration):
Actor.lean amass and discard bodies, Zone.lean (four sites), KeywordActions.lean (the residue
bodies), ActionFamilies.lean, Counters.lean, CallerScope.lean (a fateseal body inside a
forwardedSubject test), Faces.lean, ActionForms.lean, Anaphora.lean (four sites), Choice.lean,
MovementScopes.lean, HistoricalMove.lean, and `merges` in LinkedReadback.lean and
KeywordCastPermissions.lean (a fragment of `mutate` without production's
`doIf (choiceWasTaken …)`). Authoring.lean and DefinitionsAndTemplates.lean's are intentional.
Standard constraints apply.
