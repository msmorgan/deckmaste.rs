---
needs: []
---
**Remove the declarations for mechanics outside the project's scope, and say
in CLAUDE.md that Unfinity is out for now.** Split from
`plugins-v2-keyword-body-defects` on 2026-10-05. Standard constraints apply.

## What is decided

- **Removed:**
  - `hiddenAgenda` — Conspiracy [CR#702.106a]; 15 cards, none legal.
  - `visit` — Attractions [CR#702.159a,717.1]; 50 entries, none
    Vintage-legal.
  - `spaceSculptor` and the `sector` designation [CR#702.158a] — Space
    Beleren is Unfinity. Owner: "Space Beleren is in a funny set. we'll add
    support for the unfinity cards at a later time, but they're a pain in the
    ass we can justifiably avoid dealing with at this time".
- **Kept:** `assemble`, as a bare verb with no body. Owner: "assemble being
  blank seems in line with CR". Steamflogger Boss is a Future Sight card, and
  the rule itself says "Cards and mechanics from the Unstable set aren't
  included in these rules" [CR#701.45a].
- **CLAUDE.md.** Add one clarifying clause to the Scope rule: Unfinity's
  eternal-legal cards are excluded for now with the rest of Unfinity. Leave
  the word "permanently" alone.

## The work

1. Delete `plugins_v2/builtin/macros/keyword_abilities/hiddenAgenda.ron`,
   `visit.ron`, `spaceSculptor.ron` and `designations/sector.ron`.
2. Remove the matching rows wherever a table lists them: the Lean facts table
   (`lean/Semantics/Check/Facts.lean`, the `HiddenAgenda`, `SpaceSculptor` and
   `Visit` rows), `crates/xtask/src/facts.rs` (the `HiddenAgenda`,
   `SpaceSculptor`, `Visit` rows and the three `…Sector` designations), and
   anything `cargo xtask facts generate` regenerates.
3. Tests that list the removed names lose those entries:
   `crates/deckmaste_construction_core/tests/builtin_v2_designations.rs`
   (`"sector"`), `crates/deckmaste_construction_core/tests/builtin_v2_keyword_abilities.rs`
   (`"visit"`, twice). The Lean pins in `lean/Semantics/Proofs/Tables.lean`
   whose subject is the sector designations
   (`everySectorLabelIsAnEffectfulDesignation`,
   `anUndeclaredSectorIsNotADesignation`) assert properties of designation
   tables: re-spell each against another declared designation if the property
   still has a subject, and remove it only if not. Search for further sites
   before deleting (`rg -i 'hidden.?agenda|space.?sculptor|sector|\bvisit\b'`).
4. Add the CLAUDE.md clause.

## Proof (Assurance accounting)

The landing record names every test entry and pin removed or re-spelled, each
with this ticket's reason (out of scope: Conspiracy, Attractions, Unfinity),
and gives the counts: restored, re-spelled, ignored with blockers, added,
removed. Docs that describe these mechanics (`docs/designations.md`,
`docs/rules-taxonomy.md`) are listed in the record; edit them only to drop the
removed rows.

## Out of scope

`assemble`'s body (it stays blank).
