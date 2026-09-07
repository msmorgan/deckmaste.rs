---
needs: []
---
**`TurnPart` lacks two parts the CR names.** Found by
`semantics-v2-macro-bodies-turn-parts` (2026-09-06): the beginning of combat
step [CR#506.1] has no constructor distinct from the `combat` phase, and the
ending phase [CR#500.1] has no constructor symmetric to `beginningPhase`.
Add `beginningOfCombat` and `endingPhase` to `lean/Semantics/Words.lean`
with their citations, thread them through every `TurnPart` read in `Check/`
(window ordering, skip and insert laws), pin one card each (a real
"beginning of combat" trigger and a real ending-phase reference, verified
against the corpus), mirror both in `crates/deckmaste_semantics_v2`
(drift test), then give `BeginningOfCombatStep` and `EndingPhase` under
`plugins_v2/builtin/macros/stubs/turn_parts/` their bodies. Standard
constraints apply.

## Landing record

**Both constructors added, threaded, mirrored, pinned and bodied.** One STOP:
no Vintage-legal card names the ending phase, so `endingPhase` lands with Lean
pins and no canon card (details below).

### PROVE

- **No silent loss.** Nothing stopped being covered. No pin, theorem, test or
  declaration was deleted or weakened. `TurnPart` gained two constructors and
  lost none; every existing `TurnPart` term still elaborates and every existing
  pin still closes by `decide`.
- **Structural laws.** `lean/scripts/build` (Semantics + Cards + Proofs, 77
  jobs) completes successfully with no warnings. `cargo xtask lean-check`
  proves `Card.check = []` for 24/24 `plugins_v2/canon` and 2/2
  `plugins_v2/testing` cards — no baseline, every card proves.
  `cargo test -p deckmaste_semantics_v2 --test lean_drift`: 4 passed, 0 failed
  (Lean↔Rust name-for-name, field-for-field, and the emitter mapping
  round-trips on both new names). `cargo xtask facts check`: both generated
  fact files up to date.
- **No word-naming.** No law added or changed. Both new constructors are read
  only through the existing feature-reading laws — `TurnPart.proper`
  (`.turn => false`, `_ => true`) and `windowOk` (`.turn, none => false`,
  `_ => partPossessorOk`) — whose catch-alls already classify them correctly.
  No guard anywhere names `beginningOfCombat`, `endingPhase`, or a card.

### DISCLOSE

**Newly covered identities and their analyses.**

- `Semantics.TurnPart.beginningOfCombat` — the combat phase's own first step
  [CR#506.1], distinct from the `combat` phase. Proper (`TurnPart.proper` =
  `true`), so it is a legal window, a legal `beginningOf` header part, a legal
  `insertPart` part and anchor.
- `Semantics.TurnPart.endingPhase` — the turn's fifth phase [CR#500.1], the one
  holding the end and cleanup steps; symmetric to `beginningPhase`. Also
  proper.

**Threading through `Check/`.** Every `TurnPart` read in `lean/Semantics/Check/`
was surveyed (`grep TurnPart` plus each constructor name):
`Check/Words.lean` `TurnPart.proper`, `Check/Phrase.lean` `windowOk`,
`Check/Triggers.lean` (`duringPart`, `beforePart`), `Check/PhraseRules.lean`
(`duringPart`, `beginningOf`), `Check/AbilityRules.lean` (`skipPart`,
`insertPart` part/anchor/followedBy), `Check/Abilities.lean`
(`insertPart` binding profile, `part == .turn`). Every one of them
discriminates `.turn` against a catch-all and reads no other constructor, so
the two new parts are threaded by construction, with no edit and no
enumeration to keep in sync. **No `Check/` file needed changing** — that is the
finding, not an omission; the pins below prove each read behaves.

**Pins added (6, all `decide`-closed, in `lean/Semantics/Proofs/Turn.lean`).**
Three positive/negative twins:

| pin | sentence | outcome |
| --- | --- | --- |
| `okBeginningOfCombatPossessor` | "At the beginning of combat on your turn, draw a card." | `[]` |
| `badBeginningOfCombatPlural` | "At the beginning of combat on all players' turns, draw a card." [CR#102.1] | `[.windowOk]` |
| `okEndingPhaseWindow` | "{2}: Draw a card. Activate only during each player's ending phase." | `[]` |
| `badEndingPhaseWindow` | "{2}: Draw a card. Activate only during all players' ending phase." [CR#102.1] | `[.windowOk]` |
| `okAdditionalEndingPhase` | "After this postcombat main phase, there is an additional ending phase." | `[]` |
| `badAdditionalEndingPhaseAfterTurn` | "After this turn, there is an additional ending phase." | `[.windowOk]` |

The three pairs exercise, in order, the `beginningOf` header-possessor law, the
`duringPart` window law, and the `insertPart` part/anchor law — the three reads
the ticket named.

**Canon card (1).** `plugins_v2/canon/cards/Luminarch Aspirant.ron` — "At the
beginning of combat on your turn, put a +1/+1 counter on target creature you
control." ({1}{W} Creature — Human Cleric 1/1, Vintage-legal), text, cost,
types, subtypes, P/T all verified with `jq` against
`data/mtgjson/AtomicCards.json`. It selects `BeginningOfCombatStep` at the
`TurnPart` position of `GameEvent::BeginningOf`; the emitted term in
`lean/Generated/Canon.lean` reads
`Semantics.GameEvent.beginningOf Semantics.PartQuant.the
Semantics.TurnPart.beginningOfCombat (Semantics.HeaderPossessor.byPlayer
Semantics.NounPhrase.you)`, and proves `Card.check = []`.

Spelling decision: the printed possessive sits on the turn ("on your turn"),
but `HeaderPossessor` is *whose turn part* the header names, and `.byTurn`
takes a `turnRef` NounPhrase (the "that turn" anaphor). "Your beginning of
combat step" and "the beginning of combat step on your turn" are the same
semantics, so `ByPlayer(You)` is the analysis, exactly as
"at the beginning of your upkeep" is spelled.

**STOP — no Vintage-legal card references the ending phase.** The ticket asks
for "a real ending-phase reference" as a canon card. A `jq` sweep of
`data/mtgjson/AtomicCards.json` for `ending phase` (case-insensitive, all
cards) returns exactly two: **Clocknapper** (UST/ULST — Unstable, an un-set)
and **Runed Terror** (MB2 playtest, empty `legalities`). Both are outside
`CLAUDE.md`'s permanent scope rule (Vintage-playable Magic only). A wider sweep
of every Vintage-legal card line matching `ending` finds no ending-phase
reference at all. Resolution: `endingPhase` lands as CR vocabulary
(§16 inert-vocabulary reasoning does not apply — it *is* read, by
`TurnPart.proper` and `windowOk`) with two positive/negative pin pairs and its
declaration body, and **no canon card**. It is needed regardless: the
`EndingPhase` declaration exists in `plugins_v2/builtin/macros/stubs/turn_parts/`
and its STOP comment named this gap. Not force-fit onto an out-of-scope card.

**Declaration bodies (2).** Both STOP comments removed and replaced with the
family's ordinary citation comment:

- `stubs/turn_parts/BeginningOfCombatStep.ron` → `body: BeginningOfCombat`
- `stubs/turn_parts/EndingPhase.ron` → `body: EndingPhase`

That completes the turn-parts family at 19 of 19 bodied.

**Deviations and additions.**

- **Added beyond the ticket's letter: a one-line fix to `cargo xtask cite audit
  --diff`** (`crates/xtask/src/cite.rs`, `added_diff_lines`). It split the
  `+++ b/<path>` header on *whitespace*, so every path containing a space was
  truncated at the first space and its citation sites were silently dropped
  from the audit. Every canon card file name contains a space, so the audit had
  been silently skipping all of `plugins_v2/canon/cards/` — including this
  ticket's own new card, whose `[CR#506.1]` did not appear in the audit until
  the fix. A unified diff separates path from timestamp with a TAB, so the fix
  splits on `\t`. Regression test added:
  `cite::tests::added_diff_lines_keeps_a_path_containing_spaces`. Justification:
  the citation audit is a gate this landing must actually run, and it was
  under-reporting on the file this landing adds — an unambiguous defect in the
  landing's own path.
- No other constructions, laws, macros or tests added or deleted. No glossary
  gap: `docs/contexts/game-model/CONTEXT.md`'s turn vocabulary already covers
  phases and steps, and both new names are the CR's own printed names.

**Assurance counts.** Restored 0 · re-spelled 0 · ignored-with-blocker 0 ·
added 7 (6 Lean pins + 1 xtask regression test) · **removed 0**.

### REPORT

Provenance, measured on this tree: change id `vnlypwxu`, `plugins_v2/canon`
24 cards / `plugins_v2/testing` 2 cards, turn-parts declarations 19/19 bodied.
No coverage lock applies to this lane (no `deckmaste_english_v2` corpus change;
`english-v2-coverage.lock` untouched).

- `lean/scripts/build`: **success, no warnings**, 77 jobs. Cold-ish wall time
  56 s (Semantics 28 jobs, Proofs 59, Cards 77 cumulative); a no-op re-run is
  0 s. Host load average 25.7 (six parallel sibling workspaces on the same
  box), so this is a loaded-host figure, not a quiet-host one.
- `cargo xtask lean-check`: 2 plugins, 26 cards, 4.5 s cold / 0.6 s warm.
- `cargo xtask gate --changed` derived line:
  `cargo test -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p deckmaste_semantics_v2 -p xtask`
  — run with `--run`: 45 `test result:` lines, **all `ok`, 0 failed**, no
  panics, no `error:`.
- `cargo test -p deckmaste_semantics_v2 --test lean_drift`: 4 passed, 0 failed.
- `cargo xtask facts check`: both files up to date.
- `cargo xtask cite check --list-noncompliant`: **0** non-compliant.
  `cargo xtask cite check`: 15208 citations against cr.txt (eff. 2026-08-07),
  **0 stale**. No `cite bless` needed — [CR#500.1], [CR#506.1] and [CR#102.1]
  are all already registered in `cr-citations.lock`.
  `jj diff --git | cargo xtask cite audit --diff`: **14** citation sites, each
  read against its rule text (13 before the audit fix, 14 after — the
  fourteenth is the new canon card's).
- `cargo fmt --all`: clean. `cargo clippy -p deckmaste_semantics_v2
  --all-targets` and `-p xtask --all-targets`: clean, no warnings.

Parked with `@` empty; not integrated.
