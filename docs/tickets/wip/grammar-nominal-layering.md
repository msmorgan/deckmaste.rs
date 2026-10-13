---
needs: []
---
# Nominal layering

Replace the flat recursion on `Nominal` with category layers so that the order in which modifiers are applied to one head stops producing extra trees. Today `Nominal` wraps itself for premodifiers, postmodifiers and relatives alike, so n premodifiers and one postmodifier give n+1 bracketings: "red creatures you control" has 2 trees, "red artifact creatures you control" 3, "red nonlegendary artifact creatures you control" 4. An estimate from a stratified sample puts the removable trees at about 78,000 of 482,041 (plausibly 57,000 to 106,000; the heavy tail was under-sampled).

## The gate, built first

A restricted normal-form check in the census tool, with no grammar annotations. The normal form of a Reading is the tree itself except that, on one connected nominal-head chain, the interleaving of commuting premodifier and postmodifier applications is ignored: `(base tree, ordered premodifier records, ordered postmodifier records)`. Order within each side is kept. Each modifier record keeps its constructor, form, optional Category field and every non-head field, recursively normalised. Everything else is compared exactly: no global sorting, no transparent wrappers.

- The wrappers it knows today: `PremodifiedNominal`, `NounPremodifiedNominal`, `ObjectRelativeNominal`, `SubjectRelativeNominal`, `PostmodifiedNominal`, `ParticipialPostmodifiedNominal`, `NamedNominal`. After layering it needs an adapter from the new layers to the same modifier records.
- Commuting modifiers, to start: the colour words, *nonlegendary*, adjectival *tapped*, card-type noun premodifiers. Everything else is a barrier: *other*, *another*, focus words, determiners, cardinals, superlatives, comparatives, targeting and slash-modified premodifiers, postpositive, participial and coordinated premodifiers. The list is stated in one place, by lexical class where a class exists, and is extended only with a reason; "nonred", "enchanted" and creature-subtype premodifiers ("Goblin creatures you control") are the first candidates and are not yet in it.
- The census records each unit's set of normal forms; a new baseline is taken with them. The check fails on a lost face, an incomplete enumeration, incompatible unit boundaries, or any unit whose set of normal forms differs. A tree lost whose normal form has no survivor is a failure, never a report line.
- It must keep distinct: a modifier attached to a different head; a different lexeme, form or variant; relative-gap shape; scope of a barrier modifier; coordination hosts; selected frames; quotation boundaries. Required cases, with expected counts of trees and normal forms, are the 30 probed in `docs/evidence/grammar-review-2026-10-11/analysis-key-measurement.md` (for example "other red creatures you control" 3 trees, 2 normal forms; "Destroy target creature with flying." 2 and 2; "Artifact spells you cast have convoke." 4 and 2).
- Landing the check alone changes no Reading: identical digests.

## The change

- Layers as plain categories: a nominal core, then postmodifiers and relatives, then premodifiers, then the noun phrase. That order (premodifiers outside postmodifiers) is a convention for commuting modifiers, where the two bracketings are one analysis; CGEL treats such a nominal as structurally ambiguous with usually no practical difference (CGEL, Ch. 5, §14.2, pp. 446–447). A barrier modifier keeps every scope it has today.
- The "has this already happened" features that police the flat recursion go where layering makes them unnecessary; each removal is listed.
- Design first and stop: the layer categories and which existing constructions land in which layer, how barrier modifiers keep both scopes, how coordination of nominals and fused heads fit, what happens to each policing feature, and the inventory of existing tests that pin nominal tree shapes with their re-spelling for the same witnesses. Tests are re-spelled, never deleted.

## Gate

No face lost; per unit, the set of normal forms unchanged while tree count falls; full tests. Note: census delta (faces, Readings before and after, lost faces none); the judgment-set line does not apply yet.
