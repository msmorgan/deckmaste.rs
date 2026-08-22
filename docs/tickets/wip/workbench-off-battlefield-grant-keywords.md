---
needs: []
---
# Mint the keywords behind off-battlefield grants

`grantSubjectOk` now admits an ability granted to an off-battlefield subject
([CR#113.6b]), but every printed witness grants a keyword the `Keyword` sum
lacks: unearth [CR#702.84], flashback [CR#702.34], dredge [CR#702.52],
retrace [CR#702.81], cycling [CR#702.29], ninjutsu [CR#702.49], miracle
[CR#702.94], warp. Mint each through the `KA` shape-indexed applicator
(`idris-keyword-model`) with the CR's own expansion as its meaning, then bench
the grant lines ("Creature cards in your graveyard have unearth {…}",
"Target instant or sorcery card in your graveyard gains flashback until end of
turn", "Blue, black, and/or red creature cards in your graveyard have
unearth"). Verify each rule number in `data/rules/cr.txt` before citing.

## Consumption boundary
`idris/src/Experimental/Words.idr` (`Keyword`, param shapes),
`idris/src/Experimental.idr`, `idris/src/Experimental/Macros.idr`,
`idris/src/Experimental/Cards.idr`, `Proofs*.idr`.

## Acceptance
Eight keywords minted with their CR expansions; the grant witnesses bench;
build PASS; no implicits in cards; cites 0/0. Standard constraints apply.
