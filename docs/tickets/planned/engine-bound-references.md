---
needs: []
---
`Reference::Bound(Ident)` resolution. Needs a named-role binding store: roles
bound by an event pattern or instruction during resolution ([CR#608.2]), read
back by later instructions of the same effect. Seamed by
`engine-resolve-selections`: `eval_reference` in
`crates/deckmaste_engine/src/resolve/query.rs` currently turns an unbound read
into a null reference because the named-role store is not wired.

A concrete role this store should own (from the 2026-07-16 resolve-layer
cross-boundary audit): the `AdditionalCost` **paid object** (Fling reads the
sacrificed creature) has no channel of its own — it is written into the *trigger*
role slot `that_object` (`resolve/effect.rs:1046-1051`, capture `:1154-1183`; the
dual use is codified at `stack.rs:278-281`), so `EventObject` means two unrelated
things depending on which parent built the frame. The capture is also shape-gated
(`effect.rs:1175-1177`: only `Sacrifice`/`Move`/bound-`Discard`, and `!That(_)`),
so a choice-bearing cost silently leaves `EventObject` unbound. Bind the paid
object through this named-role store (or the existing `With(Produce)` One-`That`
channel, `effect.rs:716-738`) and keep `that_object` trigger-only.

Related minor from the same audit: `Count::TimesPaid` / `Condition::PaidCost` scan
`self.stack` for `e.id == frame.source` (`count.rs:390-395`, `condition.rs:162-166`),
so a read after the entry leaves the stack (the ETB "if it was kicked" recheck)
silently returns 0/false — the paid record should ride the frame or the resolved
permanent (an [[engine-alt-costs]] seam). See
[Effect atom independence](../../decisions/effect-atom-independence.md).
