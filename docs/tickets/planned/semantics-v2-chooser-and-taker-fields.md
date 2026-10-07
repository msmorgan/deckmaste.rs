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
