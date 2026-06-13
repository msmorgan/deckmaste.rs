# Ticket catalog

2026-06-10. The gap between what the engine, grammar, and card pipeline support
today and what *every Modern-legal card and mechanic* needs. Census source: the
local MTGJSON snapshot (22,050 distinct Modern-legal card names) intersected
with the Scryfall keyword catalogs; rules references are the CR snapshot in
`data/rules/`. "Cards" columns count distinct Modern-legal card names that use
a mechanic — use them to prioritize. Mechanic names only in this census; the
policy on committing real cards (the ~100–200-card canon slice, hand-written
edge cases) is `card-data.md`.

Already in place, for orientation: the full turn/priority/stack loop, casting
with mana payment and targets, zone-change pipeline with LKI, core combat with
seven native keywords (the true intrinsics first/double strike, deathtouch,
trample, plus flying, vigilance, lifelink), layers 4–7 with timestamps and CDAs, zone-move /
step / attacking triggers, a handful of SBAs, the core grammar on a combined
`SupportsMacros` derive, and an extract→resolve→graduate card pipeline with
mana/keyword/spell/triggered parsers.

## Folder layout

A ticket's STATUS is the folder it lives in:

| Folder | Meaning |
|---|---|
| `critical/` | Engine happy path — the seams converting ~90% of MTG abilities; old tiers 0–1 |
| `planned/` | Oracle-text coverage, keyword authoring, convenience macros, game-wide systems; old tiers 2–4 |
| `maybe/` | Design-gated or speculative items; old tier 5–6 or `[design]`-tagged |
| `wip/` | Claimed and in progress |
| `done/` | Integrated into the default line |

Tickets move between folders as work progresses:

```
critical/planned/maybe  →  wip  →  done
```

`scripts/workflow claim <slug>` moves the ticket from its current folder into
`wip/` and provisions the feature workspace `../<slug>`. `scripts/workflow
integrate <slug>` folds the finished work into the default line and moves the
ticket to `done/`.

## Priorities

When picking "the next" item, run `scripts/todo ready` to list claimable
tickets — those whose dependencies are all in `done/`. From that list, work
down this ordering: take the highest tier that has an unclaimed,
non-conflicting item; within a tier, use the "Cards" counts where available.
This is an ordering, not an exclusion list, and it is not exhaustive —
anything unlisted (e.g. format/runner items) ranks below these unless the user
says otherwise.

0. ~~**Skill alignment, core-first**~~ — **CLOSED 2026-06-12** (all eight
   waves done; meter: docs/conformance.md). What remains of it lives in
   two named backlogs, picked up under the priorities below: the SEAM
   inventory (`rg 'todo!\("P0\.' crates/` — convert to behavior, mostly
   priority 1) and the post-P0 GRAMMAR backlog (conformance rows tagged
   "post-P0 grammar backlog" — each needs a design dialogue first).
1. **Engine happy path** (`critical/`) — the engine supports the normal
   resolution path of ~90% of MTG abilities (engine machinery tickets, plus
   whichever grammar tickets that path needs).
2. **Oracle-text coverage** (`planned/`) — extraction and parsers graduate an
   increasing subset of oracle text (parser tickets, card-shape tickets).
3. **Keyword authoring** (`planned/`) — keyword abilities, keyword actions, and
   ability words get real macro bodies.
4. **Convenience macros** (`planned/`) — shared macros for common mechanics
   (intertwined with 2 and 3).
5. **Noncanon tests** — keep the noncanon suite growing alongside engine work.
6. **Performance** — optimization passes.

## How to claim an item

When starting work on a ticket:

1. **Claim the ticket:** from `default`, run `scripts/workflow claim <slug>`.
   This moves `<slug>.md` from its current folder into `wip/` and provisions
   a feature workspace at `../<slug>`.
2. **Work in the feature workspace:** `cd ../<slug>` and do the actual
   implementation there.
3. **Integrate when done:** from `default`, run `scripts/workflow integrate
   <slug>`. This folds the feature into the default line and moves the ticket
   to `done/`.

A ticket in `wip/` is claimed and in progress — pick the highest-priority (see
Priorities) item from `critical/` or `planned/` (or `maybe/` if the user
directs) that doesn't conflict with active `wip/` tickets (same files, same
engine subsystem, or one item's dependencies naming the other). Run
`scripts/todo ready` to filter to items whose dependencies are all in `done/`.

Tickets tagged **[design]** require a design dialogue with the user before
implementation — claiming one means opening that conversation, not coding solo.
All CLAUDE.md jj constraints apply in full.

Census tables for card shapes, keyword abilities, keyword actions, and ability
words live in `census.md` alongside this file.
