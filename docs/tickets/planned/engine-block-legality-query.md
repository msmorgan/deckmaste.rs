---
needs: []
---
Block legality is currently validate-on-submit only (decide.rs DeclareBlockers
arm: point-wise `Cant(Block)` rows via `block_forbidden_by`, arrangement bounds
via `arrangement_forbidden_by` (menace), Must(Block) demand sets). Two
consequences the user ruled against (2026-07-12):

1. **Surfacing:** `DeclareBlockers.legal` lists blockers that cannot legally
   block anything (a ground creature when every attacker point-wise forbids
   it, e.g. a lone flyer). Refine `legal_blockers`: surface a blocker only if
   at least one declared attacker is point-wise permitted for it. Arrangement
   bounds must NOT prune here — a lone menace-blocker is illegal alone but
   legal with a partner, so it stays surfaced.

2. **Proposal query:** there is no way to ASK whether a proposed whole
   assignment would be legal without submitting it. Menace makes this
   set-level (no per-pair enumeration can answer it). Extract the decide.rs
   validation into a public pure query on GameState — shape sketch:
   `validate_blocks(&self, pairs: &[(ObjectId, ObjectId)]) -> Result<(), _>`
   (same reason strings as the submission errors) — and have the submission
   arm call it, so query and enforcement can't drift.

3. **Consumers (the play error):** `StrategyEvaluator::decide_blocks` proposes
   blind pairs (BlockAll round-robin / ChumpBiggest) while `sim::play`
   `.expect`s legality — a flyer attacking under BlockAll with a ground-only
   board panics the sim. Once the query exists, `decide_blocks` filters/repairs
   its proposal through it (drop forbidden pairs; drop arrangements the bounds
   reject), and the sim's "a strategy submits only legal decisions" expectation
   becomes honest.

Consider (cheap, same pattern): the symmetric probe for DeclareAttackers
(attack restrictions/requirements are also validate-on-submit).
