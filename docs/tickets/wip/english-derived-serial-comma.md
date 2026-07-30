---
needs: []
---
**Stop storing the serial comma; derive it from member count.**

Eleven `pub comma: bool` fields across the coordination syntax types
(`NominalPhraseCoordination`, `NounPhraseCoordination`, `ModifierCoordination`,
`ClauseCoordination`, `CoordinationJunction`, `ExceptionConjunct`,
`SetExceptionNounPhrase`, `Attachment<T>`, …) record a value that is never
computed from data. Lowering assigns it as a per-rule-tag constant:

~~~rust
RuleTag::NominalPostpositiveAdjectiveConjoined => (Some(1), 2, false),
RuleTag::NominalPostpositiveAdjectiveAsyndetic => (None,    2, true),
RuleTag::NominalPostpositiveAdjectiveOxford    => (Some(2), 3, true),
~~~

so it carries no information beyond which production fired, and the production
is already recoverable from `conjunction: Option<_>` plus member count.

## Why it matters

The supported corpus is strictly serial-comma over type lists: 1364 two-member
coordinations with no comma and 0 exceptions; 139 three-member lists with the
comma and 0 exceptions. The flag can therefore only ever agree with the count —
or be wrong. And a wrong value **round-trips clean**, because the renderer
faithfully prints whatever lowering recorded, so no fidelity gate can catch it.

This is not hypothetical. Round `ppcoord` (2026-07-30) removed the field from
the one type it introduced and the corpus gate immediately failed on Giant
Oyster, exposing a real over-application in that round's own new production:
`during [X], and at [Y]` had been read as a two-member coordination, when the
comma marks a clause boundary. With the flag stored the bad bracketing was
invisible.

## Shape

Follow `Polarity`, which already derives rather than stores on the same
grounds — its `non-` hyphen "is never recorded structurally; the renderer
derives it at render time instead". Prefer making the grammar enforce the style
rule where it can: `PrepositionalPhraseList`'s base is a *pair*, so the list
always holds two members, the Oxford close needs three, and `A, and B` matches
no rule at all rather than being merely unattested.

Each of the eleven wants its own check that the count genuinely determines the
comma for that construction — clause coordination and exception riders may not
share the nominal rule. Where a construction is genuinely free, keep the field
and say why in its doc.

Standard constraints apply.
