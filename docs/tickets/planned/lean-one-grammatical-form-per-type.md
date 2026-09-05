---
needs: []
---
**One grammatical form per type; spelling and ordering only, no semantic
change.** The 2026-09-05 review found the Lean constructors and macros mixing
imperative, third-person and noun forms with no rule other than the card
sentence each was first written for (`exile x` beside `exiles agent x`,
`sacrifice agent x` beside `mills agent n whose`, `StaticSpec.gains` beside
`StaticSpec.modify`). Fix, in one landing:

- `Instruction` constructors and the instruction macros in
  `Semantics/Macros.lean` use the dictionary form; the agent is an optional
  trailing named argument, so `exile x` and `exile x (agent := you)` replace the
  `exile`/`exiles` pair, `mills` becomes `mill`, `chooses`/`secretlyChooses`
  fold into `choose`, `losesLife`/`gainsLife` become `loseLife`/`gainLife`.
- `GameEvent` constructors stay third-person: an event is a predication.
- `StaticSpec` constructors become nouns naming the kind of continuous effect
  (`modify` → `modification`, `gains` → `abilityGrant`, `costs` → `costShift`,
  `skips` → `partSkip`, `becomes` → `qualityChange`, `intercepts` →
  `replacement`, and so on); the claimant lists the full map in the landing
  record.
- `CounterBatch.last` is renamed for what it is, the removal that leaves none
  of that kind (`emptying`), and its docstring records that the emptiness is
  part of the trigger event, read at trigger time, and NOT an intervening if:
  [CR#603.4] rechecks an intervening if on resolution, and the CR itself
  writes the phrase as a trigger event ([CR#310.12b], [CR#702.62a], where a
  separate "if it's exiled" follows it). Do not re-express it as a condition.
- `TurnPart` variants are reordered to the [CR#500.1] sequence (turn, beginning
  phase, untap, upkeep, draw, main, first main, combat, declare attackers,
  declare blockers, first-strike damage, combat damage, end of combat,
  postcombat main, end step, cleanup). Only `DecidableEq` is derived, so the
  order carries no meaning today; it is a reading aid.

Every pin keeps its name and expected list; the verdict-diff script with
spelling normalisation must report 0 problems. Update
`docs/contexts/game-model/CONTEXT.md` entries **Instruction** and **Static
Spec** if their wording assumes the old forms.
