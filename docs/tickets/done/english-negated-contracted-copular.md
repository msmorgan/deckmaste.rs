---
needs: []
---
**Negated contracted copular clauses (`it's not your turn`) do not
parse.** Found 2026-07-25 (round ctrsubj, §7): the affirmative contracted
copular parses fine — `it's an artifact` (Topple the Statue), `it's a
creature` (Earth Surge), `it's your turn` (Horned Loch-Whale) — but the
NEGATED form does not: Ghost Town (`Activate only if it's not your turn.`)
and Zealous Display (`If it's not your turn, untap those creatures.`) both
recover with `NoCompleteParse` today. The site is the copular path, not
the passive-VP path (verified unaffected by ctrsubj's
`fold_auxiliary_passive`).

**Likely much larger than its two direct witnesses**: coordevent's E1
intervening-condition bucket (80 groups / 81 occ / 1841 words) is full of
`if it's not their turn` shapes (Adrenaline Jockey, Scytheclaw Raptor,
Glademuse, Price of Glory, Lighthouse Chronologist). Coordevent's Edit-2
staged-retry hypothesis for E1 was FALSIFIED at Stage 0 (both witnesses
fail identically under Clause and Sentence roots), and "negated copula"
was one of the diverse content gaps recorded then — this defect is
plausibly E1's real blocker, making it a far larger prize than its
witness count suggests.

Diagnosis entry point: compare how the contracted-subject path carries
negation against the ordinary copular path — the contracted auxiliary
instance has a `contracted_negation` field, and ctrsubj proved this path
family is prone to the two-arms-drift defect class (raw child feature
passed where the ordinary arm recomputes). Run a binding Stage-0 probe
(affirmative vs negated minimal pair through the copular reduce) before
building anything.

## Completion

The implementation had already landed on the structural-recovery campaign
line as change `lsyvlyrm`: `CopularRemainderNegated` admits the free-standing
`not`, `CopularPredicate::negated` records it independently of ordinary
precomplement adverbs, and the renderer replays it after the contracted copula.
The round recovered 31 structural spans / 690 source tokens and added causal
affirmative/negative pairs, composition coverage, renderer-order coverage, and
double-negation guards. Focused tests remain green after the campaign
integration; this follow-up closes the stale ticket bookkeeping.
