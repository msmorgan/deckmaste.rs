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

Coordinator amendment (modal landing review F2/F3): no fresh ADR ruling is
needed — `quote_terminated_statement` (constructions.rs ~:4606) already
implements the rule (the quoted interior's period discharges the enclosing
sentence's terminator), narrowed by `require predicate is
QuotedAbilityPredicate`, a construction-naming guard of the same defect
class as `require possessor is Their`. Generalize that construction to any
sentence whose final constituent is a quoted block and delete the guard;
record the ownership row in the ADR as the now-general rule. Sizing: 312
is a string count; ~215 units fail only at the terminator, the rest fail
mid-text for unrelated reasons — expect ~215 returned.
