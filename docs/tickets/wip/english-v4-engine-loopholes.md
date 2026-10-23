---
needs: []
---
# Engine loopholes: gap_followed, selected-PP final preposition, item_at_dot

Implements owner-rulings-12b findings D1, D2 and E (the round-12 re-scan's
additional findings). Each is an unverified audit claim: verify the mechanism
and find a witness before changing code.

## Change

- D1: `interleaved` (compile.rs) does not set `gap_followed` when a
  gap-introducing production places an adjunct at the right edge, so
  `gap_right_edge` is not cleared. Proposed: set `gap_followed` there.
- D2: `final_preposition` / `unextended` (step.rs) fires on any non-head
  daughter, including a noun's selected PP complement. Proposed: restrict it
  to unselected postmodifiers.
- E: `item_at_dot` (reading.rs) follows `families.first()`, so an ambiguous
  prefix split at different indices can give a forest key that differs from
  the tree `ranking_key`.

For each: with a witness (a corpus unit or judgment sentence showing a wrong
admission or rejection, or for E a forest/tree key mismatch in the
`ranking_keys` cross-check), fix it generally and pin it with a judgment or
test; with no witness, change no code and record "no witness" here with the
evidence.

## Acceptance

- No new Summary, State or ProductionDecl field; nothing ungrammatical admitted.
- No face lost; judgments' positive assertions hold; `strata --check` passes.
- Tests and clippy pass for changed crates.

## Outcome

- D1, no witness, no code change. Each `valence_order` draft already carries
  the right `gap_followed`. The `prefix` introduces no Gap, and the `more`
  production has none. The `/interleavedN` suffix keeps the slot order of the
  draft it splits, and its head (the prefix) precedes every slot, so the
  original `!later.is_empty() && !shiftable` still holds. A Gap before the
  split is followed by the remaining slots. A Gap after the split keeps its
  original right-edge status, and a shiftable Object Gap stays shiftable. The
  proposed one-liner would wrongly mark a Gap in the suffix's last slot as
  followed. Applied as a probe, it left the census report byte-identical.
  The other Gap-introducing productions are `$.Gap` (SubjectGapClause, which
  has no coordination and so never reaches Right Node Raising) and the
  stranded preposition (Gap at the right edge). Neither shows the defect.
- D2, no witness, no code change. Only LocatedExistentialBe's Complement
  closes prepositions (`In`, `On`, `Among`, `Under`, `Outside`, `After`), and
  that set alone fills `unextended_prepositions`. No noun in the lexicon or
  catalogs selects a PP Complement headed by one of them: every noun PP
  Complement uses *of*, *to*, *between* or *for*. A noun's selected PP
  Complement therefore never sets `final_preposition`, and the proposed
  restriction would be a no-op.
- E, witnessed and fixed. *Return to the battlefield tapped creature cards
  at random that were put into your graveyard this turn.* has
  PostposedRelative prefixes whose head and Object meet before or after
  *tapped*, and they pack into one item. `item_at_dot` followed only the
  first family, so *at random* read one head's words for every family. The
  result was forest keys of 16/22/27/33 Right Association words where the
  trees have 20/24/31/35. The forest walks (`tier_item`, `item_sequences`)
  now keep one result per `Anchors`: the extents of the head and first
  conjunct that a later child's key reads. `edge_key` reads those extents,
  and `item_at_dot` is gone. The corpus has no such packed prefix (an
  instrumented census found none), so the census is byte-identical.
- Tests: added `forest_keys_follow_each_packed_prefix_s_own_division_of_the_text`
  (`tests/ranking_keys.rs`). It failed before the fix. None rewritten or
  retired.
- Statistics: the cheapest tier is unchanged (singleton in 15,624 of 18,344
  ambiguous units, summed 21,723, largest 8). S/A/P/F is 513/163/9/89 before
  and after.

Standard constraints apply.

1. **Census delta** (measured on `yktlsnxr` → `mtnxvwxz`): faces read 28,801 →
   28,801; total Readings 245,781 → 245,781; Surplus 106,817 → 106,817; lost
   faces: none.
2. **Judgment set**: 3,052 enforced at `COMPLETED_STRATUM` 6, all hold; 206 pending.
