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
