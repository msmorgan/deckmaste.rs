---
needs: [engine-snapshot-predicate-breadth]
---
**Engine: preserve the combat designation needed by snapshot predicates.** A
creature that dies has already disappeared from the live combat registry when a
later trigger body asks whether it was attacking. The faithful
`Matches(EventObject, Attacking)` path therefore reaches the snapshot matcher,
where `Attacking` currently falls into the generic panic.

Capture the relevant designation before zone-change cleanup and evaluate
`StatePredicate::Attacking` from that value. Do not infer it from the object's
old battlefield zone, and do not consult the destination object. Keep sibling
combat predicates outside this ticket unless a shared representation makes
them an inseparable, tested addition.

Foundations witness: Garna, Bloodfist of Keld. Test both branches of its
attacking/otherwise behavior with creatures dying in and outside combat, plus a
negative snapshot match.
