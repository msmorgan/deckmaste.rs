---
needs: []
---
**One modification carries both the power and the toughness delta, so "gets
+P/+T" writes its subject once.** Decided with the owner on 2026-10-05. Lean
first, then the mirror and RON. Standard constraints apply.

## The defect

The RON helper `gets(subject, power, toughness, duration)`
(`plugins_v2/builtin/macros/instructions/gets.ron`) writes the subject into
both modifications:

```ron
Establish(Conjunction(None, [Modification(Param(subject), Power, Param(power)),
                             Modification(Param(subject), Toughness, Param(toughness))]), Param(duration))
```

So `gets(target(creature), …)` is two target phrases, two targets where the
card has one. Lean's `getsPt` (`lean/Semantics/Macros.lean` ~L762) avoids this
by reading the toughness half's subject back as a computed pronoun
(`itsOther`, ~L755), which a RON macro cannot do (RON has no computed
arguments; see `semantics-v2-macro-capture-and-plurality`, which lists
`itsOther`). The six keyword bodies that call `gets` (bushido, exalted,
flanking, melee, prowess, rampage) all write the subject twice; it is harmless
only where the subject introduces no binding.

## What is decided

Change the model so ONE modification carries both deltas and the subject is
written once. `getsBoth(subject, delta, duration)` (same delta for both, from
`plugins-v2-keyword-helper-additions`) stays useful and is written over the
new form.

The exact constructor is the Lean landing's call: a new `StaticSpec` arm with
a power delta and a toughness delta, or a `Stat` that names both. It must also
carry `getsBase`'s set-both form ("has base power and toughness P/T"), which
Lean writes through `getsPt` with `.set` deltas. Whether a single-stat change
keeps the existing single-stat arm or becomes the new node with one side
unchanged is also the landing's call; record the choice and the reason.

## The work

1. **Lean.** Add the both-deltas modification in `lean/Semantics/Abilities.lean`;
   check it as one subject (`Check/`); re-spell `getsPt`, `getsBase` and their
   callers in the bench and pins. Every pin keeps its asserted outcome; add a
   pin that "Target creature gets +2/+2" introduces exactly one target.
2. **Mirror.** Follow in `crates/deckmaste_semantics_v2`; `lean_drift` holds.
3. **RON.** `gets` writes the new node with the subject once; the six keyword
   bodies follow. Counter conferrals that write a power and a toughness
   modification (`p1p1Counter`, `m1m1Counter`) may use it; that rewrite
   belongs to `semantics-v2-counter-kind-is-a-name`.

## Proof

`cargo xtask lean-check` passes; `cargo xtask expansions` before/after differs
only in the terms that wrote two modifications for one "gets", each listed.
