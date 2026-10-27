---
needs: [plugins-v3-builtin-primitives-and-templates]
---
# Combat damage rules as rows: first strike, double strike, trample, banding, deathtouch

Standard constraints apply.

First strike, double strike, trample, banding (with "bands with other") and deathtouch's lethal
assignment [CR#702.2c] modify the combat damage turn-based actions: announcing how each attacking
or blocking creature assigns its combat damage [CR#703.4k,510.1] and dealing it [CR#703.4m,510.2],
and the combat damage step's split into a first-strike step [CR#510.4,702.7b,702.4b]. Banding also
modifies declaring attackers and who assigns a banded creature's damage
[CR#702.22c,702.22h,702.22j,702.22k].

Rules are DSL data (`docs/decisions/semantics-v3.md` §0.1 item 4), so these are rules-table rows,
not engine procedure and not keyword bodies: each keyword stays bodiless, and its row reads it
with `hasKeyword`, as toxic's damage-result row reads toxic (`Proofs/Rules.lean`, `toxicPoison`).
Compose the rows from existing primitives first (ADR item 5b); a turn-based action is a
`Conferral.turnBased` row. Which creature assigns damage when, the order among simultaneous
assignments and APNAP order stay the engine's.

`lean/CONTRACTS.md` "Keyword abilities with no body" points here for these keywords; update it
when the rows land.
