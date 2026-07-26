---
needs: []
---
**Bare `IndependentClause::Imperative` does not reduce under
`Nonterminal::Clause`.** Root-caused 2026-07-25 (round midtrigger,
`recovery-harness/out/midtrigger-mechanic-report.md` "precise diagnosis"):
`parse_exact(effect, Nonterminal::Clause)` succeeds for Deontic
(`target creature can't block this turn`) and coordinated
(`draw a card, then discard a card`) effects, but a bare imperative
(`copy that spell`) reduces only under `Nonterminal::Sentence`, never under
`Nonterminal::Clause`. This caps midtrigger's conversion: 234 of 428
positional trigger groups (255 occurrences) remain honestly recovered, and
it is the dominant blocker for the loyalty-header (S3: Chandra the
Firebrand, Twinferno, Gadwick's First Duel, Magus Lucea Kane, Sunken
Palace, Breeches the Blastmaker — all `copy that spell` shapes) and the
`At …,` (S6: 0/10 sampled) sub-families, plus Tranquil Frillback's modal
header group. Work: find where the imperative production attaches
(presumably at the Sentence level only) and either license it under Clause
or give `parse_triggered_sentence` (grammar/ability.rs) a principled second
attempt — NOT a widening to `parse_paragraph` (explicitly forbidden by the
midtrigger plan §2.3: a sentence body is one sentence). Likely
repair-class (an under-predicted nonterminal), and directly unlocks most
of the midtrigger residue; measure with the midtrigger attribution file as
the baseline. Related enumerated residue: the 332 paragraph-initial
content-blocked trigger groups (coordinated events — `X enters and
whenever …`, `X dies or is put into exile while …`) are a SEPARATE gap
(event coordination), enumerated at
`recovery-harness/out/midtrigger-initial.txt`.
