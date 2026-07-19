---
needs: [engine-transform]
---
Harden the by-index triggered/target ability re-reads now that ability sourcing is
face-aware — a latent panic surfaced by the `engine-transform` whole-branch review.
Gated behind Transform-emitting cards (none exist until `engine-day-night` or a
canon transform card lands), so latent today.

`engine-transform` made `abilities_of_source` face-aware: a back-up permanent
sources its section-1 printed abilities from its BACK face. Back-up-at-fire-time
triggers are captured BY VALUE (`showing_back`, `trigger.rs`), and one re-read
(`resolve/mod.rs`) was hardened to `.get()` + graceful fizzle. Three sibling
re-reads still index raw `[*ability]` + `unreachable!()`:
- `resolve/targets.rs:46` (`targets_still_legal`)
- `trigger.rs:1110` (`trigger_targets`)
- `cast.rs:1216` (stack-object target getter)

Failure scenario ([CR#603.3] — an ability on the stack resolves independently of
its source): a FRONT-up printed trigger fires (`created: None`, captured by index),
its source transforms to back before the trigger resolves, and the re-read now
returns the back face's shorter/differently-typed list → index-out-of-bounds panic
or `unreachable!()`. Before this feature these sites were panic-proof because
`abilities_of_source` always returned the immutable front face.

Fix: either extend the by-value capture to ALL `TwoFaced` printed triggers (not just
back-up-at-fire), or harden the three sites to `.get()` + fizzle, symmetric with
`resolve/mod.rs`. The replacement path (`replace.rs`, `cast.rs` replacement gather)
iterates rather than storing an index — no analogous hazard. Prerequisite of
`engine-day-night`, the first consumer that makes back-up permanents reachable.
