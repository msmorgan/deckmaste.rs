---
needs: []
---
**Close the three shapes fused-variants-3 stopped on.** Residue of
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
- **Agent re-read for the look bodies (review R6).** `LookReq` is two
  constructors, but `Macros.scryBody`/`surveilBody` still carry four clauses
  because `may`'s decider is typed at `agentIntro decider`, `move`'s
  `Movable` search needs concrete bindings, and `nounDelta You = []` leaves
  nothing for an `Own`-style read to reach. This is a grammar-level choice —
  a core "same as the agent" `Noun bs Player`, or an `agentIntro` that binds
  the agent uniformly — and needs a ruling before implementation.
- **Spell-level cast timing.** "Cast this spell only before the combat damage
  step" (Berserk, Blood Frenzy) needs a timing slot reachable from a spell:
  `Triggers.Timing` is reachable only from `Effect.AbilityAt.Activated`. Add
  the positional slot (RON-shaped), bench Berserk through `BeforePart
  CombatDamage`, pin a CR-meaningless timing on a spell.

Size: M (the second bullet gated on a ruling). Done when: `SkipsAllOf` is
gone or its retention is ruled; the look bodies are one clause each or the
ruling records why not; Berserk is benched; build at its module count.
Standard constraints apply, including the RON-shaped constraint.
