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

Standard constraints apply.

1. **Census delta**: not yet measured.
2. **Judgment set**: not yet measured.
