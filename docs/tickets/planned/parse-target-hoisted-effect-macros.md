---
needs: [macro-slot-codec]
design: true
---
Macro-route the **multi-reference / target-hoisted** effect lines that a flat
`${i}` slot reader structurally cannot express — the compound tail left behind
by `parse-params-via-macros` (done: its single-reference core was already
delivered by `macro-slot-codec` + the parse-batch campaign).

The shape: an effect naming two or more independent object references that must
be lifted OUT of the body into `Effect::Targeted` target declarations, e.g.
- fights — `Target creature you control fights target creature you don't control`
  (~98 lines), which the bespoke `resolve.rs::fight_effect` handles today;
- exchange / then-return — `Exile target creature you control, then return that
  card…` and the ~136 `exile target …` compound lines;
- any "target A … target B …" body where the two targets differ in filter.

**Why design-gated.** The current slot codec signature is
`slot_reader -> (arg, consumed)` (`macro_template::match_with`): a slot reads a
contiguous English span into ONE typed arg. It has no way to (a) declare a target
at the frame level while (b) leaving an anaphor (`It`/`That`) in the body that
binds to that target. Lifting a target declaration out of a slot is a new codec
capability, not a new reader — it changes what a macro invocation *is*, so it
needs a design pass before any card graduates through it.

Open design questions to settle first:
- How does a macro template mark "this slot is a hoisted target" vs an inline arg
  (a new sigil? a typed `Target` slot kind that the matcher promotes to the
  `Targeted` wrapper)?
- Round-trip: render must re-inline the hoisted target back into the English
  span, so the codec needs the inverse (target decl → span) too.
- Interaction with the standing "pump/fight bespoke is richer" ruling — do NOT
  regress the ~98 fight lines or the pump path onto a macro route unless the
  macro route reaches full parity (fight also scales / grants riders today).

Do not claim without that design pass. Surfaced by the `parse-params-via-macros`
batch execution, which confirmed the single-reference core is already live and
that only this multi-reference class remains.
