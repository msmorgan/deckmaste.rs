---
needs: []
---
# Data batch 2: missing frames, nouns, compounds and plugin frame data that existing routes already read

Priority: high. Batch data ticket (owner decision 2026-10-10: batching applies
to declarations an existing Construction already reads). Source: the blocker
report at trunk `vwlkmvns` (5,792 unread faces) and its 2026-10-10 triage;
every item below has a measured fail/control probe pair. Face figures are
sole-blocker upper bounds, not forecasts.

## Goal

Make each item read through routes that exist today, by declaration only.

| # | Declaration | Failing probe | Control that reads | Faces |
|---|---|---|---|---:|
| 1 | Verb *end*: frames `[]` and `[ObjectNounPhrase]` | "End the turn." / "You don't lose this mana as steps end." | "Sacrifice the turn." | ~29 |
| 2 | Common noun *flip* (the verb exists) | "If you lose the flip, draw a card." | "If you win the game, draw a card." | 43 |
| 3 | Common noun *total* and the compound *life total*, if batch 1 left a gap | "X is your life total." | "…equal to your life." | ≤44 |
| 4 | Compound *mana ability* | "It is a mana ability." | "It is an ability." | 40 |
| 5 | *remove*: add `Predicate([ObjectNounPhrase])` | "Add {R} for each counter removed this way." | "…each creature sacrificed this way." | 35 |
| 6 | Source Complement admits the *combat* turn part | "Remove it from combat." | "Remove a counter from this land." | 17 |
| 7 | *put*: frame Object + *back* (optionally followed by the manner or locative phrase its other frames take) | "Put them back." | "Put them on top of your library in any order." | 32 |
| 8 | Locative pro-form *there* as a Locative Complement | "It was put there." | "It was put onto the battlefield this turn." | 31 |
| 9 | *game*: `NominalAdjunctClass: Temporal` | "You cast a spell this game." | "You cast your commander this turn." | 20 |
| 10 | *any* as a temporal-adjunct dependent (bare *any time*) | "You may look at the top card of your library any time." | "…at any time." | ~35 |
| 11 | *devotion*: noun Complement *to <colour(s)>* | "Your devotion to black is 5." | "Its power is 5." | 20 |
| 12 | `transform.ron`: frames `[]`, Object and *into* NP | "When this permanent transforms, draw a card." | "When this permanent becomes …" | 37 |
| 13 | `surveil.ron`: add the bare frame `[]` | "Whenever you surveil, …" | "Whenever you scry, …" | ~10 |
| 14 | `typecycling.ron`: supertype + type stem (*basic land*) | "Basic landcycling {2}" | "Landcycling {2}" | 34 |

## Analysis

Orchestrator rulings 2026-10-10:

- D1. An item is in scope only while it stays a declaration. If one turns
  out to need a Construction, a frame kind, a Role, a feature or a loader
  change, drop it to the follow-up list with the evidence and continue; do
  not STOP for a single item.
- D2. Item 5: "A counter was removed from this land." and "This creature is
  removed from combat." also read 0. Diagnose in one paragraph whether the
  passive of *remove*'s Source frame is a data gap (fix it here) or a grammar
  gap (follow-up list).
- D3. Items 12–14 edit only `grammar:` fields of `plugins_v2` definitions;
  plugin bodies are untouched. `cargo xtask facts check` after.
- D4. Attested frames and categories only; the short declaration form;
  mechanically derived forms are exempt from the pruning rule (R16e).
- D5. One lexical reading per use: no Reading-count increase on a previously
  covered face. An increase is a defect: narrow the declaration or drop the
  item.

## Method

Standard constraints apply. Deltas: one bounded census after all
declarations on every supported face matching any item's surface pattern
(`cargo xtask english-v3 grep -i --names`, unioned) plus 300 random covered
controls, `--workers 6 --samples-per-face 0` under `/usr/bin/time -v`, compared
with `cargo xtask english-v3 diff` against the claim parent. Zero lost, zero
increases on covered faces. Tests: one table-driven test, each row pinning
the new declaration's role (not the whole unit), plus lexical-source tests per
declaration class. Full suites and clippy for every crate whose code or
consumed data changed. No grammar edits.

## Landing record

Six items as specified in `docs/tickets/README.md`; item 1 adds a per-item
table (item, where declared, faces gained) and item 4 the follow-up list.
