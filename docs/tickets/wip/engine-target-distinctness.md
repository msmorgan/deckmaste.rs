---
needs: []
design: true
---
Close the P0.W7 seam in `resolve.rs` (`target_spec_filter`): a
`TargetSpec::Distinct(sibling_indices, inner)` spec — the co-target
set-distinctness constraint ([CR#115.7e], Arc Trail's "any *other* target") —
currently trips `todo!` rather than being enforced. Per the core doc
(`target_spec.rs`), distinctness is evaluated on the FINAL target set (never a
fixed-binding exclusion): at announce ([CR#601.2c]) and at the [CR#608.2b]
resolution re-check, where retargeting may have swapped members.

Adjacent gap in the same seam, fold-in candidate (design call): the
`TODO(stage-4)` above it — `Target(quantity, f)` quantity is unenforced
(callers assume exactly one target slot), so plain multi-target specs ("two
target creatures", "up to three targets") aren't announceable either. Cross-
spec `Distinct` without within-spec quantity covers only the rarer half of
multi-targeting.

Touches: announce-time target legality/choice surface (`decide.rs`
ChooseTargets flow), the [CR#608.2b] re-check, retarget validation
(`ChooseNewTargets` — keep-current strategy in `sim.rs` assumes per-slot
independence), and the legal-set enumeration the runner sees (engine
enumerates every choice explicitly).
