---
needs: []
---
**Generalise the paid-cost reads into one `Amount.Paid` over a `PaidFacet`.**
Fresh workbench review 2026-09-03, R5, resolved by ruling.

**Ruling (settled 2026-09-03): cast-time payment facts are readable amounts,
and there is one reader.** `Amount.Paid (f : PaidFacet) (n : Noun bs Object)`,
with

    PaidFacet = ColorsSpent | ManaValueSpent | TimesPaid | PaidCostReadback

in the first cut. The existing readers fold into facets rather than sitting
beside the new one: `Phrase.Amount.TimesPaid` (`Phrase.idr:1635`) becomes the
`TimesPaid` facet and `Phrase.Predicate.PaidCost` (`Phrase.idr:295`) the
`PaidCostReadback` facet, keeping their `Words.PaidCostName` /
`PaidCostNamed` gate. `Phrase.Condition.ManaSpentToCast` (`Phrase.idr:2267`)
stops being an independent fact and re-reads through `Paid`, so "if {R} was
spent to cast it" and "for each color of mana spent to cast it" are two reads
of one thing.

`ColorsSpent` is the read the grammar lacks entirely: converge and sunburst
(48 supported cards, e.g. Engineered Explosives) have no `Amount` today. The
subject noun keeps the stack/`This` gate the existing readers use — a paid
cost is only readable of an object whose payment happened.

The facet set is a first cut, deliberately: a facet is added when a printed
card needs it, and the enumeration above is not a claim that the CR has
exactly four.

Size: M.

Done when: Engineered Explosives is a typechecking bench witness reading
`ColorsSpent`, and a sunburst card is a second one; `TimesPaid` and
`PaidCost` no longer exist as separate constructors and every former site
reads `Paid`; `ManaSpentToCast` reads through `Paid`; a pin refuses a paid
read of an object that never paid a cost (a battlefield permanent with no
cast), probed non-vacuous; the existing paid-readback pins still refute,
re-spelled against the facet; the build is 44/44 with 0 errors and 0 warnings.
Standard constraints apply, plus the RON-shaped constraint: a core constructor
is admissible only if the RON re-emitter can produce it from a RON node, and a
macro only if it names a RON macro
(`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).
