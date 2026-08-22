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

## As landed (2026-08-22)

Eight keywords minted as `Keyword` constructors carrying the CR expansion as
their meaning — no expansion body is written, matching how Ward/Equip/Cumulative
upkeep already sit in this model. The rule each expansion states decides the
parameter shape, the stack regime (hence which zones a grant may reach through
`grantSubjectOk`), the keyword-counter row and the card-class row.

| keyword | rule | param shape | expansion node(s) | witness |
| --- | --- | --- | --- | --- |
| unearth | [CR#702.84a] | `CostParam` | `KeywordAbility` + `ParamCost` | `Cards.dregscapeZombie` |
| flashback | [CR#702.34a] | `CostParam` | `KeywordAbility` + `ParamCost` | `Cards.thinkTwice` |
| dredge | [CR#702.52a] | `NumberParam` | `KeywordAbility` + `ParamNumber` | `Cards.greaterMossdog` |
| retrace | [CR#702.81a] | `NoParam` | `KeywordAbility` bare | `Cards.ravensCrime` |
| cycling | [CR#702.29a] | `CostParam` | `KeywordAbility` + `ParamCost` | `Cards.barkhideMauler` |
| ninjutsu | [CR#702.49a] | `CostParam` | `KeywordAbility` + `ParamCost` | `Cards.ninjaOfTheNewMoon` |
| miracle | [CR#702.94a] | `CostParam` | `KeywordAbility` + `ParamCost` | `Cards.thunderousWrath` |
| warp | [CR#702.185a] | `CostParam` | `KeywordAbility` + `ParamCost` | `Cards.bygoneColossus` |

Warp's rule is [CR#702.185a], a number the ticket did not name; every other was
re-read in `data/rules/cr.txt` before citing.

`keywordStackRegime`: warp alone is `Just AtCasting` — [CR#702.185a] leaves its
two statics on the stack. The other seven are `Nothing`: each rule names a
graveyard or a hand, which [CR#113.6b] makes the zone it functions from, so a
grant may reach a subject there. `keywordCounterOk` is `False` for all eight —
[CR#122.1b] closes that list by enumeration.

`keywordCardOk` refusals, each on a rule: unearth and ninjutsu are `False` on a
spell card, because [CR#702.84a] and [CR#702.49a] put the card onto the
battlefield and [CR#110.4] denies an instant or sorcery card that; flashback is
`False` on a permanent card, because [CR#702.34a] permits the graveyard cast only
if the resulting spell is an instant or sorcery spell. Dredge, retrace, cycling,
miracle and warp are `True` on both — their rules impose no type gate.

New node: `Cost.ItsManaCost` ("its mana cost", [CR#202.1a]), the cost the
sentence that fixes a granted keyword's parameter names. Without it the two
printed grants whose cost is written as a following sentence have no term.

Grant lines benched (`Experimental.Cards`):

- `sedrisTheTraitorKing` — "Each creature card in your graveyard has unearth {2}{B}."
- `grixis` — "Blue, black, and/or red creature cards in your graveyard have unearth. The unearth cost is equal to the card's mana cost."
- `dralnuLichLord` — "Target instant or sorcery card in your graveyard gains flashback until end of turn. The flashback cost is equal to its mana cost."

Pins added (`Experimental.ProofsG`): `badBareUnearth`, `badCostedRetrace`,
`badUnearthOnSpellCard`, `badFlashbackOnPermanentCard`,
`badWarpGrantInGraveyard`.

Keywords stopped: none. No expansion body is written in this model, so
ninjutsu's "return an unblocked attacker" needed no node.
