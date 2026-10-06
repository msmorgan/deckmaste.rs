---
needs: [plugins-v2-implicit-actor-spelling, semantics-v2-group-handoff-publishes-no-outcome, semantics-v2-deed-performer-roles]
---
**Delete the agent fields from the model once nothing spells them.** Landing
(3) of the actor-handoff design agreed with the owner on 2026-10-05 (see
`semantics-v2-actor-handoff`). After `plugins-v2-implicit-actor-spelling`
every helper fills these fields with `actor`, so the field says nothing the
enclosing handoff does not. Owner: "Maybe the explicit actors version is more
a concern for lowering/core." Lean first, then the mirror. Standard
constraints apply.

## The change

1. **Lean** (`lean/Semantics/Abilities.lean`). Delete the 16 required agent
   fields — `conclude`, `separateIntoPiles`, `vote`, `copy`, `changeLife`,
   `addMana`, `draw`, `expose`, `search`, `shuffle`, `flipCoins`, `rollDice`,
   `rerollStored`, `createObject`, `pay`, `skipPart` — the 3 optional ones —
   `choose`, `enact`, `insertPart` — and `ContinuationPolicy.optional`'s
   agent. The checker reads the performer from the enclosing handoff (or the
   controller when there is none) wherever it read the field:
   `agentIntro`/`agentCtx` (`Check/Phrase.lean`), `Instruction.check`
   (`Check/AbilityRules.lean`), `Instruction.profile`, `mayCtx`,
   `enactKeepsOuter`, and the cost rules that read `NounPhrase.isYou` on an
   agent (`Check/Abilities.lean` ~L805, `changeLife` and `enact`). `agentRef`
   (`Macros.lean` ~L732) and the macros that pass `agent :=` follow.
2. **The Lean bench** (`lean/Semantics/Cards`) is re-spelled over `act`: the
   453 `agent := Primitives.NounPhrase.you` arguments are dropped, and every
   other explicit agent (`each …`, `target …`, `controllerOf …`, `they`,
   `that …`, about 214 more sites) becomes an `act` around the instruction.
   Pins keep their asserted outcomes.
3. **Mirror.** `crates/deckmaste_semantics_v2` drops the same fields;
   `lean_drift` holds.
4. **RON.** The helpers stop writing `actor` into the deleted slots;
   `keywords.rs` writes `Enact(Action(label), body)`.

Two tickets come first. `semantics-v2-group-handoff-publishes-no-outcome`:
until a group handoff publishes the total "that much" reads, extort must
write `changeLife` with each opponent as its agent, so that field cannot be
deleted. `semantics-v2-deed-performer-roles`: until the twelve deeds the
checker gives no player performer accept the actor, their declarations write
`agent: None` and the loader keeps the `agent` field.

Routed here by the 2026-10-05 decision on
`semantics-v2-group-handoff-publishes-no-outcome` (option A): once extort is
written `act(each(opponent), loseLife(1))`, the `changeLife` alias loses its
`agent` parameter in this landing.

Decided 2026-10-05 on `semantics-v2-deed-performer-roles`: the `act` handoff
accepts a permanent as well as a player ("target creature explores" is
`act(target(creature), explore)`), so the handoff carries an object
performer, and this landing deletes the wrapper's performer slot as planned
once that ticket has landed.

2026-10-05, `semantics-v2-group-handoff-publishes-no-outcome` landed: extort
writes `act(each(opponent), loseLife(1))`, no caller in `plugins_v2/` passes
`changeLife` a performer, and its `agent` parameter can now go.

## Left by plugins-v2-implicit-actor-spelling (2026-10-05)

After that landing, these are the only places in `plugins_v2/` that still
pass a performer (or an explicit "no performer") to an instruction helper,
an alias or a keyword action. Line numbers are as of that landing.

- **The `changeLife` alias keeps its `agent` parameter, for extort alone.**
  `plugins_v2/builtin/macros/keyword_abilities/extort.ron:20` writes
  `changeLife(down(1), each(opponent))`: handed off as
  `act(each(opponent), loseLife(1))`, the following `gainLife(thatMuch)`
  ("the total life lost this way") is refused, Lean
  `quantOutcomeInScope 0` (canon card Syndic of Tithes). Every other life
  change goes through `loseLife`/`gainLife`, under `act` for another player.
- **The keyword-action loader accepts `agent: None`** (and refuses any other
  value) for the twelve deeds whose `actFacts` row gives no player agent, so
  Lean `enactAgentOk` refuses a recorded actor on them:
  `keyword_actions/adapt.ron:14`, `bolster.ron:18`, `counter.ron:14`,
  `detain.ron:16`, `endure.ron:14`, `explore.ron:19`, `harness.ron:16`,
  `heal.ron:14`, `meld.ron:15`, `monstrosity.ron:17`, `populate.ron:17`,
  `timeTravel.ron:19` (all under `plugins_v2/builtin/macros/`). The meta
  `macros/meta/KeywordAction.ron` and `deckmaste_semantics_v2::keywords`
  carry the field for them.
- **The optional-agent helpers keep their parameter** (`enact`,
  `insertPart`, `returnTo`, `returnToBattlefield`, `meldInto`,
  `exileWithCounters`; Lean `Option NounPhrase := none`). Calls that pass
  one:
  - `plugins_v2/builtin/macros/keyword_abilities/unearth.ron:16`:
    `returnToBattlefield(subject: this, agent: actor, from: graveyardOf(you))`.
  - `plugins_v2/builtin/macros/keyword_abilities/station.ron:14` (agent at
    line 20): `enact(Action("Tap"), setStatus(…), you)`.
  - `plugins_v2/canon/cards/Glimpse of Freedom.ron:18` (agent at line 21):
    `enact(verb: Action("Exile"), …, agent: actor)`.
  - `plugins_v2/canon/cards/Incarnation Technique.ron:17` (agent at line
    20): `enact(verb: Core(Return), …, agent: actor)`.
  - `agent: None` written out: `Krosan Grip.ron:17`, `Capsize.ron:18`,
    `Dead Revels.ron:21`, `Grim Harvest.ron:17`,
    `Sublime Exhalation.ron:17` (all `plugins_v2/canon/cards/`, on `enact`).
- **Bodies that write `actor` in a constructor's agent slot** (these follow
  the fields when they go): `instructions/createToken.ron:11` and the other
  helpers and aliases whose body says `actor`; `keyword_actions/search.ron`,
  `shuffle.ron`, `vote.ron` (raw `Search`, `Shuffle`, `Vote`).

2026-10-05, `semantics-v2-deed-performer-roles` landed: no declaration
writes `agent: None`, the loader and the meta accept no `agent` field, and
every wrapper records `Some(Actor)`, the permanent under a handoff to one and
the source permanent by default (Lean `enactDefaultsToThis`, `enactCtx`).
When the wrapper's slot goes, `enactAgentOk` and `enactAgentKind` must read
the actor in context (the innermost handoff frame, or that default) instead
of the slot.

## Proof

- `cargo xtask lean-check` passes with every pin at its old outcome.
- `cargo xtask expansions` before/after differs only by the removed fields:
  for every declaration and card, the expansion after equals the expansion
  before with each `agent: actor` (or the field's position) removed. List any
  other difference.
- Every bench theorem re-spelled is counted in the landing record
  (re-spelled, none removed).

## Out of scope

- How lowering redistributes a handoff to the performing players
  (`lowering-v2`).

## STOP: two optional fields stay (2026-10-06)

Deleting two of the nineteen fields changes whether a bench sentence checks,
so they stay, and the owner decides:

- **`choose`'s optional chooser.** `Proofs/Anaphora.lean`
  `badChooseSomeOf`, "Look at the top four cards of your library. Choose one
  of them.", is refused `[.choiceClause]` because the chooser is not recorded
  (`choiceClauseOk none n = n.choosable`, which refuses `someOf`), while its
  twin `okAgentChoiceOfSome` (chooser `you`) is `[]`. With the slot gone
  every choice records the actor and the first sentence checks. RON's
  `choose` helper has recorded the actor since
  `plugins-v2-implicit-actor-spelling`; only the Lean bench writes the
  unrecorded form.
- **`insertPart`'s optional taker.** `Proofs/ActionFamilies.lean`
  `badExtraTurnWithoutPlayer` (an extra turn with no player) is refused
  `[.windowOk]`; with the slot gone every added part has the actor as its
  taker and the term cannot be written. The Lean `addPart` macros ("there is
  an additional combat phase") write `none` [CR#500.8]; nothing else changes
  outcome.

Found by the diagnostic that made every rule read the actor while the fields
still existed (the E2 run in the record below). Everything else in the ticket
landed.

## Landing record

The series, oldest first: `zkznylqolkok` (claim), `stplvnkoxvtk` (stage 1,
Lean: the group-performer rule and the bench's explicit agents re-spelled as
handoffs, fields still present), `vlyvxqnymyly` (stage 2: the fields deleted
in the Lean model, checker, macros, bench and pins, the Rust mirror, the RON
helpers, aliases, callers, loader and tests), `nosukwwpxppo` (ADR §7, §11,
§12.1, `lean/CONTRACTS.md`, `lean/README.md`, glossary), and this record
(with the dated note the deed-roles landing left, re-added). The Rust mirror
and the RON go in the same commit as the Lean model: with the mirror's fields
gone every RON body that writes `actor` in a slot fails to load, and with the
RON changed alone `lean-check` emits terms the Lean model no longer takes.

**What was deleted.** Lean `Instruction` (`lean/Semantics/Abilities.lean`)
and the mirror (`crates/deckmaste_semantics_v2/src/abilities.rs`) now read:
`conclude verb`, `separateIntoPiles group piles faces`, `vote first disclosure
ballot`, `copy sort subject times exceptions`, `changeLife delta`, `addMana
amount produced riders`, `draw amount`, `expose verb exposed`, `search scope
quantity predicate`, `shuffle` (a unit variant), `flipCoins count`, `rollDice
count sides`, `rerollStored quantity whose`, `createObject count spec`,
`enact verb instruction`, `pay cost times`, `skipPart part count`, and
`ContinuationPolicy.optional` (a unit variant). Kept (STOP above): `choose
first chosen disclosure when (agent : Option NounPhrase := none)`, `insertPart
part anchor count followedBy (agent : Option NounPhrase := none)`. Core
constructors still take every field required; no default was added.

**The rules** (all `lean/Semantics/Check/`).
- `Instruction.checkWith bs perf` (`AbilityRules.lean:153`) and
  `Instruction.profileWith bs perf` (`Abilities.lean:1214`) replace the
  mutual `check`/`profile`; `Instruction.check` (`AbilityRules.lean:814`) and
  `Instruction.profile` (`Abilities.lean:1456`) are `… none`. `perf` is the
  group a handoff hands this very instruction; every arm that read an agent
  field reads `performerOf perf` (`Abilities.lean:420`, the actor when
  `none`): `conclude`, `separateIntoPiles`, `vote`, `copy`, `changeLife`,
  `addMana`, `draw`, `expose`, `search`, `shuffle`, `flipCoins`,
  `rollDice`, `rerollStored`, `createObject`, `pay`, `skipPart`, and the
  offer's decider (`ContinuationPolicy.contextBy`, `Abilities.lean:493`;
  `context` is `contextBy none`).
- The group-performer rule (stage 1): `.act who body` with `who.plur ==
  .many` and `body.performed` (`Instruction.performed`, `Abilities.lean:409`:
  the nineteen field-bearing constructors and an offer) checks and profiles
  the body with `perf := some who` in `performerFrame bs who`
  (`Abilities.lean:444`: the handoff frame alone), the frame left by
  `leaveHandoff 0` (`AbilityRules.lean:341`, `Abilities.lean:1324`). This is
  the deleted explicit plural agent's reading (`enactKeepsOuter`,
  `doesProfile .many`, `chosenIntroBy .many`, one `voteHeld`, the offer's
  body in `mayCtx`). Any other group handoff, and every singular one, keeps
  the frame-and-pluralize reading.
- `enactPerformer v perf` (`Abilities.lean:436`): the group handed, else the
  actor where the deed's row names a performer (`deedNamesPerformer`,
  `:431`), else none. `enactAgentOk`, `enactAgentKind`, `enactCtx` and
  `enactKeepsOuter` read it, so they read the actor in context, its default to
  the source permanent included (`enactDefaultsToThis`). A singular handoff
  hands its body `handedOne body` (`:425`: the actor for an `enact`, so a deed
  whose row names no performer refuses being handed, to "you" included).
- `Instruction.paidByYouAs` (`Abilities.lean:943`): a life loss is the
  performer's; an `enact` is unless its row names no performer.
  `heldUntilOk` (`:1017`) takes any enacted move. `reflexEncloseUse`
  (`:1039`), `costActionOk` (`:1088`: the performed arms read
  `NounPhrase.actor.costNounOk`), `numberSlots` (`:1698`) and
  `lookedLibraryOwner`'s `expose` arm follow the new arities.
- Unchanged, and why: `agentCtx`, `chooseIntro`, `choiceOrderOk`,
  `choiceClauseOk` (`Phrase.lean`) serve `choose`, whose field stays;
  `optAgentIntro` serves `insertPart` and the event clauses; the
  `verbedEvent` performer is a `GameEvent` field ("whenever a player draws"),
  not an instruction's, and is out of scope; `mayCtx` is unchanged and called
  with the performer in context; `doesProfile`, `eachStackOk`,
  `distributedDelta` are reached through the group-performer rule.

**Twin pins and results.**
- Stage 1, the field still read: every explicit agent on the bench that was
  not "you", "actor" or `none` (260 sites by script, 4 by hand) was re-spelled
  as `act p (X … (agent := actor))`. A diagnostic build (every failing
  `decide` turned to a `sorry` warning, every failing `spelled` card to a
  logged warning, scratch only) found 26 pins whose twin differed under the
  old rules: the vote cards and pins (Tyrant's Choice, Council's Judgment,
  Orchard Elemental, Plea for Power, Coercive Portal, Custodi Squire,
  Lieutenants of the Guard, Truth or Consequences, `okVoteReadsAfterVote`,
  `voteStartingWithSpecifiedPlayer`, `okChoiceStartingWithYou`), the
  distributed choices and sacrifices (`okDistributedRestOfOwnChoice`,
  `badDistributedRestDisposedTwice`, `okDistributedLoopParts`,
  `badDistributedRestOfSharedGroup`, `badDistributedRestOfSingularChoice`,
  `badDistributedZoneMoveRead`, `distributedDeedRiderReadsBackPlural`, Stick
  Together, Disciple of Caelus Nin, Vaevictis Asmadi), the drains (Gray
  Merchant, Defiling Daemogoth: `openLetter x`), Truce
  (`okDrawUpToTwoThenGainPerShortfall`), `okShortOfCeilingAnnounced` and
  Goblin Assassin (`coinFlipInScope`). The group-performer rule closes all 26:
  the stage-1 build checks every pin at its outcome with the handoff
  spelling while the explicit form still exists, `Proofs/Actor.lean`'s
  explicit-agent twins unchanged and passing.
- Stage 2 diagnostic (E2: every rule reading the actor, fields still
  present): beyond the explicit-agent pins of `Proofs/Actor.lean` and two
  foreign-payer pins, the differences were the deeds whose row names no
  performer (fight: `okFightCreatures`, `badFightGroup`, Prey Upon, Brash
  Taunter, Savage Swipe, Ulvenwald Tracker, …; regeneration: Drudge
  Skeletons, Asphodel Wanderer, Death Ward, Matopi Golem, Clergy of the Holy
  Nimbus, `okThisWayOnRegenerate`, …; `badUnknownKeywordAction`,
  `badAgentedAgentlessAct`), closed by `enactPerformer`/`handedOne`, and the
  two STOP pins.
- Stage 2: `cd lean && ./scripts/build` Build completed successfully (82
  jobs); every pin at its asserted outcome but the re-spelled ones below.

**The bench.** Agent arguments of the deleted fields on the claim's bench
(`Cards` 691, `Proofs` 464; named and positional), by old agent: you 879,
each 64, they 39, controllerOf 35, target 34, that 27, anOpponent 24, actor
22, none 5, those 5, ownerOf 4, two each of `eachOf (both you (target
opponent))`, `splitOverPlaneswalker`, a macro's own `agent`/`payer`
parameter, one each of `a opponent`, `the (…)`, `combatPlayer .defending`,
`combatPlayer .attacking`, `possessorOf .owner it`, two pronoun players and
a captured `who`. Every non-you, non-actor, non-none agent became a handoff
(stage 1, `drop-agent-fields/respell1.py`); then (stage 2,
`respell2.py`) "you" was dropped (834) or, inside a handoff's body, became
`act you` (Rhystic Study, tidied by hand to one), "actor" dropped (217,
including the stage-1 ones), `none` dropped (4); 43 more stage-1 handoffs sit
at the kept `choose`/added-part heads. 928 bench declarations changed
(535 card definitions, 353 pin theorems, 21 pin definitions, 4 private
macros, guard examples); none removed. The scripts and their logs are
session scratch, not tracked.

**Pins re-spelled** (old → new; same English, same asserted outcome unless
noted).
- `Proofs/Actor.lean`: `actorDrawIsYouDraw`, `actorLoseLifeIsYouLoseLife`
  (`… (agent := .you)` → `act .you (…)`); `okActorMatchedPayer`,
  `okActorOwnPayerCost`, `ownPayerCostHandoffTwin` (`payLife actor|you n` →
  `payLife n`); `badActorMismatchedPayer`, `okHandedPayerIsTheTarget`,
  `badNestedHandoffInnermost`, `badGrantedAbilityKeepsItsOwnPerformer`
  (`payLife anOpponent 1` → `.perform (act anOpponent (loseLife 1))`); the
  cost twins `wallOfShardsHandoffTwin`, `varchildsWarRidersHandoffTwin`,
  `invigorateHandoffTwin`, `heatWaveHandoffTwin`, `killingWaveHandoffTwin`,
  `foreignSacrificeCostHandoffTwin`, `foreignPayerCostHandoffTwin`,
  `opponentPaysYourCostHandoffTwin`, `mismatchedPayerHandoffTwin`, and
  `extortHandoffTwin`, `handedGroupPublishesTotal`: the explicit-agent side
  is now the handoff the bench writes, so each second conjunct is
  reflexive; the first conjunct keeps its outcome, and the explicit twin was
  proven equal on `stplvnkoxvtk`; `okCapturedActorIsYou` (`youPayWith` →
  `act .you (offer …)`, `capturedLifeLoss` → `.act who (changeLife …)`);
  `badCapturedActorIsTheHandedPlayer`: the payer "you" is now a handoff,
  inside which `actor` is "you", so the captured `actor` became a captured
  `they` and the uncaptured `loseLife` an `act they (loseLife 1)`, both still
  `[.payAgrees]`, and the new pin `capturedActorIsTheHandedPlayer` keeps
  "a captured `actor` is the handed player" (`[.costPaidByYou]`);
  `okEachOpponentMayPayElseYouDrawUnless` (`doUnless … (agent := each
  opponent)` → `act (each opponent) (doUnless …)`); `okDiscardWrapperWithAgent`
  (`.enact … (some (target anyPlayer))` → `act (target anyPlayer) (.enact
  …)`); `okDiscardWrapperCarriesActor`, the explore, endure and adapt
  literals (`some .actor`, `.optional .actor` dropped).
- `Proofs/Zone.lean` `badAgentedAgentlessAct` (`.enact Fight … (some .you)` →
  `act .you (.enact Fight …)`, `[.enactAgentOk]`); `Proofs/InstructionForms.lean`
  `exileOmittedAgent`, `millNamedAgent` (the right side loses `none` / `some
  .you`), `exileNamedAgent` (`exile x (agent := .you) = enact … (some .you)`
  → `act .you (exile x) = Instruction.act .you (enact …)`);
  `Proofs/Authoring.lean` and `Proofs/MacroParameters.lean` guard examples
  (`draw 1 (agent := .you)`, `drawTwice 1 .you` → `act .you (…)`, the same
  raw-constructor message); `MacroParameters` `capturedPayment`,
  `capturedLifePayment` (`.act payer …`), `scopedLookRetainsOpponentRequirement`,
  `capturedLibraryOwnerRetainsOpponentClassification`,
  `scopedLibrarySliceRetainsItsOwner`; `Proofs/Composition.lean`
  `okOptionalSuccessReadsBody`, `badOptionalFailureReadsBody` (`.optional
  .you` → `.optional`); `Proofs/ActionFamilies.lean` `okCreateSeveralEmblems`,
  `okEmblemPublication`, `badEmblemWithSpellAbility`,
  `exiledAbilityCannotBeCopiedOnStack`, `conditionalForgettingKeepsTheOperandScope`
  (positional `.you` dropped); `Proofs/Choice.lean` `badMismatchedPayer`,
  `Proofs/Keyword.lean` `badForeignPayerCost` (`payLife anOpponent n` →
  `.perform (act anOpponent (loseLife n))`).
- Pins added: `capturedActorIsTheHandedPlayer`. Pins removed: 0. Outcomes
  changed: 0.

**Macros.** Lean: every macro of a deleted field lost its `agent`
parameter; `doUnless` offers `pay cost .once` with no re-read payer;
`scry`/`surveil` look at the performer's own library (`lookAndSort .actor`);
`controllerSacrifices` and `regenerationApplication` hand their deed
(`.act controller (sacrifice …)`, `.act (possessorOf controller subject)
(tap …)`); `payLife amount`. Retired: `Actor.draw`, `Actor.loseLife`,
`Actor.gainLife`, `Actor.create`, `Actor.sacrifice` (each now wrote the term
of the unprefixed macro; their uses re-spelled to it). Kept: `Actor.revealHand`,
`Actor.choose` (records its chooser; `choose`'s field stays),
`Actor.discard` (the performer's own hand), `Actor.mayCastFrom`,
`Actor.army`, `Actor.amass`, and `agentRef`/`agentPlur`, which
`Proofs/ReferenceScopes.lean` `agentWithoutOuter`/`agentWithOuter` pin.

**RON.** Helpers and aliases drop the slot: `addMana`, `conclude`, `copy`,
`copySpell`, `createObject`, `createToken`, `createTappedAttacking`, `draw`,
`expose`, `flipCoins`, `gainLife`, `loseLife`, `setLife`, `lookAt`,
`lookAtHandOf`, `revealCards`, `revealHand`, `pay`, `payLife`, `put`,
`rerollStored`, `rollDice`, `search*`, `separateIntoPiles`, `shuffleInto`,
`skipPart`, `voteStartingWith`, `returnToHand`, `exileFrom`,
`manifestPlacement`, `may` and `doUnless` (`Optional`); the parameter goes
from `changeLife`, `enact`, `returnTo`, `returnToBattlefield`, `meldInto`,
`exileWithCounters`; the keyword actions `search`, `shuffle` (`Shuffle`),
`vote`; `regenerationApplication`'s tap is handed to its controller.
Callers: unearth's `returnToBattlefield`, station's `enact`, Glimpse of
Freedom and Incarnation Technique (`agent: actor`), Krosan Grip, Capsize,
Dead Revels, Grim Harvest, Sublime Exhalation (`agent: None`). The loader
writes `Enact(verb: Action(label), instruction: body)`; the meta comment says
so. `insertPart` keeps its optional `agent`; `choose` and `chooseWhile` still
write the chooser slot.

**Term change, classified** (`cargo xtask expansions` and the scratch card
dumper, before on `zkznylqolkok`, after on `vlyvxqnymyly`; 1598 files, a
classifier that normalizes both sides and counts what vanished): unchanged
1336; (A) an `agent` slot that held `Actor` vanished, otherwise identical:
199 (121 expansions, 78 cards), and with the next classes 14 more; (A-you) a
slot that held `You` vanished, the expansions' own sample arguments
(`Shuffle(agent: You)`, `changeLife`'s sample) and station's written `you`:
37 expansions, with (A) in `doUnless`, `may`, `mayOrElse`,
`faceAVillainousChoice`; (A0) an optional slot that held `None` vanished, so
the deed is now performed by the actor in context where its row names a
performer (Destroy, Return, Exile, Meld, Manifest): expansions
`exileWithCounters`, `manifestPlacement`, `meldInto`, `returnTo`,
`returnToBattlefield`, `returnToBattlefieldTransformed`, `persist`,
`undying`, `enact` (with A-you), `madness`, `reconfigure`, `earthbend` (with
A); cards Capsize, Dead Revels, Krosan Grip, Putrid Goblin, Sublime
Exhalation, Young Wolf, Arrogant Wurm, Grim Harvest, Leech Gauntlet, all
proved by `lean-check`; (B) a slot that held another player became
`Act(p, …)`: `regenerationApplication`, `PossessorOf(Controller, subject)`.
Unexplained: 0. No slot grew anywhere.

**Proof** (stage 2 tree).
- `cd lean && ./scripts/build`: Build completed successfully (82 jobs).
- `cargo xtask lean-check`: canon 127/127, testing 6/6 (2 min 1 s), on stage 1
  and on stage 2.
- `cargo xtask facts check`: up to date.
- Gate, stage 1: `cargo xtask gate --changed` derived `cargo test -p
  deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`, run with
  `--no-fail-fast`: 62 binaries, 603 passed, 0 failed, 2 ignored. Stage 2:
  `cargo test -p deckmaste_construction_core -p deckmaste_lexical_source -p
  deckmaste_semantics_v2 -p deckmaste_construction_v3 -p deckmaste_english_v3
  -p xtask --no-fail-fast`: 92 binaries, 1198 passed, 1 failed
  (`a_keyword_declaration_builds_its_wrapper`, its `shuffle` expectation still
  `Shuffle(actor)`; re-spelled to `Shuffle`, the binary re-run 6/6), 2
  ignored (both pre-existing, each naming its blocker). `lean_drift` 4/4.
- `cargo xtask cite check --list-noncompliant` 0; `cite check` 0 stale;
  `cite audit --diff` read for every stage. rustfmt clean on the changed
  lines; `cargo clippy -p deckmaste_semantics_v2 --all-targets` clean.

**Tests** (Rust). Re-spelled, each keeping its subject and its value
comparison: `keywords.rs` `a_keyword_action_enacts_its_deed` (the wrapper
names no performer; an `agent` field is still refused);
`plugins_v2_declarations.rs` `a_keyword_declaration_builds_its_wrapper`
(destroy, heal, shuffle) and its inline `Draw`s; `reader.rs` (src)
`a_non_identity_native_collision_is_refused`, `an_identity_native_collision_is_exempt`,
`a_camel_case_declaration_does_not_collide_with_its_constructor` (inline
`Shuffle`/`Draw`);
`tests/reader.rs` `amass_expands_to_the_term_its_constructor_body_spelled`,
`the_create_token_and_add_subtype_helpers_expand_to_their_basis_terms`,
`an_undeclared_field_on_a_constructor_is_refused`, `a_declared_field_still_reads`;
`tests/corpus.rs` `every_ported_alias_expands` (`Shuffle`);
`xtask/src/expansions.rs` the sample `Shuffle` and
`a_helper_spelling_and_its_constructor_spelling_print_the_same`. Restored 0,
ignored 0 new, added 0, removed 0. `keyword_bodies.rs` still reads an
`agent` a keyword file writes (none does; the loader refuses one).

**Deviations and additions.**
1. The group-performer rule is beyond the ticket's letter: without it the 26
   pins above change outcome when the explicit plural agent goes.
2. `enactPerformer`/`handedOne`: a deed whose row names no performer records
   none, as its `none` slot did, and a handoff naming a performer for it is
   refused, as the explicit agent was (`badAgentedAgentlessAct`). `act you`
   is therefore not transparent for such a deed; `lean/CONTRACTS.md` says so.
3. Partial landing: `choose` and `insertPart` keep their optional agent (STOP).
4. The Rust mirror and RON land in the Lean stage's commit (see above).
5. Cost twin pins keep a reflexive second conjunct rather than lose it.
6. `exists`, `agentRef`, `agentPlur` and the `Actor` helpers named above kept.

**STOPs.** One, above: `choose` and `insertPart`.

**Glossary.** **Actor**'s `_Avoid_` line no longer says the agent fields will
be deleted; it names the two left. No gap found.

**Not applicable.** Coverage lock, selection census, licensing-checker
totals, homograph and form-literal inventories and the performance advisory:
no English grammar, lexicon or corpus input changed.

**Ticket text that proved wrong.** "Delete … the 3 optional ones": two cannot
go without an owner decision. The ticket's "`agentIntro`/`agentCtx` …
`mayCtx`" read the field only through `choose` and offers; `agentCtx` stays
for `choose`. The ticket's `Enact(Action(label), body)` is
`Enact(verb: Action(label), instruction: body)`.

**After refresh.** `kata refresh` rebased the series onto twelve commits from
the default line, `mlmrrpuzpnxr` (complete `semantics-v2-deed-performer-roles`,
already in the claim's base before the first refresh) excluded:
`wnmnkpsqoolk`, `rqprnwtynxro`, `vyzkyyrrulpv`, `quzyqpyyxwzz`
(english-v3 targeting projection), `runqsyypltqs`, `mlsztrukkkpy`,
`lkokrvwznmpx` (english-v3 relative-clause adjuncts), `urtrlrlkspyy`,
`rkkokprpqkrn`, `swlrnxpooovz` (english-v3 by-complement functions),
`oxyxrpknnlwq` (claim `semantics-v2-designation-is-a-name`), `uutxrkxtpsxs`
(claim `english-v3-frame-coordination`); they touch `deckmaste_english_v3`,
the lexicon, the Oracle English glossary and tickets, and no Lean, RON,
mirror or xtask file. No conflicts. On the refreshed tree:
`lean/scripts/build` completed (82 jobs); `lean-check` canon 127/127, testing
6/6; `facts check` up to date; the derived command `cargo test -p
deckmaste_construction_core -p deckmaste_lexical_source -p
deckmaste_semantics_v2 -p deckmaste_construction_v3 -p deckmaste_english_v3
-p xtask`, run with `--no-fail-fast`: 95 binaries, 1213 passed, 0 failed, 1
ignored (pre-existing, naming its blocker); `cite check
--list-noncompliant` 0, `cite check` 0 stale (16051); `kanban check` OK.
