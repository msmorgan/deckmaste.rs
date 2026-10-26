---
needs: [semantics-v3-peer-files-and-toposort]
---
# A term for the active player

Standard constraints apply.

The grammar has no noun phrase for the active player [CR#102.1]. The phasing turn-based action
(`Proofs/Rules.lean`, `phasingTurnBased`) needs one: "all phased-out permanents that the active
player controlled when they phased out phase in" [CR#703.4a,502.1]. The row reads the phased-out
permanent's present controller during that controller's turn instead. Phasing changes no control
[CR#702.26d], so the two agree unless a control effect ended while the permanent was phased out
[CR#702.26f]. In that case the rule phases it in during the untap step of the player who
controlled it as it phased out, and the row would wait for its present controller's.

Add the term, then rewrite the phase-in half as a lookback: this permanent phased out while the
active player controlled it.
