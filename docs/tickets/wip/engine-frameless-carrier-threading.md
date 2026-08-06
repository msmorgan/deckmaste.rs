---
needs: []
---
**Engine: self-referential filters panic wherever the caller threads no
carrier.**

`target::matches_with`'s arms for `Predicate::Where`, `Ref(This)`, `Ref(You)`,
and `Ref(AttachHostOf(This))` all panic when a caller passes `watcher: None`.
Those callers are not hypothetical:

- `activate.rs` — `Binder::ChooseOne` cost and effect filters call
  `candidates()` with no watcher. "Sacrifice **another** creature" is
  `And([Creature, Not(Ref(This))])`, which needs one.
- `replace.rs` — `enters_attached_host`, the `AsEnters(Attach(This, to))` host
  filter, calls `candidates_with(..., None)`. A reanimation aura's "return ~ to
  the battlefield attached to target creature **you control**" is
  `ControlledBy(Ref(You))` ([CR#303.4f] governs the attach choice when an Aura
  enters other than by resolving as an Aura spell).

**Why the owner was wrong.** The seams named `engine-filter-breadth`, but that
ticket only added the match arms. `engine-static-scope-carrier` says so
outright: "`engine-filter-breadth` added the `ControlledBy`/`Ref` match ARMS
but did not thread the carrier". That ticket then fixed the threading for
exactly one path, and scoped itself explicitly — "the live trigger/count path
already threads its watcher … so this is specifically the derived /
continuous-effect path". `activate.rs` and `replace.rs` were never touched, so
these four arms were left with an owner that had closed.

Fix: thread a watcher into the `Binder::ChooseOne` candidate legality checks
(the binder's owning ability or spell source is already in scope) and into
`enters_attached_host` (the entering object is the natural watcher for the
`to` filter), mirroring `engine-static-scope-carrier`'s fix. `AttachHostOf`
([CR#701.3]) resolves the same way once a watcher exists.

Done when a `ChooseOne` cost filter carrying `Not(Ref(This))` resolves and
excludes its source, an `enters_attached_host` filter carrying
`ControlledBy(Ref(You))` resolves against the entering object's controller, and
the existing `engine-static-scope-carrier` tribal-lord fixtures stay green.

Effort: **S**.
