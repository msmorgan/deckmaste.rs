---
needs: []
---
**Give piles their own kind.** Residue of `semantic-query-domains`
(2026-09-04), whose core side split `Sort` into `ReferentSort | Amount | Pile`
and left the workbench's twin untouched by region discipline: `kindOfW PileW =
Object` and `PileP : … -> Payload Object` type a pile exactly as a singular
Object. Fix: `Pile : Kind`, retyping `PileOf`/`InPile`/`PileMention`/
`Macros.onePile`; this forces kind-indexing on `Effect.Move`/`SetStatus` and
on `instrIntro`/`doesInstrIntro`/`doesPreIntro`/`riderIntro`/`doesRiderIntro`/
`doesAnnIntro`/`costActionOk`/`heldUntilOk` (the loop intros, free now that
`workbench-loop-delta` has landed).

Size: M. Done when: no pile is typed as `Object`; the pile witnesses
(`Cards/Piles.idr`) and pins still typecheck and refute; build at its module
count. Standard constraints apply, including the RON-shaped constraint.
