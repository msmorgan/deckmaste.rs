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
