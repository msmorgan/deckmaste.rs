---
needs: []
---
# The Ability kind's missing heads: bare "ability" and the ability target

Routed from `workbench-event-zone-3-targeting-and-disjunction-arms` (close,
2026-08-26). Two adjacent gaps at the `Ability` kind:

1. **No `AbilityClass` arm denotes a bare "ability"** — the joined head for
   "a spell or ability" (171 supported targeter lines) has no ability half,
   though kind, gate, and `Joined`/`PhJoin` all admit `Object \/ Ability`.
   What a bare "ability" denotes is the design question ([CR#113.3]'s four
   kinds; only activated and triggered target) — answer it from the rules,
   then the cost is one constructor + two `Eq` lines.
2. **`Targetable` has no `Ability` arm**, so "counter target activated
   ability" stays unwritable though [CR#115.2] admits abilities as targets.

## Consumption boundary

`idris/src/Experimental/Words.idr`, `Phrase.idr`; `Cards.idr` bench;
`Proofs*.idr`. No Rust crate.

## Acceptance

- Both arms land with their rule basis or end in a written verdict; at
  least one "spell or ability" targeter line and one ability-target line
  bench.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

- **Routed from workbench-event-disjunction-seat (close, 2026-08-27):** the coordinated anaphor "that spell or ability" (Repeated Reverberation's body — the delayed seat's missing witness): a demonstrative over an `Object \/ Ability` union antecedent, which needs this ticket's bare-ability head. It lands here.
