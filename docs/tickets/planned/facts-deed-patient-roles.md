---
needs: []
---
**Three deeds have no patient role in the facts overlay, so no deontic can
name them.** Found across the keyword-ability rounds (2026-09-07):
`Destroy` (blocks Indestructible, "can't be destroyed" [CR#702.12b]),
`Attach` (blocks Protection's "can't be enchanted, equipped, or fortified"
rows [CR#702.16c,702.16d]), and `Reveal` (blocks Ripple) carry a
`patientRole` of `fieldObject` with `bare := false` and an empty type list
in `crates/xtask/src/facts/action_overlay.rs`, so `deedFits` refuses every
noun phrase in the patient position. Declare each deed's patient role from
the CR, regenerate, `facts check` clean, then give Indestructible,
Protection's remaining rows, and Ripple their bodies with a canon card each
proving through `lean-check`. Standard constraints apply.
