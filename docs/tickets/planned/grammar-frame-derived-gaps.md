---
needs: [grammar-coordination-level]
---
# Frame-derived gaps, passives and shared complements

**Placeholder: not claimable.** The README has no marker for this, so the `needs:` chain is what blocks it, and it must not be claimed or worked until it has been specified. It is specified only after its predecessor lands, following a short design pass the owner approves; no rulings are pre-written here.

Goal: make verb frames the single source of valency. Ordinary saturation uses `selected_frame`; gaps (four categories plus five series twins), passives, 14 `Shared*Complement` schemas and complement clusters are separate hand-kept inventories per verb form, and the holes in the cross-product produce wrong-analysis passes: `SelectedGapPrepositionPredicate` (`crates/deckmaste_english_v3/src/declarations.rs:5526`) has rows for finite, bare and passive only, so "the card you are putting into your hand" gets 0 readings while "the card you have put into your hand" gets 1, with the wrong structure. Object-gap, passive and shared-complement productions are generated from each frame, with one finite `gap` feature carried through auxiliaries.

Gate decided so far: no face lost; analyses may only be added, each addition reviewed, by a check specified with the ticket (this step needs binder, gap-function and frame-slot identity, so it may be where the general analysis key in `maybe/grammar-analysis-key` is built); the perfect and progressive relative-gap cases pass in the judgment set; two-line note. Be + participle keeps both the adjectival and the verbal-passive reading.

Open, owner has not decided: whether the frame-derived productions stay within finite summaries without inflating the chart (settle by prototyping on the transitive and object-plus-PP frames and comparing chart items); the gap domain; extraction and sharing constraints; how passives derive from a frame; and the compiler's restriction that a `selected_frame` form has exactly two fields, a lexical Verb or frame-exporting node followed by `selected_frame(head)` (`crates/deckmaste_construction_v3_core/src/ir.rs:855`), which frame-derived forms may need lifted. "Keys may only grow" also preserves today's known-wrong keys (the wrong fallback analyses behind `SelectedGapPrepositionPredicate`); whether retiring them is separate work is for the owner to approve.

The specification must include an inventory of the existing tests that pin the affected tree shapes and re-spell each against the new shape for the same witnesses; tests are never deleted to pass, and the landing reports restored, re-spelled, added and removed counts. The landing updates the affected crate README.
