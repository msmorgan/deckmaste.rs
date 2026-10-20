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
  `Stamp` (`Check/Words.lean:232`, `applyInPlaceStamp` at `Check/Phrase.lean:439`,
  `stampedBy` at `Check/Words.lean:1012`) with accumulated `.actedOn deed` stamps
  per [`docs/decisions/semantics-v3.md`](../../decisions/semantics-v3.md) §1.2/§1.5
  so an intermediate action (e.g. `"Exile target creature. Reveal it. Return the
  exiled card..."`) does not erase an earlier deed stamp.
- **Advance `nextClause` in `Cost.compound` (§2.2):** Update the `Cost.compound`
  fold in `Check/Phrase.lean:3010` to advance `nextClause` between members per
  `lean/CONTRACTS.md` ("Enclosed traversals") so cross-cost pronouns
  (`"Sacrifice a creature, Exile it:"`) resolve.
- **Close modal boxes across all four deontics (§2.3):** Close the inner DRS box
  on `must`, `can`, and `gatedBy` in `Check/Phrase.lean:2896-2930` just as `cant`
  does (`docs/decisions/semantics-v3.md` §1.3 box 3), preventing modal-scoped
  referents from leaking into outer discourse.
- **Produce `Determiner.self` for `"this"` and `"you"` (§2.4):** Fix `.this` and
  `.you` in `Check/Phrase.lean:1982` so they use `Determiner.self` / reuse their
  referent rather than pushing a fresh `det := .the` referent on every mention
  (activating the dead `wordNow`, `reachesCharacteristic`, and `countTokenSpecs`
  guards and refusing `"Tap this creature. Untap that creature."`).
- **Wire `DRS.performer : Option ReferentId` and retire fake `Payload.actor`
  referents (§2.5):** Track the performer in `DRS.performer` per
  `docs/decisions/semantics-v3.md` §1.2/§1.4 instead of pushing `Payload.actor`
  frames with `.outcome` kind into `referents` and filtering `!b.isActorFrame` /
  `r.kind != .outcome` across counting helpers; reconcile `Payload`'s 11
  constructors with the ADR.
- **Consistent disjunction, conditional, and negation scoping (§2.6):** Fix
  `NounPhrase.or` (`stepNounAlternativesWith` in `Check/Phrase.lean:975-995`),
  `Timing.or`/`.and` (`Check/Phrase.lean:720`), `Location.or` (`:771`),
  `Location.not` (`:770`), and `doIf` (`Check/Phrase.lean:2759-2803`) so branches
  do not thread left-to-right or leak branch-local non-target indefinites into
  outer discourse (preserving announced targets `[CR#601.2c,115.1]` and outer
  mutations across both `elseInst = none` and `elseInst = some _`, matching
  `doOnlyIf`), and update `lean/CONTRACTS.md` to match.
- **Foreground non-argument possessor re-mentions in `DRS.recordMention`:** When
  an existing referent is re-mentioned in a non-argument slot (`inArgSlot = false`,
  e.g. `"that creature's owner"`), update `r.clause := drs.currentClause` and
  `r.role := .other` (matching a newly introduced possessor NP) without recording
  in `drs.pendingRoles`, so *Weeping Angel*'s raw `"it"` in
  `"that creature's owner shuffles it into their library"` (`Proofs/DrtCards.lean:240-266`)
  resolves directly to the damaged creature without `.laterMention`.

### 2. Shared Action-Clause Spine, Head Gates & Deed Identity (Opus §§2.7–2.8, §3.1–3.3)

- **Share the action-clause spine (§2.8):** Factor `stepDidClauseWith`
  (`Check/Phrase.lean:861`), `stepWouldClauseWith` (`:911`), `ev .does` (`:2200`),
  and `cant`/`must`/`can`/`gatedBy` onto a single shared action-clause stepper
  (`docs/decisions/semantics-v3.md` §3.1), eliminating drift in locus/historical
  handling and actor-kind checks (including `.draw`).
- **Reconcile `deonticHeadOk` and `eventHeadOk` (§2.7):** Align
  `Check/Events.lean:30-70` so non-battlefield zone moves (e.g. `move` to exile)
  and `ownScope` headers are admitted consistently across event and deontic
  clauses.
- **Unify deed stamping and prune dead `CoreDeed` constructors (§§3.1, 3.3):**
  - Eliminate the three competing deed-stamping mechanisms (`ownScope (some (.action ...))`
    header, `.enact`, and `Action.inferredDeed` string-guessing at
    `Check/Phrase.lean:455-478`) and remove the `ownScope` + `.enact` double-wrap.
  - Delete the 8 dead `CoreDeed` constructors (`attack`, `block`, `gainLife`,
    `loseGame`, `winGame`, `spend`, `unlock`, `tapForMana` in `Words.lean:225-228`).

### 3. `theGrantor`, `permanentActor`, Refusal Hygiene, Positional Hacks, Fuel & Dead Code (Opus §§1.16, 2.9, 2.11–2.13, 5.4, 6.7; Astra §§2, 7)

- **Honor `HeadNoun` in `NounPhrase.theGrantor` (§1.16):** In
  `Check/Phrase.lean:1944-1950`, set the pushed referent's zone and type from the
  `HeadNoun` argument (e.g. `.emblem` in the command zone `[CR#114.1]`) instead of
  hard-coding `.battlefield`.
- **Align `permanentActor` facts with the CR (§2.11):** Fix `Check/Words.lean:157-178`
  for `harness` (`[CR#701.64a]` takes an explicit patient) vs `adapt` and
  `monstrosity` (`[CR#701.46a,701.37a]`).
- **One constructor per refusal obligation (§2.12):** Stop reusing
  `.anaphor (.word .permanent)` for failed reflexive/`which`, `.lookbackSubject`
  for unused `which`, and `kindMismatch (.quality .color)` for non-replaceable
  text-change sorts (`Check/Phrase.lean:1748-1760`, `Check/TextChanges.lean`).
- **Remove one-purpose flags and positional hacks (§2.13):** Clean up
  `DeedFeature.librarySearch` / `Referent.librarySearchResult`,
  `Referent.attachmentHost` / `announcedTargetResult` / `createdResult` (deriving
  loop plural abstraction from `det == .target` and `Origin.token` + `.actedOn`
  stamps and projecting/reusing `attachHost` as a definite presuppositional
  reference), `historicalMoveFrom`, `NounPhrase.plur`'s hard-coded slice rule,
  `.addedAbilities` toxic parameter matching (`Check/Phrase.lean:1136`),
  `stepPlayPaymentWith` `zipWith`, `enact` `pendingRoles.take`, `DRS.ofList`
  id-0 renumbering, and the ambiguous `EmptyCollection DRS` instance.
- **Fuel defaults & helper depth limits (§2.9, Astra §7):** Audit silent fuel
  fallbacks (`NounPhrase.kind?`, `NounPhrase.plur`, `Quantity.plur`,
  `predicatePayload`, `GameEvent.facts`, `numberSlots`) and tight helper fuel
  limits (`stepQuantityWith`, `stepTimingWith` at 8) so fuel exhaustion cannot
  silently produce default classifications.
- **Resolve *Mangara's Equity* and *Noetic Scales* without explicit selectors
  (Astra §2, Opus §6.7):** With `NounPhrase.or` branch-local indefinites closed
  (`stepNounAlternativesWith`), prove that *Mangara's Equity* resolves raw
  `that creature` to the damage source in `Proofs/CorpusRecency.lean:119-136`;
  express *Noetic Scales* (`Proofs/CorpusRecency.lean:138-152`) as the actual
  `Action.move` / `returnToHand` AST (stepping the moved subject NP before the
  destination) so raw `that player` resolves to the upkeep player; and delete
  the `.laterMention` explicit-selector workarounds.
- **Delete dead `Check/` code (§5.4):** Remove unused `NounResult` API,
  `introducedNounResult`, `HeadTy`, `unionPayload`/`unionBinding`/`unionReferentLists`/`unionBindings`,
  `recordHead`, `overIdentity`, `markFirst*`, `spentDistributively*`,
  `partsDistributed`, `groupSpent`, `stampWordOk`, `provOfGroup`, `provOfReach`,
  `countReflexive`, `outcomesOnly`, `stampMoves`, `stampWasField`,
  `stepTokenSpecWith`, `stepCreationSpecWith`, `knownActs`,
  `Predicate.writtenTypesAll`, and rehome `Check/SelectionEvidence.lean` into
  `Proofs/`.

## Verification

- `./lean/scripts/build` (`lake build --wfail`) passes with zero warnings and
  zero `sorry`s.
- `cargo test -p xtask -p deckmaste_semantics_v2` and
  `cargo clippy -p xtask -p deckmaste_semantics_v2 --tests -- -D warnings` pass.
- `cargo xtask cite check --list-noncompliant` is empty and
  `cargo xtask cite check` reports 0 stale citations.
