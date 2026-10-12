---
needs: [grammar-coordination-level]
---
# Frame-derived gaps, passives and shared complements

**Placeholder: not claimable.** The README has no marker for this, so the `needs:` chain is what blocks it, and it must not be claimed or worked until it has been specified. It is specified only after its predecessor lands, following a short design pass the owner approves; no rulings are pre-written here.

Goal: make verb frames the single source of valency. Ordinary saturation uses `selected_frame`; gaps (four categories plus five series twins), passives, 14 `Shared*Complement` schemas and complement clusters are separate hand-kept inventories per verb form, and the holes in the cross-product produce wrong-analysis passes: `SelectedGapPrepositionPredicate` (`declarations.rs:5526`) has rows for finite, bare and passive only, so "the card you are putting into your hand" gets 0 readings while "the card you have put into your hand" gets 1, with the wrong structure. Object-gap, passive and shared-complement productions are generated from each frame, with one finite `gap` feature carried through auxiliaries.

Gate decided so far: no face lost; analysis keys may only grow, each addition reviewed; the perfect and progressive relative-gap cases pass in the judgment set; ratchet passes; three-line note. Be + participle keeps both the adjectival and the verbal-passive reading.

Open, owner has not decided: whether the frame-derived productions stay within finite summaries without inflating the chart; settle by prototyping on the transitive and object-plus-PP frames and comparing chart items.
