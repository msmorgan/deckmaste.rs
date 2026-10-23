---
needs: []
---
# Stratum 7: Cleave brackets around a conjunct and a compound premodifier

Part of `english-v4-stratum-7-discourse-and-long-tail`. Implement in `crates/deckmaste_english_v4` (and `deckmaste_syntax` if needed) per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6, Stratum 7).

## Scope

Dig Up, the one cleave face `english-v4-s7-cleave-brackets` left unread: *Search your library for a [basic land] card, [reveal it,] put it into your hand, then shuffle.* Bracketed Constituents (Oracle English glossary) are read when they are an unselected dependent or a whole sentence. Dig Up needs two more:

- *[basic land] card*: the brackets show *basic land* as one premodifier, but the grammar reads only *basic [land card]*.
- *[reveal it,]*: a non-final conjunct of a Linked Coordination, bracketed together with the comma after it, so that removing it leaves *…card, put it into your hand, then shuffle*.

## Grounding

- [CR#702.148a].
- `docs/oracle-style-guide.md` §3, "Bullets, vertical bars, and square brackets"; "Commas and the serial comma".

## Witness cards

- Dig Up

## Engine

Likely (the comma inside the closing bracket).

## Constraints

Standard constraints apply.

## Landing note

Per `docs/tickets/README.md`, end the ticket with these two lines, stamped with the change id they were measured on:

1. **Census delta**: faces read and total Readings before and after; lost faces by name, or "none".
2. **Judgment set**: the result of running it.
