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
    [ exile (target (and [isCreature, youControl])),
      returnToBattlefield (that card) [(under you)] ]
theorem okCloudshift : Action.check (DRS.ofList []) cloudshift = [] := by decide

def bitterDownfall : Action :=
  sequentially [destroy (target isCreature), instruct (controllerOf it) (loseLife 2)]
theorem okBitterDownfall : Action.check (DRS.ofList []) bitterDownfall = [] := by decide

def suspendedSentence : Action :=
  sequentially
    [ destroy (target (and [isCreature, anOpponentControls])),
      instruct (that player) (loseLife 3) ]
theorem okSuspendedSentence : Action.check (DRS.ofList []) suspendedSentence = [] := by decide

def flickeringSpirit : Action :=
  sequentially [exile thisCreature, returnToBattlefield it [(under (ownerOf it))]]
theorem okFlickeringSpirit : Action.check (DRS.ofList []) flickeringSpirit = [] := by decide

/-- Bench item: "Mill a card. Return the card put into your graveyard this way to your hand." One
milled card is read back as the one card moved [CR#701.17a]. -/
def millACardReturnIt : Action :=
  sequentially
    [ mill (.lit 1),
      returnToHand (the (and [isCard, was (putInto which yourGraveyard) thisWay])) ]
theorem okMillACardReturnIt : Action.check (DRS.ofList []) millACardReturnIt = [] := by decide

/-- Bond of Revival -/
def bondOfRevival : Action :=
  sequentially
    [ returnToBattlefield (target (and [isCreature, in_ yourGraveyard]))
        (from_ := some yourGraveyard),
      gainHaste it untilYourNextTurn ]
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
        (forEach 1 (and [isAttacking, isCreature, youControl]))),
      gainLife thatMuch ]
theorem okFoulTongueShriek : Action.check (DRS.ofList []) foulTongueShriek = [] := by decide

def phyrexianInfiltrator : Action := exchangeControl thisCreature (target isCreature)
theorem okPhyrexianInfiltrator : Action.check (DRS.ofList []) phyrexianInfiltrator = [] := by decide

def impulse : Action :=
  sequentially
    [ lookAt (topSlice 4), putInto (someOf (exactly 1) them) hand,
      putInto (theRest .object) onBottom (riders := [(inOrder .anyOrder)]) ]
theorem okImpulse : Action.check (DRS.ofList []) impulse = [] := by decide

def anticipate : Action :=
  sequentially
    [ lookAt (topSlice 3), putInto (someOf (exactly 1) them) hand,
      putInto (theRest .object) onBottom (riders := [(inOrder .anyOrder)]) ]
theorem okAnticipate : Action.check (DRS.ofList []) anticipate = [] := by decide

/-- Flow State: "Look at the top three cards of your library. Put one of them into your hand and
the rest on the bottom of your library in any order. If there is an instant card and a sorcery
card in your graveyard, instead put two of them into your hand and the rest on the bottom of
your library in any order." A self-replacement [CR#614.15]: the replacement reads what the
replaced action read, the cards looked at, not the part and remainder it would have made. The
domain its existential condition publishes ranks below those cards (owner ruling 2026-10-22,
batch 4), so "them" and "the rest" read the cards looked at. -/
def flowState : Spelled := spelled <| singleFacedCard
  { name := "Flow State", cost := [generic 1, pip .blue], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ lookAt (topSlice 3),
            replace
              (sequentially
                [ putInto (someOf (exactly 1) them) hand,
                  putInto (theRest .object) onBottom (riders := [(inOrder .anyOrder)]) ])
              (doIf (thereAre (both (a (and [isInstant, cardIn yourGraveyard]))
                                    (a (and [isSorcery, cardIn yourGraveyard]))))
                (instead (sequentially
                  [ putInto (someOf (exactly 2) them) hand,
                    putInto (theRest .object) onBottom (riders := [(inOrder .anyOrder)]) ]))) ]) ] }
theorem okFlowState : flowState.card.check = [] := flowState.ok

/-- In *Flow State*'s replacement, "them" reads the three cards looked at (referent 1, in the
library), not the instant and sorcery cards the existential published nor the part and remainder
of the replaced action, and they are the one group "the rest" is taken from. -/
theorem flowStateThemAndTheRestReadTheCardsLookedAt :
    let looked := (stepAction 512 .postEvent .obj (DRS.ofList []) (lookAt (topSlice 3))).1
    let replaced := (stepAction 512 .preEvent .obj looked (sequentially
      [ putInto (someOf (exactly 1) them) hand,
        putInto (theRest .object) onBottom (riders := [(inOrder .anyOrder)]) ])).1
    let instantAndSorcery := thereAre (both (a (and [isInstant, cardIn yourGraveyard]))
      (a (and [isSorcery, cardIn yourGraveyard])))
    let start := nextClause (replacementStart looked replaced)
    let after :=
      nextClause (markReplacementDomain start (stepCondition 512 start instantAndSorcery).1)
    (resolveCandidates .bare .many after).map (fun ref => (ref.id, ref.zone)) =
        [(1, some .library)] ∧
      countGroups .object after = 1 ∧ countedGroupSize .object after = some 3 := by
  decide

/-- "Look at the top three cards of your library. Put one of them into your hand and the rest on
the bottom of your library in any order. Instead, put two of them into your hand and the rest on
the bottom of your library in any order": the replacement reads what the replaced action read
[CR#614.15]. It is not refused `theRestFits`, and its "them" reads the cards looked at (referent
1), not the replaced action's remainder. -/
theorem selfReplacementReadsWhatTheReplacedActionRead :
    let putOne := sequentially
      [ putInto (someOf (exactly 1) them) hand,
        putInto (theRest .object) onBottom (riders := [(inOrder .anyOrder)]) ]
    let putTwo := sequentially
      [ putInto (someOf (exactly 2) them) hand,
        putInto (theRest .object) onBottom (riders := [(inOrder .anyOrder)]) ]
    let looked := (stepAction 512 .postEvent .obj (DRS.ofList []) (lookAt (topSlice 3))).1
    let replaced := (stepAction 512 .preEvent .obj looked putOne).1
    let start := nextClause (replacementStart looked replaced)
    Action.check (DRS.ofList [])
        (sequentially [lookAt (topSlice 3), replace putOne (instead putTwo)]) = [] ∧
      (resolveCandidates .bare .many start).map (·.id) = [1] ∧
      countGroups .object start = 1 ∧ countParts .object start = 0 := by
  decide

def revealFourPartition : Action :=
  sequentially
    [ reveal (topSlice 4), putInto (someOf (exactly 1) (those card)) hand,
      putInto (theRest .object) graveyard ]
theorem okRevealFourPartition : Action.check (DRS.ofList []) revealFourPartition = [] := by decide

def exileFourOfThem : Action :=
  sequentially
    [ lookAt (topSlice 8), exile (someOf (exactly 4) them),
      putInto (theRest .object) onTop (riders := [(inOrder .anyOrder)]) ]
theorem okExileFourOfThem : Action.check (DRS.ofList []) exileFourOfThem = [] := by decide

def sylvanScrying : Action :=
  sequentially [search yourLibrary (a isLand), reveal it, putInto it hand, shuffle]
theorem okSylvanScrying : Action.check (DRS.ofList []) sylvanScrying = [] := by decide

def searchToBattlefield : Action :=
  sequentially [search yourLibrary (a isCreature), putOntoBattlefield it, shuffle]
theorem okSearchToBattlefield : Action.check (DRS.ofList []) searchToBattlefield = [] := by decide

def glimpseTheUnthinkable : Action := instruct (target isPlayer) (mill 10)
theorem okGlimpseTheUnthinkable : Action.check (DRS.ofList []) glimpseTheUnthinkable = [] := by decide

def millThenReadGroup : Action :=
  sequentially [mill 3, exile (those card)]
theorem okMillThenReadGroup : Action.check (DRS.ofList []) millThenReadGroup = [] := by decide

def takeIntoCustody : Action :=
  sequentially [tap (target isCreature), skipUntap it 1]
theorem okTakeIntoCustody : Action.check (DRS.ofList []) takeIntoCustody = [] := by decide

/-- Frenzied Gorespawn -/
def frenziedGorespawnGoad : Action :=
  doForEach eachOpponent
    (gainDesignation
      (target (and [isCreature, controlledBy (that player)])) goaded)
theorem okFrenziedGorespawnGoad : Action.check (DRS.ofList []) frenziedGorespawnGoad = [] := by decide

def spaceTimeAnomaly : Spelled := spelled <| singleFacedCard
  { name := "Space-Time Anomaly", cost := [generic 2, pip .white, pip .blue],
    types := [.sorcery],
    text := [ spell (instruct (target isPlayer) (mill (lifeTotalOf you))) ] }

/-- Consecrate // Consume -/
def consume : Ability :=
  spell (sequentially
    [ instruct (target isPlayer) (sacrifice
        (a (and [ isCreature,
                   superlative .max (.stat .power)
                     (and [isCreature, controlledBy they]) ]))),
      gainLife (powerOf it) ])
theorem okConsume : Ability.check (DRS.ofList []) consume = [] := by decide

def bifurcate : Spelled := spelled <| singleFacedCard
  { name := "Bifurcate", cost := [generic 3, pip .green], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ search yourLibrary
              (a (and [isPermanentCard, (shares .cardName .share (target (and [isCreature, isNontoken])))])),
            putOntoBattlefield (that card),
            shuffle ]) ] }

/-- Mana Leak {1}{U} — Instant. "Counter target spell unless its controller pays {3}." -/
def manaLeak : Spelled := spelled <| singleFacedCard
  { name := "Mana Leak", cost := [generic 1, pip .blue], types := [.instant],
    text :=
      [ spell (instruct (controllerOf (target isSpell)) (doUnless
          (instruct you (counter it))
          [generic 3])) ] }

/-- Oust {W} — Sorcery. "Put target creature into its owner's library second from the top.
Its controller gains 3 life." -/
def oust : Spelled := spelled <| singleFacedCard
  { name := "Oust", cost := [pip .white], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ putInto (target isCreature) (fromTop 2 library),
            instruct (controllerOf it) (gainLife 3) ]) ] }

def riseFromTheGrave : Action :=
  sequentially
    [ putOntoBattlefieldUnderYourControl (target (and [isCreature, in_ graveyard]))
        (from_ := some graveyard),
      become (that creature)
        { characteristics := { colors := [.black], subtypes := [zombie] } } ]
theorem okRiseFromTheGrave : Action.check (DRS.ofList []) riseFromTheGrave = [] := by decide

def everAfter : Action :=
  sequentially
    [ returnToBattlefield (targets (upTo 2) (and [isCreature, in_ yourGraveyard]))
        (from_ := some yourGraveyard),
      become (those creature)
        { characteristics := { colors := [.black], subtypes := [zombie] } },
      putInto this_ onBottom ]
theorem okEverAfter : Action.check (DRS.ofList []) everAfter = [] := by decide

def martyrsCry : Action :=
  sequentially
    [ exile (allOf (and [isCreature, hasColor .white])),
      doForEach (each (and [isCreature, was (exile which) thisWay]))
        (instruct (controllerOf it) (draw 1)) ]
theorem okMartyrsCry : Action.check (DRS.ofList []) martyrsCry = [] := by decide

def anotherRound : Action :=
  sequentially
    [ exile (counted anyNumber (and [isCreature, youControl])),
      returnToBattlefield them [(under (ownerOf them))],
      repeat_ (moreTimes (letter .x)) ]
theorem okAnotherRound : Action.check (DRS.ofList []) anotherRound = [] := by decide

/-- Eradicate -/
def eradicateSearch : Action :=
  sequentially
    [ exile (target (and [isCreature, non (hasColor .black)])),
      searchZonesOf (controllerOf (that card)) (allOf (shares .cardName .share (that card))),
      exile them,
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
    [ lookAt (topSlice 2),
      manifestPlacement (someOf (exactly 1) them),
      putInto (theOther .object) (or [onTop, onBottom]) ]
theorem okWriteIntoBeingPlacement : Action.check (DRS.ofList []) writeIntoBeingPlacement = [] := by decide

/-- Mystical Tutor -/
def mysticalTutor : Action :=
  sequentially
    [search yourLibrary (a isInstantOrSorcery), reveal it, shuffle, putInto it onTop]
theorem okMysticalTutor : Action.check (DRS.ofList []) mysticalTutor = [] := by decide

/-- Demonic Tutor -/
def demonicTutor : Action :=
  sequentially [search yourLibrary (a isCard), putInto it hand, shuffle]
theorem okDemonicTutor : Action.check (DRS.ofList []) demonicTutor = [] := by decide

/-- Thalia's Lancers -/
def thaliasLancersSearch : Action :=
  may
    (sequentially
      [ search yourLibrary (a (and [hasSupertype .legendary])), reveal it,
        putInto it hand, shuffle ])
theorem okThaliasLancersSearch : Action.check (DRS.ofList []) thaliasLancersSearch = [] := by decide

def contrabandLivestock : Action :=
  sequentially
    [ exile (target isCreature),
      rollDice 1 (.sides 20),
      applyResultsTable
        [ rollRow (oneThrough 9)
            (instruct (controllerOf it) (createToken (creatureToken 4 4 [.green] [ox]))),
          rollRow (fromTo 10 19)
            (instruct (controllerOf it) (createToken (creatureToken 2 2 [.green] [boar]))),
          rollRow (exactly 20)
            (instruct (controllerOf it) (createToken (creatureToken 0 1 [.white] [goat]))) ] ]
theorem okContrabandLivestock : Action.check (DRS.ofList []) contrabandLivestock = [] := by decide

/-- Hypnotic Specter -/
def hypnoticSpecterDiscard : Action := instruct they (discard (aAtRandom (cardIn hand)))
theorem okHypnoticSpecterDiscard :
    Action.check (DRS.ofList [{ det := .the, plur := .one, payload := .player false }]) hypnoticSpecterDiscard = [] := by decide

/-- Wyll, Blade of Frontiers -/
def wyllExtraDie : Action :=
  replaceEvent (would you (rollDice anyAmount .anyDie))
    (instead (sequentially
      [ rollDice (plus thatMuch 1) .thoseDice .anyResult,
        ignoreOutcomes (extreme .lowest) ]))
    none
theorem okWyllExtraDie : Action.check (DRS.ofList []) wyllExtraDie = [] := by decide

/-- Vedalken Squirrel-Whacker -/
def vedalkenSquirrelWhackerReroll : Action :=
  replaceEvent (would you (rollDice anyAmount (.sides 6)))
    (instead (sequentially
      [ rollDice thatMuch .thoseDice .anyResult,
        may
          (chooseOne
            (exchange (values (theOutcome .rollResult) (basePowerOf thisCreature)))
            (exchange (values (theOutcome .rollResult) (baseToughnessOf thisCreature)))) ]))
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
def ertaisTrickery : Spelled := spelled <| singleFacedCard
  { name := "Ertai's Trickery", cost := [pip .blue], types := [.instant],
    text :=
      [ spell (doOnlyIf (counter (target isSpell))
          (keywordCostPaid kicker it)) ] }

/-- Celebrate the Harvest -/
def celebrateTheHarvest : Spelled := spelled <| singleFacedCard
  { name := "Celebrate the Harvest", cost := [generic 3, pip .green], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ search yourLibrary (counted (upToOf (letter .x)) (and [isLand, hasSupertype .basic])),
            define .x (distinctCount (.value .power) (allOf (and [isCreature, youControl]))),
            putOntoBattlefieldTapped (those card),
            shuffle ]) ] }

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
/-- After the destroy, no creature word reaches a creature on the battlefield: the destroyed
creature is in a graveyard, and the Source, "this creature", is a self-referent no noun word
reads (`Determiner.self`; owner ruling 2026-10-24), which this pin used to count. -/
theorem nekrataalNoCreatureWord : countReach (.word creature) .one nekrataalRider = 0 := by
  decide
theorem nekrataalOneDestroyed :
    (participialCandidates (.deed (.action "destroy")) (some .one) none false nekrataalRider).length = 1 := by
  decide

def sequencedRiderBody : Action :=
  sequentially [exile (target isArtifact), destroy (target isCreature)]
theorem okSequencedRiderBody : Action.check (DRS.ofList []) sequencedRiderBody = [] := by decide
def sequencedRider : DRS := (stepAction 512 .postEvent .obj (DRS.ofList []) sequencedRiderBody).1
theorem sequencedRiderOneDestroyed :
    (participialCandidates (.deed (.action "destroy")) (some .one) none false sequencedRider).length = 1 := by decide
theorem sequencedRiderOneExiled :
    (participialCandidates (.deed (.action "exile")) (some .one) none false sequencedRider).length = 1 := by
  decide

/-- Engulfing Flames -/
def engulfingFlamesBody : Action := instruct this_ (dealDamage 1 (target isCreature))
theorem okEngulfingFlamesBody : Action.check (DRS.ofList []) engulfingFlamesBody = [] := by decide
def engulfingFlamesRider : DRS := (stepAction 512 .postEvent .obj (DRS.ofList []) engulfingFlamesBody).1
theorem engulfingFlamesNoDestroyStamp :
    (participialCandidates (.deed (.action "destroy")) (some .one) none false engulfingFlamesRider).length = 0 := by decide
theorem engulfingFlamesBareReadStands : countReach .bare .one engulfingFlamesRider = 1 := by decide

def bioplasmExiledPronoun : NounPhrase := it
theorem okBioplasmExiledPronoun :
    NounPhrase.checkAt (some .object) bioplasmAfterExile bioplasmExiledPronoun = [] := by decide
theorem bioplasmNoTypedRead :
    (participialCandidates (.deed (.action "exile")) (some .one) creature false bioplasmAfterExile).length
      = 0 := by
  decide
theorem bioplasmExiledCardHasNoType :
    (participialCandidates (.deed (.action "exile")) (some .one) none false bioplasmAfterExile).map (·.ty)
      = [[]] := by decide
def bioplasmAfterTest : DRS := (stepCondition 512 (nextClause bioplasmAfterExile) bioplasmCardTest).1
theorem bioplasmTypeTestPreservesExiledIdentity :
    let before := participialCandidates (.deed (.action "exile")) (some .one) none false bioplasmAfterExile
    let after := participialCandidates (.deed (.action "exile")) (some .one) none false bioplasmAfterTest
    before.length = 1 ∧ after.map (·.id) = before.map (·.id) := by decide
theorem bioplasmTestRemarksType :
    (participialCandidates (.deed (.action "exile")) (some .one) none false bioplasmAfterTest).map (·.ty)
      = [[.creature]] := by decide
theorem bioplasmTestKeepsCardSlot : countReach (.atSlot .card) .one bioplasmAfterTest = 1 := by
  decide
theorem bioplasmTypedReadStillRefused :
    (participialCandidates (.deed (.action "exile")) (some .one) creature false bioplasmAfterTest).length
      = 0 := by
  decide
theorem bioplasmTypedCardReadWrites :
    (participialCandidates (.deed (.action "exile")) (some .one) (some (.ofType .card .creature)) false
      bioplasmAfterTest).length = 1 := by
  decide

/-- Scapeshift -/
def scapeshiftSacrifice : Action := sacrifice (counted anyNumber isLand)
theorem okScapeshiftSacrifice : Action.check (DRS.ofList []) scapeshiftSacrifice = [] := by decide
def scapeshiftSacrificed : DRS := (stepAction 512 .postEvent .obj (DRS.ofList []) scapeshiftSacrifice).1
def scapeshiftSearch : Action := search yourLibrary (counted (upToOf groupSize) isLand)
theorem okScapeshiftSearch : Action.check scapeshiftSacrificed scapeshiftSearch = [] := by
  decide
def scapeshiftAfterSearch : DRS := (stepAction 512 .postEvent .obj scapeshiftSacrificed scapeshiftSearch).1
theorem scapeshiftTwoGroups : countReach .bare .many scapeshiftAfterSearch = 2 := by decide
theorem scapeshiftOneSearchedGroup :
    (participialCandidates (.deed search) (some .many) none false scapeshiftAfterSearch).length = 1 := by decide
theorem scapeshiftOneSacrificedGroup :
    (participialCandidates (.deed sacrifice) (some .many) none false scapeshiftAfterSearch).length = 1 := by decide

/-- Scapeshift -/
def scapeshift : Spelled := spelled <| singleFacedCard
  { name := "Scapeshift", cost := [generic 2, pip .green, pip .green], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ sacrifice (counted anyNumber isLand),
            search yourLibrary (counted (upToOf groupSize) isLand),
            putOntoBattlefieldTapped them,
            shuffle ]) ] }

/-- Bind to Life, Vastlands Scavenger's adventure -/
def bindToLife : Action :=
  sequentially
    [mill 7, putOntoBattlefield (fromAmong (exactly 1) isCreature them)]
theorem okBindToLife : Action.check (DRS.ofList []) bindToLife = [] := by decide

/-- Glamdring, Foe-hammer's Gleam of Death -/
def gleamOfDeath : Action :=
  sequentially
    [mill 6, putInto (allFromAmong isInstantOrSorcery them) hand]
theorem okGleamOfDeath : Action.check (DRS.ofList []) gleamOfDeath = [] := by decide

/-- Tezzeret, Master of the Bridge -/
def tezzeretAllArtifacts : Action :=
  sequentially
    [ exile (topSlice 10),
      putOntoBattlefield (allFromAmong isArtifact them) ]
theorem okTezzeretAllArtifacts : Action.check (DRS.ofList []) tezzeretAllArtifacts = [] := by decide

def companyContext : DRS := (stepAction 512 .postEvent .obj (DRS.ofList []) (lookAt (topSlice 6))).1
def companyDescribedSlice : NounPhrase :=
  fromAmong (upTo 2) (and [isCreature, hasManaValue .atMost 3]) them
theorem okCompanyDescribedSlice :
    NounPhrase.checkAt (some .object) companyContext companyDescribedSlice = [] := by decide
def companyBareSlice : NounPhrase := someOf (exactly 2) them
theorem okCompanyBareSlice : NounPhrase.checkAt (some .object) companyContext companyBareSlice =
    [] := by
  decide

/-- Lord of the Void -/
def exileTopThenPutFromAmong : Action :=
  sequentially
    [ exile (slice .top 7 (libraryOf (target isOpponent))),
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
/-- Windfall: "Each player discards their hand, then draws cards equal to the greatest number of
cards a player discarded this way." -/
def windfallDiscardAndDraw : Action :=
  instruct eachPlayer
    (sequentially [discard (allOf (cardIn hand)), draw greatestCardsAPlayerDiscardedThisWay])
theorem okGreatestCardsAPlayerDiscardedThisWay :
    Action.check (DRS.ofList []) windfallDiscardAndDraw = [] := by decide
/-- Alone, "this way" reads no discard [CR#608.2c]. -/
theorem badGreatestCardsAPlayerDiscardedThisWayAlone :
    Amount.check (DRS.ofList []) greatestCardsAPlayerDiscardedThisWay =
      [.thisWayUngrounded] := by decide

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
  sequentially [instruct (controllerOf thisAura) (sacrifice itPermanent), instruct they (draw 2)]
theorem okSoulRansomRansom : Action.check (DRS.ofList []) soulRansomRansom = [] := by decide
def thisAurasController : NounPhrase := controllerOf thisAura
theorem okThisAurasController : NounPhrase.checkAt (some .player) (DRS.ofList []) thisAurasController = [] := by
  decide
def targetCreaturesController : NounPhrase := controllerOf (target isCreature)
theorem okTargetCreaturesController :
    NounPhrase.checkAt (some .player) (DRS.ofList []) targetCreaturesController = [] := by decide
theorem possessiveDeicticIsReadableByIt :
    countReach (.atSlot .permanent) .one ((stepNounPhrase 512 (DRS.ofList []) .obj true thisAurasController).1) = 1 := by decide
/-- "this Aura's controller": the possessor is the Source, a self-referent no demonstrative
reads ("that enchantment" finds nothing; `Determiner.self`, owner ruling 2026-10-24), while a
later "this Aura" re-mentions it, on the battlefield as an enchantment (`mentionThis`). -/
theorem possessiveDeicticSourceIsNoDemonstrativeAntecedent :
    let result := stepNounPhrase 512 (DRS.ofList []) .obj true thisAurasController
    let again := stepNounPhrase 512 result.1 .obj true thisAura
    result.2.1 = [1] ∧ resolveCandidates (.word enchantment) .one result.1 = [] ∧
      again.2.1 = [0] ∧
      (again.2.1.filterMap again.1.findById?).all
        (fun ref => ref.zone == some .battlefield && ref.ty == [.enchantment]) := by decide

theorem possessiveDescribedBaseUnchanged :
    countReach (.atSlot .permanent) .one ((stepNounPhrase 512 (DRS.ofList []) .obj true targetCreaturesController).1) = 1 := by decide

/-- Bile Blight -/
def bileBlight : Spelled := spelled <| singleFacedCard
  { name := "Bile Blight", cost := [pip .black, pip .black], types := [.instant],
    text :=
      [ spell (get
          (both (target isCreature)
            (allOf (and [ isCreature, (shares .cardName .share (that creature)),
                           isOther ])))
          (down 3) (down 3) untilEndOfTurn) ] }

/-- Echoing Ruin -/
def echoingRuin : Spelled := spelled <| singleFacedCard
  { name := "Echoing Ruin", cost := [generic 1, pip .red], types := [.sorcery],
    text :=
      [ spell (destroy
          (both (target isArtifact)
            (allOf (and [ isArtifact, (shares .cardName .share (that artifact)),
                           isOther ])))) ] }

/-- Hijack -/
def hijack : Spelled := spelled <| singleFacedCard
  { name := "Hijack", cost := [generic 1, pip .red, pip .red], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ gainControl (target (or [isArtifact, isCreature])) untilEndOfTurn (controller := you),
            untap it,
            gainHaste it untilEndOfTurn ]) ] }

/-- Zidane, Tantalus Thief. "Gain control of" is the imperative `gainControl`, a Continuous Effect
[CR#611.2a,613.1b]; "an opponent gains control of a permanent from you" names the control change
as a trigger event, `gainControlOf`, with the player it was taken from [CR#603.10d]. -/
def zidaneTantalusThief : Spelled := spelled <| singleFacedCard
  { name := "Zidane, Tantalus Thief", cost := [generic 3, pip .red, pip .white],
    supertypes := [.legendary], types := [.creature], subtypes := [human, mutant, scout],
    text :=
      [ when (does thisCreature enter)
          (sequentially
            [ gainControl (target (and [isCreature, anOpponentControls])) untilEndOfTurn
                (controller := you),
              untap it,
              establishFor it [abilityGrant it lifelink, abilityGrant it haste]
                untilEndOfTurn ]),
        whenever (does anOpponent (gainControlOf (a isPermanent) you))
          (createToken treasureToken) ],
    power := stat 3, toughness := stat 3 }
theorem okZidaneTantalusThief : zidaneTantalusThief.card.check = [] := zidaneTantalusThief.ok

/-- Open the Vaults -/
def openTheVaults : Action :=
  returnToBattlefield
    (allOf (and [ isCard, or [isArtifact, isEnchantment],
                   in_ (graveyardOf (bare isPlayer)) ]))
    [under (ownerOf them)] (from_ := some (graveyardOf (bare isPlayer)))
theorem okOpenTheVaults : Action.check (DRS.ofList []) openTheVaults = [] := by decide

/-- Codecracker Hound -/
def codecrackerHoundLook : Action :=
  sequentially
    [ lookAt (topSlice 2), putInto (someOf (exactly 1) them) hand,
      putInto (theOther .object) graveyard ]
theorem okCodecrackerHoundLook : Action.check (DRS.ofList []) codecrackerHoundLook = [] := by decide

def oneOfTheTopTwo : NounPhrase := someOf (exactly 1) (topSlice 2)
theorem okOneOfTheTopTwo : NounPhrase.checkAt (some .object) (DRS.ofList []) oneOfTheTopTwo = [] := by decide
def oneOfAnUncountedTop : NounPhrase :=
  someOf (exactly 1) (topSlice (countOf isCreature))
theorem okOneOfAnUncountedTop : NounPhrase.checkAt (some .object) (DRS.ofList []) oneOfAnUncountedTop = [] := by
  decide
def theArtifactsAmongTheTopFive : NounPhrase :=
  allFromAmong isArtifact (topSlice 5)
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
def factOrFiction : Spelled := spelled <| singleFacedCard
  { name := "Fact or Fiction", cost := [generic 3, pip .blue], types := [.instant],
    text :=
      [ spell (sequentially
          [ reveal (topSlice 5),
            instruct anOpponent (separateIntoPiles them 2),
            putInto onePile hand,
            putInto (theOther .pile) graveyard ]) ] }

def unsummon : Action := returnToHand (target isCreature)
theorem okUnsummon : Action.check (DRS.ofList []) unsummon = [] := by decide
def grismold : Action :=
  instruct eachPlayer (createToken (creatureToken 1 1 [.green] [plant]))
theorem okGrismold : Action.check (DRS.ofList []) grismold = [] := by decide
/-- Grenzo, Dungeon Warden -/
def grenzoBottomCard : Action := putInto bottomCard graveyard
theorem okGrenzoBottomCard : Action.check (DRS.ofList []) grenzoBottomCard = [] := by decide

def chronostutter : Spelled := spelled <| singleFacedCard
  { name := "Chronostutter", cost := [generic 5, pip .blue], types := [.instant],
    text := [ spell (putInto (target isCreature) (fromTop 2 library)) ] }

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
def stompAndHowl : Spelled := spelled <| singleFacedCard
  { name := "Stomp and Howl", cost := [generic 2, pip .green], types := [.sorcery],
    text := [ spell (destroy (both (target isArtifact) (target isEnchantment))) ] }

/-- Churning Eddy -/
def churningEddy : Spelled := spelled <| singleFacedCard
  { name := "Churning Eddy", cost := [generic 3, pip .blue], types := [.sorcery],
    text := [ spell (returnToHand (both (target isCreature) (target isLand))) ] }

/-- Secret Rendezvous -/
def secretRendezvous : Spelled := spelled <| singleFacedCard
  { name := "Secret Rendezvous", cost := [generic 1, pip .white, pip .white],
    types := [.sorcery],
    text := [ spell (instruct (eachOf (youAnd (target isOpponent))) (draw 3)) ]
        }

/-- Aggressive Instinct -/
def aggressiveInstinct : Spelled := spelled <| singleFacedCard
  { name := "Aggressive Instinct", cost := [generic 1, pip .green], types := [.sorcery],
    text :=
      [ spell (dealDamageOwnPower (target (and [isCreature, youControl]))
          (target (and [isCreature, youDontControl]))) ] }

/-- Arcum Dagsson -/
def arcumDagssonSacrifice : Action :=
  controllerSacrifices (target (and [isArtifact, isCreature]))
theorem okArcumDagssonSacrifice : Action.check (DRS.ofList []) arcumDagssonSacrifice = [] := by decide

/-- Vaevictis Asmadi, the Dire -/
def vaevictisAsmadiTheDire : Spelled := spelled <| singleFacedCard
  { name := "Vaevictis Asmadi, the Dire",
    cost := [generic 3, pip .black, pip .red, pip .green],
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
                  [ instruct they (reveal (slice .top 1 (libraryOf they))),
                    doIf (is itCard isPermanentCard) (putOntoBattlefield itCard) ]) ]) ],
    power := stat 6, toughness := stat 6 }

/-- Kalitas, Bloodchief of Ghet: "{B}{B}{B}, {T}: Destroy target creature. If that creature dies
this way, create a black Vampire creature token. Its power is equal to that creature's power and
its toughness is equal to that creature's toughness." "That creature" in the lookback is the
destroyed creature, read as the permanent it was. -/
def kalitasBloodchiefOfGhet : Spelled := spelled <| singleFacedCard
  { name := "Kalitas, Bloodchief of Ghet", cost := [generic 5, pip .black, pip .black],
    supertypes := [.legendary], types := [.creature], subtypes := [vampire, warrior],
    text :=
      [ lifelink,
        activated (compound [[pip .black, pip .black, pip .black], tapSymbol])
          (sequentially
            [ destroy (target isCreature),
              doIf (did (that creature) die thisWay)
                (createToken (creatureTokenOf (powerOf (that creature))
                  (toughnessOf (that creature)) [.black] [vampire])) ]) ],
    power := stat 5, toughness := stat 5 }

/-- Kaervek's Purge: "Destroy target creature with mana value X. If that creature dies this way,
Kaervek's Purge deals damage equal to the creature's power to the creature's controller." -/
def kaerveksPurge : Spelled := spelled <| singleFacedCard
  { name := "Kaervek's Purge", cost := [.variable, pip .black, pip .red], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ destroy (target (and [isCreature, hasManaValue .eq (letter .x)])),
            doIf (did (that creature) die thisWay)
              (instruct this_
                (dealDamage (powerOf (that creature)) (controllerOf (that creature)))) ]) ] }


/-- Saw in Half: "Destroy target creature. If that creature dies this way, its controller creates
two tokens that are copies of that creature, except their power is half that creature's power and
their toughness is half that creature's toughness. Round up each time." The possessor "its" and
every later "that creature" read the destroyed creature: step `C` of `AC>R1` only prefers a
referent no same-clause pronoun took, and here no other creature remains. -/
def sawInHalfBody : Action :=
  sequentially
    [ destroy (target isCreature),
      doIf (did (that creature) die thisWay)
        (instruct (controllerOf it)
          (create 2
            (copyOf (that creature)
              (copyCharacteristics
                { power := some (half .up (powerOf (that creature))),
                  toughness := some (half .up (toughnessOf (that creature))) } false))
            [])) ]
def sawInHalf : Spelled := spelled <| singleFacedCard
  { name := "Saw in Half", cost := [generic 2, pip .black], types := [.instant],
    text := [spell sawInHalfBody] }
/-- The card checks clean, and the destroyed creature (`1`) is the only referent any anaphor
resolved: "its" and all three "that creature" mentions read it. -/
theorem okSawInHalf :
    Action.check (DRS.ofList []) sawInHalfBody = [] ∧
      (stepAction 64 .postEvent .obj (DRS.ofList []) sawInHalfBody).1.resolvedIds = [1] := by
  decide

/-- Tuktuk Scrapper: "Whenever this creature or another Ally you control enters, you may destroy
target artifact. If that artifact is put into a graveyard this way, this creature deals damage to
that artifact's controller equal to the number of Allies you control." The target is chosen as the
ability is put on the stack, whether or not its controller destroys it [CR#601.2c,603.3d,115.1]:
"that artifact" reads it after the optional body, in the lookback and as the damage's possessor
(owner ruling 2026-10-23). -/
def tuktukScrapper : Spelled := spelled <| singleFacedCard
  { name := "Tuktuk Scrapper", cost := [generic 3, pip .red], types := [.creature],
    subtypes := [goblin, artificer, ally],
    text :=
      [ whenever
          (does (eitherOf thisCreature (a (and [ally, youControl, isOther]))) enter)
          (sequentially
            [ may (destroy (target isArtifact)),
              doIf (did (that artifact) (putInto theActor graveyard) thisWay)
                (instruct thisCreature
                  (dealDamage (countOf (and [ally, youControl]))
                    (controllerOf (that artifact)))) ]) ],
    power := stat 2, toughness := stat 2 }
theorem okTuktukScrapper : tuktukScrapper.card.check = [] := tuktukScrapper.ok

/-- Insatiable Appetite: "You may sacrifice a Food. If you do, target creature gets +5/+5 until end
of turn. Otherwise, that creature gets +3/+3 until end of turn." Both arms' targets are chosen as
the spell is cast, whichever arm later runs [CR#601.2c,115.1], so "Otherwise, that creature" reads
the target the "if you do" arm announces. -/
def insatiableAppetite : Spelled := spelled <| singleFacedCard
  { name := "Insatiable Appetite", cost := [generic 1, pip .green], types := [.instant],
    text :=
      [ spell (may (sacrifice (a food))
          (some (get (target isCreature) (up 5) (up 5) untilEndOfTurn))
          (some (get (that creature) (up 3) (up 3) untilEndOfTurn))) ] }
theorem okInsatiableAppetite : insatiableAppetite.card.check = [] := insatiableAppetite.ok

/-- Pippin's Bravery: "You may sacrifice a Food. If you do, target creature gets +4/+4 until end of
turn. Otherwise, that creature gets +2/+2 until end of turn." The same shape as Insatiable
Appetite. -/
def pippinsBravery : Spelled := spelled <| singleFacedCard
  { name := "Pippin's Bravery", cost := [pip .green], types := [.instant],
    text :=
      [ spell (may (sacrifice (a food))
          (some (get (target isCreature) (up 4) (up 4) untilEndOfTurn))
          (some (get (that creature) (up 2) (up 2) untilEndOfTurn))) ] }
theorem okPippinsBravery : pippinsBravery.card.check = [] := pippinsBravery.ok

/-! ## A spell's later spell abilities read its targets

Each paragraph of a spell's text is its own spell ability [CR#113.2c], but the spell's targets are
announced for the spell as a whole [CR#601.2c] and its instructions are followed in the order
written [CR#608.2c]: a later spell ability reads the targets an earlier one announced as it reads
any earlier mention (`Ability.textContextWith`). -/

/-- Dispatch: "Tap target creature. / Metalcraft — If you control three or more artifacts, exile
that creature." -/
def dispatch : Spelled := spelled <| singleFacedCard
  { name := "Dispatch", cost := [pip .white], types := [.instant],
    text :=
      [ spell (tap (target isCreature)),
        metalcraft (spell (doIf (compareAmt (countOf (and [isArtifact, youControl])) .atLeast 3)
          (exile (that creature)))) ] }
theorem okDispatch : dispatch.card.check = [] := dispatch.ok

/-- Efflorescence: "Put two +1/+1 counters on target creature. / Infusion — If you gained life
this turn, that creature also gains trample and indestructible until end of turn." -/
def efflorescence : Spelled := spelled <| singleFacedCard
  { name := "Efflorescence", cost := [generic 2, pip .green], types := [.instant],
    text :=
      [ spell (putCounters 2 p1p1Counter (target isCreature)),
        infusion (spell (doIf (did you (gainLife anyAmount) soFarThisTurn)
          (establishFor (that creature) [abilityGrant it trample, abilityGrant it indestructible]
            untilEndOfTurn))) ] }
theorem okEfflorescence : efflorescence.card.check = [] := efflorescence.ok

/-- Foolish Fate: "Destroy target creature. / Infusion — If you gained life this turn, that
creature's controller loses 3 life." "That creature" reads the target as the first ability left
it. -/
def foolishFate : Spelled := spelled <| singleFacedCard
  { name := "Foolish Fate", cost := [generic 2, pip .black], types := [.instant],
    text :=
      [ spell (destroy (target isCreature)),
        infusion (spell (doIf (did you (gainLife anyAmount) soFarThisTurn)
          (instruct (controllerOf (that creature)) (loseLife 3)))) ] }
theorem okFoolishFate : foolishFate.card.check = [] := foolishFate.ok

/-- The probes that found the defect: a later spell ability's "that creature", and its bare "it"
after an ability word, read an earlier spell ability's target. -/
theorem laterSpellAbilitiesReadEarlierTargets :
    Ability.checkText (DRS.ofList [])
        [spell (tap (target isCreature)), spell (untap (that creature))] = [] ∧
      Ability.checkText (DRS.ofList [])
        [spell (tap (target isCreature)), delirium (spell (untap it))] = [] := by decide

/-- Only targets pass on: an earlier spell ability's other mentions stay in it. -/
theorem laterSpellAbilitiesDoNotReadEarlierNonTargets :
    Ability.checkText (DRS.ofList [])
        [spell (tap (a isCreature)), spell (untap it)] = [.anaphor .bare .one 0] := by decide

/-- A triggered or activated ability's target is still no ordinary antecedent in a later ability,
which is linked to it at most [CR#607.1,607.2a]; and a spell's targets are its spell abilities'
alone, not a later ability of another kind's. -/
theorem otherAbilitiesTargetsStayTheirOwn :
    Ability.checkText (DRS.ofList [])
        [when (does thisCreature enter) (tap (target isCreature)), spell (untap (that creature))] =
        [.anaphor (.word creature) .one 0] ∧
      Ability.checkText (DRS.ofList [])
        [activated tapSymbol (tap (target isCreature)),
         activated tapSymbol (untap (that creature))] =
        [.anaphor (.word creature) .one 0] ∧
      Ability.checkText (DRS.ofList [])
        [when (does thisCreature enter) (tap (target isCreature)),
         at_ (beginningOfYour .endStep) (untap (that creature))] =
        [.anaphor (.word creature) .one 0] ∧
      Ability.checkText (DRS.ofList [])
        [spell (tap (target isCreature)),
         at_ (beginningOfYour .endStep) (untap (that creature))] =
        [.anaphor (.word creature) .one 0] := by decide

end Semantics.Cards
