---
needs: []
---
**The Lean bench writes "a card in <zone>" without `isCard`.** Residue of
`plugins-v2-keyword-helper-additions` (2026-10-05), which added the conjunct
only where a canon RON card writes `cardIn` and a Lean term mirrors it
(`okThoughtseize`). Standard constraints apply.

A token can sit in a hand, graveyard or exile until state-based actions are
checked [CR#111.7,704.5d], so `.inZone hand` alone also admits tokens. At the
landing, 139 bench lines under `lean/Semantics/Cards/` match `inZone` of a
hand, graveyard or exile (Choice 32, Damage 15, Keyword 13, Static 13,
Counters 11, Mana 9, Trigger 9, Anaphora 8, Cost 6, Description 6, Turn 6,
Piles 5, Faces 4, Copy 1, Deontic 1). Each needs reading against its printed
text ("a card", "a creature card", a token-admitting phrase), and pins whose
expected value changes are STOPs. Counted with `grep -rcE 'inZone
(hand|graveyard|exileZone|\(handOf|\(graveyardOf|yourHand|yourGraveyard)'
lean/Semantics/Cards/*.lean`.
