---
needs: []
---
# Phasing: tell a direct phase-out from an indirect one

Standard constraints apply.

An Aura, Equipment or Fortification that phases out with the permanent it is attached to phases
out "indirectly" and won't phase in by itself, only with that permanent [CR#702.26g]. One that
phased out directly phases in during its controller's untap step [CR#703.4a,702.26i]. If both
would happen at once, it phases out indirectly [CR#702.26h].

The phasing turn-based action (`Proofs/Rules.lean`, `phasingTurnBased`) reads "indirectly" from
the attachment: it doesn't phase in a phased-out permanent attached to a phased-out one. That is
wrong in one case. An attachment that phased out directly (Teferi's Protection on an Equipment,
say) and whose host phased out later, directly, is skipped, though the rule phases it in.

Fix: record how a permanent phased out. The phasing event is one `setStatus .phasedOut`, and an
indirect phase-out writes none of its own, so a lookback can't separate the two. Either give the
indirect phase-out its own event (the attachment's `setStatus` caused by the host's), so the row
reads "did not phase out caused by its host's phasing out", or add a status distinction. Choose
by the CR's structure (ADR `semantics-v3.md` §0.1 item 5).
