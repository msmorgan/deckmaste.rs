---
needs: []
---
Retire the remaining zoneless-object `.expect`s on the move paths. The mill
batch lane's instance was fixed (filter_map fizzle, 2026-07), but its two
pre-existing twins in `resolve/action.rs` — the `.expect("move a zoned
object")` sites on the single-`Move`/group-move lanes — still panic when an
authored selection resolves to a zoneless object (a player proxy: minted with
`zone: None`, and `Predicate::Any` matches it). Raw-authored selections reach
these lanes, so this violates bad-authoring-fizzles-never-panics.

Decide the fizzle shape per lane (skip the zoneless member vs fizzle the
whole instruction — the mill fix skips) and cover with a test that moves a
`SelectAll(Any)` selection. Grep for any further `zone.expect(` siblings
while in there.
