---
needs: []
---
**The model cannot write menace's printed number: it says "more than 1"
where the card says "two or more".** Found 2026-10-05 at
`plugins-v2-keyword-helper-additions` (STOP). A note about a concept gap, not
a ticket to build.

Menace is "can't be blocked except by two or more creatures" [CR#702.111b],
written `deonticRule(thisCreature, Forbid, [Block], Patient,
Counterpart(allOf(creature)), NoRider, MoreThan(1))`. The bound is Lean
`CountBound.moreThan`, so the term carries 1 where the text prints 2, and a
RON template cannot compute `n - 1`.

Decided 2026-10-05: the `cantBeBlockedByFewerThan(subject, n)` helper is
dropped, and menace stays on the positional `deonticRule`. Owner: "sure? this
may be one of those cases where we can't refer to a game concept properly
yet."

A candidate, if the gap is ever closed: an "at least n" count bound in the
model (Lean first), so the printed number can be written as printed.
