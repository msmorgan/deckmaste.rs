---
needs: []
---
# Stratum 7: postpositive *total* after a quantified Noun Phrase

Part of `english-v4-stratum-7-discourse-and-long-tail`; split from `english-v4-s7-long-tail-edge-cases`. Implement in `crates/deckmaste_english_v4` and `deckmaste_lexical_source_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) §4.4.

Witness: **Avatar of Woe**, *If there are ten or more creature cards total in all graveyards, this spell costs {6} less to cast.* (its unit is in `deferred.txt` under this ticket).

Finding (verified on the witness): only *total* fails; the card reads without it. The lexicon has *total* only as an attributive adjective and a noun. CGEL, Ch. 5 §14.2(b), pp. 445–446, has adjectives that "follow specific sums" (*fifty dollars net*), and this is that construction: the adjective qualifies a numerically quantified Noun Phrase, never a bare or definite one (*creature cards total*, *the creature cards total* are not English).

Blocker: the grammar cannot tell a numerically quantified Noun Phrase from a definite one. Features unify, so an unset `DeterminativeDistribution` passes a `Require`: *the* and *more* carry no distribution, and the coordination *ten or more* takes its last conjunct's (`proximate`). A first attempt (an `AdjectivePostposition: Sum` lexeme feature and an NP-level Production requiring `head.DeterminativeDistribution: Numerical`) read *creature cards total* and failed *ten or more creature cards total*, and was withdrawn. Needed first: a feature that every determiner values and that a cardinal numeral, or a coordination containing one (*ten or more*, CGEL, Ch. 15 §3.3(d), p. 1330), values as cardinal. Then add the sum-qualifying adjective. No other corpus unit uses postpositive *total*.

## Landing note

Per `docs/tickets/README.md`, end the ticket with the two landing lines.
