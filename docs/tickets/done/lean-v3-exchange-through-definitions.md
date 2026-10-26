---
needs: []
---
# Write every printed "exchange" through its Keyword Action Definition

Standard constraints apply. Found by the 2026-10-25 triage of an external audit ("Atraxa"),
verified against the tree at `wmyykqnv`. Paths are under `lean/Semantics/`; line numbers are as
of that change.

## The defect

Exchange is a Keyword Action [CR#701.12a]. ADR `docs/decisions/semantics-v3.md:299` lists
`exchange` among the keyword actions promoted from Template to Definition. The six Definitions
(`Definitions/Actions.lean:289-313`: `exchangeCards`, `exchangeControl`, `exchangeLifeTotals`,
`exchangeTextBoxes`, `exchangeValues`, `exchangeZones`) each wrap the Core head in
`.ownScope (some (.action "exchange"))`, so the event carries the exchange Deed.

A leftover Template builds the same Core term without the Deed:
`semantic_template exchange (exchanged) : Action := .exchange exchanged` (`Templates.lean:2441-2442`),
with its arm Templates `lifeTotals`, `controlOf`, `cardsAcross`, `zones`, `values`
(`Templates.lean:2431-2440`). Probed 2026-10-25: `(exchangeLifeTotals …).namedDeed =
some (.action "exchange" .none)`, but `(exchange (lifeTotals …)).namedDeed = none`. Nine card
sites use it, so "whenever … exchange" patterns and `did … exchange` lookbacks cannot match those
cards, and these sites are what keep the `.exchanged` EventVerb class alive
(`Check/Words.lean:176`, `Check/Phrase.lean:1206-1207`).

## Change

Rewrite each site to the Definition. Each spelling below was probed and checks `[]` in context
(Deadpool's in isolation as `may (exchangeTextBoxes …)`).

| Site | Card, Oracle | Write |
|---|---|---|
| `Cards/Turn.lean:374` | Mirror Universe: "Exchange life totals with target opponent." | `exchangeLifeTotals (youAnd (target isOpponent))` |
| `Cards/Choice.lean:1659` and `:1661` | Juxtapose: "You and target player exchange control of the creature you each control with the greatest mana value. Then exchange control of artifacts the same way." | `exchangeControl (greatestManaValue isCreature you) (greatestManaValue isCreature (target isPlayer))`, and at `:1661` the same with `isArtifact` and `(that player)` |
| `Cards/Cost.lean:775` | Soul Conduit: "Two target players exchange life totals." | `exchangeLifeTotals (targets (exactly 2) isPlayer)` |
| `Cards/Anaphora.lean:338-339` | Vedalken Squirrel-Whacker: "…you may exchange one result with this creature's base power or base toughness…" | `exchangeValues (theOutcome .rollResult) (basePowerOf thisCreature)` and the same with `baseToughnessOf thisCreature` |
| `Cards/Keyword.lean:1636` | Avarice Totem: "{5}: Exchange control of this artifact and target nonland permanent." | `exchangeControl thisArtifact (target (and [isPermanent, non isLand]))` |
| `Cards/Keyword.lean:1640-1643` (`arcanumWingsAuraSwap`) | Arcanum Wings: "Aura swap {2}{U} ({2}{U}: Exchange this Aura with an Aura card in your hand.)" | `auraSwap [generic 2, pip .blue]` (the keyword, `Definitions/Keywords.lean:278`, whose body already uses `exchangeCards`). The fixture now writes the reminder text instead of the keyword. Keep `okArcanumWingsAuraSwap` (`:1644`) as the check of the keyword ability. |
| `Cards/Piles.lean:201` | Harness Infinity: "Exchange your hand and graveyard." | `exchangeZones hand graveyard` |
| `Cards/Keyword.lean:1717-1720` | Deadpool, Trading Card: "As Deadpool enters, you may exchange his text box and another creature's." It writes `may (establish (textChange thisCreature (exchangeWith …)))` raw. | `may (exchangeTextBoxes thisCreature (a (and [isCreature, isOther])))` |
| `Proofs/Lookback.lean:855` (in `thisWayClassesTheEvent`) | fixture | `may (exchangeLifeTotals (youAnd (target isOpponent)))`; the "lost life this way" read must still check `[]` |
| `Proofs/CardFidelity.lean:303-309` (`squirrelWhackerExchangesWithBasePowerOrToughness`) | an `rfl` on Squirrel-Whacker's term | rewrite its right-hand side to the new term |

Before editing each site, confirm the argument order of the Definition at
`Definitions/Actions.lean:289-313`; if a Definition's parameters do not fit a site as written
above, STOP and report rather than reaching for the raw head.

Then delete `semantic_template exchange` and the five arm Templates (`Templates.lean:2431-2442`).
They have no other users; the Definitions write the raw `Exchanged` arms.

## What must not change

- Core `Action.exchange` and `Exchanged` (`Abilities.lean:615-621`, `:1314`) and the six
  Definitions' bodies.
- The Core-level checker pins that write raw `.exchange` stay as Core pins:
  `okExchangeControlOnField` (`Proofs/Zone.lean:578`), `okExchangeCardsAcrossZones` (`:583`),
  `okExchangeTwoZones` (`:591`), `okExchangeTwoValues` (`:608`), `okExchangeTextBoxes` (`:615`),
  `okExchangeValuesSigned` (`Proofs/Numbers.lean:135`), and the `Proofs/CallerScope.lean` pins at
  `:216`, `:218`, `:271`.
- Phyrexian Infiltrator (`Cards/Anaphora.lean:75`) and aura swap (`Definitions/Keywords.lean:282`)
  already use Definitions.
- The `.exchanged` EventVerb is not deleted here; `lean-v3-deed-duplicates` ruling 2 deletes it
  once no bare head writes it. After this ticket, none does outside the Core pins above.

## Pins

- Must stay green: `mirrorUniverse`, `juxtapose`, `deadpoolTradingCard` (spelled),
  `okJuxtapose` (`Cards/Choice.lean:1665`), `okSoulConduitExchange` (`Cards/Cost.lean:776`),
  `okVedalkenSquirrelWhackerReroll` (`Cards/Anaphora.lean:341`), `okAvariceTotemExchange`
  (`Cards/Keyword.lean:1637`), `okArcanumWingsAuraSwap`, `okHarnessInfinityExchange`
  (`Cards/Piles.lean:202`), `thisWayClassesTheEvent`, `phyrexianInfiltratorExchangesControl`
  (`Proofs/Damage.lean:1257`), `keywordBodiesKeepTheirActionDeeds` (`Proofs/Keyword.lean:551`).
- Rewritten: `squirrelWhackerExchangesWithBasePowerOrToughness` (new term, same claim).
- Added, in `Proofs/Damage.lean` beside `phyrexianInfiltratorExchangesControl` and in its form
  (`X.ownDeed? == some <Definition>.deed`): one deed pin per rewritten card: Mirror Universe,
  Juxtapose, Soul Conduit, Vedalken Squirrel-Whacker, Avarice Totem, Harness Infinity and
  Deadpool, Trading Card.

## Acceptance

- `./lean/scripts/build` (`--wfail`), `cargo xtask lean check`.
- `grep -rn 'exchange (' lean/Semantics/Cards lean/Semantics/Templates.lean` finds no use of the
  deleted Template; `grep -rn 'exchangeWith' lean/Semantics/Cards` is empty.
- CR citation gates.
