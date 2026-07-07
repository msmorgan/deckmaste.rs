---
needs: []
design: true
---
**Extend the Idris EventKind model to close the remaining idris-check emit
gaps.** After the scry recompose and the Do-or-Die/`DivideAndChoose` ticket,
canon idris-check sits at 51/57; the residual failures are event-model shapes
the Rust grammar expresses but the Idris north-star's `EventKind` cannot yet
represent. Each needs a north-star design decision, hence `maybe/` + `design`.

## The gaps (from `cargo xtask idris-check plugins/canon`)

1. **`ZoneChange { cause }`** — Collective Resistance, Darksteel Myr.
   *"Idris EventKind has no cause coordinate."* Rust carries the cause-verb
   (sacrifice/destroy/mill/…) on the zone-change fact; the Idris `EventKind`
   ZoneChange has no `cause` facet, so "when a creature dies from being
   sacrificed"-style filters can't emit. Add a cause coordinate to the Idris
   zone-change event.
2. **`BlockDeclared { of }`** — Deepwood Tantiv. The defender-side / block
   relation has no Idris `EventKind` counterpart ([CR#509.1]). Needs the
   block-declared event (attacker-side `AttackDeclared` exists; the block side
   and its `of` role do not).
3. **`BecomesTarget`** — Phantasmal Bear. *"no Idris EventKind counterpart"*
   (a becomes-target trigger event, [CR#603.2]). Add the becomes-target event
   with its targeting-object vs source facets.
4. **Arcane subtype** — Otherworldly Journey. *"unmapped subtype: Arcane."* Not
   strictly event-model — a subtype-registry gap; the smallest fix (add Arcane
   to the Idris subtype axis) rides along here.

## Relationship to `engine-relation-spine`

`engine-relation-spine` (maybe/) proposes ONE `Relation` enum projected into
durative/inchoative/deontic aspects — which would subsume the block-declared and
becomes-target gaps structurally (its item 2, the defender-side event family).
If that refactor is taken, gaps 2–3 fold into it; if not, this ticket closes
them pointwise. Gap 1 (zone-change cause) and gap 4 (subtype) are independent of
it either way.

## Done

- idris-check on canon reaches 55/57 (or better), with only genuinely
  out-of-scope shapes remaining; each newly-supported card emits and typechecks.
- New Idris `EventKind` facets carry CR-cited caps rows, consistent with the
  master-form event model.
