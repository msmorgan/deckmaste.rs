---
needs: []
---
SHIPPED: the remaining zoneless-object `.expect`s on the move paths are
retired. The mill batch lane's instance was fixed earlier (filter_map fizzle);
its two pre-existing twins in `resolve/action.rs` — the
`.expect("move a zoned object")` sites on the group-move (`MoveGroup`) and
single-`Move` (reference-set) lanes — now skip a zoneless member instead of
panicking on its absent zone. A member with no zone to leave (a player proxy:
minted `zone: None`, matched by `Predicate::Any`) is dropped from the group;
an all-zoneless selection then fizzles via the existing empty guard. This is
the same "skip the member" fizzle shape the mill fix chose (not
whole-instruction fizzle), keeping every zoned member's move intact.

Covered by `move_group_skips_zoneless_members` (a `MoveGroup` over
`SelectAll(Any)` that sweeps in a player proxy alongside a battlefield
creature: the creature still reaches its graveyard, the proxy is untouched, no
panic). The single-`Move` lane's fix mirrors the gone-object skip already
sitting one line above it. The only remaining `zone.expect(` sibling
(`lki.rs`, "a zoned object has a zone to leave") is a different invariant — an
LKI snapshot taken at a zone change, where the object definitionally has a
zone — and is intentionally left in place.
