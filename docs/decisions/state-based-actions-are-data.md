# State-based actions are data

## Decision

Ordinary state-based actions are swappable plugin rules evaluated over an
explicit binding scope. State-based actions that require a player choice remain
imperative, and the sweep repeats only when applying a batch performed a state
change.

## Rationale

Rules-as-data supports variant rule sets while keeping universal evaluation
generic. Choice cannot be represented by an unconditional effect, and
emit-count termination loops when all proposed events are replaced or
prohibited.

## Consequences

Each data rule binds its subject through its scope before evaluating its
condition. Putting an object into a graveyard is distinct from destroying it,
so rules must choose the appropriate primitive; choice-requiring rules surface
a pending decision, and suppressed batches do not trigger another sweep.

## Tracked references

- [Engine ADRs](../engine-adrs.md)
- [Conformance matrix](../conformance.md)
- [Rules taxonomy](../rules-taxonomy.md)
