---
needs: [engine-sba-breadth]
---
Migrate the lethal-damage [CR#704.5g] and deathtouch [CR#704.5h] destroy SBAs
out of the hardcoded `sweep()` block (sba.rs) into `rules/sba/` data rows with
`then: Destroy(This)`, matching the toughness/loyalty/defense rules landed by
`engine-sba-breadth`. `Destroy` already exists as an `Action`; the net-new
grammar is a marked-damage `Count` (e.g. `MarkedDamage(This)`, reading
`obj.damage`) for `Compare(MarkedDamage(This), AtLeast, StatOf(This, Toughness))`,
and a deathtouch-strike `Condition` reading `obj.struck_by_deathtouch`. Each new
`Count`/`Condition` variant also owes a render arm (total-renderer floor).
Removing the hardcoded block makes the destroy SBAs declarative and gets the
indestructible/regeneration interaction through the shared `WillDestroy`
replacement path uniformly. Re-evaluating each sweep is correct (indestructible
is mutable); caching is a later optimization.
