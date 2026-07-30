---
needs: []
---
**`bands with other [quality]` is a keyword ability and we parse it as a
sentence.** [CR#702.22b] makes "bands with other" a special form of banding, and
[CR#702.22c] shows it takes a **quality parameter** —
`bands with other [quality]`. The keyword catalog carries only `Banding`, so the
surface is unrecognizable and the text falls through to the clause grammar.

Master of the Hunt, `It has "bands with other creatures named Wolves of the
Hunt."`, currently parses as a transitive clause whose **subject is the plural
noun `bands`**, with `with other creatures named …` as a prepositional
complement. That is a misparse of the same class as
`english-coordination-comma-defects`: it round-trips clean, so no gate fails,
but every consumer of the tree sees a noun where a keyword ability belongs.

The five land-legends recover rather than misparse — Adventurers' Guildhouse,
Cathedral of Serra, Mountain Stronghold, Seafarer's Quay, Unholy Citadel all
carry `"bands with other legendary creatures."` as a `RecoveredText` span, for
the same root cause.

## Why this blocks a diet decision

Round `sfdiet` kept `QuotedAbility.initial_uppercase` on the strength of a
witness pair that this bug manufactures. The claim was that Takklemaggot's
`gains "At the beginning of that player's upkeep, …"` (capitalized) and Master
of the Hunt's `It has "bands with other …"` (lowercase) share a structure — both
a `Paragraph` whose first sentence is a parsed `Independent` clause — and differ
only in the stored bit.

They do not actually share a structure. Master of the Hunt *should* be a keyword
ability, and the derivation that was tested already predicts keyword interiors
correctly: a keyword line's leading text is its `CatalogAtom`'s **observed**
spelling, so it must not be re-cased. Fix the parse, then **re-test whether
`QuotedAbility.initial_uppercase` is derivable** — the refutation may evaporate,
and with it the last stored casing bit.

## Shape

Add the `bands with other [quality]` surface to the keyword catalog with a
phrasal argument, alongside `Banding`. The argument is an ordinary noun-phrase
quality (`other legendary creatures`, `other creatures named Wolves of the
Hunt`), which the keyword-argument grammar already models. Watch that lowering
does not treat the two as redundant instances — [CR#702.22m] makes multiple
instances of "bands with other" *of the same kind* redundant, which is a rules
fact about the ability, not a licence to collapse distinct qualities.

## Verify

The five land-legends and Master of the Hunt stop recovering / misparsing;
`cargo xtask english recovery` should show the noun-opacity and clause-recovery
counts drop rather than stay byte-identical (this one is *not* a
representation-only change). Round-trip stays 31685 clean. Then re-run the
quoted-casing derivation experiment described above. Standard constraints apply.
