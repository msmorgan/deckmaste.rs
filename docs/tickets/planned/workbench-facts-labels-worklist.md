---
needs: []
---
**Make `cargo xtask facts labels` green by closing the stub/table label gaps
and re-scoping the direction the check cannot mean.** Residue of
`workbench-facts-from-ron` (2026-09-04), which landed the generator
(`FactsGen.idr` from the keyword-ability stubs, per-row shape lists, named
membership witnesses) and the two-way label check, which exits 1 today:

- 102 keyword-ability stubs have no `keywordFacts` row (rowless keywords are
  refused by `KnownKeyword` until rowed); 3 rows have no stub
  (BandsWithOther, Multikicker, PartnerWith).
- `actFacts`: 17 rows have no stub — the wider turn-and-game deed vocabulary
  (draw, untap, …) that the keyword-action stubs never cover; that direction
  of the check needs a scope rule (a deed row is not obliged to have a stub),
  not silencing.
- 42 counter rows have no stub; designations are not covered because stub
  names are not the Idris constructor names.
- The gate columns (`bodied`, `paidCost`, `regime`) stay hand-kept until the
  stub schema (`meta/KeywordAbility.ron`) carries them.

Size: M. Done when: `cargo xtask facts labels` exits 0 under a recorded scope
rule per table; every rowed keyword has a stub or a recorded reason; build at
its module count. Standard constraints apply.
