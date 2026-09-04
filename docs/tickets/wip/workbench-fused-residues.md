---
needs: []
---
**Close the two shapes fused-variants-3 stopped on that are not the look bodies.** Residue of
`workbench-fused-variants-3` (2026-09-04); each was a STOP with its evidence
in that ticket's landing record.

- **Skip-duration re-read.** `Effect.SkipsAllOf` stays because both bench
  cards (Empty City Ruse, False Peace) say "of their next turn", and
  `Duration.DuringNextTurnOf who` cannot be reached from `Continuously`'s
  span: the span is typed at `staticIntro se` and `staticIntro (Skips _ _) =
  bs`, so it cannot re-read the skipping player. Decide between
  `staticIntro (Skips who _) = nomIntro who` (moves every `Static (Skips …)`
  bench card) and a duration that re-reads the static's subject; then fold
  `SkipsAllOf` away.
- **Look bodies** — ruled 2026-09-04 and split out as `workbench-one-scry`.
- **Spell-level cast timing.** "Cast this spell only before the combat damage
  step" (Berserk, Blood Frenzy) needs a timing slot reachable from a spell:
  `Triggers.Timing` is reachable only from `Effect.AbilityAt.Activated`. Add
  the positional slot (RON-shaped), bench Berserk through `BeforePart
  CombatDamage`, pin a CR-meaningless timing on a spell.

Size: M. Done when: `SkipsAllOf` is gone or its retention is ruled; Berserk
is benched; build at its module count.
Standard constraints apply, including the RON-shaped constraint.
