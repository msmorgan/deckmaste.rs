---
needs: [english-v2-stage-5-grammar-buildout-13-10]
---
A quoted granted ability that ends its enclosing sentence: decide who owns
the sentence-final period. Today `quoted_ability` wraps a `DocumentBlock`
whose interior owns its own period (style guide: the final period sits
inside the quotes), so `Create a 1/1 red Goblin creature token with "This
token can't block."` dies at end of input — the outer sentence still wants
a terminator. `has "…"` escapes only because the quoted-ability predicate
is sentence-final as a verb phrase. 312 corpus units carry ` with "` (149
`token with "`, 86 `emblem with "`), all uncovered.

This is a per-boundary byte-ownership ruling, the same class as the ADR's
"Trigger → following ASCII space" row: either the quoted block's interior
terminator also discharges the enclosing sentence's terminator, or the
enclosing sentence takes a terminator-less form when its last constituent
is a quoted block. Record the row in the ADR's ownership table, implement
it generally (any nominal position, not the `with` postmodifier alone),
and prove it with `ambiguity --require-resolved` and the byte-exact gates.
Standard constraints apply.
