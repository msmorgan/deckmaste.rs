---
needs: []
---
**A lookback is a game event with a gap, not a parallel event-name taxonomy.**
`lean/Semantics/Events.lean` declares `EventName`, a bare enumeration of event
classes (`death`, `departure`, `damageTaken`, …, `verbedAct deed`), and
`lean/Semantics/Phrase.lean` builds a lookback ("creature that died this turn") as
`LookbackClause.mk (event : EventName) (lookback) (complement : Option
EventComplement)`. `GameEvent.name : GameEvent → EventName` projects the
sentence form onto the class so `EventName.facts` can key the checker's event
table, and six small `eventName` projections (`CombatRelation`, `AttachMove`,
`StatusCat`, `FlipCall`, `PaymentOutcome`, `LifeMove`, plus
`counterEventName`) keep the two taxonomies aligned by hand.
`EventComplement` (`involving`, `fromZones`, `intoZone`, `atZone`) re-invents
the zone fields `GameEvent.leaves`/`enters`/`putInto` already carry.

Change: a lookback carries a `GameEvent` whose relativised participant is
marked (a `NounPhrase` gap standing for the noun the clause modifies), and the
event facts (`subjectKinds`, `complementKinds`, `bareRefused`, `hasMagnitude`,
`interceptable`, `countable`, `boundsDuration`, `underway`) become a function
of the `GameEvent` constructor. Delete `EventName`, `EventComplement`, the
seven projections and `GameEvent.name`; the lookback complement checks become
the ordinary event participant checks. The Anaphora and Zone pin suites are
the ones this reaches; preserve every meaningful positive and negative case. Diagnostic lists may
change. Cases made unrepresentable by the new shape may be retired, with each
case named and explained in the landing record.

Decisions already made: a law reads a declared feature or matches a closed CR
constructor, never a lexeme; `TriggerWord` was deleted 2026-09-05 for the same
reason (a header word is not a model concept). Glossary entry **Event**
([CR#700.1]) already covers the concept; no new term.


## Confirmed decisions (2026-09-06)

The user confirmed the refactor is wanted if feasible. Start by checking that
one lookback can contain a real `GameEvent` with a scoped participant gap and
share the ordinary event checker. Do not preserve a second event taxonomy or
a compatibility representation merely to reproduce old diagnostics.

The user accepted changed diagnostics and explicit retirement of malformed
subjects made unrepresentable by the new shape. Preserve all meaningful
positive and negative cases and account for each eliminated case individually.
This supersedes the original exact-refusal-list requirement.

Other family decisions: the card-soundness emitter is deferred until
`semantics_v2` exists; CI publication is deferred, with local validation
sufficient for now; the joined-kind contract remains explicitly open.

## Landing record

Measured on change `pvoyxrltolyxqonozwpokrvytllxzpsw`, with lock `covered`
20,254, on 2026-09-06. The pre-change Lean source inventory was retained outside
version control for comparison.

### PROVE

- `./lean/scripts/build` passes all 68 jobs, including every card and proof
  module. The existing `okGreatestCardsAPlayerDiscardedThisWay` and `okTheFallen`
  assertions remain unchanged and pass after correcting their migrated values.
- All 815 `Spelled` card definitions retain their identities. Named assertions:
  2,235 before, 2,245 after; restored 0, re-spelled 55, ignored 0, added 20,
  removed 10. No meaningful positive or negative case was removed; the ten
  malformed shapes retired below have no direct representation in the new AST.
- Lean LSP diagnostics are clean for the changed binding proofs and The Fallen.
  LSP axiom verification of `nestedLookbackConsumesItsGapUse`,
  `nestedLookbackCannotCaptureOuterGap`, and `eventModifiersPreserveFacts`
  reports only `propext`; source scanning finds no suspicious patterns. No
  `sorry` or new axiom occurs in Lean source.
- `EventName`, `EventComplement`, their projections, and the former lookback
  compatibility checks are absent from Lean source. Lookbacks use the ordinary
  recursive event checker; nested binders consume their own gap-use flag.
- `cargo xtask gate --changed` reports no affected Rust crates. No English
  grammar, lowering, declaration data, or coverage lock changed. The English
  structural-law and licensing inventories were not remeasured for this Lean
  change. New checks read kinds, event constructors, or declared action features;
  no card or lexeme identity is used as a licensing guard.
- Citation checks: zero noncompliant strings; 14,943 citations checked, zero
  stale. All seven moved citation sites were audited against their rule text.
  Final Kata refresh was a no-op, so these test results still apply.

Retired cases (explicitly authorized by the confirmed decisions):

- `badStateMatchLookback`: The old event-name value carried no condition or participant.
- `badBareTokenCreationLookback`: Token creation now requires the token participant.
- `badDeathOriginZone`: The death event has no independently authorable origin field.
- `badNestedOriginPayload`: The event has one source field; nested duplicate origins cannot be constructed.
- `badDeathOriginAnywhere`: The death event has no independently authorable origin field.
- `badNestedDestination`: The placement event has one destination field.
- `badLocusOnDeath`: The death event has no independently authorable locus field.
- `badBareActivationLookback`: Activation now requires its ability participant.
- `badBarePaymentLookback`: Cost payment requires its owner and keyword payload.
- `badPlacementLookback`: Placement requires a destination; an unspecified move is a leaves event.

### DISCLOSE

No new card identity or English coverage gain is claimed. The new named
assertions are all in `Semantics.Proofs.Lookback`:

- `castCanCarryExcludedOrigins`.
- `castRejectsEmptyOriginExclusion`.
- `designatedSpellCanBeMatched`.
- `designatedSpellRetainsItsReference`.
- `eventModifiersPreserveFacts`.
- `gapRequiresLookback`.
- `historyDoesNotIntroduceDamageOutcome`.
- `historyRetainsNamedParticipant`.
- `innerGapDoesNotSupplyOuterParticipant`.
- `locusUsesDeclaredActionFeatures`.
- `lookbackRequiresBoundParticipant`.
- `namedActionCanCarryItsLocus`.
- `nestedLookbackBindsItsOwnParticipant`.
- `nestedLookbackCannotCaptureOuterGap`.
- `nestedLookbackConsumesItsGapUse`.
- `participantCanOccurMoreThanOnce`.
- `playerGapCannotStandForObject`.
- `scopedCreatureDeath`.
- `scopedPlayerDraw`.
- `unspecifiedSpellDoesNotIntroduceAReference`.

Changed diagnostics, with the original negative outcomes retained:

- `badAttackerComplementOnObject`: `lookbackComplement` becomes `attackable`.
- `badPlayerCastComplement`: `lookbackComplement` becomes an object/player kind
  mismatch plus the required stack-zone refusal.
- `badCastOriginFromStack`, `badEmptyOriginCoordination`, and
  `badEmptyOriginExclusion`: `lookbackSource` becomes `playableFrom`.
- `badCombatDamageComplement`: `lookbackComplement` becomes an object/color
  kind mismatch.

`joinedDealerDamageComplement` now checks a real damage event with a joined
recipient instead of inspecting the deleted complement table; acceptance stays
unchanged. The existing Storm value-equality proof is re-spelled against the
new event form and still proves the full expansion.

Deviations and additions:

- The three duplicate event-fact fields `subjectKinds`, `complementKinds`, and
  `bareRefused` are deleted, rather than moved as originally proposed. Required
  constructor fields and the shared participant checker carry those obligations.
  The remaining five facts are read directly from `GameEvent`.
- `GameEvent`, `Causing`, `Condition`, and their recursive phrase payloads now
  share a mutual syntax block. Pure event profiles live in `Check/EventContext`;
  phrase, condition, and event checks share the mutual checker in `PhraseRules`.
- A cast's spell is optional to preserve an unspecified historical spell without
  inventing a named reference. Its source uses ordinary `EventSource` sets and
  exclusions. The 42 existing explicit-spell sites gained `some`; the existing
  Melek source gained the single-zone `EventSource` wrapper.
- Named-action events gain an optional locus, checked against declared action
  features. The 26 existing sites gained an explicit absent-locus argument.
  Search and Shuffle lookbacks retain their existing locus cases.
- The 94 migrated lookbacks use event fields and the registered `relative`
  macro. Obsolete event-name-based macro variants are deleted. Historical gap
  markers state event-time roles without moving the current host reference.
- Designated object references may receive marker/type ascriptions, like the
  source and bound participant, so the existing commander identity can be viewed
  as a spell in a cast event. The new proofs check acceptance and that this
  ascription introduces no fresh reference.
- `okPlacementOriginBattlefield` uses a departure with an unspecified destination.
  Faith's Reward carries the destination already specified by its surrounding
  graveyard predicate. Cast and token-creation examples explicitly describe
  their spell/token participants. The combat-damage examples preserve the
  source/patient direction stated in their existing captions.
- Review/build failures exposed two migration mistakes. The discard aggregate's
  historical participant now has the hand origin required by the ordinary
  discard check. The Fallen binds the existing player/object join rather than
  just an object. Both original acceptance assertions pass unchanged; neither
  card's domain was narrowed.
- `lean/CONTRACTS.md` states the bounded checker guarantees. The Game Model
  Reference Scope entry now describes the locally bound historical participant;
  no new glossary term or English grammar construct was needed.

STOP resolutions: the original exact-diagnostic and blanket test-retention
requirements were discussed before implementation; the user explicitly allowed
changed diagnostics and named retirement of newly unrepresentable malformed
subjects. No unresolved regression or re-coverage obligation remains. The
joined-kind engine payload design stays open; the existing Lean join is used
without choosing a Rust representation. No engine history-query implementation,
Lean/Rust bridge, or card-soundness emitter is introduced.

### REPORT

Provenance only: change `pvoyxrltolyxqonozwpokrvytllxzpsw`, lock `covered`
20,254. The English construction count, unique/specificity selection census,
permitted/forbidden licensing totals, homograph list, and form-literal/vocabulary
overlap lists are unchanged by this diff and were not remeasured. No coverage
performance run was needed; wall time, host-load/worker telemetry, and ns/B are
therefore not reported as measurements of this tree. Nothing was published.
