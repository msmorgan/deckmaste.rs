---
needs: []
---
**Two optional performer fields outlived `semantics-v2-drop-agent-fields`:
`choose`'s chooser and `insertPart`'s taker.** That landing deleted every
other agent field. Removing either of these changes whether a bench pin
checks, so they stayed until the owner decided. The STOP is recorded in
`docs/tickets/done/semantics-v2-drop-agent-fields.md`; this ticket owns it.
Decided 2026-10-06: (A), delete both (below). This ticket goes first in the
2026-10-06 order. Standard constraints apply.

## The two fields

- `Instruction.choose first chosen disclosure when (agent : Option NounPhrase
  := none)` (Lean `lean/Semantics/Abilities.lean`, mirror `Choose { agent:
  Option<NounPhrase> }`): the chooser. `none` means no chooser is recorded.
- `Instruction.insertPart part anchor count followedBy (agent : Option
  NounPhrase := none)` (mirror `InsertPart { agent }`): the player who takes
  the added turn or gets the added part. `none` means no player is named.

## Why each was kept

- **Chooser.** `Proofs/Anaphora.lean` `badChooseSomeOf`: "Look at the top
  four cards of your library. Choose one of them." is `[.choiceClause]` only
  while the chooser is unrecorded (`choiceClauseOk none n = n.choosable`,
  which admits `a`, `target` and counted determiners but not `someOf` or
  `pileOf`). Its twin `okAgentChoiceOfSome`, the same text with the chooser
  `some .you`, is `[]` (`choiceClauseOk (some _) n = n.agentChoosable`). Once
  every choice records the actor, the first pin's statement checks `[]`.
- **Taker.** `Proofs/ActionFamilies.lean` `badExtraTurnWithoutPlayer`:
  `insertPart .turn none (.lit 1) none none` is `[.windowOk]` (the window
  rule accepts `.turn` only with a taker, no anchor and no following part).
  It has no English sentence; it pins that an extra turn belongs to a player
  [CR#500.7]. Once the taker is always the actor, the term cannot be written.

## What RON does today

- `plugins_v2/builtin/macros/instructions/choose.ron` writes the chooser as
  `actor`. Every card choice records the actor, so `someOf`/`pileOf` choices
  are already judged with `agentChoosable`. Only the Lean bench writes the
  unrecorded form: Lean `choose` defaults to `none`, and so do `chooseWhile`
  and the `choose` inside `proliferate`. In RON, `chooseWhile.ron` ("choose …
  as you activate") and `selectRandom.ron` (a random selection, written
  `Count(…, AtRandom)`) write `agent: None`.
- `insertPart.ron` keeps an optional `agent` defaulting to `None`. No card in
  `plugins_v2/canon` or `plugins_v2/testing` calls it. In Lean, `addTurn` and
  `getAdditionalPart` write `some agent`; `addPart` and `addPartThen` ("there
  is an additional combat phase") write `none`.

## The options put to the owner

**(A) Delete both fields.** The chooser and the taker become the actor in
context, as every other performer is.
- `badChooseSomeOf` re-spelled against the actor form asserts `[]`, the same
  statement as `okAgentChoiceOfSome`. `.choiceClause` keeps a subject: every
  choice is then judged by `agentChoosable`, which still refuses a chosen
  phrase whose determiner is not `a`, `target`, counted, `someOf` or
  `pileOf` ("choose each creature", "choose the creature"). What goes is
  only the narrower `choosable` test for an unrecorded chooser.
- `badExtraTurnWithoutPlayer` has no actor form. The window rule's
  `who.isSome` clause is always true, so the pin either asserts that
  `insertPart .turn none (.lit 1) none` handed to the actor checks `[]`, or is
  re-spelled against a refusal the window rule still makes (a turn with an
  anchor or a following part).
- The random selection (`selectRandom`) and `chooseWhile` would record the
  actor as a chooser. "At random" is carried by the determiner (`AtRandom`),
  not by the missing chooser, so `selectRandom`'s meaning is unchanged.
- "There is an additional combat phase" [CR#500.8] and "you get an
  additional combat phase" [CR#500.10a] become the same term. That rule
  only bites when the part would land in another player's turn, so the "you
  get" form loses what it adds.
- Under (A):
  - `choiceOrderOk first by_` becomes `choiceOrderOkIn bs first`, which holds
    when `first` is `none` or the performer in context is distributive. The
    group-performer rule already hands the group as `perf`, so it reads `first
    = none ∨ (performerOf perf).plur = .many`.
  - `choiceClauseOk` becomes `n.agentChoosable`, and `choosable` stays only as
    its fallback arm.
  - `agentCtx bs by_` becomes `agentIntro bs (performerOf perf)`, and
    `chooseIntro bs by_ n` becomes `chosenIntroBy bs (performerOf perf).plur
    (performerOf perf) n`. `agentCtx`'s `none` arm and `chooseIntro`'s
    `none` arm go. The actor introduces nothing, so for a bare choice this
    is the same context and the same bindings.
  - `optAgentIntro`, `insertPart`'s windowOk clause, and the `insertPart`
    arms of `reflexEncloseUse` and `costActionOk` read the actor.

**(B) Keep both as optional.**
- "No chooser recorded" would mean a selection no player makes:
  - legitimate for a random selection (`AtRandom`) and for "choose … as you
    activate", whose chooser is the activating controller,
    already fixed by the clause;
  - not legitimate for a bare "choose one of them", which some player does
    make (by default the actor).
- "No taker" would mean a part added to the current turn by no player
  [CR#500.8], as against one a player gets [CR#500.7,500.10a].
- The fields would then want names that say this: a chooser slot that is set
  only for random or announced selections, and a taker slot that is set only
  for "you get" parts.

**Orchestrator's recommendation (not taken for the taker):** (A) for the
chooser and (B) for the taker.
- For the chooser, RON already records the actor everywhere, randomness lives
  in the determiner, and `.choiceClause` keeps a subject.
- For the taker, the absent taker distinguishes [CR#500.8] from
  [CR#500.10a]. Rename the field so it reads as that distinction rather than
  as a performer.

## Also seen

- `chooseWhile` and `selectRandom` write `None` in RON and Lean. Under (A)
  they change term, though not judgement (no pin covers them).
- `Proofs/InstructionForms.lean` `chooseOmittedAgent`, `chooseNamedAgent`
  and `chooseSecretlyNamedAgent` pin the chooser slot's macro forms. Under
  (A) they are re-spelled, as `exileNamedAgent` was (`act .you (choose …)`).
- `Actor.choose` (Lean) differs from `choose` only by recording the chooser.
  Under (A) it is retired as the other identical `Actor` helpers were.

## Decided 2026-10-06

**(A) for both fields** (`docs/decisions/semantics-v2.md` §7, ruling
2026-10-06). The chooser and the taker are the actor in context, as every
other performer has been since the 2026-10-05 handoff ruling; the owner
rejects performer exceptions. What the taker distinguished moves into the
part's kind: a turn is taken by the player the instruction was handed to
[CR#500.7], and a phase or step is added to the current turn
[CR#500.8,500.9]. Work under (A) above, plus the following.

- **Rename `InsertPart`.** One renamed node replaces the Lean constructor
  `Instruction.insertPart` (`lean/Semantics/Abilities.lean`), the mirror
  `InsertPart` (`crates/deckmaste_semantics_v2/src/abilities.rs`) and the RON
  alias `plugins_v2/builtin/macros/instructions/insertPart.ron`. The RON
  helpers become `extraTurn`, `additionalPhase(…)` and `additionalStep(…)`;
  the Lean helpers `addPart`, `addPartThen`, `getAdditionalPart` and `addTurn`
  (`lean/Semantics/Macros.lean`) follow. The node's name is the landing's
  call (owner: "insertPart? weirdass name"); check it against the Turn, Phase
  and Step entries of `docs/contexts/game-model/CONTEXT.md`. No card in
  `plugins_v2/canon` or `plugins_v2/testing` calls `insertPart.ron` today.
- **Pins.** `badExtraTurnWithoutPlayer` (`Proofs/ActionFamilies.lean`,
  `insertPart .turn none (.lit 1) none none` = `[.windowOk]`) is re-spelled as
  a turn handed to a permanent, which is refused (`handoffPerformer`): the
  same claim, that an extra turn belongs to a player [CR#500.7], in the
  spelling that can still be written. `badChooseSomeOf` is re-spelled as
  (A) above describes. `okExtraTurnPublishesTurnReference` loses its
  `(some .you)` and keeps its outcome.
- **Delete the Attraction subtype.**
  `plugins_v2/builtin/macros/subtypes/artifact/attraction.ron` goes: the
  CLAUDE.md scope clause excludes Unfinity, and no Vintage-legal card is an
  Attraction (owner: "no vintage-legal attractions anyhow"). Its
  `attested_plural` row in
  `crates/deckmaste_construction_core/tests/builtin_v2_noncreature_subtypes.rs`
  (`(SubtypeCategory::Artifact, "attraction")`) goes with it. The other
  matches for "attraction" are the v1 `crates/deckmaste_english/src/catalog.rs`
  (retired stack, not touched) and a doc comment in
  `crates/deckmaste_construction_core/src/macro_def/tests.rs`; re-grep before
  deleting.
- **Regroup the Lean xtasks** (owner: "sgtm"): `cargo xtask lean-check`
  (`crates/xtask/src/lean_check.rs`) becomes `cargo xtask lean check`, and
  `cargo xtask definition-check` (`crates/xtask/src/definition_check.rs`,
  which already reuses `lean_check`'s `build` and `attribute`) becomes
  `cargo xtask lean definitions`. `facts` stays top-level, because it also
  writes the Idris twin. Update every live reference: `docs/guided_tour.md`,
  `docs/decisions/semantics-v2.md` §13, `lean/CONTRACTS.md`, `lean/README.md`,
  `docs/keyword-policy.md`, `docs/decisions/lean-is-the-workbench.md`,
  `docs/decisions/idris-is-a-soundness-gate.md`, `.github/workflows/ci.yml`,
  the Rust sources and tests that name the commands (`crates/xtask/tests/lean_check.rs`,
  `crates/deckmaste_semantics_v2/src/lean_emit.rs`,
  `crates/deckmaste_semantics_v2/tests/corpus.rs`, `crates/xtask/src/expansions.rs`,
  `crates/xtask/src/facts/lean.rs`), and the open tickets. Find them with
  `rg 'lean-check|definition-check'`. Tickets in `done/` are history and stay.
- **Names confirmed kept:** `members`, `exileFrom`, `mayCastFrom(what, zone,
  paying, exclusive)`, the `handoffPerformer` refusal, and the `act` field
  `performer`.
- **Glossary.** The **Actor** entry's `_Avoid_` line names "the two optional
  model fields left (a choice's chooser, an added turn part's taker)"; amend
  it when the fields go.

## Landing record

Series, oldest first, on claim `tytuymwwpxur` (stamped after `kata refresh`
onto `oorvrlsmvmmr`): `xttprnpypuks` (xtask regroup), `swptuuspxtkl`
(Attraction), `ouysnvuwvwro` (fields deleted), `xxqksrzwmqwk` (node and
helper rename), `zskxwtpsmvrz` (glossary, ADR, this record).

**What changed.**
- `cargo xtask lean check` and `cargo xtask lean definitions` replace
  `lean-check` and `definition-check` (`crates/xtask/src/lean.rs`; the old
  names no longer parse). Every live reference moved: the CI workflow, the
  ADRs, `lean/README.md`, `lean/CONTRACTS.md`, `lean/lakefile.toml`,
  `docs/guided_tour.md`, `docs/keyword-policy.md`, the Rust sources and the
  open tickets. `done/` tickets stay.
- `plugins_v2/builtin/macros/subtypes/artifact/attraction.ron` and its
  `attested_plural` row are gone. The subtype census pin moves 462 → 461 and
  the `corpus.rs` comment with it. The `macro_def/tests.rs` doc comment names
  the retired keyword-action stub `RollToVisitYourAttractions`, not the
  subtype, so it stays; v1 `catalog.rs` is untouched.
- Lean `Instruction.choose first chosen disclosure when` and
  `Instruction.addTurnPart part anchor count followedBy` (formerly
  `insertPart`), mirror `Choose`/`AddTurnPart` without `agent`. Checker
  (`lean/Semantics/Check/`): the choose arm reads `performerOf perf`, checks
  it as a player, reads the chosen phrase in `agentIntro`, refuses with
  `choiceOrderOk first (some by_)` (the vote's form) and
  `n.agentChoosable`; its profile is `chosenIntroBy`. `choiceClauseOk`,
  `agentCtx` and `chooseIntro` are deleted; `choosable` stays as
  `agentChoosable`'s fallback and the `byCandidate` rule's test. The added
  part arm reads the performer as `skipPart` does; the window rule drops its
  `who.isSome` clause; `reflexEncloseUse` is `.notYetTaken` for a turn and
  falls to `.agentless` for a phase or step (the part's kind carries what the
  taker carried); `costActionOk` is `true` (`NounPhrase.actor.costNounOk`).
  No refusal constructor was added.
- Helpers: Lean and RON `extraTurn count`, `additionalPhase part anchor count
  (followedBy := none)`, `additionalStep part anchor count`; the RON alias is
  `addTurnPart.ron`. Lean `addPart`, `addPartThen`, `getAdditionalPart`,
  `Primitives.Instruction.addTurn`, `Primitives.Instruction.addPart` and
  `Actor.choose` are retired. Node name: `addTurnPart`, after `skipPart` and
  the `TurnPart` type of its first field; the glossary has Turn, Phase and
  Step and no umbrella term, so the type's name is the one in use. `addPart`
  was rejected because it was the name of two helpers this landing retires.

**Pins.** 1926 pin theorems before and after. Changed outcome, both ruled:
`badChooseSomeOf` is now `okChooseSomeOf`, the same text asserting `[]`.
`badExtraTurnWithoutPlayer` is re-spelled as "Target creature takes an extra
turn", `act (target creature) (.addTurnPart .turn none (.lit 1) none)`, and
asserts `[.kindMismatch .player .object]`, not `[.windowOk]`. The ticket's
"refused (`handoffPerformer`)" does not hold: `handoffPerformerOk` admits a
permanent [CR#701.44a]. The refusal that fires is the performer's existing
player check, the one `actorInPermanentHandoffIsThePermanent` pins for a
draw. 58 more theorems are re-spelled mechanically (the agent argument
dropped or a renamed helper) with an unchanged right-hand side, and the
build proves each one. Among them are `chooseOmittedAgent`, `chooseNamedAgent`
and `chooseSecretlyNamedAgent`, now spelled `act .you (choose …)`.
`okExtraTurnPublishesTurnReference` lost `(some .you)` and keeps `[]`. Its
Turn twins (`badAdditionalPartAmountReadsSubject` and
`badAdditionalPartWithoutSubjectOrOuterContext`) are now the same term and
still pass. Counts: restored 0, re-spelled 61, ignored 0, added 1 (Rust
`lean_command_family_requires_a_subcommand`), removed 0.

**Proofs** (on `zskxwtpsmvrz`, after refresh). `lean/scripts/build`
(`--wfail`) passed with 82 jobs. `lean check`: canon 127/127, testing 6/6.
`lean definitions`: 553/553 (73 counters, 461 subtypes, 19 designations).
`facts check`: up to date. Gate: `cargo test -p deckmaste_construction_core
-p deckmaste_lexical_source -p deckmaste_semantics_v2 -p
deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask` passed 1349,
failed 0, ignored 1 (pre-existing) in 111 suites, 21m12s wall. `cite check`
reported 0 non-compliant and 0 stale. `cite audit` over the series diff read
11 sites [CR#500.7,500.8,500.9]. `kanban check` OK. clippy `-D warnings` was
clean.

**Expansions** (`diff -r`). Stage (c): only `agent` lines leave `choose`,
`chooseWhile`, `insertPart`, `selectRandom` and the seven choose-bearing
keyword bodies (demonstrate, amass, behold, bolster, populate, proliferate,
timeTravel). Stage (d): `insertPart` → `addTurnPart`, identical modulo the
name, and `extraTurn`, `additionalPhase` and `additionalStep` are added.
1510 → 1513 declarations, 0 failed.

**Lexicon export** (`--card-name "Black Lotus" --export`). The only change
is the `lexeme:artifact_subtype/attraction` noun (Attraction deletion). The
stage (d) export is byte-identical to the post-Attraction one.
`--text-contains Attraction` selects 0 supported faces, so no coverage
identity is lost.

**Deviations and additions.**
- Added: the CLI parse test.
- `choiceOrderOkIn` was not minted. The arm calls `choiceOrderOk first
  (some (performerOf perf))`, as `vote` does, which is the same predicate.
- Disclosed judgement change, no pin: a phase or step handed to a permanent
  is now refused by the performer's player check, as `skipPart` already
  was. No refusal constructor was added.
- ADR §12.1 recounted: 420 = 319 declarations + 2 + 99. The `Primitives`
  bucket is now 21 and the `Actor` bucket 3. The alias paragraph now says no
  constructor has an agent field.
- Observation, not acted on: `subtypes/artifact/contraption.ron` is also an
  Un-set subtype.

No glossary gap. The Actor `_Avoid_` line is amended.
