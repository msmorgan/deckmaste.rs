# Engine and runner boundary

## Decision

Core enumerates every legal decision, including choices with one option, and
offers concession at every decision boundary. Automation, retry policy,
redaction, and convenience filtering belong to runners, servers, or other
consumers.

## Rationale

An apparently forced choice can carry rules-relevant identity or provenance.
Keeping policy outside legality preserves a complete engine API for tests,
strategies, interactive clients, and future information-set projections.

## Consequences

Core must not auto-pay, auto-select, hide concession, or cap retries as a user
experience shortcut. Consumers operate from the legal sets and pending
decisions core exposes, and may project or filter those values without changing
the underlying legality model.

## Tracked references

- [Conformance matrix](../conformance.md)
- [Engine ADRs](../engine-adrs.md)
