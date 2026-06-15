---
needs: []
---
Strip "Ability word —" prefixes during extraction and preserve them for
rendering. The ability word itself carries no rules weight; this is purely
a parsing/rendering quality item.

## Needs a design dialogue before claiming (2026-06-14)
Two unpinned forks surfaced when this was attempted in batch2:
1. **No storage slot.** `TodoAbility` is a raw `String`, core `Ability` has no
   ability-word field, and the renderer has no ability-word path. "Preserve for
   rendering" requires a core-model decision (where the prefix lives), not just a
   strip pass.
2. **"No rules weight" is not universally true.** Canonical ability words
   (Threshold/Delirium/Landfall/Morbid) are deliberately modeled as gating
   `Condition` macros (`crates/deckmaste_core/src/condition.rs`). So strip-and-relabel
   (flavor only) vs. emit-a-`Condition` (where one exists) are materially different
   implementations. Decide the policy — and how the two interact — before coding.
