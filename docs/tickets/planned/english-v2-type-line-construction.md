---
needs: [english-v2-role-preemption-depth, english-v2-person-number-agreement]
---
# Type Line Construction in english_v2

Frontier (Group E). Routed from the `english-v2-type-line-declaration` STOP
(2026-09-04): the crate has no declaration-only form for type-line order, so
the order table (`crates/deckmaste_english_v2/docs/type-line-order.md`) can
only become a declaration when a Construction consumes it. This ticket is that
consumer: a `constructions!` Construction for the Type Line (supertypes, card
types, the em-dash separator, subtypes) whose Linearization of card types is
consumed from the declared order rather than restated in the grammar. Card
Type words keep entering through the normalized open `Type` inventory (single
authority); the order lives ON that inventory as a declaration field
(ruling 2026-09-04: a single authority carries its own linearization order;
a second inventory or a crate-local copy is a STOP), so this ticket's
consumption boundary includes the normalized open `Type` declaration schema
and the builtin declarations under `plugins/builtin_v2` that populate it.
2026-09-04: pinned — was: "at the boundary the claimant designs and records
in the landing record" (the unpinned boundary is what produced the
type-line-declaration STOP). Integrating this ticket
unblocks `english-v2-type-line-declaration`, which then converts the table into
that declaration form with tests against the 12 attested sequences.

Fences: no closed vocabulary duplicating the open `Type` inventory; no guard
naming a card type. A STOP if consuming the order requires a compiler change
the rewrite ADR does not already permit.

Standard constraints apply.
