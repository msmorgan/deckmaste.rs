---
needs: [english-v2-grammar-migration-design, english-v2-grammar-family-breadth]
---
# Consolidate nominal features and adjective ownership

The first shared interface is supplied by `english-v2-grammar-family-breadth`.
Complete the acceptance below against those interfaces; other families need
not finish their inventories before this work begins.

Replace transitional `AttributiveAdjective`/`PredicativeAdjective` inventories
with one adjective inventory carrying declared distribution and morphology.
Carry the existing ordinary adjective members, including `main`, `maximum`
and `six-sided`. Preserve identity/provenance and homograph licensing; grammatical participles
continue to come from verb declarations, not a duplicate adjective inventory.
Reroute the colliding form literals `additional`, `next`, `other` before the
new inventory reaches environment validation. Include `same` and `true` for
`The same is true for creature spells you control ...`.

Consolidate count/mass use, determiners/genitives, nominal modifier stages and
Targeting Marker order on the existing NP/Nominal categories. `non-` belongs to
productive modifier morphology and Polarity; delete per-polarity modifier
families. Collapse card-type modifier partitions to a feature-driven modifier;
the separately retained card-kind noun inventory is outside this deletion.
Adjective/Adverb Phrase degree modification uses lexical distribution, not a
fixed total adjective-order guess. Use `Features.Conforms` and
`FeatureInteractions`; extend qualification/distribution witnesses as needed.

Acceptance: count noun and mass noun positive uses, a noun licensed for both,
unlicensed numeral+mass rejection, genitive agreement, Targeting before
adjectives, adjective-only and participial licenses, non-prefixed type modifiers,
and the same general postmodifier structure in ordinary and keyword-host NPs.
The keyword host itself is migrated by the document ticket after extraction.
Preserve source Onset/morphology ownership and reject the three old
literal/inventory collisions. Style-guide evidence: §6 “Describing objects,
players, and targets” and §7 “Types, subtypes, and supertypes as nouns and modifiers”.

Standard constraints apply. Production correspondence and the applicable
[obligations](../../english-grammar-migration-obligations.md) are part of this
ticket; re-spell existing tests by their independently justified outcomes.
