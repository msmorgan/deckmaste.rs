---
needs: [english-coordination-residue]
---
**Audit the two latent `coordination_agrees` over-fire arms.** Diagnosed
2026-07-24 during the host-shape round (in-code NOTE beside the coordination
tests); the behavior predates that round and is unchanged by it. Two
pre-existing arms license third-person hosts the host-shape gate correctly
rejects:

- `(Some(_), None, false)` — a bare-stem continuation lexing as Infinitive:
  `That player sacrifices a creature, then draw a card`.
- `(None, None, true)` — any agreement-less host (e.g. existential `there`)
  adopts a standalone tail regardless of shape.

Measured supported-surface populations (2026-07-24): 12 faces (existential
`there …, then <stem>`) and 221 faces (third-person subject `…, then
<stem>`) out of 2,795 supported `, then <stem>` faces — over-approximations
grep cannot narrow, since most are correct shared-agreement tails. The audit
needs instrumentation: per-arm firing counters in `coordination_agrees`, dump
the faces where each latent arm is the sole licensor, cross-check against the
host-shape gate, and tighten only with every moved row attributed and the
landed then-coordination/host-shape negative gates green. Standard
constraints apply.
