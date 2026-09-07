---
needs: [english-v2-grammar-family-breadth]
---
# Consolidate nominal features and adjective ownership

Complete this family on the shared breadth interfaces under
[the lexical-analysis contract](../../decisions/english-lexical-analysis.md).

Replace transitional `AttributiveAdjective`/`PredicativeAdjective` inventories
with one adjective inventory carrying declared distribution and morphology.
Carry the existing ordinary adjective members, including `main`, `maximum`
and `six-sided`. Preserve identity/provenance and independently declared
homographic readings; grammatical participles come from verb morphology, not a
duplicate adjective inventory. Morphology uses declared vocabulary, default
rules and explicit replacing irregular overrides, without speculative readings.
Reroute the colliding form literals `additional`, `next`, `other` to their
lexical analyses; duplicated ownership is distinct from valid homography.
Include `same` and `true` for
`The same is true for creature spells you control ...`.

Consolidate count/mass use, determiners/genitives, nominal modifier stages and
Targeting Marker order on the existing NP/Nominal categories. `non-` belongs to
productive modifier morphology of declared vocabulary and Polarity; delete per-polarity modifier
families. Collapse card-type modifier partitions to a feature-driven modifier;
the separately retained card-kind noun inventory is outside this deletion.
Adjective/Adverb Phrase degree modification uses lexical distribution, not a
fixed total adjective-order guess. Reuse the qualification/distribution evidence
in `Features.Conforms` and `FeatureInteractions` when refining shared constraints.

Acceptance: count noun and mass noun positive uses, a noun licensed for both,
unlicensed numeral+mass rejection, genitive agreement, Targeting before
adjectives, adjective-only and participial licenses, non-prefixed type modifiers,
and the same general postmodifier structure in ordinary and keyword-host NPs.
The keyword host itself is migrated by the document ticket after extraction.
Preserve lexical identity, provenance and Onset/morphology information; eliminate
the three old literal/inventory duplications without excluding legitimate POS
alternatives. Style-guide evidence: §6 “Describing objects,
players, and targets” and §7 “Types, subtypes, and supertypes as nouns and modifiers”.

Preserve the applicable [inherited witnesses](../../english-grammar-migration-obligations.md)
and check this family's structures under both roundtrip laws. Standard constraints
apply as amended by the lexical-analysis decision; targeted Lean changes belong
here only when needed to settle a changed grammatical constraint.
