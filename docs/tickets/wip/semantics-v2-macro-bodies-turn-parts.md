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

**Bodies given: 9 of 19.** The rest are blocked or STOP, not approximated.

### Gate

- `cargo test -p xtask --test plugins_v2_declarations`: `both_readers_accept_every_builtin_declaration ... ok` (1 passed).
- `cargo xtask gate --changed --run` derived line:
  `cargo test -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p deckmaste_semantics_v2 -p xtask`
  — every suite green (0 failed across all reported `test result:` lines).
- `cargo xtask cite check` — 15110 citations checked, 0 stale; `--list-noncompliant` — empty.
- `jj diff --git --from ktmmrrxm --to @ | cargo xtask cite audit --diff` — 19 sites in `stubs/turn_parts/`, all read on-topic against the quoted CR text.
- `cargo xtask lean-check --bless plugins_v2/canon`: `3/3 card(s) prove Card.check = []`, 0 recorded failures. Wall time on `plugins_v2/canon`: 0.2s (tool-reported), 0.325s real (`time cargo xtask lean-check plugins_v2/canon`). Baseline file removed before commit per the lean-check-baseline-ratchet-retirement ruling (mid-task amendment); not tracked.
- `cargo fmt --all` — no `.rs` changes (this ticket only touches `.ron`).

### 9 bodied declarations (working, verified via the gate above)

`CleanupStep→Cleanup`, `CombatDamageStep→CombatDamage`, `CombatPhase→Combat`,
`DeclareAttackersStep→DeclareAttackers`, `DeclareBlockersStep→DeclareBlockers`,
`EndOfCombatStep→EndOfCombat`, `PostcombatMainPhase→PostcombatMain`,
`PrecombatMainPhase→FirstMain`, `UpkeepStep→Upkeep`.

### 2 STOPs (no matching Lean constructor — named per the ticket's own instruction, never approximated)

- **`BeginningOfCombatStep`** (spelling "beginning of combat step"): `lean/Semantics/Words.lean`'s `TurnPart` has no `beginningOfCombat` constructor distinct from the `Combat` phase itself, even though [CR#506.1] names "beginning of combat" as one of the combat phase's five steps alongside declare attackers/declare blockers/combat damage/end of combat (all four of which DO have constructors). Left bodyless.
- **`EndingPhase`** (spelling "ending phase"): the enum has `beginningPhase` but no symmetric `endingPhase`, even though [CR#500.1] names "ending" as one of the turn's five phases. Left bodyless.

### 8 declarations BLOCKED by a confirmed macro_ron false positive (not modelling gaps — bodies are known and documented in each file's own comment)

`BeginningPhase`, `Cleanup`, `Combat`, `DrawStep`, `EndStep`, `MainPhase`, `UntapStep`, `Upkeep` — every declaration whose file name is spelled identically to its target `TurnPart` constructor (the "canonical" word for that constructor, with no diminutive/suffixed sibling to indirect through).

**Root cause.** `crates/macro_ron/src/set.rs`'s `check_cycles` treats any declaration body whose leading identifier textually equals `def.name` as an unconditional self-invocation cycle (`InsertError::Cycle`), *regardless* of whether that identifier is actually meant as raw enum data. The code has a documented escape hatch for exactly this ("an identity macro whose body spells its own variant... is not a self-cycle") gated on `self.kinds.get(kind).is_some_and(|k| k.variants.contains(&name))` — but `crates/deckmaste_semantics_v2/src/ron.rs`'s `kinds()` never calls `crate::words::TurnPart::kind()` (only `Window::kind()` is registered from `Words.lean`, since `TurnPart` carries no `[semantic_expression]` attribute there), so the exemption's variant list is always empty for kind `TurnPart` and the escape hatch never fires.

**Confirmed a false positive, not a real recursion, by two probes** (built, run, and discarded before the final fixture set): (1) a throwaway card writing `part: Upkeep` directly, with `Upkeep.ron` left bodyless, proved `Card.check = []`; (2) `UpkeepStep.ron` (body: `Upkeep`, referencing the still-bodyless `Upkeep.ron`) invoked from a card also proved `Card.check = []`. Real expansion substitutes a declared macro's stored body text exactly once at the usage site, then deserializes the result directly against the field's concrete type (`TurnPart`, not itself a `[semantic_expression]`/recursive position) — it never re-dispatches the substituted identifier as a further macro invocation. The static register-time check has no way to know this and conservatively (here, wrongly) assumes unbounded recursion is possible.

**Not fixed here**: `CLAUDE.md`'s "Do not edit `lean/`, `crates/`, or `docs/decisions/` unless your ticket says so" is explicit and this ticket doesn't authorize it; the fix (`kinds.add(crate::words::TurnPart::kind());` in `ron.rs`'s `kinds()`, mirroring the `Window::kind()` line already there) is one line and matches an established pattern, but touches a crate shared by the other three `semantics-v2-macro-bodies-*` tickets running in parallel, so it belongs to the coordinator to land once, not to four workspaces independently.

**Intended bodies**, ready to paste in once the crate fix lands (each already lives, uncommitted-as-a-body, in the file's own STOP comment): `BeginningPhase→BeginningPhase`, `Cleanup→Cleanup`, `Combat→Combat`, `DrawStep→DrawStep`, `EndStep→EndStep`, `MainPhase→MainPhase`, `UntapStep→UntapStep`, `Upkeep→Upkeep`.

### Additions

- `plugins_v2/canon/cards/` (new directory, per mid-task amendment — real fixture cards belong in canon, not `testing`): `Eon Hub.ron` (`UpkeepStep`, via `StaticSpec::PartSkip`), `Moment of Silence.ron` (`CombatPhase`, via `Instruction::SkipPart`), `Angus Mackenzie.ron` (`CombatDamageStep`, via `Timing::BeforePart` on an activated ability with `StaticSpec::DamageRule`). All real card text, verified with `jq` against `data/mtgjson/AtomicCards.json`. A fourth candidate, "Fog Patch" (`DeclareBlockersStep`), was built and discarded: modelling "attacking creatures become blocked" as `Combat{update: Participation{state: Blocked(true)}}` on an `InCombat(relation: AttackerOf)`-described subject produced a real `Card.check` refusal (`Refusal.deedNounOk (Deed.core CoreDeed.attack)`) — a genuine law violation from an unfamiliar deed-grammar interaction, not force-fit into passing.
- No helper macros added; no other declarations touched.

### Deviations from the original brief

- Fixture cards live in `plugins_v2/canon/cards/`, not `plugins_v2/testing/cards/` (mid-task coordinator amendment).
- `lean-check-baseline.ron` was blessed to read the per-card verdicts, then deleted before commit and is not tracked (second mid-task amendment, ratchet retirement).
- Only 3 of the 9 working macros are exercised by fixture cards (not all 9) — the other 6 (`CleanupStep`, `DeclareAttackersStep`, `DeclareBlockersStep`, `EndOfCombatStep`, `PostcombatMainPhase`, `PrecombatMainPhase`) have essentially no real-card usage as literal "beginning of X" / "during X" trigger text at this specificity in `AtomicCards.json`; their correctness rests on the `plugins_v2_declarations` registration test plus the identical mechanism verified for `UpkeepStep`/`CombatPhase`/`CombatDamageStep`.

## Handoff

Next session (or the coordinator, after the shared `macro_ron`/`ron.rs` fix lands): apply the 8 intended bodies quoted above (delete each file's BLOCKED comment, add `body: <Ctor>,`), re-run the two gates in this record, and fold `BeginningPhase`/`Cleanup`/`Combat`/`DrawStep`/`EndStep`/`MainPhase`/`UntapStep`/`Upkeep` into the fixture-card sample if convenient (e.g. swap `Moment of Silence` for a card using the now-working `Upkeep`/`EndStep` directly, since those are the phrasings real oracle text actually uses).
