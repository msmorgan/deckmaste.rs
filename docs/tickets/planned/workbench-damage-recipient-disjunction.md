---
needs: []
---
**Admit a bare type disjunction as a damage recipient, through the same
head-type alternatives the deed gates already compute.** Fresh workbench
review 2026-09-03, F1.

`Phrase.DamageRecipient.ObjectTakes` demands `DamageableTy (nounTy n)`.
`nounTy (Described _ (Or ps))` is `seedTyJoin`, which is `Nothing` unless all
arms agree, and `DamageableTy Nothing` is `So False`. The join path does the
opposite: `JoinTakes` / `damageableKind Object (SoleTy Nothing)` is
*permissive* on `Nothing`. The same words are therefore admitted inside a
player join and refused alone.

Probes (`idris2 --check` against the core):

- P1 `DealDamage This (Lit 5) (target (Or [HasType Creature, HasType Planeswalker]))`
  (Fry) — **refused**: `DamageRecipient (Described … (Or …))`.
- P1b the same disjunction inside `Joined (Or …) AnyPlayer` — admitted.
- P1c `IsDealtDamage AnyDamage (a (Or [creature, planeswalker]))` — refused by
  the same gate.

82 supported cards print exactly "damage to target creature or planeswalker",
plus the `IsDealtDamage` triggers that read it.

Fix: replace the `nounTy` check with the alternatives already computed for
deeds — `all (all damageableType) (nounHeadTys n)`, with `[]` meaning
unknown-therefore-permissive — and have `damageableKind` read the same
function, so the lone path and the join path agree by construction instead of
by coincidence. `nounHeadTys` is already `List (List CardType)`, one entry per
disjunctive alternative (`workbench-zone-gate-unify`).

Size: S.

Done when: Fry is a typechecking bench witness and an `IsDealtDamage` trigger
over the same disjunction is a second one; `ProofsDamage` keeps its existing
refusals and gains a pin for a disjunction no arm of which can be dealt
damage, probed non-vacuous; the build is 44/44 with 0 errors and 0 warnings.
Standard constraints apply, plus the RON-shaped constraint: a core constructor
is admissible only if the RON re-emitter can produce it from a RON node, and a
macro only if it names a RON macro
(`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).
