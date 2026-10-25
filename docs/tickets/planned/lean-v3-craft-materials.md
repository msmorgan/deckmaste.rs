---
needs: []
---
# Craft's body and its materials parameter

Standard constraints apply.

"Craft with [materials] [cost]" means "[Cost], Exile this permanent, Exile [materials] from
among permanents you control and/or cards in your graveyard: Return this card to the battlefield
transformed under its owner's control. Activate only as a sorcery." [CR#702.167a]. `craft`
(`Definitions/Keywords.lean`) takes only the cost and is bodiless.

Decided (from ADR `semantics-v3.md` §0.1 items 4 and 5): write the body as it composes. Add a
materials parameter that holds a counted or coordinated noun phrase, and keep the [CR#702.167b]
exception in the body: a material described only by a card type or subtype, without "card",
is a permanent of that type on the battlefield or a card of it in a graveyard.

Why it was not done in `lean-v3-keyword-bodies-and-catalog-sync`: the printed materials take
every noun-phrase shape. They can be counted ("six artifacts", "one or more creatures"),
coordinated ("a Dinosaur, a Merfolk, a Pirate, and a Vampire", `oneEachOf`) or an agreement
("two that share a card type", `agree`), and the body has to restrict whatever the caller wrote
to its domain, "from among permanents you control and/or cards in your graveyard". Neither a
`NounPhrase` nor a `KeywordParam` can be restricted from outside. A `Location` cannot say
"permanents you control": `andOr [battlefield, yourGraveyard]` would admit an opponent's
permanents. Options:

1. A `KeywordParam` holding the materials as a noun phrase over a pool the body supplies (a
   pool placeholder the Definition fills with the domain).
2. A materials phrase whose `NounPhrase.someOf`/`oneEachOf`/`agree` group is the domain,
   written by a Template the card calls, with the Definition receiving the whole phrase.

Vintage-legal craft cards: 24 in the MTGJSON snapshot. Their phrasings (`data/derived/cards.jsonl`)
are the test set.
