---
needs: []
---
[design] **Compose noun-word refinements instead of adding a constructor for
each combination.** `NounWord.typedCard` and `NounWord.abilityCopy` in
`lean/Semantics/Words.lean` combine a noun with a type or copy distinction.
Audit the related `Reach` cases and their matching rules before choosing the
replacement: these reads must preserve carrier, type, origin, and ambiguity
behavior, not merely accept the same broad kind.

The semantics_v2 contract calls for a decomposed noun-word vocabulary, and the
current refinement direction keeps a constructor only where its expansion
cannot carry its distinction. Decide which refinements compose and which
primitives must remain; do not replace the vocabulary with an unrestricted
predicate that discards the existing read discipline.

Done when typed-card and ability-copy reads use the selected compositional
shape, or a retained primitive has an explicit irreducibility justification;
the affected cards and pins are re-spelled with the same asserted outcomes;
and positive/negative twins distinguish a copied ability from its source and a
typed card from an incompatible type or carrier. Verify with Lean LSP and
`lean/scripts/build`. Coordinate representation choices with
`lean-conjunction-type-evidence`; neither design is prescribed here.
