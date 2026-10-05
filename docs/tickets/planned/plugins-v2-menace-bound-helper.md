---
needs: []
---
**`cantBeBlockedByFewerThan(subject, n)` cannot be written over today's
bound.** Found 2026-10-05 at `plugins-v2-keyword-helper-additions` (STOP).
Standard constraints apply.

Menace is "can't be blocked except by two or more creatures" [CR#702.111b],
written `deonticRule(thisCreature, Forbid, [Block], Patient,
Counterpart(allOf(creature)), NoRider, MoreThan(1))`. The accepted helper
name takes the number the text prints, 2, but the term carries `MoreThan(1)`
(Lean `CountBound.moreThan`); a RON template cannot compute `n - 1`, so no
body over that bound is byte-identical for menace. The owner chooses one of:
a helper named for the bound it writes ("can't be blocked by N or fewer
creatures", `n` = 1 for menace), an `atLeast` count bound in the model (Lean
first), or arithmetic in the amount. Until then menace
writes `deonticRule` positionally.
