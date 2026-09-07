---
needs: [semantics-v2-crate, lean-card-soundness-gate]
---
**Give every `plugins_v2/builtin/macros/stubs/turn_parts` declaration its semantic
`body`** (19 declarations, all bodyless today). Each body is a term-for-term
expansion over the v2 basis with no default slots, one macro per printed
phrase shape, invoking other macros where the CR definition does. The Lean
gate checks each body as it lands; Lean `Macros.lean` is the reference for
shapes already modelled there. Mechanical per declaration once the family's
shape is settled: terra tier. Standard constraints apply.

`TurnPart` is a closed enum in Lean; each body is the constructor the
printed word denotes (`Upkeep` and `UpkeepStep` both `Upkeep`), `params:
[]`. The body is the printed-word to constructor map, nothing more.

## Landing record

**Bodies given: 17 of 19.** The remaining 2 are genuine STOPs (no matching
Lean constructor), routed to `lean-turn-part-steps`.

### The macro_ron blocker — resolved mid-ticket

The first pass (below, historical) found 8 declarations blocked by a false
positive in `macro_ron`'s register-time `check_cycles`: a declaration whose
body names its own constructor (`Upkeep.ron`'s body `Upkeep`, etc.) was
rejected as a self-invocation cycle, because `crates/deckmaste_semantics_v2/src/ron.rs`'s
`kinds()` never registered `TurnPart::kind()`, so the code's own "identity
macro spells its own variant" exemption never fired. The coordinator landed
the one-line fix on trunk (`TurnPart` now derives `SupportsMacros` and is
registered with its variants) and refreshed this workspace onto it; a new
regression test, `a_turn_part_declaration_may_name_its_own_constructor`, now
guards it. All 8 declarations were given their (already-known, previously
documented) bodies in this workspace once the fix landed; their STOP comments
are removed.

### Final gate

- `cargo test -p xtask --test plugins_v2_declarations`: 4 passed, 0 failed —
  `both_readers_accept_every_builtin_declaration`,
  `a_turn_part_declaration_may_name_its_own_constructor`,
  `an_ability_word_invocation_expands_at_an_ability_position`,
  `a_keyword_action_invocation_expands_at_an_instruction_position` all `ok`.
- `cargo xtask lean-check plugins_v2/canon` (no baseline — the ratchet
  retired with the coordinator's fix): `23/23 card(s) prove Card.check = []`
  (trunk's canon now carries 20 cards from sibling landings alongside this
  ticket's 3), 4.8s.
- `cargo xtask gate --changed --run` derived line:
  `cargo test -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p deckmaste_semantics_v2 -p xtask`
  — every reported `test result:` line `ok`, 0 failed; totals ~1517 passed,
  0 failed, 1 ignored (pre-existing, unrelated to this ticket) across the
  closure.
- `cargo xtask cite check --list-noncompliant` — empty. `cargo xtask cite
  check` (full) reports 19 stale, all in `plugins_v2/builtin/macros/stubs/keyword_actions/`
  and `plugins_v2/canon/cards/{Aeromunculus,Inaction Injunction}.ron` —
  sibling-ticket files this ticket never touched (confirmed: this session's
  only uncommitted diff before the final commit was the 8 turn_parts files).
  Not this ticket's citations to fix.
- `cargo fmt --all` — no `.rs` changes (this ticket only touches `.ron`).

### 17 bodied declarations

`BeginningPhase→BeginningPhase`, `Cleanup→Cleanup`, `Combat→Combat`,
`DrawStep→DrawStep`, `EndStep→EndStep`, `MainPhase→MainPhase`,
`UntapStep→UntapStep`, `Upkeep→Upkeep` (the 8 unblocked this round),
`CleanupStep→Cleanup`, `CombatDamageStep→CombatDamage`, `CombatPhase→Combat`,
`DeclareAttackersStep→DeclareAttackers`, `DeclareBlockersStep→DeclareBlockers`,
`EndOfCombatStep→EndOfCombat`, `PostcombatMainPhase→PostcombatMain`,
`PrecombatMainPhase→FirstMain`, `UpkeepStep→Upkeep` (the original 9).

### 2 remaining STOPs — routed to `lean-turn-part-steps`

No matching Lean constructor exists; named per the ticket's own instruction,
never approximated. Both still carry their STOP comments, unchanged:

- **`BeginningOfCombatStep`** (spelling "beginning of combat step"):
  `lean/Semantics/Words.lean`'s `TurnPart` has no `beginningOfCombat`
  constructor distinct from the `Combat` phase itself, even though
  [CR#506.1] names "beginning of combat" as one of the combat phase's five
  steps alongside declare attackers/declare blockers/combat damage/end of
  combat (all four of which DO have constructors).
- **`EndingPhase`** (spelling "ending phase"): the enum has `beginningPhase`
  but no symmetric `endingPhase`, even though [CR#500.1] names "ending" as
  one of the turn's five phases.

Both need a Lean-side addition (`lean/Semantics/Words.lean`'s `TurnPart`
enum plus its Rust mirror), out of this ticket's `crates/`/`lean/`
authorization; routed to the new ticket `lean-turn-part-steps`.

### Additions

- `plugins_v2/canon/cards/{Eon Hub,Moment of Silence,Angus Mackenzie}.ron`
  (real card text, verified with `jq` against `data/mtgjson/AtomicCards.json`):
  `Eon Hub` exercises `UpkeepStep` via `StaticSpec::PartSkip`, `Moment of
  Silence` exercises `CombatPhase` via `Instruction::SkipPart`, `Angus
  Mackenzie` exercises `CombatDamageStep` via `Timing::BeforePart` on an
  activated ability with `StaticSpec::DamageRule`. A fourth candidate, "Fog
  Patch" (`DeclareBlockersStep`), was built and discarded: modelling
  "attacking creatures become blocked" as `Combat{update:
  Participation{state: Blocked(true)}}` on an `InCombat(relation:
  AttackerOf)`-described subject produced a real `Card.check` refusal
  (`Refusal.deedNounOk (Deed.core CoreDeed.attack)`) — a genuine law
  violation from an unfamiliar deed-grammar interaction, not force-fit into
  passing.
- No helper macros added; no other declarations touched.

### Deviations from the original brief

- Fixture cards live in `plugins_v2/canon/cards/`, not `plugins_v2/testing/cards/`
  (mid-task coordinator amendment).
- No `lean-check-baseline.ron` is tracked for `plugins_v2/canon` — the
  ratchet retired (second mid-task amendment, then landed on trunk).
- Only 3 of the 17 working macros are exercised by this ticket's own fixture
  cards — the other 14 have no fixture here, since correctness for all 17
  rests on `plugins_v2_declarations`'s registration/drift test plus the
  `Card.check` law proof of the 3 representative samples (`UpkeepStep`,
  `CombatPhase`, `CombatDamageStep`) under the identical, now-fixed
  expansion mechanism.

## Handoff

None — the family is complete except the 2 STOPs, now routed to
`lean-turn-part-steps`. Park with `@` empty; do not integrate.
