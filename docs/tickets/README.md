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

`jj-kata claim <slug>` moves the ticket from its current folder into `wip/` and
provisions the feature workspace `.workspaces/<slug>`. `jj-kata integrate
<slug>` folds the finished work into the default line and moves the ticket to
`done/`.

Those moves are made by jj-kata's bundled folder-Kanban driver, configured in
`kata.toml`. It reads this tree directly — the folders above are its
`columns`, and a ticket's `needs:` frontmatter is the graph. Nothing in this
repository implements ticket semantics any more.

## Writing a ticket

The frontmatter (`needs: [...]`) is the only prescribed part. Everything after
it is up to the ticket writer, subject to exactly two requirements: the body
describes an **actionable change**, and it **records the design decisions
already made** that bear on it. Length, headings, and structure are free —
do not survey sibling tickets to mimic their shape; heavier tickets are not
the house style, they're accumulated drift. (Meta-tickets/"epics" that
coordinate other tickets are the exception and may carry more structure —
no formal mechanism for them yet.)

## The dependency graph (`jj-kata kanban`)

Each ticket's frontmatter carries a `needs: [...]` list — the other tickets (by
slug) that must reach `done/` before it can be worked. Those lists are the edges
of a dependency graph whose nodes are the ticket files, and whose status for any
node is simply the folder it sits in.

**That `needs:` list is machine-read graph data, not a reading list.** To find
what a ticket depends on, what it blocks, or whether it is claimable yet, **query
the graph with `jj-kata kanban` — do not open the dependency tickets to work that
out by hand.** (Reading a dependency's *body* to understand its design is fine
when you actually need it; the command is for everything about dependency
*status* and *shape*.)

`jj-kata kanban [--slugs-only] <command> [<slug>]`:

| Command | What it prints |
|---|---|
| `board` | Every ticket grouped by column, in the configured column order. |
| `ready` | Claimable triage items — those whose every need is in `done/`. One `slug (folder)` per line, ordered by column then name. |
| `blocked` | Triage items with an unmet need: `slug (folder) <- need1, need2 …` (only the needs not yet done). |
| `order` | Every unfinished ticket (triage plus `wip/`), each prerequisite printed before its dependents. Ties break by column priority, then slug. |
| `graph <slug>` | One ticket's dependency picture: recursive upstream needs + direct downstream blocks. **Run this on a ticket instead of opening its `needs:` files.** |
| `needs <slug>` | That ticket's *direct* needs, one slug per line. |
| `check` | Integrity sweep over the whole graph — reports duplicate slugs, dependency cycles, and dangling needs (a need naming no node). Prints `OK: …`, or a `FAIL` line plus one problem per line and exits 1. Run it after editing any `needs:`. |

`--slugs-only` reduces any line-oriented command to just the bare slug column —
handy for piping. **It goes before the subcommand**, e.g. `jj-kata kanban
--slugs-only ready`; placed after, argparse rejects it.

Two behaviours differ from the retired `scripts/todo`, both stricter: a slug
appearing in two folders at once is a `check` failure rather than silently
resolving to whichever folder was read last, and a dangling need counts as
blocking rather than only surfacing in `check`.

## Priorities

When picking "the next" item, run `jj-kata kanban ready` to list claimable
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

1. **Claim the ticket:** from `default`, run `jj-kata claim <slug>`.
   This moves `<slug>.md` from its current folder into `wip/` and provisions
   a feature workspace at `.workspaces/<slug>`.
2. **Work in the feature workspace:** `cd .workspaces/<slug>` and do the actual
   implementation there.
3. **Integrate when done:** from `default`, run `jj-kata integrate
   <slug>`. This folds the feature into the default line and moves the ticket
   to `done/`.

A ticket in `wip/` is claimed and in progress — pick the highest-priority (see
Priorities) item from `critical/` or `planned/` (or `maybe/` if the user
directs) that doesn't conflict with active `wip/` tickets (same files, same
engine subsystem, or one item's dependencies naming the other). Run
`jj-kata kanban ready` to filter to items whose dependencies are all in `done/`.

Tickets tagged **[design]** require a design dialogue with the user before
implementation — claiming one means opening that conversation, not coding solo.
All CLAUDE.md jj constraints apply in full.

## Claiming additional todos into a live workspace

Partway through a feature you may realize the workspace should also take on
another todo ("oh, I guess we're fixing this too now"). Rather than spin up a
separate workspace, fold the extra todo into the one already going:

```
jj-kata claim <other-slug> --into <name>
```

This moves `<other-slug>.md` into `wip/` and **amends `<name>`'s claim commit** to
carry the move, so the single workspace `../<name>` now owns both tickets. Fold in
several at once with `claim <slug-a> <slug-b> --into <name>`. Notes:

- Run it from `default`, like every other claim. `<name>` may be any live
  workspace — a `claim`ed feature or an ad-hoc `start`ed one.
- Amending the claim commit leaves `../<name>`'s working copy **stale** (its parent
  was rewritten). That is routine: in `../<name>`, run `jj workspace
  update-stale` (commit your work first) before your next commit.
- `integrate <name>` then finishes **every** todo the claim owns — each moves
  `wip/ → done/`, all in one completion commit — and `drop --force --return-items
  <name>` reverts them all back to triage. (Ownership is derived from the
  feature's own tree against its base revision, so it is always exactly the todos
  that claim brought into `wip/`.)

## Census

Census tables for card shapes, keyword abilities, keyword actions, and ability
words live in `census.md` alongside this file. It is a **reference table, not a
ticket database**: the per-mechanic Modern card counts are prioritisation data,
and nothing reads the file mechanically. Its rows were once synthesised into
graph nodes claimable by name; that machinery is gone, and no ticket's `needs:`
ever pointed at one. A mechanic worth working gets a real ticket file like
anything else.
