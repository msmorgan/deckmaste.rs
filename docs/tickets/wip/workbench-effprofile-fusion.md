---
needs: [workbench-pin-hygiene]
---
**Finish the `Effect` table fusion or remove `EffProfile`.** Residue of
`workbench-table-machinery` (2026-09-03): the round added `Effect.EffProfile`
but the nine per-constructor tables it was meant to replace remain the
authority (the full fusion pushed one module past the five-minute build
watchdog). A record beside the tables is more surface, not less. Either land
the fusion incrementally (one table per build, measuring `Effect`'s check
time each step, stopping at the last step that stays under the watchdog) or
delete `EffProfile` and record why the fusion is not affordable.

Size: M. Done when: no projection of `Effect` exists twice (record and
table), build 23/23 under the watchdog, `Effect` check time reported before
and after. Standard constraints apply.
