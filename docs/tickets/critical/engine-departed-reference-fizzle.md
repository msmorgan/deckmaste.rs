---
needs: []
---
**Engine: never emit a one-shot action event for a null or departed current-only
operand.** After partial target loss, `Reference::Target(n)` falls back to the
slot's first stale id so downstream code can no-op; `eval_reference_set` then
wraps that id as a real singleton. Damage and counter actions emit events whose
appliers assert a live target. A departed `This` recipient has the same shape.

Make the boundary deliberate. Current-only singular selections must discard
null and nonlive ids before event construction, while a partially legal spell
continues resolving its legal, independent target slots. Remove the stale
`Target(n)` fallback or ensure every consumer sees an unavailable reference;
add defensive liveness handling at event application where queued state can
still invalidate an operand. Coordinate count dependencies with
`engine-statof-reference-channel` rather than reading a departed target through
LKI.

Foundations witnesses: Drakuseth, Maw of Flames when its first damage target
leaves but a later target remains; Felling Blow and Bite Down when target slot
0 leaves and slot 1 remains; Heroes’ Bane when its source leaves before its
ability resolves. Pin which remaining packets/instructions still happen in
each case, not only the absence of a panic.

Related but distinct: [[engine-unbound-ref-oneshot-fizzle]] covers an anaphor
that never bound and therefore resolves to the null sentinel. This ticket owns
faithfully bound operands that later cease to be live.
