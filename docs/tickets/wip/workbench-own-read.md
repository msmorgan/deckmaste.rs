---
needs: [workbench-pronoun-read-index]
---
**Add one core noun read `Own` that reads the enclosing constructor's own
earlier-slot delta, and delete `DealDamageOwn`.** Ruling 2026-09-02 on audit
item R5, as amended by the user.

`Effect.DealDamageOwn src c to:1075` (3 bench uses) exists because "deals
damage equal to ITS power" cannot be written `DealDamage src (StatOf It c) to`
when a second singular object is in scope. The recorded reason in
`done/workbench-counted-anaphora-narrowings.md` ("Row 4") is a call-site count
— "would have touched all 178 existing call sites" — not a rules argument.

## The ruling

`Own` is **not** a grammatical-role read (`TheSubject` was considered and
rejected). It is one positional core noun read that reads **only the enclosing
constructor's own earlier-slot delta** — in `DealDamage src amt to`, that is
`nounDelta src` — and ignores the outer binding stack entirely. Three
guardrails are part of the ruling:

1. **Never an arbitrary scope reset.** `Own` sees the own-slot delta and
   nothing else. It is not a "reset to the nearest clause" mechanism and must
   not be given a scope argument.
2. **The `= 1` uniqueness gate stays**, evaluated within that delta. `Own` is
   counted-unique like every other read; no recency, no ranking.
3. **The intended spelling is the macro layer.** `Macros.dealsDamageOwnPower
   src to` expands to `DealDamage src (StatOf Own c) to`. A raw `Own` written
   in a bench card is a review flag.

Motivation to record on the declaration: a term like `Sequentially [exile
target artifact, target creature deals damage equal to its power …]` must
typecheck, and plain `It` counts two singular objects there.

Then delete `DealDamageOwn` and re-spell its three bench witnesses through the
macro.

Size: S.

Done when: build is 23/23; `Own` is one constructor with its own-delta count
gate and the motivating comment; `DealDamageOwn` is gone from `Effect.Effect`;
its three bench witnesses read through `dealsDamageOwnPower` and still
typecheck; the `Sequentially` two-singular-objects term is a bench witness that
typechecks; a pin refutes `Own` where the own-slot delta binds zero or two
matching objects, and it is non-vacuous. Standard constraints apply.
