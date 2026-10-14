---
needs: []
---
# Assess a general instruction performed as an event occurs (`semantics-v3` port)

Parked by the user on 2026-09-06. Compare with the `semantics-v3` Unified
Action-Clause Spine (`lean-v3-action-clause-spine`,
[`docs/decisions/semantics-v3.md`](../../decisions/semantics-v3.md) §3). The
local fold of `entryChoice` and `attachmentChoice` with an occasion enum must
not depend on the more general construct proposed here.

Explore whether a shared as-event instruction can express these choices and
other useful cases while preserving the original event and publication of
declared choices across ability lines. An ordinary replacement whose body
only chooses would erase obligations the current forms retain. Put disclosure
on the choice itself and preserve its DRT scope (`DRS`). Sanctuary Blade and
Psychic Paper supply the attachment-choice/readback witnesses.

Before implementation, demonstrate a concrete carrier, full expansions, and
DRT checks that preserve event occurrence and linked choice publication.
Compare its independent usefulness with the smaller choice-plus-occasion
family. Leave that family in place if the general mechanism does not earn its
complexity. Standard constraints apply.
