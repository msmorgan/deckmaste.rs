---
needs: [lean-v3-alternative-cost-unification]
---
# Paying something "rather than" an ability's cost is not a spell's alternative cost

Standard constraints apply. Found in passing by `lean-v3-alternative-cost-unification` (B3),
whose ruling covers only spells' alternative costs [CR#118.9].

Two card fragments in `lean/Semantics/Cards/Keyword.lean` write `StaticSpec.altCost`, a spell's
alternative cost, over an ability's cost:

- *New Perspectives* (`newPerspectivesCyclingAltCost`): "As long as you have seven or more cards
  in hand, you may pay {0} rather than pay cycling costs." The encoding drops the condition. The
  cost replaced is cycling's activation cost [CR#702.29a].
- *Thick-Skinned Goblin* (`thickSkinnedGoblinEchoAltCost`): "You may pay {0} rather than pay the
  echo cost for permanents you control." Echo is a triggered ability whose cost is paid while it
  resolves, in "sacrifice it unless you pay [cost]" [CR#702.30a]; it is neither a spell's mana
  cost nor an activation cost.

Change: decide the Core form for paying something instead of an ability's cost (an activation cost
and a cost paid during resolution may want different forms), encode both cards with it, add New
Perspectives' condition, and make `StaticSpec.altCost` (documented as a spell's alternative cost)
refuse a subject that names abilities, or say why it should not.

## Ruling (owner, 2026-10-22, batch 4): option B

- **`altCost` covers spell casting [CR#118.9] and ability activation [CR#602.2b].** Extend
  `StaticSpec.altCost` (`Abilities.lean:865`, `altCost (subject : NounPhrase) (cost : Option Cost)`)
  and its documentation to say so; its subject may name an activated ability as well as a spell.
  *New Perspectives*' cycling uses it (as `newPerspectivesCyclingAltCost` does today,
  `Cards/Keyword.lean:947-950`), **with its missing condition added**: "As long as you have seven
  or more cards in hand" (a `conditional … .asLongAs` around the `altCost`, or `onlyWhile`).
- **Echo's resolution-time "unless you pay [cost]" [CR#702.30a] stays separate from `altCost`.**
  It is paid while the echo trigger resolves, neither a spell's cost nor an activation cost.
  *Thick-Skinned Goblin* (`thickSkinnedGoblinEchoAltCost`, `Cards/Keyword.lean:952-958`, which
  wrongly uses `altCost` today) composes over the existing `Action.pay` with `keyword := some
  (.named "echo")`, plus `StaticSpec.replace` or `.can`. No echo-cost constructor is minted.
- **Replace versus can: the implementer picks, with reasons recorded in the landing.** Replace
  models "you may pay {0} rather than pay the echo cost" as a replacement of the echo payment
  event (the payment is a would-event; the choice is the replacement's), which is the shape
  [CR#614.1] gives "instead" effects. `.can` models it as a permission to pay a different cost.
  Pick the one under which the checker can tie the substituted payment to the echo payment (the
  `keyword` field), and say what the other cannot express.
- VERIFY (done 2026-10-22): `Action.pay` is `Abilities.lean:1226-1227`, `pay (cost : Cost) (times :
  PayTimes) (outcome : PaymentOutcome) (whose : Option NounPhrase) (keyword : Option
  KeywordLabel)`; it is used for "cumulative upkeep is paid" in `Proofs/Trigger.lean:122,134`
  (`.pay .itsManaCost .once .paid (some thisCreature) (some (.named "cumulativeUpkeep"))`).

## Change

1. Document `StaticSpec.altCost` as covering spell casting [CR#118.9] and ability activation
   [CR#602.2b]; keep the checker's subject laws consistent (a subject naming a triggered ability's
   resolution payment, such as echo's, is now refused by `altCost`, which closes the second
   fragment's misuse).
2. *New Perspectives*: add the "seven or more cards in hand" condition to the existing `altCost`
   spelling.
3. *Thick-Skinned Goblin*: re-spell over `Action.pay` (`keyword := some (.named "echo")`) with
   `StaticSpec.replace` or `.can` (the implementer's pick, above).

## Tests

- `okNewPerspectivesCyclingAltCost` (`Cards/Keyword.lean:949`): **rewritten** to check the spelling
  with the condition (it pinned the condition-less encoding, which dropped the printed text).
- `badNewPerspectivesWithoutCondition` (new): the condition-less spelling is not what the card
  prints; pin that the condition appears in the card's structure (a `conditional … .asLongAs`
  node wraps the `altCost`).
- `okThickSkinnedGoblinEchoAltCost` (`:957`): **rewritten** onto the `Action.pay` spelling and
  renamed `okThickSkinnedGoblinEchoPayment`; the old name pinned `altCost` over echo, which the
  ruling removes.
- `badAltCostOverEchoPayment` (new): `altCost` whose subject names echo's resolution payment is
  refused, with the reason code named.
