---
needs: []
---
Parse counter-placement onto mass patients: `Put a +1/+1 counter on each
creature you control.` (and `… on each of up to X target creatures`, `two
+1/+1 counters on each …`). The targeted form (`Put a +1/+1 counter on
target creature.`) parses today; the `each <selection>` patient declines.
The declarative-subject production already handles `Each player …` subjects
by wrapping in `Each` over the player set — this is the same distribution on
the PATIENT side of `PutCounters`, reusing the existing selection parsers.

Check whether other verbs share the gap (`tap each …`, `exile each …`) and
fold them into the same patient-cardinality arm rather than a counters-only
fix — `core-verb-patient-cardinality` (done) is the precedent to extend.

**~171 of 17,022 one-away cards** (2026-07-16 tally) for the counter forms;
more via the shared-verb generalization.

Verify: `cargo xtask generate plugins/wizards` graduation delta;
`cargo xtask fidelity` on a counters-on-each-creature card.
