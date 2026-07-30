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

## Completion

Each of the eleven was measured separately by swapping the derivation into the
renderer with the field still stored and populated, then reading
`cargo xtask english roundtrip --list`. That is the only way to see a
disagreement: while the bit is stored the renderer replays it and the face
round-trips clean either way.

**Five delete**, derivation exact at 0 mismatches —
`ModifierCoordination`, `AdjectivePhraseCoordination`,
`PredicateObjectCoordination`, `ExceptionConjunct`, `RestrictionCoordination`,
all as `conjunction.is_none() || rest.len() >= 2`.

**Three keep, refutation documented on the field.**
`CoordinationJunction`: the count rule misses 1011 faces because the sequencing
connective `Then` takes a comma even in a bare two-member shared-subject
coordination; a `Then`-aware refinement reaches 8 residual faces (Turnabout,
Angel of Jubilation, Armed with Proof, Arterial Alchemy, Gogo Mysterious Mime,
Neverending Torment, Nightmare Incursion, Yasharn Implacable Earth) that take a
comma for conjunct length, so it is not exact.
`SetExceptionNounPhrase`: a binary wrapper, not a list; `comma` varies
independently of `marker` across its four productions, recording only whether
the surface printed a comma before `except`.
`Attachment<T>`: attachments are independent riders, not list members, and the
field takes three distinct forms across its sites including a rule-tag-threaded
variable.

**Three split out**, because deleting them would convert a silent misparse into
a render regression: `NounPhraseCoordination` (39) and
`NominalPhraseCoordination` (1) to `english-coordination-comma-defects`, and
`ClauseCoordination` (281) to `english-clause-coordination-comma-rule`.

The ticket's central claim held and then some. The 39 `NounPhraseCoordination`
disagreements are the Arrest/Pacifism aura template, whose stored tree
coordinates *`block`* with *`its activated abilities`* — a verb conjoined with a
noun phrase, invisible for as long as the comma bit was replayed. This is the
Giant Oyster finding again, on far more common cards.

Round-trip 31685/31685 clean, 2937 tests passing, recovery census
byte-identical, fidelity 0 failing.
