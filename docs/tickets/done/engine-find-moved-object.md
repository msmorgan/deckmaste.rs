---
needs: []
---
[CR#400.7]: an object that changes zones becomes a NEW object with a new
identity and no memory of its prior existence. A family of exceptions lets *the
same effect* (or a spell/ability whose **cost** moved the object) find the new
object the card became in its destination **public** zone, so later parts of
that effect can act on it: the general rule [CR#400.7j], plus the
cast-from-effect cases [CR#400.7h..400.7k].

The engine has no such tracking. `Effect::With` evaluates its selection once and
freezes `Selection::Those` to the **pre-move** `ObjectId`s (`resolve.rs`, the
`Effect::With` arm); a later `Action::Move` remints the object to a new id, but
nothing re-binds `Those`/`ThatObject` to it. So "put a creature onto the
battlefield, then sacrifice **it**", "exile a permanent, then return **that
card**", and "mill a card, then cast **it**" all break — the anaphor points at
the gone pre-move object.

Build resolution-time moved-object tracking:

- During a resolution (and during cost payment — [CR#400.7j] second sentence),
  record an old→new identity mapping for objects moved to a **public** zone.
- Re-bind the active anaphors (`Selection::Those`, `Reference::ThatObject`, and
  any named `Reference::Bound` role) to the reminted object when a later
  instruction of the same effect references it.
- Identity is by the new object: if it has since left that zone (possibly
  returning as yet another new object per [CR#400.7]), it is NOT found — the
  later instruction does nothing. This is the same zone-presence semantics a
  delayed trigger needs ([CR#603.7c]).

Unblocks `engine-delayed-reflexive-triggers`: Sneak Attack's delayed "sacrifice
the creature" and Flickerwisp's delayed "return that card" both reference an
object the creating effect relocated, so they need this find-the-moved-object
capability before the delayed-trigger mechanism can encode them correctly.

---

DONE (2026-07-07): `GameState.moved_chain: Vec<(ObjectId, ObjectId)>` — the
resolution-scoped old→new record (ordered, so recency is the R1-nearest
antecedent). Written by `apply_zone_will_change` on remint for PUBLIC
destinations only (`!to.is_hidden()` — a hidden move records nothing, the id
goes stale, [CR#400.7,400.7e]); cleared in `resolve_object` beside
`that_much` ([CR#400.7j] is per-effect). `chase_moved(id)` follows it
transitively (identity on unrecorded ids; terminates on the ids-never-repeat
invariant). Bound-role reads chase: `Reference::It` (both the `It` binding and
the lone-target fallback), the singular `That(Sort)` `that`-slot, and
`Selection::They`/`Them` per element; `Target(n)` never chases. A product-sited
`That(Sort)` with no frame binding reads `newest_move_product()` (the newest
still-live public-zone product, else null — [CR#400.7]). `move_items` skips a
bound role that resolved to a gone object ([CR#400.7,603.7c] no-op). `With`
special-cases `Binder::Produce(Move(..))` (bind the pre-move id, schedule the
move then the body). `step.rs` gained the `Zone::Exile` source-removal arm.

Tests: 7 unit (`resolve.rs`) + 1 (`step.rs`) + a full-cast e2e in
`tests/stack.rs` (`blink_exiles_and_returns_the_target_in_one_resolution`,
an inline instant — see below). The cite lock gains the [CR#400.7e]
registration (zone-change triggers finding the moved object).

**Deferred:** a *canon* blink card (Cloudshift/Flicker) is NOT added — its
"return … to the battlefield under your control" oracle needs enter-rider
rendering AND execution (the `Action::Move` riders `todo!` seam,
`core-action-riders-cost-modes`); until that lands the e2e uses an inline
card. The **delayed** cousin (Otherworldly Journey's `Delayed` "return that
card") is `engine-delayed-reflexive-triggers`, which this now unblocks.

Recovery note: this claim was rebuilt from scratch atop current trunk after
the original rotted into an all-conflicted state (every feature commit went
conflicted through repeated rebases); the design was re-derived from the
pre-conflict clean commits.
