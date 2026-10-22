---
needs: []
---
# Fix keyword-action numbers and misclassified mechanics in the ADRs and census

Three documents misnumber keyword actions or file a mechanic under the wrong
kind of rule. The Lean citations are already right in each case. Only the
documents need fixing.

Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-docs.md`
entries 16, 17 (the ADR part) and 47 (gitignored; read it from the default
checkout). All are confirmed at head as of 2026-10-21.

## `docs/decisions/semantics-v3.md` §2.1, "Complete Classification" (lines 262, 264)

- Five cites are off by one, because behold is [CR#701.4]. Correct them:
  `cast` [CR#701.5], `counter` [CR#701.6], `create` [CR#701.7],
  `manifestDread` [CR#701.62] and `endure` [CR#701.63]. Every other cite in
  the two bullets matches its rule's heading.
- `plot` is listed as a keyword action at the slot that [CR#701.62] occupies.
  Plot is a keyword ability [CR#702.170a]. `firebend` is listed at the slot
  that [CR#701.68] occupies. Firebending is a triggered keyword ability
  [CR#702.189a]. Move both to the ability lists.
- The list calls itself complete but omits these in-scope actions: behold
  [CR#701.4], adapt [CR#701.46], venture into the dungeon [CR#701.49],
  harness [CR#701.64], blight [CR#701.68], heal [CR#701.69], recruit
  [CR#701.70] and empower Jace [CR#701.71]. Add each one that is Vintage-legal,
  or drop "Complete". Leave out the actions that are out of scope by
  CLAUDE.md (planeswalk, set in motion, abandon) and those of the deferred
  Un-sets.

## `docs/decisions/semantics-v3.md` off-stack statics (line 565)

- This line lists daybound and nightbound under `StaticSpec.whileHolds` state
  statics [CR#702.131b,702.145b,702.195a]. The statics that daybound and
  nightbound represent are of three kinds: an entry replacement ("enters
  transformed"), an event static ("As it becomes night"), and a can't-transform
  restriction [CR#702.145b,702.145e]. None of them is a state static. The
  any-time transforms and initialisations [CR#702.145c,702.145d,702.145f,702.145g]
  are game rules for the engine to apply. They are not the card's statics.
  Remove daybound and nightbound from the `whileHolds` examples, and list them
  under `asEvent` plus the entry replacement. Ascend and storied stay
  [CR#702.131b,702.195a]. Lean's `Keywords.lean` already uses
  replace/asEvent/cant here.
- `docs/archive/decisions/semantics-v2.md` (line 624) makes the same claim.
  That file is archived, so add an erratum note that points to the corrected
  semantics-v3 text rather than editing the archived ruling.

## `docs/decisions/english-v4.md` §2 (line 40)

- The keyword-action catalog list includes `firebend`, which is a triggered
  keyword ability [CR#702.189a]. Remove it from the list. (`earthbend`,
  `waterbend` and `airbend` are keyword actions [CR#701.66,701.67,701.65].)
  The lexical catalog already files firebending as a keyword ability
  (`crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron`).
  Only the ADR is wrong.

## `docs/tickets/census.md` (line 64)

- The Ninjutsu row reads "special action: swap unblocked attacker". Ninjutsu
  is an activated ability that functions from the hand and uses the stack
  [CR#702.49a]. Reword the row: activated ability (from hand) that returns an
  unblocked attacker you control and puts this card onto the battlefield
  tapped and attacking.

## Out of this ticket

The engine half of evidence entry 17 is not here: the day/night machinery
listed in `engine-day-night.md`. It is reported to the engine owner.
