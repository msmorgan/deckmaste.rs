---
needs: []
---
**Engine: an enters-replacement can hold only one self-augment fold.**

`apply_as_enters` (`replace.rs`) folds a permanent's own
`Also(would: Enters(This), also: …)` replacement into its `EnterStatus`
([CR#614.1c] makes "enters with" / "as this enters" text a replacement
effect). Self-tap, self-attach ([CR#303.4] — an Aura enters attached),
self-counters ([CR#122.6a]), and a conditional `If` gate each work — but only
as a single standalone `also` body. There is no arm for `Sequentially` or
`Simultaneously` wrapping two of them.

**This blocks 18 real cards.** "Enters the battlefield tapped with N counters
on it" — `Arixmethes, Slumbering Isle`, `Hickory Woodlot`, the Vivid lands,
and 14 others in the wizards corpus. The natural shape is
`Also(would: Enters(This), also: Sequentially([Act(Tap(This)),
Act(PutCounters(This, kind, n))]))`, where both children already have working
folds individually. Only the composing arm is missing.

Fix: add `Sequentially` and `Simultaneously` arms that fold each child into
the same `EnterStatus` in turn. Order is irrelevant here — tap, attach, and
counters touch disjoint fields — so both variants can share one fold loop.
Leave the existing single-effect arms alone.

Not in scope: widening *which* single-effect shapes are supported, and
entry-time player choices ("as this enters, choose a color"), which are
`core-as-enters-choices` — a different lane, about storing a choice rather
than composing folds.

Done when a card authored with the composed shape enters both tapped and with
counters in one resolution, pinned by a semantic test.

Effort: **S**.
