---
needs: [lean-v3-authoring-impl-10-definitions-and-cards-polish]
---
# Fix Lean v3 checker structural and DRT defects

**Resolve the structural DRT, scoping, deed-stamping, and refusal defects in
`lean/Semantics/Check/` identified in
[`docs/evidence/lean-v3-opus-findings.md`](../../evidence/lean-v3-opus-findings.md)
(§§2.1–2.13, §3.1–3.3, §1.16, §5.4, §6.7) and
[`docs/evidence/lean-v3-astra-findings.md`](../../evidence/lean-v3-astra-findings.md)
(§§2, 7).** Standard constraints apply.

## Scope

### 1. Referent Stamps, Clauses, Modal Boxes, Self/Performer Identity & Disjunction Scoping (Opus §§2.1–2.6)

- **Accumulate deeds on a referent (§2.1):** Replace single-verb overwrite in
  `Stamp` (`Check/Words.lean:135-141`, `verb : Option Deed`; `applyInPlaceStamp` at
  `Check/Phrase.lean:622-635` still sets `verb := some v, through := []`; `stampedBy` at
  `Check/Words.lean:1124`) with accumulated `.actedOn deed` stamps
  per [`docs/decisions/semantics-v3.md`](../../decisions/semantics-v3.md) §1.2/§1.5
  so an intermediate action (e.g. `"Exile target creature. Reveal it. Return the
  exiled card..."`) does not erase an earlier deed stamp.
  **Reconciled 2026-10-24: live** (the Zone Change now rides on its own `Stamp.zoneChange`
  field from [lean-v3-move-deeds-and-verbs](../done/lean-v3-move-deeds-and-verbs.md), but the
  deed itself is still a single overwritten `verb`).
  **Done 2026-10-24: mmxlylvw** (`Stamp.verb`/`through` are one accumulated `Stamp.deeds`, latest
  first, reset by a move [CR#400.7]; `applyThroughStamp` folded into `applyInPlaceStamp`; pin
  `Proofs/Turn.lean` `deedsAccumulateOnAReferent`).
- **Advance `nextClause` in `Cost.compound` (§2.2):** Update the `Cost.compound`
  fold in `Check/Phrase.lean:4453-4456` to advance `nextClause` between members per
  `lean/CONTRACTS.md` ("Enclosed traversals", `:627`) so cross-cost pronouns
  (`"Sacrifice a creature, Exile it:"`) resolve.
  **Reconciled 2026-10-24: live** (the fold still threads `ops.cost d x` with no `nextClause`).
  **Done 2026-10-24: qxrvulpl** (pins `okLaterCostMemberReadsAnEarlierOne`, and
  `badTwoCostMentions` rewritten as `okTwoCostMentionsItReadsTheNearer`: with a clause per member,
  `D′` reads "it" as the nearer, sacrificed creature).
- **Close modal boxes across all four deontics (§2.3):** Close the inner DRS box
  on `must`, `can`, and `gatedBy` in `Check/Phrase.lean:4315-4353` just as `cant`
  (`:4302-4314`, via `publishProhibitionSources` at `:1761`)
  does (`docs/decisions/semantics-v3.md` §1.3 box 3), preventing modal-scoped
  referents from leaking into outer discourse.
  **Reconciled 2026-10-24: live** (`must`/`can` return `drs4` with every inner referent;
  `gatedBy` filters only the payer).
  **Needs ruling 2026-10-24.** Closing `must`/`can`/`gatedBy` exactly as `cant` does
  (`publishProhibitionSources`: only targets, definite mentions and attachment hosts escape)
  breaks *Nowhere to Run* (`Cards/Deontic.lean` `okNowhereToRunWardLine`): "Creatures your
  opponents control can be the targets … as though they didn't have hexproof. Ward abilities of
  those creatures don't trigger." reads the permission's universal patient ("creatures your
  opponents control", det `.all`) by "those creatures"; every other pin passes. The ADR's box 3
  closes "hypothetical indefinite or universal participants" of a prohibition, and says nothing of
  a permission's. Options: (a) close all four as `cant` does, and re-express *Nowhere to Run*'s
  second sentence without the anaphor (against its Oracle wording); (b) close only indefinite
  participants (det `.a`, `.part`, `.count`) in all four deontics, publishing universal domains
  (det `.all`/`.each`), and amend ADR §1.3 box 3 for `cant` too; (c) as (b) for `must`/`can`/
  `gatedBy` only, leaving `cant` as it is. The attempt (a closing helper that also keeps the
  events a permission writes and the referents' resolution bookkeeping, `resolvedIds` and
  `contrastAnchors`, which `publishProhibitionSources` drops) did not land.
  **Owner ruling 2026-10-24: option (b),** in all four deontics, keeping a permission's events and
  the resolution bookkeeping (`resolvedIds`, `contrastAnchors`); ADR §1.3 box 3 amended.
  **Done 2026-10-24: tynwqmzx** (`closeModalBox` replaces `publishProhibitionSources`; pins
  `Proofs/Deontic.lean` `permissionClosesItsBox`, `requirementClosesItsBoxButPublishesItsTarget`,
  `gatedProhibitionClosesItsBoxButPublishesItsTarget`; `SelectionEvidence`'s
  `hypotheticalProhibitionParticipantsStayClosed` rewritten as
  `prohibitionClosesItsIndefiniteAndPublishesItsUniversal`, since a prohibition's universal domain
  is now published; *Guardian Beast*'s "can't be enchanted" re-spelled over "an Aura" (`a aura`,
  closed) instead of "all Auras", which, published, tied with the artifacts for "they").
- **Produce `Determiner.self` for `"this"` and `"you"` (§2.4):** Fix `.this` and
  `.you` in `Check/Phrase.lean:3158-3164` (`.this`) and `:3135-3140` (`.you`) so they use
  `Determiner.self` (`Check/Words.lean:19`) / reuse their
  referent rather than pushing a fresh `det := .the` referent on every mention
  (activating the dead `.self` guards in `wordNow` (`Check/Words.lean:1096-1098`),
  `reachesCharacteristic` (`:1172-1175`), and `countTokenSpecs` (`:1527-1529`)
  and refusing `"Tap this creature. Untap that creature."`).
  **Reconciled 2026-10-24: live** (no checker site produces `det := .self`).
  **Needs ruling 2026-10-24.** Producing `.self` for "this" and "you" activates `wordNow`'s guard,
  so no noun word reads the Source or "you"; about 30 pins then fail, in three groups:
  (1) Definitions written in the CR's own words, where "that permanent"/"that creature" reads the
  performing permanent, which "this creature" may be: endure, "that permanent's controller …
  on that permanent" [CR#701.63a]; explore and the owner's spelling pin
  `Proofs/Actor.lean` `okExploreReadsThatCreature` ("The owner's spelling 'that creature' reaches
  the permanent too, handed to … 'this creature'"); haste's "that creature"; riot, connive,
  enchant, crew, and the cards built on them (*Akki Lavarunner*, *Chittering Host*, *Revoke
  Privileges*, …); (2) "they"/"that player" (`.pro (.word .player)`) reading "you"
  (`Proofs/Actor.lean:457` handoff, `Proofs/Static.lean:349` `duringNextTurnOf (that player)`);
  (3) state pins on the Source's determiner (`Proofs/Actor.lean:821`, `Proofs/ActionFamilies.lean
  :133`, `Cards/Anaphora.lean:594` `thisAurasController`). (1) contradicts an owner spelling, so it
  is not settled here. Options: (a) `.self` excluded from every noun word, as `wordNow` says, and
  the Definitions re-spelled with `this`/`actor` (against [CR#701.63a]'s and the owner's
  wording); (b) inside a handoff to "this …" the performer is put in view as an ordinary
  referent (`det := .the`), so a Definition's "that permanent" reads it, while outside one the
  Source and "you" are `.self`; (c) `.self` excluded only from demonstratives over an object
  ("that creature"), "they"/"that player" still reading "you". Reuse of one referent per "you"
  mention (`mentionYou`) was clean in the attempt; "this" was kept one referent per mention, since
  each mention may name the Source in another zone. Also needed whatever is chosen:
  `publishProhibitionSources` and `Condition.matches`'s `subjectOk` must treat `.self` as definite.
  No change landed.
  **Owner ruling 2026-10-24: option (b).** Outside a handoff, `.this` and `.you` get
  `det := .self` and reuse their referent; inside a handoff (`instruct`, or a permanent-performed
  Definition's implicit performer) the performer is an ordinary referent (`det := .the`), so
  CR-worded Definitions read it as "that permanent"/"that creature"; `publishProhibitionSources`
  and `subjectOk` treat `.self` as definite; group 2 pins are audited one by one and group 3 pins
  updated to `.self`.
  **Done 2026-10-24: ypqprsvk** (`mentionThis`, `mentionYou`, `performerInView`/
  `performerOutOfView`; "you" stays `.self` in its own handoff, since the ruling's group 2 has no
  third-person reference to "you"; a linked referent is never re-mentioned. Exposed and fixed: a
  prohibition that modifies the preceding action ("Destroy target creature. That creature can't
  be regenerated.", *Nekrataal*) read the Source, not the destroyed creature; it now reads that
  action's objects by last known information [CR#608.2h]. Definition re-spelled: haste's "that
  creature" is "this creature". Pins added: `Actor.thisThenThatCreatureIsRefused`,
  `Actor.twoYousAreOneReferent`. Group 2, rewritten: `Static.durationPossessorReadsTheExplicitPlayer`
  (now "your next turn"; "that player" reading "you" is refused), `Actor.handedToYouPossessorReadsExplicitPlayer`
  → `handedToYouThirdPersonDoesNotReadYou` (it asserted "they" reading "you", against its own
  docstring). Third-person noun words reading the Source, rewritten to read it by "it" or "this
  creature" and to assert the refusal: `Attachments.sourceCounterpartRemainsAvailable`,
  `sourceCounterpartDemonstrativeSelectsTheSourceIdentity` →
  `sourceCounterpartReMentionSelectsTheSourceIdentity`, `Trigger.selfAttackPublishesItsCreature`,
  `Counters.alternateHeadersPublishTheirSelf`, `Counters.threeHeadersPublishTheirSelf`,
  `Damage.damagedSelfIsReadableAsThatCreature` → `damagedSelfIsReadableByItNotThatCreature`,
  `GetsBothDeltas.modernStaticSelfRemainsReadableAsThatCreature` →
  `modernStaticSelfRemainsReadableAsThisCreature`, `Keyword.staticSelfRemainsReadableAsThatCreature`
  → `staticSelfRemainsReadableByIt`, `Cards/Anaphora` `possessiveDeicticDemonstrativeSelectsTheSource`
  → `possessiveDeicticSourceIsNoDemonstrativeAntecedent`, `nekrataalOneCreatureWord` →
  `nekrataalNoCreatureWord` (it counted the Source). Group 3, updated to `.self`:
  `ActionFamilies.movingThisAbilityAllocatesFreshIdentity`, `Actor.handedToThisPublishesSourceAndController`,
  `Damage.karplusanYetiItsReadsThatCreature`, `Counters.agreeingAlternativeHeadersSelectTheOtherCreature`.
  Ids renumbered because the "you"s are one referent: `Choice.chosenCreatureReadsBackTheTarget`,
  `Counters.thranduilsCompanyItIsTheTargetCreature`, `GetsBothDeltas.jeskaiAscendancyUntapsItsPumpedGroup`,
  `GetsBothDeltas.archpriestOfIonaGivesFlyingToItsPumpedTarget`; `Cards/Choice`
  `joustConditionReadsTheClauseSubject` counts only the objects resolved, "you" being resolved
  too. `ControllerSacrifice`'s `controllerInstructState` mirrors the handoff; statements unchanged).
  **Owner ruling 2026-10-24 (HIGH-1, refines (b)).** Inside a handoff the performer is in view
  as an ordinary referent (`det .the`) only inside `ownScope` Definition bodies (explore, connive,
  endure, …) and in nested clauses after the handoff's own clause; in the handoff's own immediate
  clause `thisCreature`/`thisPermanent` stays `.self`, so *Terror of the Peaks*'s "this creature
  deals damage equal to that creature's power" and *Rampaging Ferocidon*'s "that creature's
  controller" read the triggering creature.
  **Done 2026-10-24: yoyruwqs** (`ActorShape.selves`: a handoff to "this …" records its Source
  performer, put in view by `nextClause` and `enterOwnScope` (`performersInView`), restored to
  `.self` on leaving the handoff; `performerInView` is kept for a Definition's implicit performer.
  Pins added: `Actor.terrorOfThePeaksThatCreatureIsTheEnteringCreature`,
  `Actor.rampagingFerocidonThatCreatureIsTheEnteringCreature`. Rewritten:
  `Counters.threeHeadersPublishTheirSelf` (it read the Source by "that creature" in the handoff's
  own clause; now "its power" checks and "that creature" there is refused). `ControllerSacrifice`'s
  `controllerInstructState` and `controllerAlreadyInView` mirror the new handoff; statements
  unchanged. `okExploreReadsThatCreature` and the explore, connive and endure pins pass unchanged).
- **Wire `DRS.performer : Option ReferentId` and retire fake `Payload.actor`
  referents (§2.5):** Track the performer in `DRS.performer` per
  `docs/decisions/semantics-v3.md` §1.2/§1.4 instead of pushing `Payload.actor`
  frames with `.outcome` kind into `referents` and filtering `!b.isActorFrame` /
  `r.kind != .outcome` across counting helpers; reconcile `Payload`'s 11
  constructors with the ADR.
  **Reconciled 2026-10-24: live** (`DRS.performer` exists at `Check/Words.lean:460` but no
  site assigns it; `Payload.actor` (`Check/Words.lean:251`, 11 constructors at `:235-252`) frames
  are still pushed at `Check/Phrase.lean:854,3458,3497` and `Check/AbilityRules.lean:163`, and
  `isActorFrame` (`Check/Words.lean:386`) is filtered at 21 `Check/` sites).
  **Needs ruling 2026-10-24.** The ADR's `performer : Option ReferentId` cannot hold what an
  actor frame carries: a performer of a kind with no referent (`ActorShape.refIds = []`, stepped
  as `kindBinding shape.kind`), a group performer (several `refIds`), and `isYou`/`ascribable`;
  and frames nest (one per handoff, dropped by `dropActorFrame`). "Reconcile `Payload`'s 11
  constructors with the ADR" also needs a decision: `.difference`, `.ability`, `.pile` and `.join`
  are all live. Options: (a) `DRS.performers : List ActorShape` (innermost first) outside
  `referents`, and amend ADR §1.2's field to it; (b) `performer : Option ReferentId` as the ADR
  says, with a referent always pushed for a kind-only performer and a group referent for a group,
  `isYou`/`ascribable` derived from it; and for `Payload`, amend the ADR's list to the live
  constructors, or fold `.difference` into `OutcomeSort` and keep `.ability`/`.pile`/`.join`.
  Removing the frames renumbers every referent after a handoff, so the `Proofs/ControllerSacrifice`
  structural proofs and many id pins move with either. No change landed.
  **Owner ruling 2026-10-24: option (a).** `DRS.performers : List ActorShape`, innermost first,
  outside `referents`; `Payload.actor` and `isActorFrame` retired; ADR §1.2 amended to list
  `performers` and the live `Payload` constructors; id pins that move only by renumbering are
  updated with that reason; the `ControllerSacrifice` structural proofs keep their statements.
  **Done 2026-10-24: qzllommp** (`ActorShape` gains `plur`, the frame's number; `DRS.performer`
  is replaced by `performers`; `DRS.pushPerformer`/`dropActorFrame` push and pop. Id pins
  renumbered only because a performer no longer takes a `ReferentId`: `DrtCards`'s three
  *Weeping Angel* pins, `EventPatterns.damagePublishesParticipantsAndOutcome`,
  `MovementScopes.distributedMovementKeepsAnOuterCardSingular`, `Choice`'s
  `distributedRestPreservesTheUnrelatedOuterGroup` and
  `distributedSacrificeConsumesTheOuterRemainder`, `Damage`'s
  `fightUsesTheTwoBoundParticipantsInOppositeDirections`, `recentSecondOpponentIsSelected`,
  `okShortOfCeilingAnnounced`, `okThatCreatureAfterDamage`, `karplusanYetiItsReadsThatCreature`,
  `garrukRelentlessItsReadsThatCreature`, `deflectingPalmThatSourceReadsTheChosenSource`,
  `ContrastAnchor.inquisitorsFlailAnchorsOnTheEquippedCreature`,
  `Turn.okDelayedOtherAnchorsOnTheSettledTarget`, `Counters`'s
  `agreeingAlternativeHeadersSelectTheOtherCreature` and `thranduilsCompanyItIsTheTargetCreature`,
  `GetsBothDeltas`'s `jeskaiAscendancyUntapsItsPumpedGroup` and
  `cycleOfLifeKeepsItsCastCreatureTarget`, and `ControllerSacrifice`'s
  `existingReferenceIsNotIntroducedAgain`, `riderReadsTheStampedPatient`,
  `repeatedControllerSacrificesKeepDistinctPatients`, `laterCardIsTheSacrificedObject`,
  `outerArtifactDoesNotStealThePatient`. `EventPatterns.patientOnlyDoesNotManufactureSource`
  reads `performers = []`. `ControllerSacrifice`: the conclusions of
  `sacrificeMovePreservesEveryUnrelatedOuterReferent` and
  `nestedDescriptionMovesOnlyItsOwnReferent` are unchanged; each drops the hypothesis about actor
  frames in the outer context (`notActor`, `actorLinks`), which named the retired `Payload.actor`
  and is now vacuous; the private helpers lost the frame (`argumentPrefix`, `afterPrefix` and the
  moved creature renumbered from `fresh + 5` to `fresh + 4`), and `dropFirst_append`,
  `dropActorFrame_append` and `remapFixed` are gone with it).
- **Consistent disjunction, conditional, and negation scoping (§2.6):** Fix
  `NounPhrase.or` (`stepNounAlternativesWith` in `Check/Phrase.lean:1532-1553`),
  `Timing.or`/`.and` (`Check/Phrase.lean:965-968`), `Location.or` (now
  `Location.coord`, `:1016-1020`), `Location.not` (`:1015`), and `doIf`
  (`Check/Phrase.lean:4030-4081`) so branches
  do not thread left-to-right or leak branch-local non-target indefinites into
  outer discourse (preserving announced targets `[CR#601.2c,115.1]` and outer
  mutations across both `elseInst = none` and `elseInst = some _`, matching
  `doOnlyIf`, now a Template over `doIf`), and update `lean/CONTRACTS.md` (`doIf` row at
  `:640`) to match.
  **Reconciled 2026-10-24: live** (partly: `doIf` now announces then-branch targets before the
  condition and hands them, with named amounts, to the else-branch, per `lean/CONTRACTS.md:640`,
  no single landing ticket; `NounPhrase.or` branches now each start from the pre-disjunction
  context. Still live: `NounPhrase.or` keeps every branch's additions; `Timing.and/.or` and
  `Location.coord` thread left-to-right; `Location.not` returns the inner output; `doIf` with no
  else-branch keeps the branch's non-target indefinites (`CONTRACTS.md:640` currently says "its
  output continues"), and with an else-branch returns `withTargets`, dropping the branch's
  outer mutations).
  This ticket owns the `lean/CONTRACTS.md:640` `doIf` row and rewrites it in the same landing as
  the `doIf` fix (cross-noted 2026-10-24 in `lean-v3-proofs-and-docs-alignment` §3).
  **Partly done 2026-10-24: qyupknzq, vunkvywv.** qyupknzq: `NounPhrase.or`, `Timing.or` and
  `Location.coord .or` step each alternative from the enclosing context and keep what every
  alternative introduces (`commonHeaderContext`, as `Condition.or` does), with no joined head
  referent (one over "you or a white creature you control" is read by "that creature" as a join's
  creature half, which is *Mangara's Equity*'s wrong reading); `Location.not` closes as
  `Timing.not` does; `Timing.and` and `Location.coord .and`/`.andOr` still thread in order, as
  `Condition.and` does (dynamic conjunction, not a branching); pins in `Proofs/Composition.lean`,
  and `mangarasEquityRawDemonstrativeSelectsWrongCreature` rewritten as
  `mangarasEquityRawDemonstrativeSelectsTheDamageSource` (§3's *Mangara's Equity* bullet keeps the
  explicit-selector deletion). vunkvywv: a `doIf` with an else-branch keeps the outer referents and
  announced targets as either branch left them and closes what the condition or a branch
  introduced (`Proofs/Description.lean` `otherwiseKeepsWhatEitherBranchDidToAnOuterReferent`);
  the `CONTRACTS.md` `doIf` row's else sentence is rewritten.
  **Needs ruling 2026-10-24 (no-else `doIf`).** Closing a no-else branch's non-target
  introductions contradicts the landed ruling in
  [lean-v3-authoring-impl-14-ast-and-authoring-cleanup](../done/lean-v3-authoring-impl-14-ast-and-authoring-cleanup.md)
  ("the branch's output continues as `doIf`'s always did … in 'Create a token if you control a
  creature. Put a counter on it.' 'it' is the token … retired on purpose;
  `conditionalArmAntecedentIsTheToken` pins it"), and breaks `noElseConditionExportsItsOpponent`,
  *Kalitas*'s `kalitasSelectsTokenThenDestroyedCreature`, and the fight Definition's guard, whose
  targets are introduced in its condition (`Proofs/Damage.lean`, `Proofs/CallerScope.lean`,
  `Proofs/BindingIdentity.lean`). Options: (a) keep the impl-14 ruling: a no-else branch's
  output continues, and the ADR's box 2 sentence is narrowed to the else case; (b) close a no-else
  branch's non-target introductions per box 2, retiring the impl-14 pin, while targets announced
  anywhere in the `doIf` (its condition included) and caller-owned arguments survive.
  The `CONTRACTS.md` no-else sentence ("its output continues when there is no else-branch") is
  unchanged until this is ruled.
  **Owner ruling 2026-10-24: option (a).** The impl-14 ruling stands: a no-else `doIf`/`doOnlyIf`
  branch's output continues; a two-armed `doIf` closes branch-local non-target introductions (as
  vunkvywv does). ADR §1.3 box 2 and the `CONTRACTS.md` no-else sentence are narrowed to match.
  **Done 2026-10-24: qyupknzq, vunkvywv, suvlwolo** (suvlwolo narrows the ADR and the
  `CONTRACTS.md` `doIf` row's no-else sentence; no code changes for the no-else case).
- **Foreground non-argument possessor re-mentions in `DRS.recordMention`:** When
  an existing referent is re-mentioned in a non-argument slot (`inArgSlot = false`,
  e.g. `"that creature's owner"`), update `r.clause := drs.currentClause` and
  `r.role := .other` (matching a newly introduced possessor NP) without recording
  in `drs.pendingRoles`, so *Weeping Angel*'s raw `"it"` in
  `"that creature's owner shuffles it into their library"` (`Proofs/DrtCards.lean:264-290`)
  resolves directly to the damaged creature without `.laterMention`.
  **Reconciled 2026-10-24: live** (`DRS.recordMention` at `Check/Words.lean:664-671` updates
  neither `clause` nor `role`; `weepingAngelRawPronounCurrentlySelectsTheDamageSource` at
  `Proofs/DrtCards.lean:284-289` still pins the raw `it` reading the source).
  **Done 2026-10-24: vwqxxutk** (in `DRS.recordMention`, keyed on the characteristic read a
  possessor, a stat read or a copy source sets, since a nested modifier mention such as "Auras
  attached to those artifacts" must not foreground, as *Guardian Beast* shows; pin rewritten as
  `weepingAngelRawPronounSelectsTheDamagedCreature`, and
  `weepingAngelOwnerCanShuffleItsCreatureIntoTheirLibrary` also checks the raw "it").

### 2. Shared Action-Clause Spine, Head Gates & Deed Identity (Opus §§2.7–2.8, §3.1–3.3)

- **Share the action-clause spine (§2.8):** Factor `stepDidClauseWith`
  (`Check/Phrase.lean:1349`), `stepWouldClauseWith` (`:1458`), `ev .does` (`:3409`),
  and `cant`/`must`/`can`/`gatedBy` (`:4302-4353`) onto a single shared action-clause stepper
  (`docs/decisions/semantics-v3.md` §3.1), eliminating drift in locus/historical
  handling and actor-kind checks (including `.draw`).
  **Reconciled 2026-10-24: live** (they share `stepEventSubjectWith` (`:892`) and
  `stepSubjectAfterWith`, but each still hand-rolls the head gate, the `.draw` actor-kind
  exemption (`:1463-1467`, `:3412-3416`) and the locus check).
  **Done 2026-10-24: ltorsnkp** (`stepActionClauseWith`: `does`, `would`, `did`, `cant`,
  `can`/`must`, `gatedBy` and `asThough` step one spine, each adding its own part through hooks;
  the deontic rules now check their performer's kind and a passive agent's as event clauses do;
  the `.draw` exemption is one named predicate, `Action.checksItsPerformer`, since a draw refuses a
  non-player as it steps; the locus check went with `.enact` (mqmzkunq). Pins
  `Deontic.prohibitionAndEventRefuseANonPlayerCasterAlike`,
  `Deontic.drawRefusesANonPlayerOnceInEveryClause`; ADR §3.1 describes the spine).
- **Reconcile `deonticHeadOk` and `eventHeadOk` (§2.7):** Align
  `Check/Events.lean:25-67` (`eventHeadOk` `:25-45`, `deonticHeadOk` `:48-67`) so
  non-battlefield zone moves (e.g. `move` to exile)
  and `ownScope` headers are admitted consistently across event and deontic
  clauses.
  **Reconciled 2026-10-24: live** (`deonticHeadOk` admits only `.move … battlefield` (`:64`)
  and only `ownScope (some (.action _ _))` (`:52`); `eventHeadOk` admits any move and any
  `ownScope (some _)`).
  **Needs ruling 2026-10-24 (raw non-battlefield moves).** ADR §3.7 ("Closed Deontic
  Head-Gating") rules that in a deontic clause "a raw `move` must name the battlefield as its
  destination"; this bullet asks that a move elsewhere (to exile) be admitted there as in an event
  clause. With deed guessing retired (§3.1 below), a raw move to exile names no deed, so
  `cant(…, exile …)` (the `exile` Definition, admitted by its header) and `cant(…, move … exile)`
  are no longer one act written two ways. Options: (a) keep the ADR rule: a deontic clause
  constrains a Zone Change other than onto the battlefield only through its Definition (`exile`,
  `destroy`, `sacrifice`, …); (b) admit every raw move in a deontic clause, as the event gate
  does, and amend ADR §3.7. The `ownScope` half needs no ruling and is done: both gates admit any
  Definition header through one traversal (`Action.headGate`).
  **Partly done 2026-10-24: yoqskptw** (the `ownScope` half: `Action.headGate` over
  `Action.eventHead`/`Action.deonticHead`; pin
  `DefinitionsAndTemplates.headGatesAgreeOnADefinitionHeader`; ADR §3.7 reworded. The raw-move half
  awaits the ruling above).
  **Owner ruling 2026-10-24: option (b)**, admitting any `.move` in a deontic clause, as the event
  gate does (the review's witness search found no Vintage-legal card that needs it). `leave`,
  `putInto`, `returnTo`, `returnToHand` and `returnToBattlefield` are verified Templates over
  `.move` (`Templates.lean`), not Definitions.
  **Done 2026-10-24: yoqskptw, yxouwpnx** (yxouwpnx: `Action.deonticHead` admits `.move ..`; ADR
  §3.7 amended; pin `Deontic.prohibitionAdmitsAnyZoneChange`).
- **Unify deed stamping and prune dead `CoreDeed` constructors (§§3.1, 3.3):**
  - Eliminate the three competing deed-stamping mechanisms (`ownScope (some (.action ...))`
    header, `.enact`, and `Action.inferredDeed` string-guessing at
    `Check/Phrase.lean:178-200`) and remove the `ownScope` + `.enact` double-wrap (still
    written by every Action Definition, e.g. `Definitions/Actions.lean:283-284,622-623`; the
    stepper ignores the `ownScope` deed, `Check/Phrase.lean:3445-3450`, and stamps from
    `.enact`, `:3451-3469`).
  - Delete the 8 dead `CoreDeed` constructors (`attack`, `block`, `gainLife`,
    `loseGame`, `winGame`, `spend`, `unlock`, `tapForMana` in `Words.lean:318-321`).
    Added 2026-10-24: `CoreDeed.copy` (`Words.lean:319`) is a ninth, never constructed anywhere
    in `lean/` (spell and ability copies go through `Action.copy`, not a `CoreDeed`).
    **Reconciled 2026-10-24: live.**
  - Added 2026-10-24: the `ownScope` header's deed is read by the head gates and the deed guess
    (`eventHeadOk` `Check/Events.lean:29`, `deonticHeadOk` `:52`, `inferredDeed`
    `Check/Phrase.lean:184`, `eventLayers` `:1190`) but discarded by the stepper itself
    (`let _ := deed`, `Check/Phrase.lean:3445-3450`), so the header's deed and the `.enact`
    stamp can disagree unchecked. Unifying the stamping mechanisms must leave one deed that
    both the gates and the stepper read. **Reconciled 2026-10-24: live.**
  - [lean-v3-move-deeds-and-verbs](../done/lean-v3-move-deeds-and-verbs.md) removes
    `CoreDeed.put` and `.return_`, the `inferredDeed` cases that guess them, and
    gives a move its own stamp (owner ruling 2026-10-21). Neither ticket needs the
    other; whichever lands second rebases onto the other's stamp.
  **Reconciled 2026-10-24: live** (partly: the third sub-bullet is done by
  [lean-v3-move-deeds-and-verbs](../done/lean-v3-move-deeds-and-verbs.md): `CoreDeed` has no
  `put`/`return_` (`Words.lean:318-321`), `inferredDeed` keeps only the exile move case
  (`Check/Phrase.lean:187`), and a move stamps `Stamp.zoneChange` (`Check/Words.lean:138`), so
  this ticket rebases onto that stamp. Still live: the three stamping mechanisms and the
  double-wrap, and the 8 dead `CoreDeed` constructors, all still declared and never
  constructed).
  **Done 2026-10-24: xvknqmor, mqmzkunq.** xvknqmor deletes the nine dead `CoreDeed` constructors
  (`copy` included) and points ADR §3.6's `tappedForMana` row at the `Action.tapForMana` head.
  mqmzkunq: the `ownScope` header is the action's one deed (ADR §1.4, §2.1), read by the head
  gates, the caused event, the deed lookup and the stepper alike (`Action.ownDeed?`). `Action.enact`
  is deleted: the stepper's `ownScope (some deed)` case does what `.enact` did (the source
  permanent as performer, the patients stamped), and a move in the body carries the deed too
  (`OwnScopeFrame.deed`, `DRS.scopeDeed`), so cascade's run and stopping card are both exiled. Every
  Action Definition loses its inner `enact`; devour, madness's discard and reconfigure's unattach
  are `ownScope` over `inCaller` arguments; `transformsInto` moves to `Definitions/Actions.lean`
  and writes its face on `turnOver`'s new `into` [CR#701.27e]; the redundant locus is gone, and
  "shuffled your graveyard" is refused by the shuffle itself (`Refusal.shuffleZone`, replacing
  `lookbackLocus`, [CR#701.24a]). `inferredDeed` is `namedDeed` and guesses nothing: the exile,
  tap, untap, create and reveal shape cases are deleted. `checkDefinition` is gone with the
  double wrap it stripped. The hand-copied `exploreDef`/`conniveDef` in `Check.lean` hand off
  explicitly, so their headers no longer claim a permanent performer. Pins added:
  `DefinitionsAndTemplates.headerDeedIsReadByTheGateAndTheStepper`, `bareShapesNameNoDeed`.
  Rewritten: `Lookback.resolveRecordsNoStackMove` (the resolved spell now carries the resolve
  deed, still no Zone Change: "stamped by no deed" was the ignored header),
  `Lookback.locusIsReadFromTheEnactedBody` → `shuffledGraveyardNamesNoShuffle`,
  `Anaphora.okShuffleLocusAtLibrary` → `okShuffledYourLibraryThisWay`,
  `Anaphora.badShuffleLocusAtGraveyard` → `badShuffledYourGraveyardThisWay`,
  `DefinitionsAndTemplates.atomicDefinitionLaw` (`check`, not `checkDefinition`),
  `Actor.scopedBodyCannotReadTheCaller` (its unscoped half is the bare Definition), the
  `ControllerSacrifice` scope-step lemmas (one `ownScope` step, the move carrying the sacrifice
  deed; the public theorems' statements unchanged), and pins that wrote `.enact` as a transparent
  deed node (`Faces.redundantSameClauseShuffleHandoffIsRefused`, now the bare handoff;
  `MovementScopes.consumingActionChecksThePreMoveZone`, `SelectionEvidence`,
  `CorpusRecency`'s cycling context, now reading their caller arguments `inCaller`). Renamed:
  `counterIsSingleEnactedMove` → `counterIsOneScopedMove`, `okEnactedKeywordAction` →
  `okScopedKeywordAction`, `templateBareThisRetainsEnactedZoneExemption` →
  `templateBareThisRetainsScopedZoneExemption`. Retired: `Lookback.namedActionCanCarryItsLocus`
  (the locus field is gone; `Anaphora.searchLookbackCarriesItsScope` pins the search's own
  scope) and the `Authoring` witnesses `enactedDeedRecordsDefinitionAuthorship`, the "descry" and
  the "destory" examples (the `enact` template was the only card surface that named a deed).
- **Review fixes to this section (2026-10-24).** A review of the section's changes found one
  MEDIUM and seven LOWs, fixed as follows.
  - **MEDIUM, madness lost the exile deed** [CR#702.35a,702.35c]. **Done 2026-10-24: qqpvkmzq**
    (the instead body is `ownScope discard (exileFrom (.inCaller thisCard) hand)`, the trigger
    watches `exileFrom thisCard hand` caused by the discard; pin
    `Keyword.madnessCardIsDiscardedAndExiled`; `KeywordCastPermissions.castPermissionsKeepPayingInstead`
    rewritten to the new trigger).
  - **LOW, raw reveals.** **Done 2026-10-24: kuknqryk** (amplify, ninjutsu, transmute and
    *Vaevictis Asmadi, the Dire* use the `reveal` Definition; amplify counts "for each card revealed
    this way" by `forEach … was (reveal which) thisWay` [CR#702.38a]; pin
    `OffStack.amplifyCountsTheCardsRevealedThisWay`).
  - **LOW, reconfigure's unattach stamped with the attach deed** [CR#701.3d,702.151a].
    **Done 2026-10-24: qyzpptsz** (header dropped: unattaching is no attaching, so the bare
    attachment change names no deed; no unattach Definition added, since the CR defines no
    "unattach" keyword action of its own).
  - **LOW, fixtures.** **Done 2026-10-24: vzlwtkkr** (relabeled: `exploreDef`/`conniveDef` are
    `revealThenCountDef`/`drawDiscardCountDef` over the deeds `revealThenCount`/`drawDiscardCount`,
    plainly fixtures; `explore_def_sound`/`connive_def_sound` renamed
    `reveal_then_count_def_sound`/`draw_discard_count_def_sound`. Rewriting over production is
    `lean-v3-proofs-and-docs-alignment`'s, whose reference is updated).
  - **LOW, `Actor.scopedBodyCannotReadTheCaller`.** **Done 2026-10-24: otlrzvlq**
    (`.ownScope none (destroy it)`).
  - **LOW, `bareShapesNameNoDeed`'s citation.** **Done 2026-10-24: rvuowpmv** (cites ADR §2.1).
  - **LOW, `shuffleZone` refused an exiled face-down pile** [CR#701.24a]. **Done 2026-10-24:
    zvyplnpw** (a shuffle of exile is admitted; pin `Lookback.exiledPileCanBeShuffled`).
  - **LOW, misspelled-deed pin.** **Done 2026-10-24: ttsomvqz** (`#guard_msgs` on
    `example : Deed := destory` in `Proofs/Authoring.lean`, beside `(exile : Deed) = exile.deed`;
    `elabDeedName`'s docstring no longer talks about cards).

### 3. `theGrantor`, `permanentActor`, Refusal Hygiene, Positional Hacks, Fuel & Dead Code (Opus §§1.16, 2.9, 2.11–2.13, 5.4, 6.7; Astra §§2, 7)

- **Honor the head noun in `NounPhrase.theGrantor` (§1.16):** In
  `Check/Phrase.lean:3119-3126` (`| .theGrantor head => … payload := .object [] (some
  .battlefield) …`), set the pushed referent's zone and type from its `head : NounWord`
  (`Abilities.lean:359`; the earlier `HeadNoun` is retired), using the mapping the `this…` heads
  use: an emblem is put "into the command zone" and its abilities "function in the command zone"
  [CR#114.2,114.4]. Flip the pin `emblemGrantorReadsABattlefieldObject`
  (`Proofs/MovementScopes.lean:172-177`) to `some .command`. Re-verified 2026-10-21
  (mtg-rules review, `departures-lean.md` #54).
  **Reconciled 2026-10-24: live** (payload still hard-codes `some .battlefield`).
  **Done 2026-10-24: qkmxqwqo** (`NounWord.readPayload` is the one Head Noun reading, shared by
  `.headed` and `.theGrantor`; pin `emblemGrantorReadsABattlefieldObject` rewritten as
  `grantorReadsItsHeadNoun`: the emblem in the command zone, the permanent on the battlefield).
- **Align `permanentActor` facts with the CR (§2.11; mtg-rules review 2026-10-21,
  `departures-lean.md` #22):** adapt, harness and monstrosity are "this permanent" instructions
  [CR#701.46a,701.64a,701.37a] that the ability's controller follows [CR#608.2c] (a player is
  instructed "to 'monstrosity X'" [CR#701.37c]); only fight, explore, connive and endure instruct
  a permanent [CR#701.14a,701.44a,701.50a,701.63a]. Set `permanentActor := false` for all three
  (the facts rows are gone; the flag is now the `true` argument of `Deed.action` in each
  Definition's `ownScope`/`enact` header: monstrosity `Definitions/Actions.lean:283-284`, adapt
  `:622-623`, harness `:718-719`; read by `Deed.permanentActor` `Check/Words.lean:112-114`;
  consumed at `Check/Phrase.lean:1210,3453`). For adapt and
  monstrosity also replace the `.actor` patient with the source permanent (`this_`) in their
  bodies (`Definitions/Actions.lean:621-626,282-289`); harness (`:717-725`) already takes an
  explicit subject. Correct the [CR#701.46a] claim in `Proofs/Actor.lean:735-737`, the
  `permanentPerformedActions` pin and its docstring (`Proofs/Tables.lean:55-64`), the `Deed.action`
  docstring (`Words.lean:335-337`), `lean/CONTRACTS.md:485-496` (which still names
  `ActFacts.permanentActor`), and the "A permanent performs it" comments in
  `plugins_v2/builtin/macros/keyword_actions/{adapt,monstrosity,harness}.ron` (`:4-5`). Pin: under
  *Doc Samson*'s `would you (putCounters …)` (`Cards/Counters.lean:354-359`), Adapt 2 puts 3
  counters.
  **Reconciled 2026-10-24: live** (adapt, harness and monstrosity still write
  `.action … true`).
  **Done 2026-10-24: vzxszzmo** (adapt, harness and monstrosity headers drop the flag; adapt and
  monstrosity read `this_`; CONTRACTS, the `Deed.action` docstring and the three `.ron` comments
  corrected. Pins added: `Actor.bareAdaptIsTheControllers` (replacing `okBareAdaptDefaultsToThis`,
  whose "performed by the source permanent" was the defect), `Counters.docSamsonWatchesAdaptsCounters`.
  Rewritten: `Tables.permanentPerformedActions` (four, not seven), `badAdaptHandedToOpponent` →
  `adaptHandedToAPlayerReadsTheSource`, `badMonstrosityHandedToOpponent` →
  `monstrosityHandedToAPlayerReadsTheSource`, `badPlayersCantExploreOrAdapt` →
  `badPlayersCantExploreButCanBeForbiddenToAdapt` (a player follows adapt and monstrosity
  [CR#608.2c], and is instructed "to 'monstrosity X'" [CR#701.37c], so neither is a refusal). Retired: `okAdaptHandedToAPermanent`, which asserted that the
  body reads a handed permanent; it now reads the Source, and no card hands adapt to a permanent).
- **One constructor per refusal obligation (§2.12):** Stop reusing
  `.anaphor (.word .permanent)` for failed reflexive/`which`, `.lookbackSubject`
  for unused `which`, and `kindMismatch (.quality .color)` for non-replaceable
  text-change sorts (`Check/Phrase.lean:2882,2889` for `refl`/`which`, `:1440` for unused
  `which`; `Check/TextChanges.lean:32`, and the `.color` default in `TextWord.sortCheck` `:21`).
  **Reconciled 2026-10-24: live** (all three reuses remain;
  [lean-v3-checker-rules-scope-refusals](../done/lean-v3-checker-rules-scope-refusals.md) §1
  widened `QualitySort.replaceable` to creature types but left the refusal constructor to this
  ticket).
  **Done 2026-10-24: skyzplml** (new `Refusal.reflexiveAntecedent`, `whichUnbound`, `whichUnused`
  (replacing `lookbackSubject`, whose only site was the unused `which`), `textChangeSort` and
  `textWordSort` (the word's sorts listed, no `.color` default). Pin added:
  `Anaphora.reflexiveReadsItsClausesSubject`. Rewritten to the new constructors, statements
  otherwise unchanged: `Lookback.gapRequiresLookback`, `lookbackRequiresBoundParticipant`,
  `innerGapDoesNotSupplyOuterParticipant`, `TextChanges.badReplaceCreatureTypeWithLandType`,
  `badReplaceWordSort`, `badReplaceWordCardName`. `TextWord.sameSort`'s mismatch between two words
  stays `kindMismatch`: two constraints on one sort disagree).
- **Remove one-purpose flags and positional hacks (§2.13):** Clean up
  `DeedFeature.librarySearch` / `Referent.librarySearchResult` (`Check/Words.lean:339`, read
  `:1146,1157`, set `Check/Phrase.lean:596`),
  `Referent.attachmentHost` / `announcedTargetResult` / `createdResult` (`Check/Words.lean:336-338`;
  `Check/LoopResults.lean:13-27`) (deriving
  loop plural abstraction from `det == .target` and `Origin.token` + `.actedOn`
  stamps and projecting/reusing `attachHost` as a definite presuppositional
  reference), `historicalMoveFrom` (`Check/Words.lean:487`), `NounPhrase.plur`'s hard-coded
  slice rule (`Check/Phrase.lean:340-345`),
  `.addedAbilities` letter-parameter matching (`Check/Phrase.lean:1834-1840`; no longer named
  for toxic, it matches any keyword whose sole parameter is a letter, positionally),
  `stepPlayPaymentWith` `zipWith` (`Check/Phrase.lean:2332`), `enact` `pendingRoles.take`
  (`Check/Phrase.lean:3462-3464`), `DRS.ofList` (`Check/Words.lean:554-558`)
  id-0 renumbering, and the ambiguous `EmptyCollection DRS` instance (`Check/Words.lean:564`).
  **Reconciled 2026-10-24: live** (partly: `DeedFeature` is gone, deleted by
  [lean-v3-action-facts-deleted](../done/lean-v3-action-facts-deleted.md); everything else
  listed remains).
  **Partly done 2026-10-24.** kozxmszv: `Referent.librarySearchResult` deleted; a search's stamp
  (`Stamp.deeds` holding the search deed, reset by a move [CR#400.7]) is what `stampedBy` and
  `survivesShuffle` read; `MovementScopes.revealedSearchResultSurvivesShuffleWithSameIdentity` and
  `zoneMoveClearsSearchProvenance` read the stamp instead of the flag, statements otherwise
  unchanged.
  xsypzvrm: `DRS.ofList` numbers its referents by position, always (no id-0 sentinel), and the
  contexts written with their own ids use the new `DRS.ofNumbered`
  (`Check/Queries.lean`'s two caller contexts, `Check/AbilityRules.lean`'s two,
  `EventPatterns.uniqueCounterpartSurvivesVoiceRoleChange`,
  `indistinguishableCounterpartsCannotShareOneAlternativeId`,
  `commonMovedHeaderSuccessorDoesNotResurrectOldIdentity`,
  `SelectionEvidence.preexistingTargetsAreNotReaggregated`; statements otherwise unchanged); the
  unused, ambiguous `EmptyCollection DRS` instance is deleted.
  lsqzsrwt: the `ownScope` step (where `enact`'s `pendingRoles.take` now lives) reads its body's
  last-clause mentions as those not pending when the body began, not by the length of a prefix;
  `ControllerSacrifice`'s private mirror (`sacrificeStampState`, `sacrificeStampProjected`,
  `actualTailReduction`'s `change`) follows; public statements unchanged.
  sqklwsrp: `DRS.historicalMoveFrom` carried two things in one `Option Zone`: the mode in which
  the historical referents are read as the permanents they were (only ever `.battlefield`) and the
  origin a "this way" lookback hands its move. It is split into `readsLastKnown : Bool`
  [CR#608.2h] and `handedOrigin : Option Zone` [CR#608.2c], each set and restored where its own
  use is; a mode no longer reads as a handed origin. `HistoricalMove`'s exit pin reads both
  fields; three proof `simp` sets in `ComparisonBox`/`ReferenceScopes` lose the unused
  `Option Zone` lemma.
  mqyprupu: `.addedAbilities` introduces the letter any parameter of a granted keyword writes for
  its number (`KeywordParam.writtenLetter?`), not only a sole one; pin
  `GetsBothDeltas.grantedKeywordLetterIsIntroducedBesideACost` (reinforce X—{2}).
- **Fuel defaults & helper depth limits (§2.9, Astra §7):** Audit silent fuel
  fallbacks (`NounPhrase.kind?` `Check/Phrase.lean:101-103` → `some .object`,
  `NounPhrase.plur` `:331-332` → `.one`, `Quantity.plur` `:54-56` → `.many`,
  `predicatePayload` `:506-508` → `.difference`, `GameEvent.facts` `Check/Events.lean:87-89` →
  `{}`, the `numberSlots` family `Check/NumberSlots.lean:65-200` → `[]`) and tight helper fuel
  limits (`stepQuantityWith` `Check/Phrase.lean:725`, `stepTimingWith` `:948`, and
  `stepLocationWith` `:996`, at 8) so fuel exhaustion cannot
  silently produce default classifications.
  **Reconciled 2026-10-24: live** (every listed fallback is unchanged).
  **Partly done 2026-10-24: ktxtwxmq** (`predicatePayload` recursed on `Kind` only, which has a
  recursor, so its fuel and `.difference` exhaustion default are gone; statements unchanged).
  **Needs ruling 2026-10-24 (the readers over the AST).** The other readers and helpers recurse on
  the one mutual block of `Abilities.lean`, which has no `brecOn` (structural recursion on it is
  refused: "the type Location does not have a `.brecOn` recursor") and no generated `SizeOf`
  (`sizeOf` is the default instance, 0), so fuel stays; what is open is how running out of it is
  reported. Of the listed defaults, `NounPhrase.kind?`'s and `NounPhrase.plur`'s (32) are
  reachable under an accepted term: the stepper's budget is 64 and theirs is 32; `Quantity.plur`'s
  (16) is shadowed by `stepQuantityWith`'s refusal at 8, when the quantity is stepped. Options:
  (a) readers return an exhaustion signal (`Option`/`Except`) that the checker turns into
  `Refusal.recursionLimit`, changing every caller of `kind?`, `plur`, `facts` and the
  `numberSlots` family; (b) one budget: the stepper's helpers (`stepQuantityWith`,
  `stepTimingWith`, `stepLocationWith`, at 8) draw on the stepper's own fuel (a `StepOps.fuel`
  field), every reader's default is at least that budget, and a pin per reader shows the stepper
  refuses `recursionLimit` before the reader's default is reached; (c) as (b), with the
  guarantee stated in `CONTRACTS.md` instead of pinned per reader; (d) enable a recursor or
  `SizeOf` for the mutual block (`brecOn`/`genSizeOf`) and drop fuel from the readers, at a
  build-time cost not yet measured.
- **Resolve *Mangara's Equity* and *Noetic Scales* without explicit selectors
  (Astra §2, Opus §6.7):** With `NounPhrase.or` branch-local indefinites closed
  (`stepNounAlternativesWith`), prove that *Mangara's Equity* resolves raw
  `that creature` to the damage source in `Proofs/CorpusRecency.lean:120-136`;
  express *Noetic Scales* (`Proofs/CorpusRecency.lean:138-152`) as the actual
  `Action.move` / `returnToHand` AST (stepping the moved subject NP before the
  destination) so raw `that player` resolves to the upkeep player; and delete
  the `.laterMention` explicit-selector workarounds.
  **Reconciled 2026-10-24: live** (`mangarasEquityRawDemonstrativeSelectsWrongCreature`,
  `noeticScalesRawDemonstrativeSelectsWrongPlayer` and both `…ExplicitSelectorRestores…`
  workarounds remain; *Noetic Scales* is still a `stepLocationWith` over `.zone .hand`).
  **Done 2026-10-24: nxntrvon** (retired `mangarasEquityExplicitSelectorRestoresDamageSource` and
  `noeticScalesExplicitSelectorRestoresUpkeepPlayer`, the workarounds; `noeticScalesRawDemonstrativeSelectsWrongPlayer`
  rewritten as `noeticScalesRawDemonstrativeSelectsTheUpkeepPlayer`, the trigger over
  `returnToHand (each … that player … their hand)` to an owner's hand scoped to nobody
  [CR#400.3]: it checks, and the upkeep player is the only player in the result;
  `mangarasEquityRawDemonstrativeSelectsTheDamageSource` also checks the whole line, "this
  enchantment deals that much damage to that creature").
- **Delete dead `Check/` code (§5.4):** Remove unused `NounResult` API,
  `introducedNounResult`, `HeadTy`, `unionPayload`/`unionBinding`/`unionReferentLists`/`unionBindings`,
  `recordHead`, `overIdentity`, `markFirst*`, `spentDistributively*`,
  `partsDistributed`, `groupSpent`, `stampWordOk`, `provOfGroup`, `provOfReach`,
  `countReflexive`, `outcomesOnly`, `stampMoves`, `stampWasField`,
  `stepTokenSpecWith`, `stepCreationSpecWith`, `knownActs`,
  `Predicate.writtenTypesAll`, and rehome `Check/SelectionEvidence.lean` into
  `Proofs/`.
  **Reconciled 2026-10-24: live** (partly: no single landing ticket; every listed name is gone
  from `lean/Semantics` except `spentDistributively*`, which survives as
  `spentDistributivelyList` (`Check/Words.lean:859-863`, referenced only by itself), and
  `SelectionEvidence` now lives at `Proofs/SelectionEvidence.lean`. Still live: delete
  `spentDistributivelyList`).
  **Done 2026-10-24: wuspqokq** (`spentDistributivelyList` deleted; every other listed name was
  already gone, and `SelectionEvidence` already lives in `Proofs/`).

## Verification

- `./lean/scripts/build` (`lake build --wfail`) passes with zero warnings and
  zero `sorry`s.
- `cargo test -p xtask -p deckmaste_semantics_v2` and
  `cargo clippy -p xtask -p deckmaste_semantics_v2 --tests -- -D warnings` pass.
- `cargo xtask cite check --list-noncompliant` is empty and
  `cargo xtask cite check` reports 0 stale citations.
