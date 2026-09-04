---
needs: []
---
**Use Choice, Decision Point, and Decision for three different layers.** The
[`Choice`, `Decision Point`, and `Decision` definitions](../../contexts/game-model/CONTEXT.md)
define the target vocabulary: a Choice is a Magic rules selection or
procedure, a Decision Point is where engine progress waits for an external
agent, and a Decision is the submitted response. A Decision may encode either
a Choice or a proposed Action.

Scope (ruling 2026-09-04): the Idris workbench, core, engine, and lowering's
core-facing output only; `deckmaste_semantics` and `idris/src/Semantics.idr`
are deletion-bound and untouched; their side converges at cutover when the
RON re-emit path points at `Experimental`.

Audit semantic/core request types, engine suspension and resume state, player
interfaces, and tests. Rename generic `Choice` or `Decision` identifiers where
they currently name another member of the trio, without renaming CR-facing
rules operations merely for uniformity. Prefer domain-qualified names at
cross-layer APIs when the unqualified noun would be ambiguous. Preserve the
existing serialization shape only where it remains honest; this research
project does not need compatibility aliases.

## As landed

Inventory command: `grep -rnwE "Choice|Decision|Decisions|Choices|DecisionPoint|Suspend|Resume|Pending" crates/deckmaste_core/src crates/deckmaste_engine/src crates/deckmaste_lowering/src crates/deckmaste_plugin/src --include=*.rs | grep -vE "^\s*//" | wc -l` → 371 hits, all but 2 in `deckmaste_engine` (2 in `deckmaste_core`, 0 in `deckmaste_lowering`/`deckmaste_plugin`). Consumers outside that grep's crate list — `deckmaste_tui` (player interface) and `deckmaste_noncanon` (AI player/strategy) — also reference the renamed engine types and are included below.

| Identifier | Layer | Named as (before) | Actually names (trio member) | Disposition |
|---|---|---|---|---|
| `PendingDecision` (enum, `deckmaste_engine::decide`) | Engine | A "decision" | **Decision Point** — the doc comment says it outright: "What the engine is waiting on"; each variant is an engine boundary kind (`Priority`, `ChooseTargets`, `PayMana`, …), not a submitted answer | **Renamed → `DecisionPointKind`** (the discriminant of the existing `DecisionPoint` record, which already carries this plus `decider`/`lock`/`visibility`). 386 mechanical occurrences across `deckmaste_engine` (src + tests), `deckmaste_tui`, `deckmaste_noncanon`. |
| `ChoiceContinuation` (enum, `deckmaste_engine::state`) | Engine | A "choice" (rules-level) | **Decision**-adjacent resume state — its own doc comment: "What to resume once a resolution-time **decision** is answered… set while that decision is pending… taken on submit" | **Renamed → `DecisionContinuation`**. The `choice: Option<DecisionContinuation>` field (in `GameState`/`ControlSnapshot`) is left as `choice` — see Deviations. |
| `DecisionPoint` (struct: `pending`+`decider`+`lock`+`visibility`) | Engine | "Decision Point" | Decision Point — matches the glossary exactly ("the boundary record of a pending decision… the kind plus its schema row") | Keep. |
| `Decision` (enum, submitted answer) | Engine | "Decision" | Decision — the submitted response to a `DecisionPointKind`; some variants (`ReplacementChoice`, `CostOptions`) encode a rules Choice, matching the glossary's "A Decision may encode either a Choice or a proposed Action" | Keep. |
| `DecisionHandler` (trait) | Engine | "Decision" | Decision — validates/applies the submitted `Decision` against the pending kind | Keep. |
| `DecisionError` (enum) | Engine | "Decision" | Decision — rejection of a submitted `Decision` | Keep. |
| `Progress::NeedsDecision(DecisionPointKind)` | Engine | "Decision" | Decision — accurately says the engine needs a `Decision` to proceed past the carried decision-point kind | Keep. |
| `Decision::ReplacementChoice`, `crate::replace_registry::ReplacementKey` | Engine | "Choice" inside `Decision` | A Decision encoding a rules Choice among applicable replacements [CR#616.1] | Keep — honest per the glossary's "may encode … a Choice". |
| `ChooseTargets`, `ChooseManaColor`, `ChooseManaMode`, `ChooseModes`, `ChooseXValue`, `ChooseCostOptions`, `ChooseNoteNumber`, `ChooseNoteCardName`, `ChooseObjects`, `ChooseReplacement`, `ChooseManaReversals` (`DecisionPointKind` variants) | Engine | "Choose" (verb) | CR-facing rules-selection kinds — the same naming family as core's `Choose`/`ChooseValue` | Keep, CR-facing — not renamed for uniformity with the enclosing enum's rename, per `core-explicit-regions`'s ruling on `Choose`/`ChooseValue`/`Search`/`SeparatePiles`/`ChoosePile`. |
| `SymbolChoice`, `CostOptionChoices` (`cost_options.rs`) | Engine | "Choice"/"Choices" | CR rules choice: one chosen reading of a hybrid/Phyrexian mana symbol, and their aggregate | Keep, CR-facing. |
| `BindChoice` (`DecisionContinuation` variant) | Engine | "Choice" | Names the CR choice being bound (a `ChooseObjects` answer's picks), not the Decision Point/Decision distinction | Keep. |
| `NoteChoiceOpened` (`Progress` variant) | Engine | "Choice" | A resolving `ChooseAndNote` (CR-facing) surfacing its choice, i.e. opening the decision point for it | Keep. |
| `MissingDecision` (`ReplayError` variant), `DecisionTranscript` (replay log) | Engine | "Decision" | A submitted `Decision` absent from / recorded in a replay log | Keep. |
| `DeciderSpec`, `Visibility`, `ChosenValueKind`, `NotedKind` (`deckmaste_core::decision`) | Core | Neither "Choice" nor "Decision" substrings | Schema/attribute vocabulary describing a Choice's nominal decider, visibility, and chosen-value kind — attached to the engine's `DecisionPoint` record; not trio nouns themselves | Not in the audit's grep scope (no `Choice`/`Decision` substring) — left unchanged, including the `decision.rs` filename/module. |
| Idris workbench (`idris/src/Experimental/Cards/Choice.idr` and module/card-name occurrences: "Tyrant's Choice", "Multiple Choice") | Idris workbench | "Choice" | CR-facing card names and a module named after them; no engine Decision Point/Decision identifiers appear | Untouched, as anticipated by the ticket ("unlikely" to need renames). |
| `Choose`, `ChooseValue`, `Search`, `SeparatePiles`, `ChoosePile` (`deckmaste_core`) | Core | CR-facing instruction names | Choice (CR rules operations) | Untouched per `core-explicit-regions`'s explicit ruling — not audited for renaming. |

Nothing was renamed and then reverted; every renamed identifier's mechanical propagation was verified by `cargo check --workspace` and `cargo fmt`.

## Landing record

- Scope executed: `deckmaste_engine` (definitions + all internal consumers), `deckmaste_tui` (player interface), `deckmaste_noncanon` (AI player interface). `deckmaste_core`, `deckmaste_lowering`, `deckmaste_plugin`, and the Idris workbench were audited (per the inventory grep and manual `Experimental/` grep) and needed no renames. `deckmaste_semantics`/`idris/src/Semantics.idr` untouched per the 2026-09-04 scope ruling.
- Renames: `PendingDecision` → `DecisionPointKind` (386 occurrences); `ChoiceContinuation` → `DecisionContinuation` (7 files). Purely mechanical (`sed` with word-boundary matching, no collisions found by pre-check); every call/match/import site updated by the same substitution.
- `cargo check --workspace`: clean (last line: `Finished \`dev\` profile [unoptimized + debuginfo] target(s) in 1m 33s`).
- `cargo fmt --check`: failed once (two match arms in `deckmaste_tui/src/ui/format.rs` exceeded the line-width limit once `PendingDecision` grew to `DecisionPointKind`); ran `cargo fmt`, then `cargo fmt --check` passed clean.
- `cargo test --workspace` (foreground, full log at `/tmp/claude-1000/.../scratchpad/full_test.log`): every crate's test binary reports `0 failed` — including `deckmaste_engine` (748 tests, 1 ignored — pre-existing, unrelated), `deckmaste_tui`, and `deckmaste_noncanon` — **except** `deckmaste_migrations::resolve::tests::ascend_gate_const_matches_canonical_condition` (426 run, 425 passed, 1 failed). This failure is **pre-existing and out of this ticket's scope**: `deckmaste_migrations` is untouched by this diff (`jj diff --git` confirms), and `jj log` shows the immediate parent of this workspace's claim is `core: split ObjectKind into the Entity boundary and nonexclusive CR object classes`, which changed a region-provenance `Candidate(Object)` → `Candidate(Entity)` in core; `deckmaste_migrations`'s hardcoded `ASCEND_GATE` string constant was not updated to match, so the drift-guard test now fails comparing the literal against the canonically-rebuilt condition. Nothing in this ticket's diff touches `Candidate` provenance, `deckmaste_migrations`, or the Ascend/CitysBlessing condition. Per the assurance rule ("finding a genuine regression is a successful outcome — stop and report it"), this is reported rather than fixed (out of scope) or hidden; it should be routed to a ticket against the `core-entity-object-classes` line of work.
- Assurance counts (this ticket's own scope): restored 0, re-spelled 0, ignored-with-blocker 0, added 0, removed 0. No test's subject was renamed in a way that changed its assertions — every renamed identifier is a pure type-name substitution verified by the compiler and by the full (green, in-scope) test run.
- `cargo xtask cite check --list-noncompliant`: `0 non-compliant citation-looking string(s)`.
- `cargo xtask cite check`: `checked 18042 citations against cr.txt (eff. 2026-08-07); 0 stale`.
- `cargo xtask cite audit --diff` (fed `jj --no-pager diff --git`): `audited 0 citation site(s) — nothing selected` (expected: no citation text was added or changed, only identifiers).
- `plugins/wizards` regeneration: not needed — no card/grammar/wizard-generating code was touched.
- `idris/scripts/build`: not run — no file under `idris/` was touched (the workbench audit found no rename targets).
- Deviations and additions: none beyond the two renames above. The `choice: Option<DecisionContinuation>` field name (in `GameState`/`ControlSnapshot`) was deliberately left as `choice` rather than renamed alongside its type, to avoid a broad, collision-prone `choice` token rename (the bare word `choice`/`choices` is reused for unrelated, genuinely CR-facing local bindings and fields elsewhere, e.g. `cost_options.rs`'s `choices: Vec<SymbolChoice>` and `strategy.rs`'s local rules-choice variables) that the ticket's mechanical-rename latitude does not require.
- STOP taken: none. The `deckmaste_migrations` test failure above was investigated to the point of confirming it is pre-existing and unrelated (not a STOP on ambiguous evidence — the causal chain is directly traceable via `jj log`), then reported per the assurance rule rather than escalated as a blocking STOP.
