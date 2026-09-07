---
needs: [semantics-v2-crate]
---
**Model the rules tables in Lean so they port to v2 with the cards.** The
six files under `plugins/builtin/rules` are semantics-language RON: four
state-based actions [CR#704.5f,704.5g,704.5h,704.5i,704.5w], one conferral
(planeswalker enters with loyalty, [CR#306.5b]), one damage result (damage to
a planeswalker removes loyalty, [CR#120.3c]). Lean models none of them:
`StateBasedCause` has one constructor used only as a deontic rider, and there
is no conferral, damage-result, or predefined-token catalog (`TokenSpec`
covers created tokens only; the seven predefined tokens under
`plugins/builtin/tokens` have no Lean shape). Idris never modelled them
either.

Add the three table shapes and the predefined-token catalog to the Lean
syntax with their checker reads and pins, then mirror them in
`deckmaste_semantics_v2` and its reader. Governing decisions:
`state-based-actions-are-data.md`, `conferrals-come-from-registries.md`,
`minimal-core-data-driven-rules.md`. Standard constraints apply.
