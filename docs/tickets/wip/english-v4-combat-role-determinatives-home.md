---
needs: []
---
# The combat-role determinatives belong in the core closed class

*attacking* and *defending* [CR#506.2] sit in `catalogs/turn_parts.ron` as
`turn_part_det:<lemma>`. They are not turn parts, and nothing in the CR
enumerates them as a catalog item. Their verbs *attack*, *block* and *defend*
are core vocabulary in `lexicon/english/verbs.ron`, and attacking is not a
keyword action under [CR#701.1].

*enchanted* stays in `keyword_abilities.ron` for the opposite reason: it derives
from the Enchant keyword ability, which the CR enumerates [CR#702.5]. That
contrast is why these two move.

## Change

- Move both entries into the `determinatives:` list of
  `lexicon/english/closed_class.ron`, with the explanatory comment, and the
  id form the other closed-class determinatives have (`det:<lemma>`).
- Delete the `turn_part_det` branch of the loader, and the `determinatives`
  field of `TurnPart` (`noun` becomes required again), which only this entry
  used. Restore the `turn_parts.ron` header to phases and steps.
- Update every reference: the id table in the crate README, the test
  `combat_role_determinatives_retain_verbal_homographs` (the Lexemes are now
  Core, not Catalog) and `phases_and_steps` in `tests/cr_coverage.rs`.

## Acceptance

- `cargo xtask english-v4 strata` census is identical before and after. If
  the move would change any analysis, stop and report; do not adjust the grammar.
- Every judgment holds.
- Tests and clippy clean for the crates touched.

Standard constraints apply.
