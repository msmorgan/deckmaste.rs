import Semantics.Check
import Semantics
import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Cards.Description

/-!
# Semantics.Cards.Anaphora

Port of `idris/src/Experimental/Cards/Anaphora.idr`: the printed cards of the Anaphora family
and the phrase-level bench items beside them, each paired with an `ok…` theorem that runs the
checker on it at the bindings its Idris type named; the file's own `Refl` pins are theorems.

Not ported: `ichorElixirPlanarDice` (a planar die is Planechase, out of scope).
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

def cloudshift : Action :=
  sequentially
    [exile (target (and [isCreature, youControl])), putOntoBattlefieldUnderYourControl (that card)]
theorem okCloudshift : Action.check (DRS.ofList []) cloudshift = [] := by decide

def bitterDownfall : Action :=
  sequentially [destroy (target isCreature), instruct (controllerOf it) (loseLife (.lit 2))]
theorem okBitterDownfall : Action.check (DRS.ofList []) bitterDownfall = [] := by decide

def suspendedSentence : Action :=
  sequentially
    [ destroy (target (and [isCreature, anOpponentControls])),
      instruct (that player) (loseLife (.lit 3)) ]
theorem okSuspendedSentence : Action.check (DRS.ofList []) suspendedSentence = [] := by decide

def flickeringSpirit : Action :=
  sequentially [exile thisCreature, putInto it battlefield (riders := [(under (ownerOf it))])]
theorem okFlickeringSpirit : Action.check (DRS.ofList []) flickeringSpirit = [] := by decide

/-- Bond of Revival -/
def bondOfRevival : Action :=
  sequentially
    [ returnToBattlefield (target (and [isCreature, in_ yourGraveyard])),
      gainHaste (theVerbed (.core .return_) .one) (some untilYourNextTurn) ]
theorem okBondOfRevival : Action.check (DRS.ofList []) bondOfRevival = [] := by decide

def vraskasStoneglare : Action :=
  sequentially [destroy (target isCreature), gainLife (toughnessOf it)]
theorem okVraskasStoneglare : Action.check (DRS.ofList []) vraskasStoneglare = [] := by decide

def phthisis : Action :=
  sequentially
    [ destroy (target isCreature),
      instruct (controllerOf it) (loseLife
        (plus (powerOf it) (toughnessOf it))) ]
theorem okPhthisis : Action.check (DRS.ofList []) phthisis = [] := by decide

def foulTongueShriek : Action :=
  sequentially
    [ instruct (target isOpponent) (loseLife
        (forEach (.lit 1) (and [isAttacking, isCreature, youControl]))),
      gainLife thatMuch ]
theorem okFoulTongueShriek : Action.check (DRS.ofList []) foulTongueShriek = [] := by decide

def phyrexianInfiltrator : Action :=
  simultaneously
    [ gainControl thisCreature none (controller := controllerOf (target isCreature)),
      gainControl (that creature) none (controller := controllerOf thisCreature) ]
theorem okPhyrexianInfiltrator : Action.check (DRS.ofList []) phyrexianInfiltrator = [] := by decide

def impulse : Action :=
  sequentially
    [ lookAt (topSlice (.lit 4)), putInto (someOf (exactly 1) them) hand,
      putInto (theRest .object) onBottom (riders := [(inOrder .anyOrder)]) ]
theorem okImpulse : Action.check (DRS.ofList []) impulse = [] := by decide

def anticipate : Action :=
  sequentially
    [ lookAt (topSlice (.lit 3)), putInto (someOf (exactly 1) them) hand,
      putInto (theRest .object) onBottom (riders := [(inOrder .anyOrder)]) ]
theorem okAnticipate : Action.check (DRS.ofList []) anticipate = [] := by decide

def revealFourPartition : Action :=
  sequentially
    [ reveal (topSlice (.lit 4)), putInto (someOf (exactly 1) (those card)) hand,
      putInto (theRest .object) graveyard ]
theorem okRevealFourPartition : Action.check (DRS.ofList []) revealFourPartition = [] := by decide

def exileFourOfThem : Action :=
  sequentially
    [ lookAt (topSlice (.lit 8)), exile (someOf (exactly 4) them),
      putInto (theRest .object) onTop (riders := [(inOrder .anyOrder)]) ]
theorem okExileFourOfThem : Action.check (DRS.ofList []) exileFourOfThem = [] := by decide

def sylvanScrying : Action :=
  sequentially [searchLibraryFor (exactly 1) isLand, reveal it, putInto it hand, shuffle]
theorem okSylvanScrying : Action.check (DRS.ofList []) sylvanScrying = [] := by decide

def searchToBattlefield : Action :=
  sequentially [searchLibraryFor (exactly 1) isCreature, putOntoBattlefield it, shuffle]
theorem okSearchToBattlefield : Action.check (DRS.ofList []) searchToBattlefield = [] := by decide

def glimpseTheUnthinkable : Action := instruct (target isPlayer) (mill (.lit 10))
theorem okGlimpseTheUnthinkable : Action.check (DRS.ofList []) glimpseTheUnthinkable = [] := by decide

def millThenReadGroup : Action :=
  sequentially [mill (.lit 3), exile (those card)]
theorem okMillThenReadGroup : Action.check (DRS.ofList []) millThenReadGroup = [] := by decide

def takeIntoCustody : Action :=
  sequentially [tap (target isCreature), skipUntap it (.lit 1)]
theorem okTakeIntoCustody : Action.check (DRS.ofList []) takeIntoCustody = [] := by decide

/-- Frenzied Gorespawn -/
def frenziedGorespawnGoad : Action :=
  doForEach eachOpponent
    (gainDesignation
      (target (and [isCreature, controlledBy (that player)])) goaded)
theorem okFrenziedGorespawnGoad : Action.check (DRS.ofList []) frenziedGorespawnGoad = [] := by decide

def spaceTimeAnomaly : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Space-Time Anomaly", cost := some [generic 2, pip .white, pip .blue],
      types := [.sorcery],
      text := [ spell (instruct (target isPlayer) (mill (lifeTotalOf you))) ] } }

/-- Consecrate // Consume -/
def consume : Ability :=
  spell (sequentially
    [ instruct (target isPlayer) (sacrifice
        (a (and [ isCreature,
                   superlative .max (.stat .power)
                     (and [isCreature, controlledBy they]) ]))),
      gainLife (powerOf it) ])
theorem okConsume : Ability.check (DRS.ofList []) consume = [] := by decide

def bifurcate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bifurcate", cost := some [generic 3, pip .green], types := [.sorcery],
      text :=
        [ spell (sequentially
            [ searchLibraryFor (exactly 1)
                (and [isPermanentCard, (shares .cardName .share (target (and [isCreature, isNontoken])))]),
              putOntoBattlefield (that card),
              shuffle ]) ] } }

/-- Mana Leak {1}{U} — Instant. "Counter target spell unless its controller pays {3}." -/
def manaLeak : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mana Leak", cost := some [generic 1, pip .blue], types := [.instant],
      text :=
        [ spell (instruct (controllerOf (target isSpell)) (doUnless
            (counter it)
            (mana [generic 3]))) ] } }

/-- Oust {W} — Sorcery. "Put target creature into its owner's library second from the top.
Its controller gains 3 life." -/
def oust : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Oust", cost := some [pip .white], types := [.sorcery],
      text :=
        [ spell (sequentially
            [ putInto (target isCreature) (fromTop 2 library),
              instruct (controllerOf it) (gainLife (.lit 3)) ]) ] } }

def riseFromTheGrave : Action :=
  sequentially
    [ putOntoBattlefieldUnderYourControl (target (and [isCreature, in_ graveyard])),
      become (that creature)
        { characteristics := { colors := [.black], subtypes := [zombie] } } ]
theorem okRiseFromTheGrave : Action.check (DRS.ofList []) riseFromTheGrave = [] := by decide

def everAfter : Action :=
  sequentially
    [ putOntoBattlefield (targets (upTo 2) (and [isCreature, in_ yourGraveyard])),
      become (those creature)
        { characteristics := { colors := [.black], subtypes := [zombie] } },
      putInto this_ onBottom ]
theorem okEverAfter : Action.check (DRS.ofList []) everAfter = [] := by decide

def martyrsCry : Action :=
  sequentially
    [ exile (allOf (and [isCreature, hasColor .white])),
      doForEach (theVerbed (.action "exile") .many (some creature) .thisWay)
        (instruct (controllerOf it) (draw (.lit 1))) ]
theorem okMartyrsCry : Action.check (DRS.ofList []) martyrsCry = [] := by decide

def anotherRound : Action :=
  sequentially
    [ exile (counted anyNumber (and [isCreature, youControl])),
      putInto them battlefield (riders := [(under (ownerOf them))]),
      repeat_ (moreTimes (letter .x)) ]
theorem okAnotherRound : Action.check (DRS.ofList []) anotherRound = [] := by decide

/-- Eradicate -/
def eradicateSearch : Action :=
  sequentially
    [ exile (target (and [isCreature, non (hasColor .black)])),
      searchZonesOf (controllerOf (that card)) anyNumber (shares .cardName .share (that card)),
      exile (theVerbed (.action "search") .many),
      instruct (that player) shuffle ]
theorem okEradicateSearch : Action.check (DRS.ofList []) eradicateSearch = [] := by decide

/-- Deem Inferior -/
def deemInferior : Action :=
  instruct (ownerOf (target (and [isPermanent, non isLand])))
      (put it (or [(fromTop 2 library), (fromBottom 2 library)]))
theorem okDeemInferior : Action.check (DRS.ofList []) deemInferior = [] := by decide

/-- Write into Being -/
def writeIntoBeingPlacement : Action :=
  sequentially
    [ lookAt (topSlice (.lit 2)),
      manifestPlacement (someOf (exactly 1) them),
      putInto (theOther .object) (or [onTop, onBottom]) ]
theorem okWriteIntoBeingPlacement : Action.check (DRS.ofList []) writeIntoBeingPlacement = [] := by decide

/-- Mystical Tutor -/
def mysticalTutor : Action :=
  sequentially
    [searchLibraryFor (exactly 1) isInstantOrSorcery, reveal it, shuffle, putInto it onTop]
theorem okMysticalTutor : Action.check (DRS.ofList []) mysticalTutor = [] := by decide

/-- Demonic Tutor -/
def demonicTutor : Action :=
  sequentially [searchLibraryFor (exactly 1) (and []), putInto it hand, shuffle]
theorem okDemonicTutor : Action.check (DRS.ofList []) demonicTutor = [] := by decide

/-- Thalia's Lancers -/
def thaliasLancersSearch : Action :=
  may
    (sequentially
      [ searchLibraryFor (exactly 1) (and [hasSupertype .legendary]), reveal it,
        putInto it hand, shuffle ])
theorem okThaliasLancersSearch : Action.check (DRS.ofList []) thaliasLancersSearch = [] := by decide

def contrabandLivestock : Action :=
  sequentially
    [ exile (target isCreature),
      rollDice (.lit 1) (.sides 20),
      applyResultsTable
        [ rollRow (oneThrough 9)
            (instruct (controllerOf it) (createToken (creatureToken 4 4 [.green] [ox]))),
          rollRow (fromTo 10 19)
            (instruct (controllerOf it) (createToken (creatureToken 2 2 [.green] [boar]))),
          rollRow (exactly 20)
            (instruct (controllerOf it) (createToken (creatureToken 0 1 [.white] [goat]))) ] ]
theorem okContrabandLivestock : Action.check (DRS.ofList []) contrabandLivestock = [] := by decide

/-- Hypnotic Specter -/
def hypnoticSpecterDiscard : Action := discard (aAtRandom (cardIn (handOf they)))
theorem okHypnoticSpecterDiscard :
    Action.check (DRS.ofList [{ det := .the, plur := .one, payload := .player false }]) hypnoticSpecterDiscard = [] := by decide

/-- Wyll, Blade of Frontiers -/
def wyllExtraDie : Action :=
  replaceEvent (would you (rollDice anyAmount .anyDie .many))
    (instead (sequentially
      [ rollDice (plus thatMuch (.lit 1)) .thoseDice .many .anyResult,
        ignoreOutcomes (extreme .lowest) ]))
    none
theorem okWyllExtraDie : Action.check (DRS.ofList []) wyllExtraDie = [] := by decide

/-- Vedalken Squirrel-Whacker -/
def vedalkenSquirrelWhackerReroll : Action :=
  replaceEvent (would you (rollDice anyAmount (.sides 6) .many))
    (instead (sequentially
      [ rollDice thatMuch .thoseDice .many .anyResult,
        may
          (exchange (values (theOutcome .rollResult) (powerOf thisCreature))) ]))
    none
theorem okVedalkenSquirrelWhackerReroll :
    Action.check (DRS.ofList []) vedalkenSquirrelWhackerReroll = [] := by decide

/-- Investigator's Journal's count -/
def greatestCreaturesAPlayerControls : Amount :=
  aggregateOver .max isPlayer (countOf (and [isCreature, controlledBy they]))
theorem okGreatestCreaturesAPlayerControls :
    Amount.check (DRS.ofList []) greatestCreaturesAPlayerControls = [] := by decide
/-- Cavern-Hoard Dragon -/
def greatestArtifactsAnOpponentControls : Amount :=
  aggregateOver .max isOpponent (countOf (and [isArtifact, controlledBy they]))
theorem okGreatestArtifactsAnOpponentControls :
    Amount.check (DRS.ofList []) greatestArtifactsAnOpponentControls = [] := by decide

/-- "each creature that player or that planeswalker's controller controls" -/
def eachCreatureThatSplitControls : NounPhrase :=
  each (and [isCreature, controlledBy splitOverPlaneswalker])

def anyTargetAnnounced : NounPhrase := target isAnyTarget
theorem okAnyTargetAnnounced : NounPhrase.checkAt none (DRS.ofList []) anyTargetAnnounced = [] := by decide
def playerOrPlaneswalkerAnnounced : NounPhrase := targetPlayerOrPlaneswalker
/-- Chain of Plasma -/
def chainOfPlasmaOfferee : NounPhrase := splitOverPermanent
theorem okChainOfPlasmaOfferee :
    NounPhrase.checkAt (some .player)
        ((stepNounPhrase 512 (DRS.ofList []) .obj true anyTargetAnnounced).1) chainOfPlasmaOfferee =
        [] := by
  decide
/-- Chain Lightning -/
def chainLightningPayer : NounPhrase := splitOverPermanent
theorem okChainLightningPayer :
    NounPhrase.checkAt (some .player)
        ((stepNounPhrase 512 (DRS.ofList []) .obj true anyTargetAnnounced).1) chainLightningPayer =
        [] := by
  decide
/-- Flames of the Blood Hand -/
def flamesOfTheBloodHandSubject : NounPhrase := splitOverPlaneswalker
theorem okFlamesOfTheBloodHandSubject :
    NounPhrase.checkAt (some .player)
        ((stepNounPhrase 512 (DRS.ofList []) .obj true playerOrPlaneswalkerAnnounced).1)
        flamesOfTheBloodHandSubject = [] := by
  decide
/-- Flaming Gambit -/
def flamingGambitOfferee : NounPhrase := splitOverPlaneswalker
theorem okFlamingGambitOfferee :
    NounPhrase.checkAt (some .player)
        ((stepNounPhrase 512 (DRS.ofList []) .obj true playerOrPlaneswalkerAnnounced).1)
        flamingGambitOfferee = [] := by
  decide
theorem okEachCreatureThatSplitControls :
    NounPhrase.checkAt (some .object)
        ((stepNounPhrase 512 (DRS.ofList []) .obj true playerOrPlaneswalkerAnnounced).1)
        eachCreatureThatSplitControls = [] := by
  decide

/-- Ertai's Trickery -/
def ertaisTrickery : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ertai's Trickery", cost := some [pip .blue], types := [.instant],
      text :=
        [ spell (doOnlyIf (counter (target isSpell))
            (keywordCostPaid kicker it)) ] } }

/-- Celebrate the Harvest -/
def celebrateTheHarvest : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Celebrate the Harvest", cost := some [generic 3, pip .green], types := [.sorcery],
      text :=
        [ spell (sequentially
            [ searchLibraryFor (upToOf (letter .x)) (and [isLand, hasSupertype .basic]),
              define .x (distinctCount (.value .power) (allOf (and [isCreature, youControl]))),
              putOntoBattlefieldTapped (those card),
              shuffle ]) ] } }

/-- Boreas Charger's description -/
def opponentWithMoreLands : Predicate :=
  compareOver isOpponent (countOf (and [isLand, controlledBy they])) .greater
    (countOf (and [isLand, youControl]))
theorem okOpponentWithMoreLands : Predicate.checkAt .player (DRS.ofList []) opponentWithMoreLands = [] := by
  decide

def bioplasmCardTest : Condition := is itCard isCreature
theorem okBioplasmCardTest : Condition.check (nextClause bioplasmAfterExile) bioplasmCardTest = [] := by decide

/-- Blessed Respite -/
def blessedRespiteShuffle : Action :=
  instruct (target isPlayer) (shuffleInto (allOf (cardIn (graveyardOf they))))
theorem okBlessedRespiteShuffle : Action.check (DRS.ofList []) blessedRespiteShuffle = [] := by decide

/-- Nekrataal -/
def nekrataalEntry : GameEvent := does thisCreature enter
theorem okNekrataalEntry : GameEvent.check (DRS.ofList []) nekrataalEntry = [] := by decide
def nekrataalDestroy : Action :=
  destroy (target (and [isCreature, non isArtifact, non (hasColor .black)]))
theorem okNekrataalDestroy :
    Action.check ((stepGameEvent 512 (DRS.ofList []) nekrataalEntry).1) nekrataalDestroy = [] := by decide
def nekrataalRider : DRS :=
  (stepAction 512 .postEvent .obj ((stepGameEvent 512 (DRS.ofList []) nekrataalEntry).1) nekrataalDestroy).1
theorem nekrataalOneCreatureWord : countReach (.word creature) .one nekrataalRider = 1 := by
  decide
theorem nekrataalOneDestroyed : countReach (.verbed (.action "destroy") none .attributive) .one nekrataalRider = 1 := by
  decide

def sequencedRiderBody : Action :=
  sequentially [exile (target isArtifact), destroy (target isCreature)]
theorem okSequencedRiderBody : Action.check (DRS.ofList []) sequencedRiderBody = [] := by decide
def sequencedRider : DRS := (stepAction 512 .postEvent .obj (DRS.ofList []) sequencedRiderBody).1
theorem sequencedRiderOneDestroyed :
    countReach (.verbed (.action "destroy") none .attributive) .one sequencedRider = 1 := by decide
theorem sequencedRiderOneExiled : countReach (.verbed (.action "exile") none .attributive) .one sequencedRider = 1 := by
  decide

/-- Engulfing Flames -/
def engulfingFlamesBody : Action := instruct this_ (dealDamage (.lit 1) (target isCreature) .noncombatOnly)
theorem okEngulfingFlamesBody : Action.check (DRS.ofList []) engulfingFlamesBody = [] := by decide
def engulfingFlamesRider : DRS := (stepAction 512 .postEvent .obj (DRS.ofList []) engulfingFlamesBody).1
theorem engulfingFlamesNoDestroyStamp :
    countReach (.verbed (.action "destroy") none .attributive) .one engulfingFlamesRider = 0 := by decide
theorem engulfingFlamesBareReadStands : countReach .bare .one engulfingFlamesRider = 1 := by decide

def bioplasmExiledPronoun : NounPhrase := theVerbed (.action "exile") .one
theorem okBioplasmExiledPronoun :
    NounPhrase.checkAt (some .object) bioplasmAfterExile bioplasmExiledPronoun = [] := by decide
theorem bioplasmNoTypedRead :
    countReach (.verbed (.action "exile") (some creature) .attributive) .one bioplasmAfterExile
      = 0 := by
  decide
theorem bioplasmExiledCardHasNoType :
    tyOfReach (.verbed (.action "exile") none .attributive) .one bioplasmAfterExile = [] := by decide
def bioplasmAfterTest : DRS := (stepCondition 512 (nextClause bioplasmAfterExile) bioplasmCardTest).1
theorem bioplasmTypeTestPreservesExiledIdentity :
    let reach := Reach.verbed (.action "exile") none .attributive
    let before := resolveCandidates reach .one bioplasmAfterExile
    let after := resolveCandidates reach .one bioplasmAfterTest
    before.length = 1 ∧ after.map (·.id) = before.map (·.id) := by decide
theorem bioplasmTestRemarksType :
    tyOfReach (.verbed (.action "exile") none .attributive) .one bioplasmAfterTest = [.creature] := by decide
theorem bioplasmTestKeepsCardSlot : countReach (.atSlot .card) .one bioplasmAfterTest = 1 := by
  decide
theorem bioplasmTypedReadStillRefused :
    countReach (.verbed (.action "exile") (some creature) .attributive) .one bioplasmAfterTest
      = 0 := by
  decide
theorem bioplasmTypedCardReadWrites :
    countReach (.verbed (.action "exile") (some (.ofType .card .creature)) .attributive) .one
      bioplasmAfterTest = 1 := by
  decide

/-- Scapeshift -/
def scapeshiftSacrifice : Action := sacrifice (counted anyNumber isLand)
theorem okScapeshiftSacrifice : Action.check (DRS.ofList []) scapeshiftSacrifice = [] := by decide
def scapeshiftSacrificed : DRS := (stepAction 512 .postEvent .obj (DRS.ofList []) scapeshiftSacrifice).1
def scapeshiftSearch : Action := searchLibraryFor (upToOf groupSize) isLand
theorem okScapeshiftSearch : Action.check scapeshiftSacrificed scapeshiftSearch = [] := by
  decide
def scapeshiftAfterSearch : DRS := (stepAction 512 .postEvent .obj scapeshiftSacrificed scapeshiftSearch).1
theorem scapeshiftTwoGroups : countReach .bare .many scapeshiftAfterSearch = 2 := by decide
theorem scapeshiftOneSearchedGroup :
    countReach (.verbed (.action "search") none .attributive) .many scapeshiftAfterSearch = 1 := by decide
theorem scapeshiftOneSacrificedGroup :
    countReach (.verbed (.action "sacrifice") none .attributive) .many scapeshiftAfterSearch = 1 := by decide

/-- Scapeshift -/
def scapeshift : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Scapeshift", cost := some [generic 2, pip .green, pip .green], types := [.sorcery],
      text :=
        [ spell (sequentially
            [ sacrifice (counted anyNumber isLand),
              searchLibraryFor (upToOf groupSize) isLand,
              putOntoBattlefieldTapped (theVerbed (.action "search") .many),
              shuffle ]) ] } }

/-- Bind to Life, Vastlands Scavenger's adventure -/
def bindToLife : Action :=
  sequentially
    [mill (.lit 7), putOntoBattlefield (fromAmong (exactly 1) isCreature them)]
theorem okBindToLife : Action.check (DRS.ofList []) bindToLife = [] := by decide

/-- Glamdring, Foe-hammer's Gleam of Death -/
def gleamOfDeath : Action :=
  sequentially
    [mill (.lit 6), putInto (allFromAmong isInstantOrSorcery them) hand]
theorem okGleamOfDeath : Action.check (DRS.ofList []) gleamOfDeath = [] := by decide

/-- Tezzeret, Master of the Bridge -/
def tezzeretAllArtifacts : Action :=
  sequentially
    [ exile (topSlice (.lit 10)),
      putOntoBattlefield (allFromAmong isArtifact
                            (theVerbed (.action "exile") .many (some .card) .thisWay)) ]
theorem okTezzeretAllArtifacts : Action.check (DRS.ofList []) tezzeretAllArtifacts = [] := by decide

def companyContext : DRS := (stepAction 512 .postEvent .obj (DRS.ofList []) (lookAt (topSlice (.lit 6)))).1
def companyDescribedSlice : NounPhrase :=
  fromAmong (upTo 2) (and [isCreature, hasManaValue .atMost (.lit 3)]) them
theorem okCompanyDescribedSlice :
    NounPhrase.checkAt (some .object) companyContext companyDescribedSlice = [] := by decide
def companyBareSlice : NounPhrase := someOf (exactly 2) them
theorem okCompanyBareSlice : NounPhrase.checkAt (some .object) companyContext companyBareSlice =
    [] := by
  decide

/-- Lord of the Void -/
def exileTopThenPutFromAmong : Action :=
  sequentially
    [ exile (slice .top (.lit 7) (libraryOf (target isOpponent))),
      putOntoBattlefieldUnderYourControl (fromAmong (exactly 1) isCreature them) ]
theorem okExileTopThenPutFromAmong : Action.check (DRS.ofList []) exileTopThenPutFromAmong = [] := by decide

/-- Thought Sponge -/
def greatestCardsAnOpponentDrew : Amount :=
  aggregateOver .max isOpponent (eventCount (draw anyAmount) they soFarThisTurn)
theorem okGreatestCardsAnOpponentDrew : Amount.check (DRS.ofList []) greatestCardsAnOpponentDrew = [] := by
  decide
def greatestCardsAPlayerDiscardedThisWay : Amount :=
  aggregateOver .max isPlayer
    (eventTally they (discard (allOf (cardIn hand))) thisWay .events)
theorem okGreatestCardsAPlayerDiscardedThisWay :
    Amount.check (DRS.ofList []) greatestCardsAPlayerDiscardedThisWay = [] := by decide

def eachPlayerBase : NounPhrase := eachPlayer
theorem okEachPlayerBase : NounPhrase.checkAt (some .player) (DRS.ofList []) eachPlayerBase = [] := by decide
theorem eachPlayerBindsOneDistributedMember :
    let result := stepNounPhrase 512 (DRS.ofList []) .obj true eachPlayerBase
    result.2.1 = [0] ∧
      (result.1.findById? 0).map (fun ref => (ref.det, ref.plur, ref.kind)) =
        some (.each, .one, .player) := by decide
theorem eachPlayerDoesNotIntroduceASeparateGroup :
    countManys .player ((stepNounPhrase 512 (DRS.ofList []) .obj true eachPlayerBase).1) = 0 := by decide

/-- Soul Ransom -/
def soulRansomRansom : Action :=
  sequentially [instruct (controllerOf thisAura) (sacrifice itPermanent), instruct they (draw (.lit 2))]
theorem okSoulRansomRansom : Action.check (DRS.ofList []) soulRansomRansom = [] := by decide
def thisAurasController : NounPhrase := controllerOf thisAura
theorem okThisAurasController : NounPhrase.checkAt (some .player) (DRS.ofList []) thisAurasController = [] := by
  decide
def targetCreaturesController : NounPhrase := controllerOf (target isCreature)
theorem okTargetCreaturesController :
    NounPhrase.checkAt (some .player) (DRS.ofList []) targetCreaturesController = [] := by decide
theorem possessiveDeicticIsReadableByIt :
    countReach (.atSlot .permanent) .one ((stepNounPhrase 512 (DRS.ofList []) .obj true thisAurasController).1) = 1 := by decide
theorem possessiveDeicticDemonstrativeSelectsTheSource :
    let result := stepNounPhrase 512 (DRS.ofList []) .obj true thisAurasController
    let selected := resolveCandidates (.word enchantment) .one result.1
    result.2.1 = [1] ∧ selected.map (·.id) = [0] ∧
      selected.all (fun ref => ref.zone == some .battlefield && ref.ty == [.enchantment]) := by decide

theorem possessiveDescribedBaseUnchanged :
    countReach (.atSlot .permanent) .one ((stepNounPhrase 512 (DRS.ofList []) .obj true targetCreaturesController).1) = 1 := by decide

/-- Bile Blight -/
def bileBlight : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bile Blight", cost := some [pip .black, pip .black], types := [.instant],
      text :=
        [ spell (get
            (both (target isCreature)
              (allOf (and [ isCreature, (shares .cardName .share (that creature)),
                             otherThan (that creature) ])))
            (down (.lit 3)) (down (.lit 3)) (some untilEndOfTurn)) ] } }

/-- Echoing Ruin -/
def echoingRuin : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Echoing Ruin", cost := some [generic 1, pip .red], types := [.sorcery],
      text :=
        [ spell (destroy
            (both (target isArtifact)
              (allOf (and [ isArtifact, (shares .cardName .share (that artifact)),
                             otherThan (that artifact) ])))) ] } }

/-- Hijack -/
def hijack : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hijack", cost := some [generic 1, pip .red, pip .red], types := [.sorcery],
      text :=
        [ spell (sequentially
            [ gainControl (target (or [isArtifact, isCreature])) (some untilEndOfTurn) (controller := you),
              untap it,
              gainHaste it (some untilEndOfTurn) ]) ] } }

def hijackDeedDescriptions : Action :=
  sequentially
    [ gainControl (target (or [isArtifact, isCreature]))
        (some untilEndOfTurn) (controller := you),
      untap (theVerbed (.core .gainControl) .one),
      gainHaste (theVerbed (.action "untap") .one) (some untilEndOfTurn) ]
theorem okHijackDeedDescriptions :
    Action.check (DRS.ofList []) hijackDeedDescriptions = [] := by decide

/-- Open the Vaults -/
def openTheVaults : Action :=
  returnTo
    (allOf (and [ isCard, or [isArtifact, isEnchantment],
                   in_ (graveyardOf (bare isPlayer)) ]))
    battlefield [under (ownerOf them)]
theorem okOpenTheVaults : Action.check (DRS.ofList []) openTheVaults = [] := by decide

/-- Codecracker Hound -/
def codecrackerHoundLook : Action :=
  sequentially
    [ lookAt (topSlice (.lit 2)), putInto (someOf (exactly 1) them) hand,
      putInto (theOther .object) graveyard ]
theorem okCodecrackerHoundLook : Action.check (DRS.ofList []) codecrackerHoundLook = [] := by decide

def oneOfTheTopTwo : NounPhrase := someOf (exactly 1) (topSlice (.lit 2))
theorem okOneOfTheTopTwo : NounPhrase.checkAt (some .object) (DRS.ofList []) oneOfTheTopTwo = [] := by decide
def oneOfAnUncountedTop : NounPhrase :=
  someOf (exactly 1) (topSlice (countOf isCreature))
theorem okOneOfAnUncountedTop : NounPhrase.checkAt (some .object) (DRS.ofList []) oneOfAnUncountedTop = [] := by
  decide
def theArtifactsAmongTheTopFive : NounPhrase :=
  allFromAmong isArtifact (topSlice (.lit 5))
theorem okTheArtifactsAmongTheTopFive :
    NounPhrase.checkAt (some .object) (DRS.ofList []) theArtifactsAmongTheTopFive = [] := by decide
theorem theOtherAfterATwoCardLook : theOtherOk .object ((stepNounPhrase 512 (DRS.ofList []) .obj true oneOfTheTopTwo).1) = true := by
  decide
theorem theOtherNeedsAStatedCount :
    theOtherOk .object ((stepNounPhrase 512 (DRS.ofList []) .obj true oneOfAnUncountedTop).1) = false := by decide
theorem theRestStandsWhereTheOtherRefuses :
    theRestOk .object ((stepNounPhrase 512 (DRS.ofList []) .obj true oneOfAnUncountedTop).1) = true := by decide
theorem theOtherRefusesTheUniversalSlice :
    (theOtherOk .object ((stepNounPhrase 512 (DRS.ofList []) .obj true theArtifactsAmongTheTopFive).1),
     theRestOk .object ((stepNounPhrase 512 (DRS.ofList []) .obj true theArtifactsAmongTheTopFive).1)) = (false, true) := by
  decide

def eachPlayerShufflesTheirHandAndGraveyard : Action :=
  instruct eachPlayer (shuffleInto
    (both (allOf (cardIn (handOf they))) (allOf (cardIn (graveyardOf they)))))
theorem okEachPlayerShufflesTheirHandAndGraveyard :
    Action.check (DRS.ofList []) eachPlayerShufflesTheirHandAndGraveyard = [] := by decide

/-- Fact or Fiction -/
def factOrFiction : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fact or Fiction", cost := some [generic 3, pip .blue], types := [.instant],
      text :=
        [ spell (sequentially
            [ reveal (topSlice (.lit 5)),
              instruct anOpponent (separateIntoPiles them 2),
              putInto onePile hand,
              putInto (theOther .pile) graveyard ]) ] } }

def unsummon : Action := putInto (target isCreature) hand
theorem okUnsummon : Action.check (DRS.ofList []) unsummon = [] := by decide
def grismold : Action :=
  instruct eachPlayer (createToken (creatureToken 1 1 [.green] [plant]))
theorem okGrismold : Action.check (DRS.ofList []) grismold = [] := by decide
/-- Grenzo, Dungeon Warden -/
def grenzoBottomCard : Action := putInto bottomCard graveyard
theorem okGrenzoBottomCard : Action.check (DRS.ofList []) grenzoBottomCard = [] := by decide

def chronostutter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chronostutter", cost := some [generic 5, pip .blue], types := [.instant],
      text := [ spell (putInto (target isCreature) (fromTop 2 library)) ] } }

theorem nekrataalTwoObjects : countOnes .object nekrataalRider = 2 := by decide
theorem bioplasmTestMintsNothing : countOnes .object bioplasmAfterTest = 2 := by decide

def wholeSliceAtBase : SliceCount := whole
theorem wholeSliceIsPluralAndUncounted :
    (NounPhrase.plur theArtifactsAmongTheTopFive, wholeSliceAtBase.exact) = (.many, none) := by
  decide
theorem describedSliceReadsAsCreature :
    NounPhrase.ty companyContext companyDescribedSlice = [.creature] := by decide
theorem bareSliceReadsUntyped : NounPhrase.ty companyContext companyBareSlice = [] := by decide
theorem describedSliceKeepsGroupZone :
    NounPhrase.zone companyContext companyDescribedSlice
      = NounPhrase.zone companyContext companyBareSlice := by
  decide

/-- Stomp and Howl -/
def stompAndHowl : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stomp and Howl", cost := some [generic 2, pip .green], types := [.sorcery],
      text := [ spell (destroy (both (target isArtifact) (target isEnchantment))) ] } }

/-- Churning Eddy -/
def churningEddy : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Churning Eddy", cost := some [generic 3, pip .blue], types := [.sorcery],
      text := [ spell (returnToHand (both (target isCreature) (target isLand))) ] } }

/-- Secret Rendezvous -/
def secretRendezvous : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Secret Rendezvous", cost := some [generic 1, pip .white, pip .white],
      types := [.sorcery],
      text := [ spell (instruct (eachOf (youAnd (target isOpponent))) (draw (.lit 3))) ]
          } }

/-- Aggressive Instinct -/
def aggressiveInstinct : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aggressive Instinct", cost := some [generic 1, pip .green], types := [.sorcery],
      text :=
        [ spell (dealDamageOwnPower (target (and [isCreature, youControl]))
            (target (and [isCreature, youDontControl]))) ] } }

/-- Arcum Dagsson -/
def arcumDagssonSacrifice : Action :=
  controllerSacrifices (target (and [isArtifact, isCreature]))
theorem okArcumDagssonSacrifice : Action.check (DRS.ofList []) arcumDagssonSacrifice = [] := by decide

/-- Vaevictis Asmadi, the Dire -/
def vaevictisAsmadiTheDire : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vaevictis Asmadi, the Dire",
      cost := some [generic 3, pip .black, pip .red, pip .green],
      supertypes := [.legendary], types := [.creature],
      subtypes := [elder, dragon],
      text :=
        [ flying,
          whenever (does thisCreature attack)
            (sequentially
              [ doForEach eachPlayer
                  (choose (target (and [isPermanent, controlledBy they]))),
                instruct (those player) (sacrifice (those permanent)),
                doForEach
                  (each (and [ isPlayer,
                                did which (sacrifice (a isPermanent)) thisWay
                                  ]))
                  (sequentially
                    [ instruct they (expose .reveal (cards (slice .top (.lit 1) (libraryOf they)))),
                      doIf (is itCard isPermanentCard) (putOntoBattlefield itCard) ]) ]) ],
      power := stat 6, toughness := stat 6 } }

end Semantics.Cards
