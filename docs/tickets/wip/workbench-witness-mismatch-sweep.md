---
needs: []
---
**Re-spell or retire every bench witness that reads something other than the
card it names.** Cleanroom review 3, 2026-09-04, §2d: 127 of 1 568 witnesses
(8%) mismatch their card, in six classes. References are as written on the
card; the workbench proves rules text, never reminder text [CR#207.2a].

- **Different card than the name (rename or re-spell).**
  `Cards.Trigger.tahngarthHeader`, `Cards.Choice.tahngarthChoosesDefender` and
  `Cards.Static.tahngarthAttacksThatJoin` (docstring "Tahngarth" against a
  Tahngarth, First Mate term; the `Choice` term also drops "that opponent is
  attacking"); `Cards.Keyword.simpleHalf` (the term is Pure);
  `Cards.Counters.crovaxTheCursed` (docstring "Yawgmoth Demon");
  `Cards.Turn.beckoningWillOWispChooser` (docstring "Sanctuary Blade"; the
  body duplicates `Cards.Turn.beckoningWillOWisp` minus its flavor word);
  `Cards.Cost.alluringSuitorPump` (front-face name, back-face text);
  `Cards.Anaphora.tezzeretAllArtifacts` (mill six / artifacts against an exile
  ten); `Cards.Static.generalJarkeldReassign` and
  `Cards.Static.sorrowsPathReassign` (subject, two-target choice, conditional
  and "the other" all absent); `Cards.Faces.warpVortexFlips` (no such card in
  the corpus — delete).
- **Wrong body pasted.** Fourteen `Cards/Mana.idr` witnesses whose oracle text
  is "draw a card" carry Delivery Moogle's `searchLibraryOrGraveyard` body:
  `deadeyeBrawler`, `femerefEnchantress`, `tocasiasWelcome`,
  `duskLegionDuelist`, `saheeliFiligreeMaster`, `sageOfFables`,
  `secretsOfTheDead`, `vegaTheWatcher`, `investigatorsJournal`,
  `latchkeyFaerie`, `militaryIntelligence`, `jemLightfooteSkyExplorer`,
  `gnarlbackRhino`, `upTheBeanstalk`. Each is `Draw You (Lit 1)`.
- **A plural deed re-spelled as iteration.** To discard a card is to move it
  from its owner's hand to that player's graveyard [CR#701.9a], so one discard
  of two cards is not two discards, and the substitution changes the game
  action. `Cards.Damage.blightning`, `Cards.Damage.rakdossReturn`,
  `Cards.Damage.nicolBolasUltimate`, `Cards.Choice.carefulStudy`,
  `Cards.Choice.zombieInfestation`,
  `Cards.Choice.discardUpToTwoThenDrawThatMany` all write
  `Repeated (Lit n) (discard …)`; the counted forms
  (`discard … (counted (exactly 2) (InZone handZ))`,
  `sacrifice … (counted (exactly 7) Permanent)`) are already admitted.
- **Reference not as printed.** "its power" as `thisCreature`
  (`Cards.Damage.karplusanYeti`, copied into
  `ProofsDamage.okThatCreatureAfterDamage`); "that player" as `They`
  (`Cards.Damage.blackVise`); "it" as `thisCreature`/`thisEnchantment`
  (`Cards.Keyword.consulsLieutenant`, `monoxaRollTrigger`, `answeredPrayers`,
  and `archfiendsVessel`, which writes `Enters This`); "those permanents … its
  controller" as `TheVerbed "Destroy" PermanentW ThisWay`
  (`Cards.Keyword.soulOfEmancipation`); "the exiled card" as
  `a (ExiledWith …)` (`Cards.Keyword.drachNyen`,
  `Cards.Choice.phyrexianIngesterPump`, `Cards.Mana.chromeMoxMana`); "those
  creatures" re-described (`Cards.Deontic.nowhereToRunWardLine`); "that
  creature" as `a creature`, a scope change
  (`Cards.Deontic.whippoorwillImmunity`); "this way" as `Attributive`
  (`Cards.Anaphora.martyrsCry`); "it" as `That SpellW`
  (`Cards.Copy.bonusRound`); "another creature" contrasted with `This` instead
  of the Aura's host (`Cards.Faces.kitsuneMystic`); "a creature" as `each`
  (`Cards.Counters.oonasBlackguard`); "a creature" as `aTheirChoice`
  (`Cards.Piles.lilianaOfTheVeil`); "the other" as `theRest` and "manifest" as
  exile (`Cards.Anaphora.writeIntoBeingPlacement`); "that player's library …
  under your control" as `topSlice` plus a bare move
  (`Cards.Anaphora.exileTopThenPutFromAmong`); the subject repeated three
  times where siblings share it (`Cards.Cost.embercleave`); a printed X
  inlined (`Cards.Turn.berserk`).
- **Dropped clause, restriction, duration, mode or optionality.**
  `Cards.Trigger.agencyOutfitterSearch` ("you may", "and/or", "put them onto
  the battlefield"; `Macros.searchZonesOf` hardwires `exactly 1`, so this one
  is M), `Cards.Trigger.dontBlinkReplacement` (duration),
  `Cards.Trigger.hollowmurkSiegeSultai` (once-per-turn limit),
  `Cards.Trigger.dreadSlaver` (`When` for "Whenever"),
  `Cards.Trigger.faridehResultRead` (first sentence),
  `Cards.Damage.keeperOfTheFlame` ("as you activate"),
  `Cards.Damage.theFallenUpkeep` (halves reversed; duplicates
  `Cards.Damage.theFallen`), `Cards.Description.glyphOfDestruction` and
  `clavilenoPhrase` (subtype), `Cards.Description.warTaxScaledPayment` ("they
  control"), `Cards.Description.xenosquirrelsShift` (invents a d6 roll) and
  `ironMastiffIgnore` (per-player roll), `Cards.Anaphora.eradicateSearch`
  ("all", exile — M), `Cards.Anaphora.flickeringSpirit` and `anotherRound`
  ("under its owner's control"), `Cards.Anaphora.thaliasLancersSearch` ("may"),
  `Cards.Anaphora.vedalkenSquirrelWhackerReroll` (replacement body equals the
  replaced event — a no-op), `Cards.Keyword.gabrielAngelfire` (hard-codes one
  of four chosen abilities), `Cards.Keyword.pirDistributive` (agent invented;
  `Macros.manyBareCountersPutBy` cannot say agentless),
  `Cards.Keyword.tourachDiscardTrigger` (zone added),
  `Cards.Choice.rankleMasterOfPranks` (third mode),
  `Cards.Choice.emissaryOfGrudgesReveal` (conjunct),
  `Cards.Choice.fulgentDistraction`, `continueSpell` and `phantomBlade`,
  `Cards.Deontic.undercoverButler` ("or tied"),
  `Cards.Deontic.glaringSpotlight` and `distortionStrikeLine` ("this turn"
  collapsed with "until end of turn"),
  `Cards.Counters.entDraughtBasinAbility` (adds "you control", drops sorcery
  timing), `Cards.Counters.captainMarvelSameKinds` (intervening if),
  `Cards.Counters.alaundoTheSeer` ("other", "you own"),
  `Cards.Mana.engineeredExplosives` ({1} for {2}, "nonland"),
  `Cards.Mana.qarsiDeceiverMorphSpend` (third purpose),
  `Cards.Mana.coalStoker` and `steelswarmOperatorMana`
  (`Lit 3 (Runs [[R,R,R]])` is nine R; the house spelling is `Lit 1` with the
  run), `Cards.Cost.aggressiveMining` ("lands" — `Macros.playerCant` has no
  patient, while `cantDoTo` is admitted),
  `Cards.Cost.hammerOfBogardan` ("from your graveyard"),
  `Cards.Turn.brazenCannonade` (ability word, second sentence),
  `Cards.Copy.pyromancersGogglesMana` ("you may choose new targets"),
  `Cards.Faces.meldThemInto` ("them" as all creatures you control),
  `Cards.Faces.cyberConversion`, `Cards.Piles.truthOrConsequencesVote`,
  `Cards.Turn.karnRestart` and `Cards.Turn.ritesOfFlourishing` (rest of the
  sentence). **Not in this list:** `Cards.Keyword.windZendikon`, `towerWinder`
  and `spinIntoMyth` collapsing two printed possessors to `handZ`/`onTopZ` is
  correct under the zone-possessor ruling of 2026-09-04 [CR#400.3]; leave them
  (`spinIntoMyth`'s fateseal defect is `workbench-gate-fixes`).
- **Reminder text and non-cards on the bench (delete, or move to the matching
  `Proofs*` module as a synthetic rules-meaningful sentence).**
  `Cards.Counters.blastodermEntersFading` and `blastodermFadeUpkeep` (Fading's
  reminder text), `Cards.Counters.ascendConferral` and `saddleConferral`
  (their docstrings say so), `Cards.Description.storiedEnduringStory`
  (Storied's reminder text; no CR rule defines the keyword),
  `Cards.Counters.counterOnGraveyardCard`, `Cards.Mana.testMox1`,
  `Cards.Choice.eachPlayerOfferBindsOneMember` and
  `eachPlayerOfferDropsTheGroup` (`Refl` lemmas),
  `Cards.Description.partyCount` and its three lemmas (private arithmetic
  related to nothing the grammar spells),
  `Cards.Faces.meldBackFacesAgree` (`X = X` over two byte-identical
  expressions), the 19 `Static`/`Choice`/`Deontic` declarations whose
  docstrings name no card (`Cards/Static.idr` 63, 204, 261, 942, 959, 970,
  979; `Cards/Deontic.idr` 600, 931; `Cards/Choice.idr` 74, 639, 645, 653,
  769, 1291, 1427, 1433, 1476, 1484, 1502 — anchor by name, the line numbers
  are from the review's copy), and the byte-identical duplicates
  `ProofsDamage.okDamageCreature`/`okDamageToCreature`,
  `ProofsTrigger.okSingularAttackDefender`/`okPlayerAttackDefender`,
  `ProofsTurn.okSingularPartPossessor`/`okTriggerAtYourUpkeep`,
  `Cards.Damage.heartOfBogardanBody` (re-inlined in `heartOfBogardan`),
  `Cards.Damage.fallOfTheTitans` (in `fallOfTheTitansCard`) and
  `Cards.Deontic.propaganda`/`ghostlyPrisonWhole`.
- **Three shapes the sweep needs a decision on.** D-Q12: `Phrase.CombatRel
  CouldBlock (allOf attacking)` cannot say "all creatures that the other is
  blocking", so `Cards.Description.sorrowsPathCouldBlock` and
  `generalJarkeldCouldBeBlocked` read something else — give the relation a
  reciprocal argument or retire both witnesses. D-Q13: `Phrase.HasDesignation
  d` has no holder, and "is your Ring-bearer" is true only of a creature on
  the battlefield under your control with the designation [CR#701.54e] — add
  the optional holder or retire the reads. D-Q15: `Effect.CostsToCast` carries
  13 "costs less to activate" witnesses; rename the row for what it means, or
  split it. D-Q16: `Cards.Keyword.pymParticlesVigilanceGrant`,
  `darkbladeAgentDeathtouch`, `Cards.Trigger.faridehResultRead` and
  `Cards.Counters.thranduilsCompany` (a `Card` missing an ability) are
  fragments — rule whether a witness may encode part of a printed sentence and
  whether a `Card` may omit a line, and apply the ruling to all four.

Size: M. Done when: every witness named above reads its card as printed, is
renamed to the card it does read, or is deleted with its removal justified by
name; no reminder text remains on the bench; the four fragment witnesses are
resolved under one recorded rule; build at its module count. Standard
constraints apply, including the RON-shaped constraint.

## As landed

**Different card than the name.**
- `Cards.Trigger.tahngarthHeader`, `Cards.Choice.tahngarthChoosesDefender`,
  `Cards.Static.tahngarthAttacksThatJoin` — docstrings renamed to
  `Tahngarth, First Mate` (the term is that card's second ability; the
  Vanguard "Tahngarth" is a different card). STOP recorded for the `Choice`
  term's dropped "that opponent is attacking".
- `Cards.Keyword.simpleHalf` → `pureHalf` (the term is Pure's half, "Destroy
  target multicolored permanent"; docstring `Pure // Simple` was already right).
- `Cards.Counters.crovaxTheCursed` — docstring `Yawgmoth Demon` →
  `Crovax the Cursed`; the body already read Crovax the Cursed.
- `Cards.Turn.beckoningWillOWispChooser` — DELETED. Its body was
  `Cards.Turn.beckoningWillOWisp`'s own `flavorWord` ability minus the flavor
  word, and its docstring named Sanctuary Blade, which is already benched
  whole at `Cards.Keyword` (`"Sanctuary Blade"`). No unique assertion lost.
- `Cards.Cost.alluringSuitorPump` → `deadlyDancerPump`, docstring
  `Alluring Suitor // Deadly Dancer` (the term is the back face's
  `{R}{R}` ability).
- `Cards.Anaphora.tezzeretAllArtifacts` — re-spelled to Tezzeret, Master of
  the Bridge's −8: exile the top ten of your library, then put all artifact
  cards from among them onto the battlefield (was a copy of `gleamOfDeath`'s
  mill six).
- `Cards.Faces.warpVortexFlips` — DELETED; no card named Warp Vortex is in
  `data/derived/cards.jsonl`.
- `Cards.Static.generalJarkeldReassign` / `sorrowsPathReassign` — see the
  D-Q12 entry below.

**Wrong body pasted.** All 14 `Cards/Mana.idr` witnesses whose oracle text is
"draw a card" now read `Draw You (Lit 1)`: `deadeyeBrawler`,
`femerefEnchantress`, `tocasiasWelcome`, `duskLegionDuelist`,
`saheeliFiligreeMaster`, `sageOfFables`, `secretsOfTheDead`, `vegaTheWatcher`,
`investigatorsJournal`, `latchkeyFaerie`, `militaryIntelligence`,
`jemLightfooteSkyExplorer`, `gnarlbackRhino`, `upTheBeanstalk`. The one
correct use of `Macros.searchLibraryOrGraveyard` (`deliveryMoogle`) is
untouched; `Cards/Mana.idr` now has exactly one.

**Plural deed re-spelled as iteration.** All six now write one counted deed:
`Cards.Damage.blightning` and `nicolBolasUltimate`
(`counted (exactly 2/7) (InZone handZ)`, and `counted (exactly 7) Permanent`
for the sacrifice), `rakdossReturn` (`counted (ExactlyOf (LetterVal X)) …`),
`Cards.Choice.carefulStudy`, `zombieInfestation` and
`discardUpToTwoThenDrawThatMany` (`counted (upTo 2) …`). No `Repeated` over a
discard or a sacrifice remains in those terms.

**Reference not as printed — landed.**
- `Cards.Damage.blackVise` — "that player" as `That PlayerW OneOf`; "their
  hand" keeps `They`.
- `Cards.Keyword.consulsLieutenant` — intervening "if it's renowned" reads
  `It OneOf`.
- `Cards.Keyword.monoxaRollTrigger` — the second and third "it gains …" read
  `It OneOf`; only the printed "Monoxa" keeps `thisCreature`.
- `Cards.Keyword.archfiendsVessel` — `Enters This` → `Enters thisCreature`.
- `Cards.Keyword.soulOfEmancipation` — "those permanents" / "its controller"
  read `That PermanentW ManyOf` / `That PermanentW OneOf` (was
  `TheVerbed "Destroy" … ThisWay` / `ItVerbed "Destroy"`).
- `Cards.Anaphora.martyrsCry` — "exiled this way" marking `Attributive` →
  `ThisWay`.
- `Cards.Copy.bonusRound` — "copies it" reads `It OneOf` (was `That SpellW`).
- `Cards.Faces.kitsuneMystic` — "another creature" now contrasts with the
  Aura's host (`OtherThan (That (TypeW Creature) OneOf)`), not with `This`.
- `Cards.Counters.oonasBlackguard` — "a creature you control with a +1/+1
  counter on it" as `a`, not `each`.
- `Cards.Piles.lilianaOfTheVeil` — "[−2] sacrifices a creature" as `a`, not
  `aTheirChoice`.
- `Cards.Anaphora.writeIntoBeingPlacement` — "manifest" is now
  `Enact Nothing "Manifest" (Move … battlefieldZ [])` (was `exile`), and "the
  other" is `theOther Object` (was `theRest`).
- `Cards.Anaphora.exileTopThenPutFromAmong` — Lord of the Void now exiles
  `LibrarySlice OnTop (Lit 7) (target Opponent)` and uses
  `putOntoBattlefieldUnderYourControl`.
- `Cards.Cost.embercleave` — the three equipped-creature clauses now share one
  subject (`AndAlso (Just (AttachHost Equipped …))` + `ownSubject`), the
  `tattooWard` idiom.
- `Cards.Turn.berserk` — the printed X is a letter again
  (`PtUp (LetterVal X)` + `DefinesLetter X (StatOf Power (It OneOf))`).
- `Cards.Deontic.nowhereToRunWardLine` — now the whole printed static ability
  (`AndAlso Nothing [ hexproof-as-though line, ward line ]`), so "those
  creatures" reads `That (TypeW Creature) ManyOf` off the first sentence.
- `Cards.Deontic.whippoorwillImmunity` — now the whole printed activated body,
  so "that creature" reads `That (TypeW Creature) OneOf` off the announced
  target of "Target creature can't be regenerated this turn".

**Reference not as printed — STOP (no shape).**
- `Cards.Damage.karplusanYeti` and `ProofsDamage.okThatCreatureAfterDamage`
  ("its power" as `thisCreature`): `DealDamage`'s amount slot is indexed at
  `nomIntro src`, which is `[]` for a `This`-headed subject, so no pronoun
  resolves (`countReach Bare OneOf [] = 1` unsolvable), and
  `Macros.dealsDamageOwnPower thisCreature` is already pinned unspellable by
  `ProofsAnaphora.badOwnEmptyDelta`. **Goal:** index `DealDamage`'s amount at
  `selfSubjIntro src` (or add an own-stat amount row) so "deals damage equal
  to its power" reads as a pronoun off a `This` subject. Left as printed-name
  re-description.
- `Cards.Keyword.drachNyen`, `Cards.Choice.phyrexianIngesterPump`,
  `Cards.Mana.chromeMoxMana` ("the exiled card" as `a (ExiledWith …)`):
  `Phrase.uniquifies (ExiledWith _)` is `False`, so `the (ExiledWith …)` is
  refused at `detOk TheDet p = Uniquifying p` (probe: `So False` at that
  slot), and the imprint exile happens in a different ability so no
  `TheVerbed "Exile" … ThisWay` binding is in scope. **Goal:** decide whether
  `ExiledWith` uniquifies (imprint exiles at most one card) and add the
  `uniquifies` row, or publish an imprint binding across the card's abilities.
- `Cards.Keyword.answeredPrayers` ("it becomes a 3/3 Angel"): with the
  intervening `If (NotCond (Matches thisEnchantment creature))` in scope,
  `It OneOf` fails `countReach Bare OneOf (condIntro …) = 1` — the entering
  creature and the condition's own subject are both compatible, so the read
  is genuinely ambiguous in the model. **Goal:** decide whether a condition's
  nouns join the read stack; today the printed "it" cannot be written here.
- `Cards.Choice.tahngarthChoosesDefender` ("a player or planeswalker that
  opponent is attacking"): `combatRelOk AttackedBy` demands
  `km == Object`, so the attacker cannot be a player. **Goal:** admit a
  Player attacker on `CombatRel AttackedBy` (a player attacks
  [CR#506.2]) so the defender restriction is writable.

**Dropped clause, restriction, duration, mode or optionality — landed.**
- `Cards.Trigger.agencyOutfitterSearch` — "you may" and "put them onto the
  battlefield" added (`may You (Sequentially [searchZonesOf …, putOntoBattlefield foundCard])`).
  Residual STOP: `Macros.searchZonesOf` hardwires `exactly 1` and joins the
  zones with a fixed `[Graveyard, Hand, Library]`, so "and/or" over the zones
  and over the two named cards is still unwritable.
- `Cards.Trigger.dontBlinkReplacement` — now
  `Continuously (Intercepts …) (Just untilEndOfTurn)`; the declaration's type
  moved from `StaticSpec []` to `Instruction []`, which is what carries a
  duration (`Intercepts` has no duration slot; its `TriggerWindow` is a turn
  part, not a span).
- `Cards.Trigger.hollowmurkSiegeSultai` — `triggeredOnlyOnce … OncePerTurn`.
- `Cards.Trigger.dreadSlaver` — `When` → `Whenever`.
- `Cards.Trigger.faridehResultRead` — the first sentence is back (gains
  flying and menace until end of turn), so the witness is a whole ability,
  not a fragment (see D-Q16).
- `Cards.Damage.theFallenUpkeep` — DELETED: it duplicated
  `Cards.Damage.theFallen` with the `Joined` halves reversed and `This` for
  `thisCreature`; `theFallen` is the correct reading and stays.
- `Cards.Description.glyphOfDestruction` — "blocking Wall you control" now
  carries the Wall subtype.
- `Cards.Description.clavilenoPhrase` — DELETED: no card named Clavileno is
  in `data/derived/cards.jsonl` (0 hits by name and by text), so the phrase
  names nothing printed.
- `Cards.Anaphora.eradicateSearch` — "and exile them" added
  (`exile You foundCard`). Residual STOP: "all cards with the same name"
  needs a quantity on `searchZonesOf`, which hardwires `exactly 1`.
- `Cards.Anaphora.flickeringSpirit`, `anotherRound` — "under its/their
  owner's control" now written as `Move … battlefieldZ [Under (ownerOf …)]`.
- `Cards.Anaphora.thaliasLancersSearch` — the whole body is under
  `may You`.
- `Cards.Keyword.tourachDiscardTrigger` — the invented `InZone handZ` on the
  discarded card is gone ("a card", `a IsCard`).
- `Cards.Choice.rankleMasterOfPranks` — the third printed mode (each player
  loses 1 life and draws a card) is back, in printed order.
- `Cards.Choice.emissaryOfGrudgesReveal` — the second conjunct ("and if it
  targets you or a permanent you control") added as an `AndCond`.
- `Cards.Choice.fulgentDistraction` — "then unattach all Equipment from them"
  added.
- `Cards.Choice.phantomBlade` — re-spelled as the whole printed trigger
  (`Ability`, `When this Equipment enters`), so "attach it" reads `It OneOf`
  and the destroy clause keeps its own target.
- `Cards.Deontic.glaringSpotlight`, `distortionStrikeLine` — "until end of
  turn" and "this turn" are two clauses with two durations again, not one
  shared-subject span.
- `Cards.Counters.entDraughtBasinAbility` — the invented "you control" is
  gone and "Activate only as a sorcery" is back
  (`activatedOnlyDuring … AsSorcery`).
- `Cards.Counters.captainMarvelSameKinds` — the intervening "if it's not a
  Kree" added.
- `Cards.Counters.alaundoTheSeer` — "you own" added
  (`HasPossessor OwnerAx You`). Residual STOP: "other" needs the ability's
  own just-exiled card as an anchor (`complementAnchorsOk` refuses a bare
  `Other` at `[]`), which a standalone `Instruction []` cannot supply.
- `Cards.Mana.engineeredExplosives` — cost {2}, "nonland" restored.
- `Cards.Mana.qarsiDeceiverMorphSpend` — the first printed purpose ("cast a
  face-down creature spell") added as `ToCast (And [creature, faceDown])`.
  (`Macros.spell` in that `And` makes `SpendPurposes` unsolvable, so the
  stack-zone conjunct is left off.)
- `Cards.Mana.coalStoker`, `steelswarmOperatorMana` — `Lit 3`/`Lit 2` over a
  run is now `Lit 1` with the run ({R}{R}{R} and {U}{U}, not nine and four).
- `Cards.Cost.aggressiveMining` — "You can't play lands" now has its patient
  (`cantDoTo "Play" You (allOf land)`).
- `Cards.Turn.brazenCannonade` — the printed ability word ("raid") added.
  Residual STOP: the second sentence ("Until end of combat on your next turn,
  you may play that card") needs a "combat on your next turn" span.
- `Cards.Copy.pyromancersGogglesMana` — "and you may choose new targets for
  the copy" added.
- `Cards.Faces.meldThemInto` — the exiled pair is no longer "all creatures
  you control"; it is narrowed to the two named cards. Residual STOP: the
  printed "them" is the intervening if's pair, and `condIntro` publishes
  nothing, so the pronoun is still a re-description.
- `Cards.Faces.cyberConversion` — the second sentence ("It's a 2/2 Cyberman
  artifact creature") added.
- `Cards.Piles.truthOrConsequencesVote` — the rest of the card (choose an
  opponent at random; 3 damage per consequences vote) added.

**Dropped clause … — STOP (no shape).**
- `Cards.Damage.keeperOfTheFlame` ("as you activate this ability"): no
  as-you-activate rider exists anywhere in the tree. **Goal:** a timing rider
  on the announced choice, so the comparison is fixed at activation.
- `Cards.Description.xenosquirrelsShift`: `Events.TriggerWord` is
  `When | Whenever | At` — there is no `After`, and `ShiftResult` demands a
  roll outcome in scope, so "After you roll a die, you may remove a +1/+1
  counter … If you do, increase or decrease the result by 1" cannot be
  benched. The invented `rollDice You 1 6` is left in place. **Goal:** add
  `After` to `TriggerWord` (the dice template's word) and re-spell.
- `Cards.Description.ironMastiffIgnore`: "roll a d20 for each player being
  attacked" needs a `Predicate bs Player` for "player being attacked";
  `CombatRel AttackedBy` requires an Object attacker, so the per-player count
  is unwritable. **Goal:** a defending-player predicate.
- `Cards.Description.warTaxScaledPayment`: "for each attacking creature they
  control" — `They` is not in scope at `Cost [letterB X]`
  (`countReach (Word PlayerW) OneOf (amtIntro (LetterVal X)) = 1` unsolvable).
  **Goal:** bench the whole deontic sentence so the cost sits under the
  clause that introduces "their controller".
- `Cards.Anaphora.vedalkenSquirrelWhackerReroll`: the replacement body is a
  no-op because "and you may exchange one result with this creature's base
  power or base toughness" needs the `Exchange` row of the 2026-09-04 ruling,
  which is not landed. **Goal:** re-spell once `Exchange (what : Exchanged bs)`
  exists.
- `Cards.Keyword.gabrielAngelfire`: "choose flying, first strike, trample, or
  rampage 3" needs a keyword/ability `QualitySort`; the enum is
  `Color | SubtypeQ | CardName | Number | CardTypeQ | CounterKindQ`.
  **Goal:** an ability-valued quality so "gains that ability" is a read.
- `Cards.Keyword.pirDistributive`: `Macros.manyBareCountersPutBy` takes a
  mandatory agent and the printed replacement is agentless ("if one or more
  counters would be put on a permanent your team controls"). **Goal:** an
  agentless counter-put event.
- `Cards.Choice.continueSpell` (Continue?): "creature cards in your graveyard
  that were put there from the battlefield this turn" needs a
  put-into-graveyard-from-battlefield lookback as a `Predicate`; only the
  `Condition` forms (`Happened`, `Macros.happenedFrom`) carry the zone
  complement. **Goal:** a `HappenedTo` lookback with a `FromZones` complement.
- `Cards.Deontic.undercoverButler` ("or tied for most life"):
  `Phrase.Superlative` has no tie flag (`VoteLead` has an `orTied : Bool`;
  `Superlative` does not). **Goal:** the same flag on `Superlative`.
- `Cards.Cost.hammerOfBogardan` ("from your graveyard"): `Noun.This` carries
  no zone and `Move` has no source slot, so the printed source zone of
  "Return this card from your graveyard to your hand" cannot be written
  without re-describing the card by name. **Goal:** a zone on the self
  reference.
- `Cards.Turn.karnRestart`: `Instruction.RestartsGame` is a bare nullary row,
  so "leaving in exile all non-Aura permanent cards exiled with Karn. Then
  put those cards onto the battlefield under your control" has nowhere to go.
  **Goal:** a restart row with an exile-retention argument.
- `Cards.Turn.ritesOfFlourishing`: "that player draws an additional card" —
  no "additional" modifier on `Draw`. **Goal:** an additional-draw spelling
  distinct from a plain draw [CR#121.1].

**Reminder text and non-cards on the bench.**
- DELETED as reminder text (the workbench proves rules text, never reminder
  text [CR#207.2a]): `Cards.Counters.blastodermEntersFading`,
  `blastodermFadeUpkeep` (Fading's parenthetical),
  `Cards.Counters.ascendConferral`, `saddleConferral` (their own docstrings
  said so), `Cards.Description.storiedEnduringStory` (Storied's
  parenthetical; no CR rule defines the keyword).
- DELETED as non-cards: `Cards.Mana.testMox1` (a fixture named `test*`
  carrying a "Mox Diamond" line the card does not print),
  `Cards.Description.partyCount` and its three `Refl` lemmas
  (`loneClericRogueIsPartyOfOne`, `clericBesideClericRogueIsTwo`,
  `oneOfEachRoleIsFullParty` — arithmetic over a private `List (List Nat)`
  model the grammar never spells), `Cards.Faces.meldBackFacesAgree`
  (`X = X` over two byte-identical expressions),
  `Cards.Static.grantedEntryCounterShape` (a shape with no card and no use;
  moving it would have dragged `masterChefGrantedAbility` with it).
- DELETED as re-inlined duplicates: `Cards.Damage.heartOfBogardanBody`
  (inlined verbatim in `heartOfBogardan`), `Cards.Damage.fallOfTheTitans`
  (inlined in `fallOfTheTitansCard`).
- DELETED as byte-identical pin twins with a surviving same-module twin:
  `ProofsDamage.okDamageToCreature` (kept `okDamageCreature`),
  `ProofsTrigger.okPlayerAttackDefender` (kept
  `okSingularAttackDefender`), `ProofsTurn.okSingularPartPossessor` (kept
  `okTriggerAtYourUpkeep`). Each surviving twin is byte-identical to the one
  removed and lives in the same module, so every pin that leaned on the
  removed twin still has its non-vacuity evidence beside it in-module.
- MOVED to the matching `Proofs*` module as synthetic rules-meaningful
  sentences, each with a docstring saying so: `Cards.Static.opponentsCantGainLife`
  → `ProofsStatic`; `Cards.Deontic.goadedAttacksOther`, `whileScrying` →
  `ProofsDeontic`; `Cards.Choice.lookAtTopThenBin`,
  `playAndCastFromGraveyardThisTurn`, `eachPlayerPlaysAdditionalLand`,
  `millThenPutFromAmongMilled`, `eachPlayerMayShuffleTheirHandAndGraveyard`,
  `eachPlayerMayDiscardTheirHandAndDrawSeven`, and the offer trio
  `eachPlayerOffered`/`eachPlayerOfferBindsOneMember`/`eachPlayerOfferDropsTheGroup`
  → `ProofsChoice`; `Cards.Counters.counterOnGraveyardCard` →
  `ProofsCounters`.
- DOCSTRINGED rather than moved, because real `Card` decls consume them (they
  are card text, only unlabelled): `Cards.Choice.playLandsAndCastSpellsFromTop`
  (`||| Future Sight; Magus of the Future`),
  `castSmallCreatureFromTopOnceEachTurn` (`||| Assemble the Players`), and
  `akiriUnattachOffer` (`||| Akiri, Fearless Voyager` — the review's
  `Choice.idr:1502` anchor, an ability with no docstring at all).
- KEPT, against the ticket's letter: `Cards.Deontic.propaganda` and
  `ghostlyPrisonWhole`. They are not one witness written twice — they are two
  distinct VINTAGE-legal printed cards (Propaganda {2}{U}, Ghostly Prison
  {2}{W}) whose oracle text happens to coincide; the bodies are byte-identical
  because the printed text is. Deleting either removes a real card from the
  bench. Recorded as a deviation.

**The three shapes the sweep needed a decision on — rulings taken.**

- **D-Q12 — retire both witnesses (the ticket's second option).**
  `Cards.Description.sorrowsPathCouldBlock` and `generalJarkeldCouldBeBlocked`
  are DELETED, and with them the two instruction halves the same reciprocal
  gates, `Cards.Static.sorrowsPathReassign` and `generalJarkeldReassign`.
  Reason for taking retire over "give the relation a reciprocal argument":
  writing "all creatures that the other is blocking" does not actually need a
  new `CombatRel` argument — it needs `theOther` in scope, and
  `Noun.TheRest` demands `theOtherOk k bs` (a group binding with parts), so
  the predicate must live under the two-target announcement of "Choose two
  target blocked attacking creatures". Both witnesses are `Predicate []`, and
  both cards' whole sentences additionally need "controlled by the same
  opponent" (a same-holder constraint with no shape) and an `OnlyIf` over
  "each of those creatures". Giving `CombatRel` a reciprocal argument would
  change all 26 call sites and still not make either card writable. STOP
  recorded, **goal:** re-bench Sorrow's Path and General Jarkeld whole, under
  a two-target `Targeted` frame, once a same-holder constraint and a
  `theOther`-in-predicate read exist.
- **D-Q13 — add the optional holder (the ticket's first option). LANDED.**
  `Phrase.Predicate.HasDesignation` gains a second positional slot,
  `(holder : Maybe (Noun bs Player))`, gated by a declared feature rather
  than a lexeme: `Words.designationPossessorOk d = designationHolder d ==
  Just Object`, lifted to `Phrase.designationPossessorFits d holder`
  (`Nothing` always fits; `Just _` only where an object holds the designation
  for a player). "Is your Ring-bearer" [CR#701.54e] is now
  `HasDesignation RingBearer (Just You)` at `Cards.Deontic` and in
  `ProofsDeontic.badRingBearerInGraveyard`; all 20 other reads take
  `Nothing`. New evidence in `ProofsDeontic`, twin above pin, same module:
  `okRingBearerHolder` (positive) and `badMonarchHolder` (refuses
  `HasDesignation Monarch (Just You)` — the monarch IS a player [CR#725.1]).
  Non-vacuity probed both ways: mis-stating `badRingBearerInGraveyard`'s zone
  to the battlefield turns its `Oh impossible` into
  `"badRingBearerInGraveyard Oh is not a valid impossible case"`, and the new
  pin's refusal names the new gate (`Can't find an implementation for So
  False` at `designationPossessorFits`).
- **D-Q15 — rename the row (the ticket's first option). LANDED.**
  `Effect.StaticSpec.CostsToCast` → `Effect.StaticSpec.Costs`. The row is a
  cost-modification static (`staticKind … = CostModification`) over a
  described object and a `CostShift`; it never named the deed, which is why
  13 "costs less to activate" witnesses sat in a row called `ToCast`. `Costs
  n sh` reads correctly for both "this spell costs {1} less to cast" and
  "abilities … cost {2} less to activate". Not split: splitting would need a
  deed slot the row does not carry and would double 51 call sites for a
  distinction the shift itself does not make. All 51 sites re-spelled.
- **D-Q16 — ruling, and it is applied to all four.**
  **A witness may be a named PART of a printed card — an `Ability`, a
  `StaticSpec`, a `Cost`, a `Predicate` — because the bench is split by
  grammar family and one card's abilities necessarily live in different
  family modules. A witness may NOT encode part of one printed SENTENCE: half
  a sentence has no rules meaning to prove, so it cannot be evidence that the
  sentence is representable. A `Card` may NOT omit a printed line: a `Card`
  asserts the whole printed face (that is what its law bundles are about), so
  a missing ability is a false frame.**
  Applied:
  - `Cards.Keyword.pymParticlesVigilanceGrant` → `pymParticlesGrant`, now the
    whole first sentence ("gains vigilance until end of turn **and can't be
    blocked this turn**"), the second clause carrying its own `ThisTurn`
    span. The card's second sentence ("Draw a card") is a separate sentence
    and stays off this witness.
  - `Cards.Keyword.darkbladeAgentDeathtouch` → `darkbladeAgentGrants`, now
    both halves of the one printed sentence: deathtouch **and** the quoted
    "Whenever this creature deals combat damage to a player, you draw a card"
    granted ability, under the one `Conditionally … AsLongAs`.
  - `Cards.Trigger.faridehResultRead` — the first sentence restored (see the
    dropped-clause section).
  - `Cards.Counters.thranduilsCompany` — the missing printed line is back
    ("As long as you control another Elf, you may play an additional land on
    each of your turns"), and the landfall ability word is on the trigger.

## Landing record

Measured on change `qtxulkoq` (working copy of
`.workspaces/workbench-witness-mismatch-sweep`), parent `zkkzonwu`
(`kata: claim workbench-witness-mismatch-sweep`).

**Numbers before → after**

| figure | before | after |
|---|---:|---:|
| `Cards/*.idr` top-level declarations (bench witnesses) | 1 659 | 1 624 |
| `Proofs*.idr` top-level declarations | 1 191 | 1 203 |
| `Unspellable` sites in `Proofs*.idr` | 645 | 646 |
| modules in the full gate | 46 | 46 |

The −35 bench declarations are exactly the 35 named removals (22 deletions,
13 moved into `Proofs*`); a name-by-name diff of every `Cards/*.idr`
declaration list confirms nothing else left the bench. The +12 proof
declarations are the 13 moved in, plus `okRingBearerHolder` and
`badMonarchHolder`, minus the 3 byte-identical twins removed.

Coverage lock, construction count, licensing-checker total and the selection
census are not applicable: this round touches no Rust crate and no
`cargo xtask coverage` input — the diff is `idris/`, the ticket, and nothing
else (`cr-citations.lock` is byte-unchanged; `cite bless` registered no new
rule).

**Gate artifacts**

- `cd idris && rm -rf build && ./scripts/build` — exit 0, `46/46: Building
  Cards (src/Cards.idr)`, 0 lines matching `Error` or `Warning`. Wall clock
  240 s for the clean 46-module build (this is the workbench gate; the
  16.26 s quiet-host ceiling is the `coverage` command's and does not apply —
  no coverage run in this round, so no per-byte thread-CPU line either).
- `cargo xtask cite check --list-noncompliant` — `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check` — `checked 14310 citations against cr.txt
  (eff. 2026-08-07); 0 stale`.
- `cargo xtask cite bless` — `blessed 1424 rules`; `cr-citations.lock`
  byte-unchanged (every rule cited was already registered).
- `jj --no-pager diff --git | cargo xtask cite audit --diff` — 6 sites, each
  read against its rule text: [CR#506.2] (the active player is the attacking
  player), [CR#121.1] (a player draws a card by …), [CR#207.2a] (reminder
  text is italicized text within parentheses …), [CR#701.54e] ×2 ("is your
  Ring-bearer" is true if the creature is on the battlefield under your
  control and has the designation), [CR#725.1] (the monarch is a designation
  a player can have). One citation was caught wrong by this pass and
  fixed: the monarch claim was first written with a rule number two sections
  away, which `cite check` flagged as UNLOCKED and whose text is about Omen
  card frames.

**Assurance counts**

- restored: 0 (no test was failing at the start of the round).
- re-spelled: 1 pin — `ProofsDeontic.badRingBearerInGraveyard` (the holder
  slot added to its Ring-bearer read); 1 pin twin —
  `ProofsDamage.okThatCreatureAfterDamage` was probed and left as it stood
  (see its STOP).
- ignored with a blocker: 0. No `#[ignore]`-equivalent was added; the Idris
  gate has no skip.
- added: 2 (`ProofsDeontic.okRingBearerHolder` positive twin,
  `ProofsDeontic.badMonarchHolder` pin for the new holder gate), plus the 13
  bench declarations relocated into `Proofs*` as synthetic sentences.
- removed: 25, each justified by name above — 22 from `Cards/*.idr`
  (2 different-card, 2 dropped-clause, 13 reminder-text/non-card,
  1 `grantedEntryCounterShape`, 4 D-Q12 retirements) and 3 byte-identical
  pin twins from `ProofsDamage`/`ProofsTrigger`/`ProofsTurn`, each with an
  identical surviving twin in the same module.

**Deviations and additions**

1. `Cards.Deontic.propaganda` / `ghostlyPrisonWhole` were KEPT against the
   ticket's letter: they are two distinct VINTAGE-legal printed cards whose
   oracle text coincides, not one witness written twice.
2. `Cards.Static.tahngarthAttacksThatJoin` was accidentally cut with the
   D-Q12 block and restored verbatim in the same pass; its class-1 docstring
   (`||| Tahngarth, First Mate`) is on it.
3. `Cards.Choice.phantomBlade` changed type from `Instruction []` to
   `Ability` and `Cards.Trigger.dontBlinkReplacement` from `StaticSpec []` to
   `Instruction []`. Both were necessary to read the printed sentence (an
   "attach it" pronoun needs the trigger's frame; a duration needs
   `Continuously`), not cosmetic.
4. `Cards.Choice.playLandsAndCastSpellsFromTop`,
   `castSmallCreatureFromTopOnceEachTurn` and `akiriUnattachOffer` were
   docstringed rather than moved or deleted (the first two are consumed by
   real `Card` decls, so they are card text; the third is Akiri, Fearless
   Voyager's own ability).
5. `Phrase.Predicate.HasDesignation` and `Effect.StaticSpec.Costs` are core
   edits, authorised by the ticket's D-Q13 and D-Q15 bullets. `Words` gains
   one function (`designationPossessorOk`) and `Phrase` one
   (`designationPossessorFits`); no other core row changed.
6. `Cards.Anaphora.writeIntoBeingPlacement` now spells manifest as
   `Enact Nothing "Manifest" (Move … battlefieldZ [])` — a label-plus-body
   `Enact` over the existing `actFacts "Manifest"` row. The 2/2 face-down
   body of [CR#701.40a] is Write into Being's reminder text and is not
   asserted.

**STOPs**

Twenty-three witnesses could not be made to read their card because the shape
they need does not exist, plus the two cards D-Q12 retires. Each is listed by
name with its exact goal above: six in the "Reference not as printed — STOP"
block (`karplusanYeti` with its `ProofsDamage` twin, `drachNyen`,
`phyrexianIngesterPump`, `chromeMoxMana`, `answeredPrayers`,
`tahngarthChoosesDefender`), twelve in the "Dropped clause … — STOP" block
(`keeperOfTheFlame`, `xenosquirrelsShift`, `ironMastiffIgnore`,
`warTaxScaledPayment`, `vedalkenSquirrelWhackerReroll`, `gabrielAngelfire`,
`pirDistributive`, `continueSpell`, `undercoverButler`, `hammerOfBogardan`,
`karnRestart`, `ritesOfFlourishing`), five residual clauses on witnesses that
otherwise landed (`agencyOutfitterSearch`, `eradicateSearch`,
`alaundoTheSeer`, `brazenCannonade`, `meldThemInto`), and D-Q12's Sorrow's
Path and General Jarkeld. None was resolved by inventing a shape, and none
was hidden: every one is left reading what it read before, or narrowed as far
as the current grammar allows, and named here.
