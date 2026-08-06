---
needs: [english-derived-family-inventory]
---
**Derive the English quantity family and land scalar holes.**

Migrate Q01 from `docs/english-derived-family-inventory.md`: `quantity_exact`,
`quantity_at_least`, `quantity_or`, `quantity_x`, `quantity_both`,
`quantity_up_to`, `quantity_that_many`, `quantity_that_much`,
`quantity_more_than`, and `quantity_fewer_than`.

Land declarations and the compiler's local scalar-hole support with this first
consumer. The declarations cover every `Quantity`/`QuantityValue` variant,
numeric value and notation, cardinality flow, and the stored-versus-derived
witnesses classified in Q01. `ComparativeWord` uses Q01's finite bound-value
mapping and the existing typed lexeme spelling form; it does not require the
generic identity holes first landed by N01, so Q01 remains
capability-independent from that ticket.

This is one vertical migration: generated parse/reduction/lowering, total
render and validated build projections; active spelling-frame and serialized
view changes; direct-AST, `inspect`, exactness, and invalid-bound fixtures;
atomic registry flips for all ten IDs; then deletion of their handwritten
registrations, reducer/lowerer arms, renderer mirrors, and bypassing
constructors. Update the inventory's owner/accounting state in the same change.
No consumer or deletion work is deferred.

Standard constraints apply.
