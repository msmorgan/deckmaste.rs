---
needs: []
---
# engine-fight-subjects-shared-helper — dedup the two Fight-subject resolve blocks

The Fight window's "resolve both fighters, dedup a self-fight to one subject
([CR#701.14c]), vet every id is live" block is duplicated **verbatim** in two
places:

- the single Fight emit arm, `crates/deckmaste_engine/src/resolve/action.rs` (the
  `"Fight"` arm of the keyword-action lowering), and
- the aggregate head, `crates/deckmaste_engine/src/resolve/effect.rs`
  (`batch_act_head`'s `"Fight"` arm).

The two copies are byte-identical today except the fizzle tail (`return vec![]`
vs `return None`). That is a drift risk: a future change to the dedup/vet logic
must be made in both or they diverge silently.

Extract a shared reader, e.g.
`fn fight_subjects(&self, body: &OneShotEffect, frame: &Frame) -> Option<Vec<ObjectId>>`,
returning the deduped, all-live subject list (or `None` to fizzle), and call it
from both arms. Keep the read-off-the-body discipline — it composes with the
shared `deckmaste_core::fight_body_fighters` reader that already backs both.

Found in the final whole-branch review of `engine-act-fight-patient` (verdict:
ready to merge; this was the sole deferred cleanup). Pure refactor — no behavior
change; the existing Fight tests pin it.
