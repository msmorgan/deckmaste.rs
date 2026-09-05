---
needs: []
---
**A keyword whose Quality parameter is spelled with a preposition has no
keyword-line shape, and in grant position it silently splits.** `Affinity` is
declared `params: [Quality]` with surface `affinity`, but the printed line is
`Affinity for artifacts` [CR#702.41a]: the quality arrives behind `for`.
`qualified_keyword_line_item` is `lex(keyword) quality` over `abstract sum
KeywordQuality { Reference: Nominal, Coordination: KeywordQualityCoordination }`,
which admits no prepositional quality, so the keyword line does not parse
(Frogmite, Myr Enforcer, Thoughtcast are parse failures).

The grant position is worse than a failure. Since
`english-v2-tail-keyword-ability-grant` (2026-09-05) `Spells you cast have
affinity for artifacts.` **is covered**, selecting the bare
`ReferencedQualityKeywordAbility` carrier for `affinity` and attaching `for
artifacts` as a clause-level prepositional predicate adjunct — the quality
modifies the having rather than naming the affinity. Two units select this
today (Sami, Wildcat Captain; Tezzeret, Master of the Bridge). The shape
predates that landing (the parent reached the same split through the
`params = Any` `lex KeywordAbility` codec) and it was not introduced by it, but
because it is covered rather than failing it will not resurface when the
keyword line is repaired.

`Protection` shows the shape that works: its declared surface is `protection
from`, so the preposition is inside the keyword surface and the quality is a
plain `Nominal`. Whether the fix is to give `Affinity` the same treatment
(surface `affinity for`), or to admit a prepositional arm on `KeywordQuality`,
is the design question; a per-keyword branch in the grammar is not an option.
Whichever is chosen, the two covered grant-position units above must change
analysis, so the landing record has to name them.

Standard constraints apply.
