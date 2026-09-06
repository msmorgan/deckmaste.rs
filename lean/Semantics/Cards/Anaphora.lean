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
  Primitives.Instruction.sequentially
    [exile (target creatureYouControl), putOntoBattlefieldUnderYourControl (that .card)]
theorem okCloudshift : Instruction.check [] cloudshift = [] := by decide

def bitterDownfall : Instruction :=
  Primitives.Instruction.sequentially [destroy (target creature), loseLife (.lit 2) (agent := (controllerOf it))]
theorem okBitterDownfall : Instruction.check [] bitterDownfall = [] := by decide

def suspendedSentence : Instruction :=
  Primitives.Instruction.sequentially
    [ destroy (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller anOpponent])),
      loseLife (.lit 3) (agent := (that .player)) ]
theorem okSuspendedSentence : Instruction.check [] suspendedSentence = [] := by decide

def flickeringSpirit : Instruction :=
  Primitives.Instruction.sequentially [exile thisCreature, Primitives.Instruction.move it (some battlefield) [Primitives.TokenRider.under (ownerOf it)]]
theorem okFlickeringSpirit : Instruction.check [] flickeringSpirit = [] := by decide

/-- Bond of Revival -/
def bondOfRevival : Instruction :=
  Primitives.Instruction.sequentially
    [ returnToBattlefield (target (Primitives.Predicate.and [creature, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)])),
      gainHaste (itVerbed (.core .return_)) (some untilYourNextTurn) ]
theorem okBondOfRevival : Instruction.check [] bondOfRevival = [] := by decide

def vraskasStoneglare : Instruction :=
  Primitives.Instruction.sequentially [destroy (target creature), gainLife (Primitives.Amount.statOf (.stat .toughness) it) (agent := Primitives.NounPhrase.you)]
theorem okVraskasStoneglare : Instruction.check [] vraskasStoneglare = [] := by decide

def phthisis : Instruction :=
  Primitives.Instruction.sequentially
    [ destroy (target creature),
      loseLife
        (plus (Primitives.Amount.statOf (.stat .power) it) (Primitives.Amount.statOf (.stat .toughness) it)) (agent := (controllerOf
            it)) ]
theorem okPhthisis : Instruction.check [] phthisis = [] := by decide

def foulTongueShriek : Instruction :=
  Primitives.Instruction.sequentially
    [ loseLife
        (forEach 1 (Primitives.Predicate.and [attacking, creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) (agent := (target
            Primitives.Predicate.opponent)),
      gainLife Primitives.Amount.thatMuch (agent := Primitives.NounPhrase.you) ]
theorem okFoulTongueShriek : Instruction.check [] foulTongueShriek = [] := by decide

def phyrexianInfiltrator : Instruction :=
  Primitives.Instruction.simultaneously
    [ gainControl thisCreature none (agent := (controllerOf (target creature))),
      gainControl (that (.type .creature)) none (agent := (controllerOf thisCreature)) ]
theorem okPhyrexianInfiltrator : Instruction.check [] phyrexianInfiltrator = [] := by decide

def impulse : Instruction :=
  Primitives.Instruction.sequentially
    [ lookAt (topSlice (.lit 4)), move (someOf (exactly 1) them) hand,
      move (theRest .object) (onBottomIn .anyOrder) ]
theorem okImpulse : Instruction.check [] impulse = [] := by decide

def anticipate : Instruction :=
  Primitives.Instruction.sequentially
    [ lookAt (topSlice (.lit 3)), move (someOf (exactly 1) them) hand,
      move (theRest .object) (onBottomIn .anyOrder) ]
theorem okAnticipate : Instruction.check [] anticipate = [] := by decide

def revealFourPartition : Instruction :=
  Primitives.Instruction.sequentially
    [ revealCards (topSlice (.lit 4)), move (someOf (exactly 1) (those .card)) hand,
      move (theRest .object) graveyard ]
theorem okRevealFourPartition : Instruction.check [] revealFourPartition = [] := by decide

def exileFourOfThem : Instruction :=
  Primitives.Instruction.sequentially
    [ lookAt (topSlice (.lit 8)), exile (someOf (exactly 4) them),
      move (theRest .object) (onTopIn .anyOrder) ]
theorem okExileFourOfThem : Instruction.check [] exileFourOfThem = [] := by decide

def sylvanScrying : Instruction :=
  Primitives.Instruction.sequentially [searchLibraryFor (exactly 1) land, revealCards it, move it hand, shuffle]
theorem okSylvanScrying : Instruction.check [] sylvanScrying = [] := by decide

def searchToBattlefield : Instruction :=
  Primitives.Instruction.sequentially [searchLibraryFor (exactly 1) creature, move it battlefield, shuffle]
theorem okSearchToBattlefield : Instruction.check [] searchToBattlefield = [] := by decide

def glimpseTheUnthinkable : Instruction := mill (.lit 10) they (agent := (target Primitives.Predicate.anyPlayer))
theorem okGlimpseTheUnthinkable : Instruction.check [] glimpseTheUnthinkable = [] := by decide

def millThenReadGroup : Instruction :=
  Primitives.Instruction.sequentially [mill (.lit 3) Primitives.NounPhrase.you (agent := Primitives.NounPhrase.you), exile (those .card)]
theorem okMillThenReadGroup : Instruction.check [] millThenReadGroup = [] := by decide

def takeIntoCustody : Instruction :=
  Primitives.Instruction.sequentially [Primitives.Instruction.setStatus .tapped (target creature), Primitives.Instruction.skipUntap it (.lit 1)]
theorem okTakeIntoCustody : Instruction.check [] takeIntoCustody = [] := by decide

/-- Frenzied Gorespawn -/
def frenziedGorespawnGoad : Instruction :=
  Primitives.Instruction.doForEach (each Primitives.Predicate.opponent)
    (Primitives.Instruction.gainDesignation
      (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller (that .player)])) "goaded"
      .instructed none)
theorem okFrenziedGorespawnGoad : Instruction.check [] frenziedGorespawnGoad = [] := by decide

def spaceTimeAnomaly : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Space-Time Anomaly", cost := some [generic 2, pip .white, pip .blue],
      types := [.sorcery],
      text := [ Primitives.Ability.spell none (mill (lifeTotalOf Primitives.NounPhrase.you) they (agent := (target Primitives.Predicate.anyPlayer))) ] } }

/-- Consecrate // Consume -/
def consume : Ability :=
  Primitives.Ability.spell none (Primitives.Instruction.sequentially
    [ sacrifice
        (a (Primitives.Predicate.and [ creature,
                   Primitives.Predicate.superlative .max (.stat .power)
                     (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller they]) ])) (agent := (target
                         Primitives.Predicate.anyPlayer)),
      gainLife (Primitives.Amount.statOf (.stat .power) it) (agent := Primitives.NounPhrase.you) ])
theorem okConsume : Ability.check [] consume = [] := by decide

def bifurcate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bifurcate", cost := some [generic 3, pip .green], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ searchLibraryFor (exactly 1)
                (Primitives.Predicate.and [permanentCard, Primitives.Predicate.named (Primitives.NameSource.sameAs (target (Primitives.Predicate.and [creature, nontoken])))]),
              putOntoBattlefield (that .card),
              shuffle ]) ] } }

/-- Mana Leak {1}{U} — Instant. "Counter target spell unless its controller pays {3}." -/
def manaLeak : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mana Leak", cost := some [generic 1, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (doUnless
            (Primitives.Instruction.counterSpell it)
            (Primitives.Cost.mana [generic 3]) (agent := (controllerOf (target spell)))) ] } }

/-- Oust {W} — Sorcery. "Put target creature into its owner's library second from the top.
Its controller gains 3 life." -/
def oust : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Oust", cost := some [pip .white], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ move (target creature) (nthFromTop (.nth 2)),
              gainLife (.lit 3) (agent := (controllerOf it)) ]) ] } }

def riseFromTheGrave : Instruction :=
  Primitives.Instruction.sequentially
    [ putOntoBattlefieldUnderYourControl (target (Primitives.Predicate.and [creature, Primitives.Predicate.inZone graveyard])),
      become (that (.type .creature))
        { characteristics := { colors := [.black], subtypes := [creatureType "Zombie"] } } none ]
theorem okRiseFromTheGrave : Instruction.check [] riseFromTheGrave = [] := by decide

def everAfter : Instruction :=
  Primitives.Instruction.sequentially
    [ move (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 2)) (Primitives.Predicate.and [creature, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)]))
        battlefield,
      become (those (.type .creature))
        { characteristics := { colors := [.black], subtypes := [creatureType "Zombie"] } } none,
      move Primitives.NounPhrase.this onBottom ]
theorem okEverAfter : Instruction.check [] everAfter = [] := by decide

def martyrsCry : Instruction :=
  Primitives.Instruction.sequentially
    [ exile (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.colorIs .white])),
      Primitives.Instruction.doForEach (theVerbed (.action "Exile") (.type .creature) .thisWay .many)
        (Primitives.Instruction.draw (.lit 1) (agent := (controllerOf it))) ]
theorem okMartyrsCry : Instruction.check [] martyrsCry = [] := by decide

def anotherRound : Instruction :=
  Primitives.Instruction.sequentially
    [ exile (counted anyNumber creatureYouControl),
      Primitives.Instruction.move them (some battlefield) [Primitives.TokenRider.under (ownerOf them)],
      Primitives.Instruction.repeat_ (Primitives.Repetition.moreTimes (Primitives.Amount.letter .x)) ]
theorem okAnotherRound : Instruction.check [] anotherRound = [] := by decide

/-- Eradicate -/
def eradicateSearch : Instruction :=
  Primitives.Instruction.sequentially
    [ exile (target (Primitives.Predicate.and [creature, Primitives.Predicate.not (Primitives.Predicate.colorIs .black)])),
      searchZonesOf (controllerOf (that .card)) anyNumber (Primitives.Predicate.named (Primitives.NameSource.sameAs (that .card))),
      exile (themVerbed (.action "Search")),
      Primitives.Instruction.shuffle (agent := (that .player)) ]
theorem okEradicateSearch : Instruction.check [] eradicateSearch = [] := by decide

/-- Deem Inferior -/
def deemInferior : Instruction :=
  put it (nthFromTopOrBottom (.nth 2)) (agent := (ownerOf (target (Primitives.Predicate.and [permanent, Primitives.Predicate.not land]))))
theorem okDeemInferior : Instruction.check [] deemInferior = [] := by decide

/-- Write into Being -/
def writeIntoBeingPlacement : Instruction :=
  Primitives.Instruction.sequentially
    [ lookAt (topSlice (.lit 2)),
      manifestPlacement (someOf (exactly 1) them),
      move (theOther .object) topOrBottom ]
theorem okWriteIntoBeingPlacement : Instruction.check [] writeIntoBeingPlacement = [] := by decide

/-- Mystical Tutor -/
def mysticalTutor : Instruction :=
  Primitives.Instruction.sequentially
    [searchLibraryFor (exactly 1) instantOrSorcery, revealCards it, shuffle, move it onTop]
theorem okMysticalTutor : Instruction.check [] mysticalTutor = [] := by decide

/-- Demonic Tutor -/
def demonicTutor : Instruction :=
  Primitives.Instruction.sequentially [searchLibraryFor (exactly 1) (Primitives.Predicate.and []), move it hand, shuffle]
theorem okDemonicTutor : Instruction.check [] demonicTutor = [] := by decide

/-- Thalia's Lancers -/
def thaliasLancersSearch : Instruction :=
  offer
    (Primitives.Instruction.sequentially
      [ searchLibraryFor (exactly 1) (Primitives.Predicate.and [Primitives.Predicate.hasSupertype .legendary]), revealCards it,
        move it hand, shuffle ]) (agent := Primitives.NounPhrase.you)
theorem okThaliasLancersSearch : Instruction.check [] thaliasLancersSearch = [] := by decide

def contrabandLivestock : Instruction :=
  Primitives.Instruction.sequentially
    [ exile (target creature),
      rollDice 1 20 (agent := Primitives.NounPhrase.you),
      Primitives.Instruction.applyResultsTable
        [ rollRow (fromTo 1 9)
            (Primitives.Instruction.create (.lit 1)
              (Primitives.TokenSpec.written (creatureToken 4 4 [.green] [creatureType "Ox"])) [] (agent := (controllerOf
                  it))),
          rollRow (fromTo 10 19)
            (Primitives.Instruction.create (.lit 1)
              (Primitives.TokenSpec.written (creatureToken 2 2 [.green] [creatureType "Boar"])) [] (agent :=
                  (controllerOf it))),
          rollRow (exactly 20)
            (Primitives.Instruction.create (.lit 1)
              (Primitives.TokenSpec.written (creatureToken 0 1 [.white] [creatureType "Goat"])) [] (agent :=
                  (controllerOf it))) ] ]
theorem okContrabandLivestock : Instruction.check [] contrabandLivestock = [] := by decide

/-- Hypnotic Specter -/
def hypnoticSpecterDiscard : Instruction := discard (aAtRandom (Primitives.Predicate.inZone hand)) (agent := they)
theorem okHypnoticSpecterDiscard :
    Instruction.check [⟨.the, .one, .player false⟩] hypnoticSpecterDiscard = [] := by decide

/-- Wyll, Blade of Frontiers -/
def wyllExtraDie : Instruction :=
  replaceEvent (Primitives.GameEvent.rollsDice Primitives.NounPhrase.you .many none Primitives.RollWatch.anyResult)
    (Primitives.Instruction.sequentially
      [ Primitives.Instruction.rollDice (plus Primitives.Amount.thatMuch (.lit 1)) .thoseDice (agent := Primitives.NounPhrase.you),
        Primitives.Instruction.ignoreOutcomes (Primitives.IgnoredOutcomes.extreme .lowest) ])
    none
theorem okWyllExtraDie : Instruction.check [] wyllExtraDie = [] := by decide

/-- Vedalken Squirrel-Whacker -/
def vedalkenSquirrelWhackerReroll : Instruction :=
  replaceEvent (Primitives.GameEvent.rollsDice Primitives.NounPhrase.you .many (some 6) Primitives.RollWatch.anyResult)
    (Primitives.Instruction.sequentially
      [ Primitives.Instruction.rollDice Primitives.Amount.thatMuch .thoseDice (agent := Primitives.NounPhrase.you),
        offer
          (Primitives.Instruction.exchange (Primitives.Exchanged.values (Primitives.Amount.theOutcome .rollResult) (Primitives.Amount.statOf (.stat .power) thisCreature)))
              (agent := Primitives.NounPhrase.you) ])
    none
theorem okVedalkenSquirrelWhackerReroll :
    Instruction.check [] vedalkenSquirrelWhackerReroll = [] := by decide

/-- Investigator's Journal's count -/
def greatestCreaturesAPlayerControls : Amount :=
  Primitives.Amount.aggregateOver .max Primitives.Predicate.anyPlayer (countOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller they]))
theorem okGreatestCreaturesAPlayerControls :
    Amount.check [] greatestCreaturesAPlayerControls = [] := by decide
/-- Cavern-Hoard Dragon -/
def greatestArtifactsAnOpponentControls : Amount :=
  Primitives.Amount.aggregateOver .max Primitives.Predicate.opponent (countOf (Primitives.Predicate.and [artifact, Primitives.Predicate.hasPossessor .controller they]))
theorem okGreatestArtifactsAnOpponentControls :
    Amount.check [] greatestArtifactsAnOpponentControls = [] := by decide

/-- "each creature that player or that planeswalker's controller controls" -/
def eachCreatureThatSplitControls : NounPhrase :=
  each (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller splitOverPlaneswalker])

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
        [ Primitives.Ability.spell none (Primitives.Instruction.doOnlyIf (Primitives.Instruction.counterSpell (target spell))
            (costWasPaid (.byKeyword "Kicker") none it) none) ] } }

/-- Celebrate the Harvest -/
def celebrateTheHarvest : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Celebrate the Harvest", cost := some [generic 3, pip .green], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ searchLibraryFor (Primitives.Quantity.upToOf (Primitives.Amount.letter .x)) (Primitives.Predicate.and [land, Primitives.Predicate.hasSupertype .basic]),
              Primitives.Instruction.define .x (Primitives.Amount.distinctCount (.value .power) (allOf creatureYouControl)),
              putOntoBattlefieldTapped (those .card),
              shuffle ]) ] } }

/-- Boreas Charger's description -/
def opponentWithMoreLands : Predicate :=
  Primitives.Predicate.compareOver Primitives.Predicate.opponent (countOf (Primitives.Predicate.and [land, Primitives.Predicate.hasPossessor .controller they])) .greater
    (countOf (Primitives.Predicate.and [land, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
theorem okOpponentWithMoreLands : Predicate.check .player [] opponentWithMoreLands = [] := by
  decide

def bioplasmCardTest : Condition := itsACard creature
theorem okBioplasmCardTest : Condition.check bioplasmAfterExile bioplasmCardTest = [] := by decide

/-- Blessed Respite -/
def blessedRespiteShuffle : Instruction :=
  shuffleInto (allOf (Primitives.Predicate.inZone (graveyardOf they))) (agent := (target Primitives.Predicate.anyPlayer))
theorem okBlessedRespiteShuffle : Instruction.check [] blessedRespiteShuffle = [] := by decide

/-- Nekrataal -/
def nekrataalEntry : GameEvent := Primitives.GameEvent.enters thisCreature none
theorem okNekrataalEntry : GameEvent.check [] nekrataalEntry = [] := by decide
def nekrataalDestroy : Instruction :=
  destroy (target (Primitives.Predicate.and [creature, Primitives.Predicate.not artifact, Primitives.Predicate.not (Primitives.Predicate.colorIs .black)]))
theorem okNekrataalDestroy :
    Instruction.check (GameEvent.after [] nekrataalEntry) nekrataalDestroy = [] := by decide
def nekrataalRider : Bindings :=
  Instruction.riderIntro (GameEvent.after [] nekrataalEntry) nekrataalDestroy
theorem nekrataalOneCreatureWord : countReach (.word (.type .creature)) .one nekrataalRider = 1 := by
  decide
theorem nekrataalOneDestroyed : countReach (.stamped (.action "Destroy")) .one nekrataalRider = 1 := by
  decide

def sequencedRiderBody : Instruction :=
  Primitives.Instruction.sequentially [exile (target artifact), destroy (target creature)]
theorem okSequencedRiderBody : Instruction.check [] sequencedRiderBody = [] := by decide
def sequencedRider : Bindings := Instruction.riderIntro [] sequencedRiderBody
theorem sequencedRiderOneDestroyed :
    countReach (.stamped (.action "Destroy")) .one sequencedRider = 1 := by decide
theorem sequencedRiderOneExiled : countReach (.stamped (.action "Exile")) .one sequencedRider = 1 := by
  decide

/-- Engulfing Flames -/
def engulfingFlamesBody : Instruction := Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 1) (target creature)
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
    tyOfReach (.stamped (.action "Exile")) .one bioplasmAfterExile = [] := by decide
def bioplasmAfterTest : Bindings := Condition.intro bioplasmAfterExile bioplasmCardTest
theorem bioplasmTestRemarksType :
    tyOfReach (.stamped (.action "Exile")) .one bioplasmAfterTest = [.creature] := by decide
theorem bioplasmTestKeepsCardSlot : countReach (.atSlot .card) .one bioplasmAfterTest = 1 := by
  decide
theorem bioplasmTypedReadStillRefused :
    countReach (.verbed (.action "Exile") (.type .creature) .attributive) .one bioplasmAfterTest
      = 0 := by
  decide
theorem bioplasmTypedCardReadWrites :
    countReach (.verbed (.action "Exile") (.ofType .card .creature) .attributive) .one
      bioplasmAfterTest = 1 := by
  decide

/-- Scapeshift -/
def scapeshiftSacrifice : Instruction := sacrifice (counted anyNumber land) (agent := Primitives.NounPhrase.you)
theorem okScapeshiftSacrifice : Instruction.check [] scapeshiftSacrifice = [] := by decide
def scapeshiftSacrificed : Bindings := Instruction.intro [] scapeshiftSacrifice
def scapeshiftSearch : Instruction := searchLibraryFor (Primitives.Quantity.upToOf Primitives.Amount.groupSize) land
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
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ sacrifice (counted anyNumber land) (agent := Primitives.NounPhrase.you),
              searchLibraryFor (Primitives.Quantity.upToOf Primitives.Amount.groupSize) land,
              putOntoBattlefieldTapped (themVerbed (.action "Search")),
              shuffle ]) ] } }

/-- Bind to Life, Vastlands Scavenger's adventure -/
def bindToLife : Instruction :=
  Primitives.Instruction.sequentially
    [mill (.lit 7) Primitives.NounPhrase.you (agent := Primitives.NounPhrase.you), move (fromAmong (exactly 1) creature them) battlefield]
theorem okBindToLife : Instruction.check [] bindToLife = [] := by decide

/-- Glamdring, Foe-hammer's Gleam of Death -/
def gleamOfDeath : Instruction :=
  Primitives.Instruction.sequentially
    [mill (.lit 6) Primitives.NounPhrase.you (agent := Primitives.NounPhrase.you), move (allFromAmong (Primitives.Predicate.or [instant, sorcery]) them) hand]
theorem okGleamOfDeath : Instruction.check [] gleamOfDeath = [] := by decide

/-- Tezzeret, Master of the Bridge -/
def tezzeretAllArtifacts : Instruction :=
  Primitives.Instruction.sequentially
    [ exile (topSlice (.lit 10)),
      move (allFromAmong artifact (theVerbed (.action "Exile") .card .thisWay .many)) battlefield ]
theorem okTezzeretAllArtifacts : Instruction.check [] tezzeretAllArtifacts = [] := by decide

def companyContext : Bindings := lookedTop [] (.lit 6)
def companyDescribedSlice : NounPhrase :=
  fromAmong (upTo 2) (Primitives.Predicate.and [creature, Primitives.Predicate.compare [.stat .manaValue] .atMost (.lit 3)]) them
theorem okCompanyDescribedSlice :
    NounPhrase.check (some .object) companyContext companyDescribedSlice = [] := by decide
def companyBareSlice : NounPhrase := someOf (exactly 2) them
theorem okCompanyBareSlice : NounPhrase.check (some .object) companyContext companyBareSlice = [] := by
  decide

/-- Lord of the Void -/
def exileTopThenPutFromAmong : Instruction :=
  Primitives.Instruction.sequentially
    [ exile (Primitives.NounPhrase.librarySlice .top (.lit 7) (target Primitives.Predicate.opponent)),
      putOntoBattlefieldUnderYourControl (fromAmong (exactly 1) creature them) ]
theorem okExileTopThenPutFromAmong : Instruction.check [] exileTopThenPutFromAmong = [] := by decide

/-- Thought Sponge -/
def greatestCardsAnOpponentDrew : Amount :=
  Primitives.Amount.aggregateOver .max Primitives.Predicate.opponent (eventCount
    (Primitives.GameEvent.draws (relative .player)) they .thisTurn)
theorem okGreatestCardsAnOpponentDrew : Amount.check [] greatestCardsAnOpponentDrew = [] := by
  decide
def greatestCardsAPlayerDiscardedThisWay : Amount :=
  Primitives.Amount.aggregateOver .max Primitives.Predicate.anyPlayer
    (eventCount (Primitives.GameEvent.verbedEvent (some (relative .player)) (.action "Discard")
      (some (allOf (Primitives.Predicate.inZone hand))) none none) they .thisWay)
theorem okGreatestCardsAPlayerDiscardedThisWay :
    Amount.check [] greatestCardsAPlayerDiscardedThisWay = [] := by decide

def eachPlayerBase : NounPhrase := each Primitives.Predicate.anyPlayer
theorem okEachPlayerBase : NounPhrase.check (some .player) [] eachPlayerBase = [] := by decide
theorem eachPlayerBindsNoSingular : countOnes .player (nomIntro [] eachPlayerBase) = 0 := by decide
theorem eachPlayerBindsAGroup : countManys .player (nomIntro [] eachPlayerBase) = 1 := by decide

/-- Soul Ransom -/
def soulRansomRansom : Instruction :=
  Primitives.Instruction.sequentially [sacrificeIt (agent := (controllerOf thisAura)), Primitives.Instruction.draw (.lit 2) (agent := they)]
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
        [ Primitives.Ability.spell none (get
            (Primitives.NounPhrase.both (target creature)
              (allOf (Primitives.Predicate.and [ creature, Primitives.Predicate.named (Primitives.NameSource.sameAs (that (.type .creature))),
                             Primitives.Predicate.otherThan (that (.type .creature)) ])))
            (Primitives.Delta.down (.lit 3)) (Primitives.Delta.down (.lit 3)) (some untilEndOfTurn)) ] } }

/-- Echoing Ruin -/
def echoingRuin : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Echoing Ruin", cost := some [generic 1, pip .red], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (destroy
            (Primitives.NounPhrase.both (target artifact)
              (allOf (Primitives.Predicate.and [ artifact, Primitives.Predicate.named (Primitives.NameSource.sameAs (that (.type .artifact))),
                             Primitives.Predicate.otherThan (that (.type .artifact)) ])))) ] } }

/-- Hijack -/
def hijack : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hijack", cost := some [generic 1, pip .red, pip .red], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ gainControl (target (Primitives.Predicate.or [artifact, creature])) (some untilEndOfTurn) (agent := Primitives.NounPhrase.you),
              untap (itVerbed (.core .gainControl)),
              gainHaste (itVerbed (.action "Untap")) (some untilEndOfTurn) ]) ] } }

/-- Open the Vaults -/
def openTheVaults : Instruction :=
  returnTo
    (allOf (Primitives.Predicate.and [ Primitives.Predicate.isCard, Primitives.Predicate.or [artifact, enchantment],
                   Primitives.Predicate.inZone (graveyardOf (Primitives.NounPhrase.playerGroup .allPlayers)) ]))
    battlefield [Primitives.TokenRider.under (ownerOf them)]
theorem okOpenTheVaults : Instruction.check [] openTheVaults = [] := by decide

/-- Codecracker Hound -/
def codecrackerHoundLook : Instruction :=
  Primitives.Instruction.sequentially
    [ lookAt (topSlice (.lit 2)), move (someOf (exactly 1) them) hand,
      move (theOther .object) graveyard ]
theorem okCodecrackerHoundLook : Instruction.check [] codecrackerHoundLook = [] := by decide

def oneOfTheTopTwo : NounPhrase := someOf (exactly 1) (Primitives.NounPhrase.librarySlice .top (.lit 2) Primitives.NounPhrase.you)
theorem okOneOfTheTopTwo : NounPhrase.check (some .object) [] oneOfTheTopTwo = [] := by decide
def oneOfAnUncountedTop : NounPhrase :=
  someOf (exactly 1) (Primitives.NounPhrase.librarySlice .top (countOf creature) Primitives.NounPhrase.you)
theorem okOneOfAnUncountedTop : NounPhrase.check (some .object) [] oneOfAnUncountedTop = [] := by
  decide
def theArtifactsAmongTheTopFive : NounPhrase :=
  allAmong artifact (Primitives.NounPhrase.librarySlice .top (.lit 5) Primitives.NounPhrase.you)
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
  shuffleInto
    (Primitives.NounPhrase.both (allOf (Primitives.Predicate.inZone (handOf they))) (allOf (Primitives.Predicate.inZone (graveyardOf they)))) (agent := (each
        Primitives.Predicate.anyPlayer))
theorem okEachPlayerShufflesTheirHandAndGraveyard :
    Instruction.check [] eachPlayerShufflesTheirHandAndGraveyard = [] := by decide

/-- Fact or Fiction -/
def factOrFiction : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fact or Fiction", cost := some [generic 3, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ revealCards (topSlice (.lit 5)),
              Primitives.Instruction.separateIntoPiles them 2 [] (agent := anOpponent),
              move onePile hand,
              move (theOther .pile) graveyard ]) ] } }

def unsummon : Instruction := move (target creature) hand
theorem okUnsummon : Instruction.check [] unsummon = [] := by decide
def grismold : Instruction :=
  Primitives.Instruction.create (.lit 1)
    (Primitives.TokenSpec.written (creatureToken 1 1 [.green] [creatureType "Plant"])) [] (agent := (each Primitives.Predicate.anyPlayer))
theorem okGrismold : Instruction.check [] grismold = [] := by decide
/-- Grenzo, Dungeon Warden -/
def grenzoBottomCard : Instruction := move bottomCard graveyard
theorem okGrenzoBottomCard : Instruction.check [] grenzoBottomCard = [] := by decide

def chronostutter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chronostutter", cost := some [generic 5, pip .blue], types := [.instant],
      text := [ Primitives.Ability.spell none (move (target creature) (nthFromTop (.nth 2))) ] } }

theorem nekrataalTwoObjects : countOnes .object nekrataalRider = 2 := by decide
theorem bioplasmTestMintsNothing : countOnes .object bioplasmAfterTest = 2 := by decide

def wholeSliceAtBase : SliceCount := Primitives.SliceCount.whole
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
      text := [ Primitives.Ability.spell none (destroy (Primitives.NounPhrase.both (target artifact) (target enchantment))) ] } }

/-- Churning Eddy -/
def churningEddy : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Churning Eddy", cost := some [generic 3, pip .blue], types := [.sorcery],
      text := [ Primitives.Ability.spell none (returnTo (Primitives.NounPhrase.both (target creature) (target land)) hand []) ] } }

/-- Secret Rendezvous -/
def secretRendezvous : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Secret Rendezvous", cost := some [generic 1, pip .white, pip .white],
      types := [.sorcery],
      text := [ Primitives.Ability.spell none (Primitives.Instruction.draw (.lit 3) (agent := (Primitives.NounPhrase.eachOf (Primitives.NounPhrase.both Primitives.NounPhrase.you (target Primitives.Predicate.opponent))))) ]
          } }

/-- Aggressive Instinct -/
def aggressiveInstinct : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aggressive Instinct", cost := some [generic 1, pip .green], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (dealDamageOwnPower (target creatureYouControl)
            (target creatureYouDontControl)) ] } }

/-- Arcum Dagsson -/
def arcumDagssonSacrifice : Instruction :=
  controllerSacrifices (target (Primitives.Predicate.and [artifact, creature]))
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
            (Primitives.Instruction.sequentially
              [ Primitives.Instruction.doForEach (each Primitives.Predicate.anyPlayer)
                  (choose (target (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller they]))),
                sacrifice (those .permanent) (agent := (those .player)),
                Primitives.Instruction.doForEach
                  (each (Primitives.Predicate.and [ Primitives.Predicate.anyPlayer,
                                Primitives.Predicate.happenedTo (Primitives.LookbackClause.mk
                                  (Primitives.GameEvent.verbedEvent (some (relative .player))
                                  (.action "Sacrifice") (some (a permanent)) none none) .thisWay)
                                  ]))
                  (Primitives.Instruction.sequentially
                    [ Primitives.Instruction.expose .reveal (Primitives.Exposed.cards (Primitives.NounPhrase.librarySlice .top (.lit 1) they)) (agent := they),
                      Primitives.Instruction.doIf (itsACard permanentCard) (move itCard battlefield)
                        none ]) ]) ],
      power := stat 6, toughness := stat 6 } }

end Semantics.Cards
