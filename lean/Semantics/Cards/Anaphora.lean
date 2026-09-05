import Semantics
import Semantics.Macros
import Semantics.Check.Card
import Semantics.Cards.Description

/-!
# Semantics.Cards.Anaphora

Port of `idris/src/Experimental/Cards/Anaphora.idr`: the printed cards of the Anaphora family
and the phrase-level bench items beside them, each paired with an `ok…` theorem that runs the
checker on it at the bindings its Idris type named; the file's own `Refl` pins are theorems.

Not ported: `ichorElixirPlanarDice` (a planar die is Planechase, out of scope).
-/

open Semantics Semantics.Macros

namespace Semantics.Cards

def cloudshift : Instruction :=
  .sequentially
    [exile (target creatureYouControl), putOntoBattlefieldUnderYourControl (that .card)]
theorem okCloudshift : Instruction.check [] cloudshift = [] := by decide

def bitterDownfall : Instruction :=
  .sequentially [destroy (target creature), losesLife (controllerOf it) (.lit 2)]
theorem okBitterDownfall : Instruction.check [] bitterDownfall = [] := by decide

def suspendedSentence : Instruction :=
  .sequentially
    [ destroy (target (.and [creature, .hasPossessor .controller anOpponent])),
      losesLife (that .player) (.lit 3) ]
theorem okSuspendedSentence : Instruction.check [] suspendedSentence = [] := by decide

def flickeringSpirit : Instruction :=
  .sequentially [exile thisCreature, .move it battlefield [.under (ownerOf it)]]
theorem okFlickeringSpirit : Instruction.check [] flickeringSpirit = [] := by decide

/-- Bond of Revival -/
def bondOfRevival : Instruction :=
  .sequentially
    [ returnToBattlefield (target (.and [creature, .inZone (graveyardOf .you)])),
      gainsHaste (itVerbed (.core .return_)) (some untilYourNextTurn) ]
theorem okBondOfRevival : Instruction.check [] bondOfRevival = [] := by decide

def vraskasStoneglare : Instruction :=
  .sequentially [destroy (target creature), gainsLife .you (.statOf (.stat .toughness) it)]
theorem okVraskasStoneglare : Instruction.check [] vraskasStoneglare = [] := by decide

def phthisis : Instruction :=
  .sequentially
    [ destroy (target creature),
      losesLife (controllerOf it)
        (plus (.statOf (.stat .power) it) (.statOf (.stat .toughness) it)) ]
theorem okPhthisis : Instruction.check [] phthisis = [] := by decide

def foulTongueShriek : Instruction :=
  .sequentially
    [ losesLife (target .opponent)
        (forEach 1 (.and [attacking, creature, .hasPossessor .controller .you])),
      gainsLife .you .thatMuch ]
theorem okFoulTongueShriek : Instruction.check [] foulTongueShriek = [] := by decide

def phyrexianInfiltrator : Instruction :=
  .simultaneously
    [ gainControl (controllerOf (target creature)) thisCreature none,
      gainControl (controllerOf thisCreature) (that (.type .creature)) none ]
theorem okPhyrexianInfiltrator : Instruction.check [] phyrexianInfiltrator = [] := by decide

def impulse : Instruction :=
  .sequentially
    [ lookAt (topSlice (.lit 4)), move (someOf (exactly 1) them) hand,
      move (theRest .object) (onBottomIn .anyOrder) ]
theorem okImpulse : Instruction.check [] impulse = [] := by decide

def anticipate : Instruction :=
  .sequentially
    [ lookAt (topSlice (.lit 3)), move (someOf (exactly 1) them) hand,
      move (theRest .object) (onBottomIn .anyOrder) ]
theorem okAnticipate : Instruction.check [] anticipate = [] := by decide

def revealFourPartition : Instruction :=
  .sequentially
    [ revealCards (topSlice (.lit 4)), move (someOf (exactly 1) (those .card)) hand,
      move (theRest .object) graveyard ]
theorem okRevealFourPartition : Instruction.check [] revealFourPartition = [] := by decide

def exileFourOfThem : Instruction :=
  .sequentially
    [ lookAt (topSlice (.lit 8)), exile (someOf (exactly 4) them),
      move (theRest .object) (onTopIn .anyOrder) ]
theorem okExileFourOfThem : Instruction.check [] exileFourOfThem = [] := by decide

def sylvanScrying : Instruction :=
  .sequentially [searchLibraryFor (exactly 1) land, revealCards it, move it hand, shuffle]
theorem okSylvanScrying : Instruction.check [] sylvanScrying = [] := by decide

def searchToBattlefield : Instruction :=
  .sequentially [searchLibraryFor (exactly 1) creature, move it battlefield, shuffle]
theorem okSearchToBattlefield : Instruction.check [] searchToBattlefield = [] := by decide

def glimpseTheUnthinkable : Instruction := mills (target .anyPlayer) (.lit 10) they
theorem okGlimpseTheUnthinkable : Instruction.check [] glimpseTheUnthinkable = [] := by decide

def millThenReadGroup : Instruction :=
  .sequentially [mills .you (.lit 3) .you, exile (those .card)]
theorem okMillThenReadGroup : Instruction.check [] millThenReadGroup = [] := by decide

def takeIntoCustody : Instruction :=
  .sequentially [.setStatus .tapped (target creature), .doesntUntapNext it (.lit 1)]
theorem okTakeIntoCustody : Instruction.check [] takeIntoCustody = [] := by decide

/-- Frenzied Gorespawn -/
def frenziedGorespawnGoad : Instruction :=
  .forEachOf (each .opponent)
    (.gainsDesignation
      (target (.and [creature, .hasPossessor .controller (that .player)])) "goaded"
      .instructed none)
theorem okFrenziedGorespawnGoad : Instruction.check [] frenziedGorespawnGoad = [] := by decide

def spaceTimeAnomaly : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Space-Time Anomaly", cost := some [generic 2, pip .white, pip .blue],
      types := [.sorcery],
      text := [ .spell none (mills (target .anyPlayer) (lifeTotalOf .you) they) ] } }

/-- Consecrate // Consume -/
def consume : Ability :=
  .spell none (.sequentially
    [ sacrifice (target .anyPlayer)
        (a (.and [ creature,
                   .superlative .max (.stat .power)
                     (.and [creature, .hasPossessor .controller they]) ])),
      gainsLife .you (.statOf (.stat .power) it) ])
theorem okConsume : Ability.check [] consume = [] := by decide

def bifurcate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bifurcate", cost := some [generic 3, pip .green], types := [.sorcery],
      text :=
        [ .spell none (.sequentially
            [ searchLibraryFor (exactly 1)
                (.and [permanentCard, .named (.sameAs (target (.and [creature, nontoken])))]),
              putOntoBattlefield (that .card),
              shuffle ]) ] } }

/-- Mana Leak {1}{U} — Instant. "Counter target spell unless its controller pays {3}." -/
def manaLeak : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mana Leak", cost := some [generic 1, pip .blue], types := [.instant],
      text :=
        [ .spell none (unless_ (controllerOf (target spell))
            (.counterSpell it)
            (.mana [generic 3])) ] } }

/-- Oust {W} — Sorcery. "Put target creature into its owner's library second from the top.
Its controller gains 3 life." -/
def oust : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Oust", cost := some [pip .white], types := [.sorcery],
      text :=
        [ .spell none (.sequentially
            [ move (target creature) (nthFromTop (.nth 2)),
              gainsLife (controllerOf it) (.lit 3) ]) ] } }

def riseFromTheGrave : Instruction :=
  .sequentially
    [ putOntoBattlefieldUnderYourControl (target (.and [creature, .inZone graveyard])),
      becomes (that (.type .creature))
        { characteristics := { colors := [.black], subtypes := [creatureType "Zombie"] } } none ]
theorem okRiseFromTheGrave : Instruction.check [] riseFromTheGrave = [] := by decide

def everAfter : Instruction :=
  .sequentially
    [ move (.described (.target (upTo 2)) (.and [creature, .inZone (graveyardOf .you)]))
        battlefield,
      becomes (those (.type .creature))
        { characteristics := { colors := [.black], subtypes := [creatureType "Zombie"] } } none,
      move .this onBottom ]
theorem okEverAfter : Instruction.check [] everAfter = [] := by decide

def martyrsCry : Instruction :=
  .sequentially
    [ exile (allOf (.and [creature, .colorIs .white])),
      .forEachOf (theVerbed (.action "Exile") (.type .creature) .thisWay .many)
        (.draw (controllerOf it) (.lit 1)) ]
theorem okMartyrsCry : Instruction.check [] martyrsCry = [] := by decide

def anotherRound : Instruction :=
  .sequentially
    [ exile (counted anyNumber creatureYouControl),
      .move them battlefield [.under (ownerOf them)],
      .repeat_ (.moreTimes (.letter .x)) ]
theorem okAnotherRound : Instruction.check [] anotherRound = [] := by decide

/-- Eradicate -/
def eradicateSearch : Instruction :=
  .sequentially
    [ exile (target (.and [creature, .not (.colorIs .black)])),
      searchZonesOf (controllerOf (that .card)) anyNumber (.named (.sameAs (that .card))),
      exile (themVerbed (.action "Search")),
      .shuffle (that .player) ]
theorem okEradicateSearch : Instruction.check [] eradicateSearch = [] := by decide

/-- Deem Inferior -/
def deemInferior : Instruction :=
  puts (ownerOf (target (.and [permanent, .not land]))) it (nthFromTopOrBottom (.nth 2))
theorem okDeemInferior : Instruction.check [] deemInferior = [] := by decide

/-- Write into Being -/
def writeIntoBeingPlacement : Instruction :=
  .sequentially
    [ lookAt (topSlice (.lit 2)),
      .enact none (.action "Manifest") (.move (someOf (exactly 1) them) battlefield []),
      move (theOther .object) topOrBottom ]
theorem okWriteIntoBeingPlacement : Instruction.check [] writeIntoBeingPlacement = [] := by decide

/-- Mystical Tutor -/
def mysticalTutor : Instruction :=
  .sequentially
    [searchLibraryFor (exactly 1) instantOrSorcery, revealCards it, shuffle, move it onTop]
theorem okMysticalTutor : Instruction.check [] mysticalTutor = [] := by decide

/-- Demonic Tutor -/
def demonicTutor : Instruction :=
  .sequentially [searchLibraryFor (exactly 1) (.and []), move it hand, shuffle]
theorem okDemonicTutor : Instruction.check [] demonicTutor = [] := by decide

/-- Thalia's Lancers -/
def thaliasLancersSearch : Instruction :=
  may .you
    (.sequentially
      [ searchLibraryFor (exactly 1) (.and [.hasSupertype .legendary]), revealCards it,
        move it hand, shuffle ])
theorem okThaliasLancersSearch : Instruction.check [] thaliasLancersSearch = [] := by decide

def contrabandLivestock : Instruction :=
  .sequentially
    [ exile (target creature),
      rollDice .you 1 20,
      .resultsTable
        [ rollRow (fromTo 1 9)
            (.create (controllerOf it) (.lit 1)
              (.written (creatureToken 4 4 [.green] [creatureType "Ox"])) []),
          rollRow (fromTo 10 19)
            (.create (controllerOf it) (.lit 1)
              (.written (creatureToken 2 2 [.green] [creatureType "Boar"])) []),
          rollRow (exactly 20)
            (.create (controllerOf it) (.lit 1)
              (.written (creatureToken 0 1 [.white] [creatureType "Goat"])) []) ] ]
theorem okContrabandLivestock : Instruction.check [] contrabandLivestock = [] := by decide

/-- Hypnotic Specter -/
def hypnoticSpecterDiscard : Instruction := discard they (aAtRandom (.inZone hand))
theorem okHypnoticSpecterDiscard :
    Instruction.check [⟨.the, .one, .player false⟩] hypnoticSpecterDiscard = [] := by decide

/-- Wyll, Blade of Frontiers -/
def wyllExtraDie : Instruction :=
  ifWouldInstead (.rollsDice .you .many none .anyResult)
    (.sequentially
      [ .rollDice .you (plus .thatMuch (.lit 1)) .thoseDice,
        .ignoreOutcomes (.extreme .lowest) ])
    none
theorem okWyllExtraDie : Instruction.check [] wyllExtraDie = [] := by decide

/-- Vedalken Squirrel-Whacker -/
def vedalkenSquirrelWhackerReroll : Instruction :=
  ifWouldInstead (.rollsDice .you .many (some 6) .anyResult)
    (.sequentially
      [ .rollDice .you .thatMuch .thoseDice,
        may .you
          (.exchange (.values (.theOutcome .rollResult) (.statOf (.stat .power) thisCreature))) ])
    none
theorem okVedalkenSquirrelWhackerReroll :
    Instruction.check [] vedalkenSquirrelWhackerReroll = [] := by decide

/-- Investigator's Journal's count -/
def greatestCreaturesAPlayerControls : Amount :=
  .aggregateOver .max .anyPlayer (countOf (.and [creature, .hasPossessor .controller they]))
theorem okGreatestCreaturesAPlayerControls :
    Amount.check [] greatestCreaturesAPlayerControls = [] := by decide
/-- Cavern-Hoard Dragon -/
def greatestArtifactsAnOpponentControls : Amount :=
  .aggregateOver .max .opponent (countOf (.and [artifact, .hasPossessor .controller they]))
theorem okGreatestArtifactsAnOpponentControls :
    Amount.check [] greatestArtifactsAnOpponentControls = [] := by decide

/-- "each creature that player or that planeswalker's controller controls" -/
def eachCreatureThatSplitControls : NounPhrase :=
  each (.and [creature, .hasPossessor .controller splitOverPlaneswalker])

def anyTargetAnnounced : NounPhrase := target anyTarget
theorem okAnyTargetAnnounced : NounPhrase.check none [] anyTargetAnnounced = [] := by decide
def playerOrPlaneswalkerAnnounced : NounPhrase := targetPlayerOrPlaneswalker
/-- Chain of Plasma -/
def chainOfPlasmaOfferee : NounPhrase := splitOverPermanent
theorem okChainOfPlasmaOfferee :
    NounPhrase.check (some .player) (nomIntro [] anyTargetAnnounced) chainOfPlasmaOfferee = [] := by
  decide
/-- Chain Lightning -/
def chainLightningPayer : NounPhrase := splitOverPermanent
theorem okChainLightningPayer :
    NounPhrase.check (some .player) (nomIntro [] anyTargetAnnounced) chainLightningPayer = [] := by
  decide
/-- Flames of the Blood Hand -/
def flamesOfTheBloodHandSubject : NounPhrase := splitOverPlaneswalker
theorem okFlamesOfTheBloodHandSubject :
    NounPhrase.check (some .player) (nomIntro [] playerOrPlaneswalkerAnnounced)
      flamesOfTheBloodHandSubject = [] := by
  decide
/-- Flaming Gambit -/
def flamingGambitOfferee : NounPhrase := splitOverPlaneswalker
theorem okFlamingGambitOfferee :
    NounPhrase.check (some .player) (nomIntro [] playerOrPlaneswalkerAnnounced)
      flamingGambitOfferee = [] := by
  decide
theorem okEachCreatureThatSplitControls :
    NounPhrase.check (some .object) (nomIntro [] playerOrPlaneswalkerAnnounced)
      eachCreatureThatSplitControls = [] := by
  decide

/-- Ertai's Trickery -/
def ertaisTrickery : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ertai's Trickery", cost := some [pip .blue], types := [.instant],
      text :=
        [ .spell none (.onlyIf (.counterSpell (target spell))
            (costWasPaid (.byKeyword "Kicker") none it) none) ] } }

/-- Celebrate the Harvest -/
def celebrateTheHarvest : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Celebrate the Harvest", cost := some [generic 3, pip .green], types := [.sorcery],
      text :=
        [ .spell none (.sequentially
            [ searchLibraryFor (.upToOf (.letter .x)) (.and [land, .hasSupertype .basic]),
              .define .x (.distinctCount (.value .power) (allOf creatureYouControl)),
              putOntoBattlefieldTapped (those .card),
              shuffle ]) ] } }

/-- Boreas Charger's description -/
def opponentWithMoreLands : Predicate :=
  .compareOver .opponent (countOf (.and [land, .hasPossessor .controller they])) .greater
    (countOf (.and [land, .hasPossessor .controller .you]))
theorem okOpponentWithMoreLands : Predicate.check .player [] opponentWithMoreLands = [] := by
  decide

def bioplasmCardTest : Condition := itsACard creature
theorem okBioplasmCardTest : Condition.check bioplasmAfterExile bioplasmCardTest = [] := by decide

/-- Blessed Respite -/
def blessedRespiteShuffle : Instruction :=
  shuffleInto (target .anyPlayer) (allOf (.inZone (graveyardOf they)))
theorem okBlessedRespiteShuffle : Instruction.check [] blessedRespiteShuffle = [] := by decide

/-- Nekrataal -/
def nekrataalEntry : GameEvent := .enters thisCreature none
theorem okNekrataalEntry : GameEvent.check [] nekrataalEntry = [] := by decide
def nekrataalDestroy : Instruction :=
  destroy (target (.and [creature, .not artifact, .not (.colorIs .black)]))
theorem okNekrataalDestroy :
    Instruction.check (GameEvent.after [] nekrataalEntry) nekrataalDestroy = [] := by decide
def nekrataalRider : Bindings :=
  Instruction.riderIntro (GameEvent.after [] nekrataalEntry) nekrataalDestroy
theorem nekrataalOneCreatureWord : countReach (.word (.type .creature)) .one nekrataalRider = 1 := by
  decide
theorem nekrataalOneDestroyed : countReach (.stamped (.action "Destroy")) .one nekrataalRider = 1 := by
  decide

def sequencedRiderBody : Instruction :=
  .sequentially [exile (target artifact), destroy (target creature)]
theorem okSequencedRiderBody : Instruction.check [] sequencedRiderBody = [] := by decide
def sequencedRider : Bindings := Instruction.riderIntro [] sequencedRiderBody
theorem sequencedRiderOneDestroyed :
    countReach (.stamped (.action "Destroy")) .one sequencedRider = 1 := by decide
theorem sequencedRiderOneExiled : countReach (.stamped (.action "Exile")) .one sequencedRider = 1 := by
  decide

/-- Engulfing Flames -/
def engulfingFlamesBody : Instruction := .dealDamage .this (.lit 1) (target creature)
theorem okEngulfingFlamesBody : Instruction.check [] engulfingFlamesBody = [] := by decide
def engulfingFlamesRider : Bindings := Instruction.riderIntro [] engulfingFlamesBody
theorem engulfingFlamesNoDestroyStamp :
    countReach (.stamped (.action "Destroy")) .one engulfingFlamesRider = 0 := by decide
theorem engulfingFlamesBareReadStands : countReach .bare .one engulfingFlamesRider = 1 := by decide

def bioplasmExiledPronoun : NounPhrase := itVerbed (.action "Exile")
theorem okBioplasmExiledPronoun :
    NounPhrase.check (some .object) bioplasmAfterExile bioplasmExiledPronoun = [] := by decide
theorem bioplasmNoTypedRead :
    countReach (.verbed (.action "Exile") (.type .creature) .attributive) .one bioplasmAfterExile
      = 0 := by
  decide
theorem bioplasmExiledCardHasNoType :
    tyOfReach (.stamped (.action "Exile")) .one bioplasmAfterExile = none := by decide
def bioplasmAfterTest : Bindings := Condition.intro bioplasmAfterExile bioplasmCardTest
theorem bioplasmTestRemarksType :
    tyOfReach (.stamped (.action "Exile")) .one bioplasmAfterTest = some .creature := by decide
theorem bioplasmTestKeepsCardSlot : countReach (.atSlot .card) .one bioplasmAfterTest = 1 := by
  decide
theorem bioplasmTypedReadStillRefused :
    countReach (.verbed (.action "Exile") (.type .creature) .attributive) .one bioplasmAfterTest
      = 0 := by
  decide
theorem bioplasmTypedCardReadWrites :
    countReach (.verbed (.action "Exile") (.typedCard .creature) .attributive) .one
      bioplasmAfterTest = 1 := by
  decide

/-- Scapeshift -/
def scapeshiftSacrifice : Instruction := sacrifice .you (counted anyNumber land)
theorem okScapeshiftSacrifice : Instruction.check [] scapeshiftSacrifice = [] := by decide
def scapeshiftSacrificed : Bindings := Instruction.intro [] scapeshiftSacrifice
def scapeshiftSearch : Instruction := searchLibraryFor (.upToOf .groupSize) land
theorem okScapeshiftSearch : Instruction.check scapeshiftSacrificed scapeshiftSearch = [] := by
  decide
def scapeshiftAfterSearch : Bindings := Instruction.intro scapeshiftSacrificed scapeshiftSearch
theorem scapeshiftTwoGroups : countReach .bare .many scapeshiftAfterSearch = 2 := by decide
theorem scapeshiftOneSearchedGroup :
    countReach (.stamped (.action "Search")) .many scapeshiftAfterSearch = 1 := by decide
theorem scapeshiftOneSacrificedGroup :
    countReach (.stamped (.action "Sacrifice")) .many scapeshiftAfterSearch = 1 := by decide

/-- Scapeshift -/
def scapeshift : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Scapeshift", cost := some [generic 2, pip .green, pip .green], types := [.sorcery],
      text :=
        [ .spell none (.sequentially
            [ sacrifice .you (counted anyNumber land),
              searchLibraryFor (.upToOf .groupSize) land,
              putOntoBattlefieldTapped (themVerbed (.action "Search")),
              shuffle ]) ] } }

/-- Bind to Life, Vastlands Scavenger's adventure -/
def bindToLife : Instruction :=
  .sequentially
    [mills .you (.lit 7) .you, move (fromAmong (exactly 1) creature them) battlefield]
theorem okBindToLife : Instruction.check [] bindToLife = [] := by decide

/-- Glamdring, Foe-hammer's Gleam of Death -/
def gleamOfDeath : Instruction :=
  .sequentially
    [mills .you (.lit 6) .you, move (allFromAmong (.or [instant, sorcery]) them) hand]
theorem okGleamOfDeath : Instruction.check [] gleamOfDeath = [] := by decide

/-- Tezzeret, Master of the Bridge -/
def tezzeretAllArtifacts : Instruction :=
  .sequentially
    [ exile (topSlice (.lit 10)),
      move (allFromAmong artifact (theVerbed (.action "Exile") .card .thisWay .many)) battlefield ]
theorem okTezzeretAllArtifacts : Instruction.check [] tezzeretAllArtifacts = [] := by decide

def companyContext : Bindings := lookedTop [] (.lit 6)
def companyDescribedSlice : NounPhrase :=
  fromAmong (upTo 2) (.and [creature, .compare [.stat .manaValue] .atMost (.lit 3)]) them
theorem okCompanyDescribedSlice :
    NounPhrase.check (some .object) companyContext companyDescribedSlice = [] := by decide
def companyBareSlice : NounPhrase := someOf (exactly 2) them
theorem okCompanyBareSlice : NounPhrase.check (some .object) companyContext companyBareSlice = [] := by
  decide

/-- Lord of the Void -/
def exileTopThenPutFromAmong : Instruction :=
  .sequentially
    [ exile (.librarySlice .top (.lit 7) (target .opponent)),
      putOntoBattlefieldUnderYourControl (fromAmong (exactly 1) creature them) ]
theorem okExileTopThenPutFromAmong : Instruction.check [] exileTopThenPutFromAmong = [] := by decide

/-- Thought Sponge -/
def greatestCardsAnOpponentDrew : Amount :=
  .aggregateOver .max .opponent (eventCount .cardDrawn they .thisTurn)
theorem okGreatestCardsAnOpponentDrew : Amount.check [] greatestCardsAnOpponentDrew = [] := by
  decide
def greatestCardsAPlayerDiscardedThisWay : Amount :=
  .aggregateOver .max .anyPlayer
    (eventCountInvolving (.verbedAct (.action "Discard")) they .thisWay everyObject)
theorem okGreatestCardsAPlayerDiscardedThisWay :
    Amount.check [] greatestCardsAPlayerDiscardedThisWay = [] := by decide

def eachPlayerBase : NounPhrase := each .anyPlayer
theorem okEachPlayerBase : NounPhrase.check (some .player) [] eachPlayerBase = [] := by decide
theorem eachPlayerBindsNoSingular : countOnes .player (nomIntro [] eachPlayerBase) = 0 := by decide
theorem eachPlayerBindsAGroup : countManys .player (nomIntro [] eachPlayerBase) = 1 := by decide

/-- Soul Ransom -/
def soulRansomRansom : Instruction :=
  .sequentially [sacrificeIt (controllerOf thisAura), .draw they (.lit 2)]
theorem okSoulRansomRansom : Instruction.check [] soulRansomRansom = [] := by decide
def thisAurasController : NounPhrase := controllerOf thisAura
theorem okThisAurasController : NounPhrase.check (some .player) [] thisAurasController = [] := by
  decide
def targetCreaturesController : NounPhrase := controllerOf (target creature)
theorem okTargetCreaturesController :
    NounPhrase.check (some .player) [] targetCreaturesController = [] := by decide
theorem possessiveDeicticIsReadableByIt :
    countReach (.atSlot .permanent) .one (nomIntro [] thisAurasController) = 1 := by decide
theorem possessiveDeicticIsNotADemonstrative :
    countReach (.word (.type .enchantment)) .one (nomIntro [] thisAurasController) = 0 := by decide
theorem possessiveDescribedBaseUnchanged :
    countReach (.atSlot .permanent) .one (nomIntro [] targetCreaturesController) = 1 := by decide

/-- Bile Blight -/
def bileBlight : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bile Blight", cost := some [pip .black, pip .black], types := [.instant],
      text :=
        [ .spell none (gets
            (.both (target creature)
              (allOf (.and [ creature, .named (.sameAs (that (.type .creature))),
                             .otherThan (that (.type .creature)) ])))
            (.down (.lit 3)) (.down (.lit 3)) (some untilEndOfTurn)) ] } }

/-- Echoing Ruin -/
def echoingRuin : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Echoing Ruin", cost := some [generic 1, pip .red], types := [.sorcery],
      text :=
        [ .spell none (destroy
            (.both (target artifact)
              (allOf (.and [ artifact, .named (.sameAs (that (.type .artifact))),
                             .otherThan (that (.type .artifact)) ])))) ] } }

/-- Hijack -/
def hijack : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hijack", cost := some [generic 1, pip .red, pip .red], types := [.sorcery],
      text :=
        [ .spell none (.sequentially
            [ gainControl .you (target (.or [artifact, creature])) (some untilEndOfTurn),
              untap (itVerbed (.core .gainControl)),
              gainsHaste (itVerbed (.action "Untap")) (some untilEndOfTurn) ]) ] } }

/-- Open the Vaults -/
def openTheVaults : Instruction :=
  returnTo
    (allOf (.and [ .isCard, .or [artifact, enchantment],
                   .inZone (graveyardOf (.playerGroup .allPlayers)) ]))
    battlefield [.under (ownerOf them)]
theorem okOpenTheVaults : Instruction.check [] openTheVaults = [] := by decide

/-- Codecracker Hound -/
def codecrackerHoundLook : Instruction :=
  .sequentially
    [ lookAt (topSlice (.lit 2)), move (someOf (exactly 1) them) hand,
      move (theOther .object) graveyard ]
theorem okCodecrackerHoundLook : Instruction.check [] codecrackerHoundLook = [] := by decide

def oneOfTheTopTwo : NounPhrase := someOf (exactly 1) (.librarySlice .top (.lit 2) .you)
theorem okOneOfTheTopTwo : NounPhrase.check (some .object) [] oneOfTheTopTwo = [] := by decide
def oneOfAnUncountedTop : NounPhrase :=
  someOf (exactly 1) (.librarySlice .top (countOf creature) .you)
theorem okOneOfAnUncountedTop : NounPhrase.check (some .object) [] oneOfAnUncountedTop = [] := by
  decide
def theArtifactsAmongTheTopFive : NounPhrase :=
  allAmong artifact (.librarySlice .top (.lit 5) .you)
theorem okTheArtifactsAmongTheTopFive :
    NounPhrase.check (some .object) [] theArtifactsAmongTheTopFive = [] := by decide
theorem theOtherAfterATwoCardLook : theOtherOk .object (nomIntro [] oneOfTheTopTwo) = true := by
  decide
theorem theOtherNeedsAStatedCount :
    theOtherOk .object (nomIntro [] oneOfAnUncountedTop) = false := by decide
theorem theRestStandsWhereTheOtherRefuses :
    theRestOk .object (nomIntro [] oneOfAnUncountedTop) = true := by decide
theorem theOtherRefusesTheUniversalSlice :
    (theOtherOk .object (nomIntro [] theArtifactsAmongTheTopFive),
     theRestOk .object (nomIntro [] theArtifactsAmongTheTopFive)) = (false, true) := by
  decide

def eachPlayerShufflesTheirHandAndGraveyard : Instruction :=
  shuffleInto (each .anyPlayer)
    (.both (allOf (.inZone (handOf they))) (allOf (.inZone (graveyardOf they))))
theorem okEachPlayerShufflesTheirHandAndGraveyard :
    Instruction.check [] eachPlayerShufflesTheirHandAndGraveyard = [] := by decide

/-- Fact or Fiction -/
def factOrFiction : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fact or Fiction", cost := some [generic 3, pip .blue], types := [.instant],
      text :=
        [ .spell none (.sequentially
            [ revealCards (topSlice (.lit 5)),
              .separateIntoPiles anOpponent them 2 [],
              move onePile hand,
              move (theOther .pile) graveyard ]) ] } }

def unsummon : Instruction := move (target creature) hand
theorem okUnsummon : Instruction.check [] unsummon = [] := by decide
def grismold : Instruction :=
  .create (each .anyPlayer) (.lit 1)
    (.written (creatureToken 1 1 [.green] [creatureType "Plant"])) []
theorem okGrismold : Instruction.check [] grismold = [] := by decide
/-- Grenzo, Dungeon Warden -/
def grenzoBottomCard : Instruction := move bottomCard graveyard
theorem okGrenzoBottomCard : Instruction.check [] grenzoBottomCard = [] := by decide

def chronostutter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chronostutter", cost := some [generic 5, pip .blue], types := [.instant],
      text := [ .spell none (move (target creature) (nthFromTop (.nth 2))) ] } }

theorem nekrataalTwoObjects : countOnes .object nekrataalRider = 2 := by decide
theorem bioplasmTestMintsNothing : countOnes .object bioplasmAfterTest = 2 := by decide

def wholeSliceAtBase : SliceCount := .whole
theorem wholeSliceIsPluralAndUncounted :
    (NounPhrase.plur theArtifactsAmongTheTopFive, wholeSliceAtBase.exact) = (.many, none) := by
  decide
theorem describedSliceReadsAsCreature :
    NounPhrase.ty companyContext companyDescribedSlice = some .creature := by decide
theorem bareSliceReadsUntyped : NounPhrase.ty companyContext companyBareSlice = none := by decide
theorem describedSliceKeepsGroupZone :
    NounPhrase.zone companyContext companyDescribedSlice
      = NounPhrase.zone companyContext companyBareSlice := by
  decide

/-- Stomp and Howl -/
def stompAndHowl : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stomp and Howl", cost := some [generic 2, pip .green], types := [.sorcery],
      text := [ .spell none (destroy (.both (target artifact) (target enchantment))) ] } }

/-- Churning Eddy -/
def churningEddy : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Churning Eddy", cost := some [generic 3, pip .blue], types := [.sorcery],
      text := [ .spell none (returnTo (.both (target creature) (target land)) hand []) ] } }

/-- Secret Rendezvous -/
def secretRendezvous : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Secret Rendezvous", cost := some [generic 1, pip .white, pip .white],
      types := [.sorcery],
      text := [ .spell none (.draw (.eachOf (.both .you (target .opponent))) (.lit 3)) ] } }

/-- Aggressive Instinct -/
def aggressiveInstinct : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aggressive Instinct", cost := some [generic 1, pip .green], types := [.sorcery],
      text :=
        [ .spell none (dealsDamageOwnPower [] (target creatureYouControl)
            (target creatureYouDontControl)) ] } }

/-- Arcum Dagsson -/
def arcumDagssonSacrifice : Instruction :=
  .controllerSacrifices (target (.and [artifact, creature]))
theorem okArcumDagssonSacrifice : Instruction.check [] arcumDagssonSacrifice = [] := by decide

/-- Vaevictis Asmadi, the Dire -/
def vaevictisAsmadiTheDire : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vaevictis Asmadi, the Dire",
      cost := some [generic 3, pip .black, pip .red, pip .green],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Elder", creatureType "Dragon"],
      text :=
        [ keyword "Flying",
          whenever (attacks thisCreature)
            (.sequentially
              [ .forEachOf (each .anyPlayer)
                  (choose (target (.and [permanent, .hasPossessor .controller they]))),
                sacrifice (those .player) (those .permanent),
                .forEachOf
                  (each (.and [ .anyPlayer,
                                .happenedTo (.mk (.verbedAct (.action "Sacrifice")) .thisWay
                                  (some (.involving (a permanent)))) ]))
                  (.sequentially
                    [ .expose .reveal they (.cards (.librarySlice .top (.lit 1) they)),
                      .if_ (itsACard permanentCard) (move itCard battlefield)
                        none ]) ]) ],
      power := stat 6, toughness := stat 6 } }

end Semantics.Cards
