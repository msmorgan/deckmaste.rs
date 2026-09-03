---
needs: [english-v2-preposition-class-data]
---
Replace the name-keyed homograph exemption with a per-member declared
feature (gate-metric-landing-review H1). `reviewed_vocab_lexicon_homograph`
(environment.rs ~:1113) exempts every member of `AttributiveAdjective`
from the vocab/lexicon collision check by vocabulary NAME; adding
`Card = "card"` to that vocab loads clean and only a hardcoded count pin
notices. Per the 2026-09-02 vocab-surface ruling: declare the exemption
on the two members that need it (`Target = "target" { feature
HomographLicense = …; }`, `Untap` likewise), emit it on the VocabSurface
row, consume it in the check, delete the name-keyed function and the
count pin's reliance on it. Probe: `ScratchCard = "card"` inside
AttributiveAdjective must now fail to load. Also (M2): VocabSurface rows
become collision OWNERS so form-literal-vs-vocab collisions are visible.
Note for the chain: adjectives need a lexeme tier (adjective inventory,
core seed + declarations) — a separate ticket after the feature chain;
record it in the landing ledger. Standard constraints apply.
