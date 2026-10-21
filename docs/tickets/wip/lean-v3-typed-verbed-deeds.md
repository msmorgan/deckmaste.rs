---
needs: [lean-v3-authoring-impl-14-ast-and-authoring-cleanup]
---
# Unify participial descriptions on `did` / `was` + `which` and retire `theVerbed`

"The exiled card", "the creature sacrificed this way" and their kin are
currently split between two encodings:
- 57 sites of `theVerbed (.action "exile") …` (27 in `Cards/`, 11 in
  `Definitions/`, 19 in `Proofs/`) over `Reach.verbed` and `VerbedMarking`,
  writing the keyword action's label as a raw string and bundling determiner
  plurality (`.one` / `.many`), nominal head (`Option NounWord`), and `.thisWay`
  into a special pronoun selector.
- Relative `Predicate.did` clauses over `which` (`connive`, `airbend`,
  `discardThenForEachNonland`, `cascade`'s `"that weren't cast"`).
- 14 of the `theVerbed` sites pass `word = none` (`theVerbed (.action "search")
  .one`, `theVerbed (.core .return_) .one`, etc.) where the Oracle text actually
  says `it`, `them`, `that card`, or has a printed head noun (`card`,
  `permanent`, `creature`).

## Design

### 1. Unify `DidClause`, `did`, and `was` over `which`
- Change `DidClause.within` in `Semantics/Abilities.lean` from `LookbackSpan`
  to `Option LookbackSpan` (`none` for an attributive participle with no
  written span, `some .thisWay` for `"… this way"`, `some (.duration d timing)`
  for a temporal window, `some .triggering` for the triggering event). Keep
  `DidClause` as the shared payload of `Predicate.did`, `Condition.did`, and
  `Amount.tally`.
- In `Semantics/Templates.lean` (`namespace Semantics.Templates`), align
  `Predicate.did` and `Condition.did` with `does` and `would`, and add
  `Predicate.was` and `Condition.was` as the unspecified-agent passive
  (`Subject.any`) shorthand, both dispatched via `expectedTypeDispatch`
  (`elabExpectedTypeDispatch`):
  - `Predicate.did (subj : Subject) (action : Action) (within : Option LookbackSpan := none) : Predicate := .did ⟨subj, action, within⟩`
  - `Condition.did (subj : Subject) (action : Action) (within : LookbackSpan) : Condition := .did ⟨subj, action, some within⟩`
  - `Predicate.was (action : Action) (within : Option LookbackSpan := none) : Predicate := .did ⟨.any, action, within⟩`
  - `Condition.was (action : Action) (within : LookbackSpan) : Condition := .did ⟨.any, action, some within⟩`
  - Register ``(`was, [(``Semantics.Predicate, `Semantics.Templates.Predicate.was), (``Semantics.Condition, `Semantics.Templates.Condition.was)])``
    alongside the existing ``(`did, …)`` entry in `expectedTypeDispatch`.
  - Keeping `within : LookbackSpan` non-optional on `Condition.did`,
    `Condition.was`, and `Amount.tally` (`eventCount` / `eventSum`) makes an
    unanchored `Condition.was (exile which)` with no span a compile-time type
    error; in `Semantics/Check/`, also refuse `within = none` outside a
    discourse-referring `NounPhrase.described`.
  - Add `instance [AuthoringContext] : Coe Duration LookbackSpan := ⟨(within · none)⟩`
    and `instance [AuthoringContext] : Coe Duration (Option LookbackSpan) := ⟨fun d => some (within d none)⟩`
    so both `LookbackSpan` (`thisWay`, `triggering`) and bare `Duration`
    (`soFarThisTurn`, `thisCombat`, `thisGame`) can be passed bare without
    `some`.
- Retire `happened`, `happenedThisWay`, `happenedTo`, and `wasCast` in favor of
  `did` and `was`:
  - Note that `happened action subj d` flips argument order to
    `did subj action d` (matching `does subj action` and `would subj action`).
  - When a `timing` argument is present (`happened action subj d (some t)`),
    write `did subj action (within d (some t))` explicitly.

### 2. Retire `Reach.verbed`, `VerbedMarking`, and `theVerbed`
- Retire `Reach.verbed`, `VerbedMarking`, and `theVerbed`.
- Give `DetPhrase.the` a `(plur : Plurality := .one)` field (`the` for singular
  `"the <noun>"` and `thePlural` for definite plural `"the <nouns>"`, e.g.,
  `"the exiled cards"`, `"the discarded cards"`, `[CR#607.2a]`).
- Rewrite the fake `word = none` sites to match their Oracle text:
  - `it` / `them` / `that card` after `search` (`transfigure`, `transmute`,
    *Scapeshift*, *Eradicate*, *Old-Growth Dryads*, *Agency Outfitter*) and on
    *Bond of Revival*, *Aim High*, *Graf Rats* (`meldThemInto`), and
    *Bioplasm*'s `"its toughness"`.
  - Actual head nouns on `cascade` (`isCard`), `devourEntry` (`isCreature` for
    `devour` `[CR#702.82a]`, `isPermanent` for `devourQuality` `[CR#702.82c]`),
    and *Soldevi Adnate* (`isCreature`).
- Rewrite participial descriptions to compose ordinary determiners (`the`,
  `thePlural`, `allOf`, `each`, `bare`, `countOf`, `forEach`, `fromAmong`,
  `allFromAmong`) with nominal predicates and `was (<verb> which)`:
  - `"the exiled card"` → `the (and [was (exile which), isCard])`
  - `"the exiled creature card"` → `the (and [was (exile which), isCreature, isCard])`
  - `"the sacrificed creature"` → `the (and [was (sacrifice which), isCreature])`
  - `"the discarded card"` → `the (and [was (discard which), isCard])`
  - `"the discarded cards"` → `thePlural (and [was (discard which), isCard])`
  - `"the card discarded this way"` → `the (and [isCard, was (discard which) thisWay])`
  - `"cards exiled this way"` (*Escape to the Wilds*) → `bare (and [isCard, was (exile which) thisWay])`
  - *Tezzeret, Master of the Bridge*'s `"Put all artifact cards from among them onto the battlefield"` → `allFromAmong isArtifact them` (its Oracle text)
  - `"for each creature destroyed / exiled / tapped / card discarded this way"` →
    - *Descent of the Dragons*: `doForEach (each (and [isCreature, was (destroy which) thisWay]))`
    - *Martyr's Cry*: `doForEach (each (and [isCreature, was (exile which) thisWay]))`
    - *Harmony of Nature*: `doForEach (each (and [isCreature, was (tap which) thisWay]))`
    - *Syphon Mind*: `doForEach (each (and [isCard, was (discard which) thisWay]))`
  - `"for each creature / permanent sacrificed this way"` (`devourEntry`, parameterized by the variant's head noun: `isCreature` for `devour` `[CR#702.82a]`, `isPermanent` for `devourQuality` `[CR#702.82c]`) → `forEach amount (and [head, was (sacrifice which) thisWay])`
  - `"the number of creatures destroyed this way"` (*Phyrexian Rebirth*) → `countOf (and [isCreature, was (destroy which) thisWay])`
  - `"all cards exiled this way that weren't cast"` (`cascade`, `[CR#702.85a]`) → `allOf (and [isCard, was (exile which) thisWay, not (was (cast which) thisWay)])`
  - `"the rest"` / `"all revealed cards not cast this way"` (`ripple`, `[CR#702.60a]`) → `allOf (and [isCard, was (reveal which) thisWay, not (was (cast which) thisWay)])`
- In `Semantics/Check/`, when a `NounPhrase.described` (`the`, `thePlural`,
  `allOf`, `each`, `bare`) carries a discourse-anchored `Predicate.did`:
  - `within = none` (`"the exiled card"`, `"the sacrificed creature"`) resolves
    against stamped referents from either the current ability or
    `Ability.textContext` (linked abilities), preserving the stamp, expected
    zone (`[CR#607.2a]`), and `[CR#608.2h]` former-type checks previously
    performed by `Reach.verbed`. It requires a unique matching referent in scope
    (`count == 1`), refusing when zero or two or more abilities/actions in scope
    could supply the stamped referent. Per `[CR#607.3]`, a singular
    `DetPhrase.the .one` with `within = none` also matches a plural (`.many`)
    referent published by a linked ability in `Ability.textContext`.
  - `within = some .thisWay` (`"… exiled this way"`) resolves only against
    stamped referents introduced within the current ability (`[CR#607.1c]`).
  - `within = some (.duration d t)` (`"… that died this turn"`) is a general
    historical lookback over `d` and requires no prior DRS referent.

### 3. Resolve cross-ability linked objects and choices via `Ability.textContext` and `which`
- Because linked abilities (`[CR#607.2a..607.2q]`) and self-linked abilities
  (`[CR#607.1c]`) use identical unmarked surface wording in Oracle English
  (`"the exiled card"`, `"the sacrificed creature"`, `"the tapped creature"`,
  `"the chosen player"`), `deckmaste_spelling` requires the same authored AST
  for both rather than an unprinted `linked` / `LinkSource.byDeed` wrapper.
- Lowering finds the link from the same resolution (with runtime zone filtering,
  such as a card leaving exile before `champion`'s return ability resolves,
  belonging to lowering and `deckmaste_core` `[CR#607.2a]`).
- In `Semantics/Check/Phrase.lean`, thread `Ability.textContext` across the
  `body : List Ability` of `Ability.keyword` (just as `Ability.checkText` folds
  `Ability.textContext` across a card's ability list).
- Extend `Ability.textContext` and `staticTextContext` in
  `Semantics/Check/AbilityRules.lean` to also publish:
  - the alternative-cost payment in `.can _ (.procedure .cast _ (.payingInstead cost) ..) _` (`emerge`, `emergeFrom`, `harmonize`, `[CR#702.119a..702.119b,702.180a]`),
  - the target referent added to `thisSpell` in `.addsTarget thisSpell pred`
    (`mutate`, `[CR#702.140a]`), stamped with `.core .target` and scoped to
    `thisSpell`'s own targets so `mutate`'s `the (was (.target which))` (or
    `the (did thisSpell (.target which))`) resolves only to the target of
    `thisSpell` and never to an earlier ability's target, and
  - any object in a public zone stamped by a printed triggered or activated
    ability of the same object (`champion`, `[CR#607.2a,607.2k,702.72a]`).
- Rewrite the 6 `linkedObject` keyword sites (`champion`, `emerge`,
  `emergeFrom`, `harmonize`, `mutate`, `offering`) and `gift`'s `chosenAsPaid`
  to ordinary descriptions (`the (and [was (exile which), isCard])`,
  `the (and [was (sacrifice which), isCreature])`,
  `the (and [was (sacrifice which), isPermanent])`,
  `the (and [was (tap which), isCreature])`, `the (was (target which))`,
  `the isChosenPlayer`):
  - `gift`'s `the isChosenPlayer` (`Predicate.chosenPlayer .theChoice`) requires
    a unique player choice in scope (`countChoice .player drs == 1`, refusing
    with `.choiceRef .theChoice .player n` when `n ≠ 1`), which holds inside
    `gift`'s `Ability.keyword` scope because `gift`'s additional cost is the
    only player choice in scope (`[CR#702.174a,607.2d,607.2i]`).
- Required pins in `Proofs/LinkedReadback.lean` and
  `Proofs/LinkedChoiceReadback.lean`:
  - Zone restriction (`[CR#607.2a]`): if the description demands a zone/carrier
    other than the zone the linked ability put the object into (e.g., asking for
    a battlefield creature instead of an exiled card), resolution refuses.
  - Ambiguity refusal (`[CR#607.2a]`): if two abilities on the same object can
    both supply `"the exiled card"`, `the (and [was (exile which), isCard])`
    refuses (`count == 2`).
  - `[CR#607.3]` number neutrality: a singular `the (and [was (exile which), isCard])`
    resolves cleanly when the linked ability in `Ability.textContext` exiled a
    `.many` group.
  - `mutate` target anchoring (`[CR#702.140a]`): `the (was (.target which))`
    resolves to the target added to `thisSpell` by `.addsTarget` and refuses if
    no target (or an unrelated target) is in scope.
  - `gift` choice uniqueness (`[CR#702.174a,607.2d]`): `the isChosenPlayer`
    resolves after `gift`'s `.addedCost` and refuses when zero or two player
    choices are in scope.
- Retire `LinkedFacet.objects`, `LinkedFacet.chosenPlayer`, `linkedObject`,
  `linkedObjects`, `chosenAsPaid`, and `LinkSource.byDeed`. `LinkSource` itself
  (`byKeyword`, `byNthKeyword`, `theAlternative`, `theAdditional`) stays because
  `LinkedFacet.timesPaid`, `LinkedFacet.readback`, and `LinkedFacet.taken` use
  it.

## Rulings and deviations

Rulings made by the orchestrator while the owner was offline (2026-10-21):
- **Devour** ([CR#702.82b]): "it devoured" means "sacrificed as a result of its devour ability
  as it entered the battlefield", a verb a keyword ability defines for itself (a Deed). Devour's
  entry sacrifices through `Action.devour subject` (`enact (.ofAbility devour) (sacrifice
  subject)`, named in its sort because `devour` is the ability), whose stamp keeps the sacrifice
  it was done by (`Stamp.through`): "each creature sacrificed this way" and "creatures it
  devoured" (`each (and [isCreature, did it (devour which)])`) both read it. A replacement's
  `also` results are published by `staticTextContext`.
- **Tezzeret, Master of the Bridge**: its Oracle text, as corrected above.
- **`costChoosesPlayer`**: nothing reads it once `LinkedFacet.chosenPlayer` is gone, so the column
  is retired from the facts generator, the Lean row type and `deckmaste_semantics_v2`.

Deviations from the text above, made while implementing it:
- A lookback with no agent, or one written with "by", hides the enclosing handoff's performer
  while it steps: "the discarded card" in "this enchantment deals damage equal to …" is not
  discarded by the enchantment.
- *Damn*'s "A creature destroyed this way can't be regenerated" is
  `a (and [isCreature, was (destroy which) thisWay])`: an indefinite, not a Participial
  Description.
- *Agency Outfitter*'s "a card named Magnifying Glass and/or a card named Thinking Cap … put them"
  searches for `upTo 2` cards named either, so that `them` reads the group. It does not yet hold
  the search to one of each.
- Scavenge's "the card you exiled" is `the (and [isCard, did you (exile which)])`.
- `cycling` is a Deed a keyword ability defines for itself ([CR#702.29c]), so *Unpredictable
  Cyclone*'s "the cycled card" checks.
- A Participial Description's agent must be none, "you", the performer, the source, or (for a
  keyword ability's own verb) the object that has it; a stamp does not record who did the deed,
  so any other agent is refused (`participialAgent`).
- A group determiner reads every candidate together, and a Card Type in it filters the group's
  members. "The" reads one site: one object of the current ability, or what one linked ability
  acted on, whose exclusive arms (`may … otherwise`, `doIf … else`) count as one.
- A triggered ability publishes only what it introduces, not what its trigger event or condition
  names ([CR#607.1]).
- `NounPhrase.paid` and `Reach.tracksObject` are retired: no facet a noun phrase could read
  remained, and nothing read `tracksObject`.

Tests retired, with reasons:
- `okHijackDeedDescriptions`: an unprinted duplicate of *Hijack*, whose Oracle text says "it".
- `NounWords.verbedPronounTracksItsObject`, `verbedDescriptionDoesNotTrackItsObject`,
  `verbedPronounIsAnObject`, `verbedDescriptionTakesItsWordsKind`: `Reach.verbed` and
  `Reach.tracksObject` are retired.
- `LinkedReadback.badKeywordDeedOfAnotherKeyword`, `badObjectsReadAsAnAmount`: they tested
  `LinkSource.performs` and `linkedFacetKind`, retired with `LinkedFacet.objects`.
- `LinkedChoiceReadback.badCostlessKeywordChoosesNoPlayer`, `badTheAdditionalChoosesNoPlayer`,
  `badFacetReadAsTheOtherKind`: they tested `LinkedFacet.chosenPlayer` and `NounPhrase.paid`.
- `GiftVariants.badUnlinkedChosenPlayer`: it asserted the retired reading, that gift's own cost
  choice is out of scope for its second ability.

