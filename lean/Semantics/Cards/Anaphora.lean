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
theorem okCloudshift : Instruction.check (DRS.ofList []) cloudshift = [] := by decide

def bitterDownfall : Instruction :=
  Primitives.Instruction.sequentially [destroy (target creature), instruct (controllerOf it) (loseLife (.lit 2))]
theorem okBitterDownfall : Instruction.check (DRS.ofList []) bitterDownfall = [] := by decide

def suspendedSentence : Instruction :=
  Primitives.Instruction.sequentially
    [ destroy (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller anOpponent])),
      instruct (that .player) (loseLife (.lit 3)) ]
theorem okSuspendedSentence : Instruction.check (DRS.ofList []) suspendedSentence = [] := by decide

def flickeringSpirit : Instruction :=
  Primitives.Instruction.sequentially [exile thisCreature, Primitives.Instruction.move it Primitives.EventSource.anywhere (some battlefield) .arrival none [Primitives.MoveRider.entry (Primitives.EntryRider.under (ownerOf it))]]
theorem okFlickeringSpirit : Instruction.check (DRS.ofList []) flickeringSpirit = [] := by decide

/-- Bond of Revival -/
def bondOfRevival : Instruction :=
  Primitives.Instruction.sequentially
    [ returnToBattlefield (target (Primitives.Predicate.and [creature, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)])),
      gainHaste (theVerbed (.core .return_) .one) (some untilYourNextTurn) ]
theorem okBondOfRevival : Instruction.check (DRS.ofList []) bondOfRevival = [] := by decide

def vraskasStoneglare : Instruction :=
  Primitives.Instruction.sequentially [destroy (target creature), gainLife (Primitives.Amount.statOf (.stat .toughness) it)]
theorem okVraskasStoneglare : Instruction.check (DRS.ofList []) vraskasStoneglare = [] := by decide

def phthisis : Instruction :=
  Primitives.Instruction.sequentially
    [ destroy (target creature),
      instruct (controllerOf it) (loseLife
        (plus (Primitives.Amount.statOf (.stat .power) it) (Primitives.Amount.statOf (.stat .toughness) it))) ]
theorem okPhthisis : Instruction.check (DRS.ofList []) phthisis = [] := by decide

def foulTongueShriek : Instruction :=
  Primitives.Instruction.sequentially
    [ instruct (target Primitives.Predicate.opponent) (loseLife
        (forEach 1 (Primitives.Predicate.and [attacking, creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))),
      gainLife Primitives.Amount.thatMuch ]
theorem okFoulTongueShriek : Instruction.check (DRS.ofList []) foulTongueShriek = [] := by decide

def phyrexianInfiltrator : Instruction :=
  Primitives.Instruction.simultaneously
    [ gainControl thisCreature none (agent := (controllerOf (target creature))),
      gainControl (that (.type .creature)) none (agent := (controllerOf thisCreature)) ]
theorem okPhyrexianInfiltrator : Instruction.check (DRS.ofList []) phyrexianInfiltrator = [] := by decide

def impulse : Instruction :=
  Primitives.Instruction.sequentially
    [ lookAt (topSlice (.lit 4)), move (someOf (exactly 1) them) hand,
      Primitives.Instruction.move (theRest .object) Primitives.EventSource.anywhere (some onBottom) .arrival none [Primitives.MoveRider.placement (Primitives.PilePlacement.inOrder .anyOrder)] ]
theorem okImpulse : Instruction.check (DRS.ofList []) impulse = [] := by decide

def anticipate : Instruction :=
  Primitives.Instruction.sequentially
    [ lookAt (topSlice (.lit 3)), move (someOf (exactly 1) them) hand,
      Primitives.Instruction.move (theRest .object) Primitives.EventSource.anywhere (some onBottom) .arrival none [Primitives.MoveRider.placement (Primitives.PilePlacement.inOrder .anyOrder)] ]
theorem okAnticipate : Instruction.check (DRS.ofList []) anticipate = [] := by decide

def revealFourPartition : Instruction :=
  Primitives.Instruction.sequentially
    [ revealCards (topSlice (.lit 4)), move (someOf (exactly 1) (those .card)) hand,
      move (theRest .object) graveyard ]
theorem okRevealFourPartition : Instruction.check (DRS.ofList []) revealFourPartition = [] := by decide

def exileFourOfThem : Instruction :=
  Primitives.Instruction.sequentially
    [ lookAt (topSlice (.lit 8)), exile (someOf (exactly 4) them),
      Primitives.Instruction.move (theRest .object) Primitives.EventSource.anywhere (some onTop) .arrival none [Primitives.MoveRider.placement (Primitives.PilePlacement.inOrder .anyOrder)] ]
theorem okExileFourOfThem : Instruction.check (DRS.ofList []) exileFourOfThem = [] := by decide

def sylvanScrying : Instruction :=
  Primitives.Instruction.sequentially [searchLibraryFor (exactly 1) land, revealCards it, move it hand, shuffle]
theorem okSylvanScrying : Instruction.check (DRS.ofList []) sylvanScrying = [] := by decide

def searchToBattlefield : Instruction :=
  Primitives.Instruction.sequentially [searchLibraryFor (exactly 1) creature, move it battlefield, shuffle]
theorem okSearchToBattlefield : Instruction.check (DRS.ofList []) searchToBattlefield = [] := by decide

def glimpseTheUnthinkable : Instruction := instruct (target Primitives.Predicate.anyPlayer) (mill (.lit 10) they)
theorem okGlimpseTheUnthinkable : Instruction.check (DRS.ofList []) glimpseTheUnthinkable = [] := by decide

def millThenReadGroup : Instruction :=
  Primitives.Instruction.sequentially [mill (.lit 3) Primitives.NounPhrase.you, exile (those .card)]
theorem okMillThenReadGroup : Instruction.check (DRS.ofList []) millThenReadGroup = [] := by decide

def takeIntoCustody : Instruction :=
  Primitives.Instruction.sequentially [Primitives.Instruction.setStatus .tapped (target creature), Primitives.Instruction.skipUntap it (.lit 1)]
theorem okTakeIntoCustody : Instruction.check (DRS.ofList []) takeIntoCustody = [] := by decide

/-- Frenzied Gorespawn -/
def frenziedGorespawnGoad : Instruction :=
  Primitives.Instruction.doForEach (each Primitives.Predicate.opponent)
    (Primitives.Instruction.gainDesignation
      (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller (that .player)])) (.named "goaded")
      none)
theorem okFrenziedGorespawnGoad : Instruction.check (DRS.ofList []) frenziedGorespawnGoad = [] := by decide

def spaceTimeAnomaly : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Space-Time Anomaly", cost := some [generic 2, pip .white, pip .blue],
      types := [.sorcery],
      text := [ Primitives.Ability.spell none (instruct (target Primitives.Predicate.anyPlayer) (mill (lifeTotalOf Primitives.NounPhrase.you) they)) ] } }

/-- Consecrate // Consume -/
def consume : Ability :=
  Primitives.Ability.spell none (Primitives.Instruction.sequentially
    [ instruct (target Primitives.Predicate.anyPlayer) (sacrifice
        (a (Primitives.Predicate.and [ creature,
                   Primitives.Predicate.superlative .max (.stat .power)
                     (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller they]) ]))),
      gainLife (Primitives.Amount.statOf (.stat .power) it) ])
theorem okConsume : Ability.check (DRS.ofList []) consume = [] := by decide

def bifurcate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bifurcate", cost := some [generic 3, pip .green], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ searchLibraryFor (exactly 1)
                (Primitives.Predicate.and [permanentCard, (Primitives.Predicate.shares .cardName .share (target (Primitives.Predicate.and [creature, nontoken])))]),
              putOntoBattlefield (that .card),
              shuffle ]) ] } }

/-- Mana Leak {1}{U} — Instant. "Counter target spell unless its controller pays {3}." -/
def manaLeak : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mana Leak", cost := some [generic 1, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (instruct (controllerOf (target spell)) (doUnless
            (Primitives.Instruction.counterSpell it)
            (Primitives.Cost.mana [generic 3]))) ] } }

/-- Oust {W} — Sorcery. "Put target creature into its owner's library second from the top.
Its controller gains 3 life." -/
def oust : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Oust", cost := some [pip .white], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ Primitives.Instruction.move (target creature) Primitives.EventSource.anywhere (some onTop) .arrival none [Primitives.MoveRider.placement (Primitives.PilePlacement.nthFromEnd (.nth 2))],
              instruct (controllerOf it) (gainLife (.lit 3)) ]) ] } }

def riseFromTheGrave : Instruction :=
  Primitives.Instruction.sequentially
    [ putOntoBattlefieldUnderYourControl (target (Primitives.Predicate.and [creature, Primitives.Predicate.inZone graveyard])),
      become (that (.type .creature))
        { characteristics := { colors := [.black], subtypes := [creatureType "Zombie"] } } none ]
theorem okRiseFromTheGrave : Instruction.check (DRS.ofList []) riseFromTheGrave = [] := by decide

def everAfter : Instruction :=
  Primitives.Instruction.sequentially
    [ move (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 2)) (Primitives.Predicate.and [creature, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)]))
        battlefield,
      become (those (.type .creature))
        { characteristics := { colors := [.black], subtypes := [creatureType "Zombie"] } } none,
      move Primitives.NounPhrase.this onBottom ]
theorem okEverAfter : Instruction.check (DRS.ofList []) everAfter = [] := by decide

def martyrsCry : Instruction :=
  Primitives.Instruction.sequentially
    [ exile (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.colorIs .white])),
      Primitives.Instruction.doForEach (theVerbed (.action "exile") .many (some (.type .creature)) .thisWay)
        (instruct (controllerOf it) (Primitives.Instruction.draw (.lit 1))) ]
theorem okMartyrsCry : Instruction.check (DRS.ofList []) martyrsCry = [] := by decide

def anotherRound : Instruction :=
  Primitives.Instruction.sequentially
    [ exile (counted anyNumber creatureYouControl),
      Primitives.Instruction.move them Primitives.EventSource.anywhere (some battlefield) .arrival none [Primitives.MoveRider.entry (Primitives.EntryRider.under (ownerOf them))],
      Primitives.Instruction.repeat_ (Primitives.Repetition.moreTimes (Primitives.Amount.letter .x)) ]
theorem okAnotherRound : Instruction.check (DRS.ofList []) anotherRound = [] := by decide

/-- Eradicate -/
def eradicateSearch : Instruction :=
  Primitives.Instruction.sequentially
    [ exile (target (Primitives.Predicate.and [creature, Primitives.Predicate.not (Primitives.Predicate.colorIs .black)])),
      searchZonesOf (controllerOf (that .card)) anyNumber ((Primitives.Predicate.shares .cardName .share (that .card))),
      exile (theVerbed (.action "search") .many),
      instruct (that .player) shuffle ]
theorem okEradicateSearch : Instruction.check (DRS.ofList []) eradicateSearch = [] := by decide

/-- Deem Inferior -/
def deemInferior : Instruction :=
  instruct (ownerOf (target (Primitives.Predicate.and [permanent, Primitives.Predicate.not land]))) (putWith it onTop [Primitives.MoveRider.placement (Primitives.PilePlacement.nthFromEnd (.nth 2)), Primitives.MoveRider.placement Primitives.PilePlacement.orOtherEnd])
theorem okDeemInferior : Instruction.check (DRS.ofList []) deemInferior = [] := by decide

/-- Write into Being -/
def writeIntoBeingPlacement : Instruction :=
  Primitives.Instruction.sequentially
    [ lookAt (topSlice (.lit 2)),
      manifestPlacement (someOf (exactly 1) them),
      Primitives.Instruction.move (theOther .object) Primitives.EventSource.anywhere (some onTop) .arrival none [Primitives.MoveRider.placement Primitives.PilePlacement.orOtherEnd] ]
theorem okWriteIntoBeingPlacement : Instruction.check (DRS.ofList []) writeIntoBeingPlacement = [] := by decide

/-- Mystical Tutor -/
def mysticalTutor : Instruction :=
  Primitives.Instruction.sequentially
    [searchLibraryFor (exactly 1) instantOrSorcery, revealCards it, shuffle, move it onTop]
theorem okMysticalTutor : Instruction.check (DRS.ofList []) mysticalTutor = [] := by decide

/-- Demonic Tutor -/
def demonicTutor : Instruction :=
  Primitives.Instruction.sequentially [searchLibraryFor (exactly 1) (Primitives.Predicate.and []), move it hand, shuffle]
theorem okDemonicTutor : Instruction.check (DRS.ofList []) demonicTutor = [] := by decide

/-- Thalia's Lancers -/
def thaliasLancersSearch : Instruction :=
  offer
    (Primitives.Instruction.sequentially
      [ searchLibraryFor (exactly 1) (Primitives.Predicate.and [Primitives.Predicate.hasSupertype .legendary]), revealCards it,
        move it hand, shuffle ])
theorem okThaliasLancersSearch : Instruction.check (DRS.ofList []) thaliasLancersSearch = [] := by decide

def contrabandLivestock : Instruction :=
  Primitives.Instruction.sequentially
    [ exile (target creature),
      rollDice 1 20,
      Primitives.Instruction.applyResultsTable
        [ rollRow (fromTo 1 9)
            (instruct (controllerOf it) (Primitives.Instruction.create (.lit 1)
              (Primitives.TokenSpec.written (creatureToken 4 4 [.green] [creatureType "Ox"])) [])),
          rollRow (fromTo 10 19)
            (instruct (controllerOf it) (Primitives.Instruction.create (.lit 1)
              (Primitives.TokenSpec.written (creatureToken 2 2 [.green] [creatureType "Boar"])) [])),
          rollRow (exactly 20)
            (instruct (controllerOf it) (Primitives.Instruction.create (.lit 1)
              (Primitives.TokenSpec.written (creatureToken 0 1 [.white] [creatureType "Goat"])) [])) ] ]
theorem okContrabandLivestock : Instruction.check (DRS.ofList []) contrabandLivestock = [] := by decide

/-- Hypnotic Specter -/
def hypnoticSpecterDiscard : Instruction := discard (aAtRandom (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.inZone (handOf they)]))
theorem okHypnoticSpecterDiscard :
    Instruction.check (DRS.ofList [{ det := .the, plur := .one, payload := .player false }]) hypnoticSpecterDiscard = [] := by decide

/-- Wyll, Blade of Frontiers -/
def wyllExtraDie : Instruction :=
  replaceEvent (Primitives.WouldClause.would (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Instruction.rollDice Primitives.Amount.anyAmount .anyDie .many Primitives.RollWatch.anyResult))
    (Primitives.Instruction.sequentially
      [ Primitives.Instruction.rollDice (plus Primitives.Amount.thatMuch (.lit 1)) .thoseDice .many .anyResult,
        Primitives.Instruction.ignoreOutcomes (Primitives.IgnoredOutcomes.extreme .lowest) ])
    none
theorem okWyllExtraDie : Instruction.check (DRS.ofList []) wyllExtraDie = [] := by decide

/-- Vedalken Squirrel-Whacker -/
def vedalkenSquirrelWhackerReroll : Instruction :=
  replaceEvent (Primitives.WouldClause.would (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Instruction.rollDice Primitives.Amount.anyAmount (.sides 6) .many Primitives.RollWatch.anyResult))
    (Primitives.Instruction.sequentially
      [ Primitives.Instruction.rollDice Primitives.Amount.thatMuch .thoseDice .many .anyResult,
        offer
          (Primitives.Instruction.exchange (Primitives.Exchanged.values (Primitives.Amount.theOutcome .rollResult) (Primitives.Amount.statOf (.stat .power) thisCreature))) ])
    none
theorem okVedalkenSquirrelWhackerReroll :
    Instruction.check (DRS.ofList []) vedalkenSquirrelWhackerReroll = [] := by decide

/-- Investigator's Journal's count -/
def greatestCreaturesAPlayerControls : Amount :=
  Primitives.Amount.aggregateOver .max Primitives.Predicate.anyPlayer (countOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller they]))
theorem okGreatestCreaturesAPlayerControls :
    Amount.check (DRS.ofList []) greatestCreaturesAPlayerControls = [] := by decide
/-- Cavern-Hoard Dragon -/
def greatestArtifactsAnOpponentControls : Amount :=
  Primitives.Amount.aggregateOver .max Primitives.Predicate.opponent (countOf (Primitives.Predicate.and [artifact, Primitives.Predicate.hasPossessor .controller they]))
theorem okGreatestArtifactsAnOpponentControls :
    Amount.check (DRS.ofList []) greatestArtifactsAnOpponentControls = [] := by decide

/-- "each creature that player or that planeswalker's controller controls" -/
def eachCreatureThatSplitControls : NounPhrase :=
  each (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller splitOverPlaneswalker])

def anyTargetAnnounced : NounPhrase := target anyTarget
theorem okAnyTargetAnnounced : NounPhrase.checkAt none (DRS.ofList []) anyTargetAnnounced = [] := by decide
def playerOrPlaneswalkerAnnounced : NounPhrase := targetPlayerOrPlaneswalker
/-- Chain of Plasma -/
def chainOfPlasmaOfferee : NounPhrase := splitOverPermanent
theorem okChainOfPlasmaOfferee :
    NounPhrase.checkAt (some .player) (((stepNounPhrase 512 (DRS.ofList []) .obj true anyTargetAnnounced).1)) chainOfPlasmaOfferee = [] := by
  decide
/-- Chain Lightning -/
def chainLightningPayer : NounPhrase := splitOverPermanent
theorem okChainLightningPayer :
    NounPhrase.checkAt (some .player) (((stepNounPhrase 512 (DRS.ofList []) .obj true anyTargetAnnounced).1)) chainLightningPayer = [] := by
  decide
/-- Flames of the Blood Hand -/
def flamesOfTheBloodHandSubject : NounPhrase := splitOverPlaneswalker
theorem okFlamesOfTheBloodHandSubject :
    NounPhrase.checkAt (some .player) (((stepNounPhrase 512 (DRS.ofList []) .obj true playerOrPlaneswalkerAnnounced).1)) flamesOfTheBloodHandSubject = [] := by
  decide
/-- Flaming Gambit -/
def flamingGambitOfferee : NounPhrase := splitOverPlaneswalker
theorem okFlamingGambitOfferee :
    NounPhrase.checkAt (some .player) (((stepNounPhrase 512 (DRS.ofList []) .obj true playerOrPlaneswalkerAnnounced).1)) flamingGambitOfferee = [] := by
  decide
theorem okEachCreatureThatSplitControls :
    NounPhrase.checkAt (some .object) (((stepNounPhrase 512 (DRS.ofList []) .obj true playerOrPlaneswalkerAnnounced).1)) eachCreatureThatSplitControls = [] := by
  decide

/-- Ertai's Trickery -/
def ertaisTrickery : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ertai's Trickery", cost := some [pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.doOnlyIf (Primitives.Instruction.counterSpell (target spell))
            (costWasPaid (.byKeyword (.named "kicker")) none it) none) ] } }

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
theorem okOpponentWithMoreLands : Predicate.checkAt .player (DRS.ofList []) opponentWithMoreLands = [] := by
  decide

def bioplasmCardTest : Condition := Primitives.Condition.matches itCard creature
theorem okBioplasmCardTest : Condition.check bioplasmAfterExile bioplasmCardTest = [] := by decide

/-- Blessed Respite -/
def blessedRespiteShuffle : Instruction :=
  instruct (target Primitives.Predicate.anyPlayer) (shuffleInto (allOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.inZone (graveyardOf they)])))
theorem okBlessedRespiteShuffle : Instruction.check (DRS.ofList []) blessedRespiteShuffle = [] := by decide

/-- Nekrataal -/
def nekrataalEntry : GameEvent := Primitives.GameEvent.enters thisCreature none
theorem okNekrataalEntry : GameEvent.check (DRS.ofList []) nekrataalEntry = [] := by decide
def nekrataalDestroy : Instruction :=
  destroy (target (Primitives.Predicate.and [creature, Primitives.Predicate.not artifact, Primitives.Predicate.not (Primitives.Predicate.colorIs .black)]))
theorem okNekrataalDestroy :
    Instruction.check ((stepGameEvent 512 (DRS.ofList []) nekrataalEntry).1) nekrataalDestroy = [] := by decide
def nekrataalRider : Bindings :=
  (stepInstruction 512 .postEvent .obj ((stepGameEvent 512 (DRS.ofList []) nekrataalEntry).1) nekrataalDestroy).1
theorem nekrataalOneCreatureWord : countReach (.word (.type .creature)) .one nekrataalRider = 1 := by
  decide
theorem nekrataalOneDestroyed : countReach (.verbed (.action "destroy") none .attributive) .one nekrataalRider = 1 := by
  decide

def sequencedRiderBody : Instruction :=
  Primitives.Instruction.sequentially [exile (target artifact), destroy (target creature)]
theorem okSequencedRiderBody : Instruction.check (DRS.ofList []) sequencedRiderBody = [] := by decide
def sequencedRider : Bindings := (stepInstruction 512 .postEvent .obj (DRS.ofList []) sequencedRiderBody).1
theorem sequencedRiderOneDestroyed :
    countReach (.verbed (.action "destroy") none .attributive) .one sequencedRider = 1 := by decide
theorem sequencedRiderOneExiled : countReach (.verbed (.action "exile") none .attributive) .one sequencedRider = 1 := by
  decide

/-- Engulfing Flames -/
def engulfingFlamesBody : Instruction := instruct Primitives.NounPhrase.this (Primitives.Instruction.dealDamage (.lit 1) (target creature) .noncombatOnly)
theorem okEngulfingFlamesBody : Instruction.check (DRS.ofList []) engulfingFlamesBody = [] := by decide
def engulfingFlamesRider : Bindings := (stepInstruction 512 .postEvent .obj (DRS.ofList []) engulfingFlamesBody).1
theorem engulfingFlamesNoDestroyStamp :
    countReach (.verbed (.action "destroy") none .attributive) .one engulfingFlamesRider = 0 := by decide
theorem engulfingFlamesBareReadStands : countReach .bare .one engulfingFlamesRider = 1 := by decide

def bioplasmExiledPronoun : NounPhrase := theVerbed (.action "exile") .one
theorem okBioplasmExiledPronoun :
    NounPhrase.checkAt (some .object) (bioplasmAfterExile) bioplasmExiledPronoun = [] := by decide
theorem bioplasmNoTypedRead :
    countReach (.verbed (.action "exile") (some (.type .creature)) .attributive) .one bioplasmAfterExile
      = 0 := by
  decide
theorem bioplasmExiledCardHasNoType :
    tyOfReach (.verbed (.action "exile") none .attributive) .one bioplasmAfterExile = [] := by decide
def bioplasmAfterTest : Bindings := (stepCondition 512 bioplasmAfterExile bioplasmCardTest).1
theorem bioplasmTestRemarksType :
    tyOfReach (.verbed (.action "exile") none .attributive) .one bioplasmAfterTest = [.creature] := by decide
theorem bioplasmTestKeepsCardSlot : countReach (.atSlot .card) .one bioplasmAfterTest = 1 := by
  decide
theorem bioplasmTypedReadStillRefused :
    countReach (.verbed (.action "exile") (some (.type .creature)) .attributive) .one bioplasmAfterTest
      = 0 := by
  decide
theorem bioplasmTypedCardReadWrites :
    countReach (.verbed (.action "exile") (some (.ofType .card .creature)) .attributive) .one
      bioplasmAfterTest = 1 := by
  decide

/-- Scapeshift -/
def scapeshiftSacrifice : Instruction := sacrifice (counted anyNumber land)
theorem okScapeshiftSacrifice : Instruction.check (DRS.ofList []) scapeshiftSacrifice = [] := by decide
def scapeshiftSacrificed : Bindings := (stepInstruction 512 .postEvent .obj (DRS.ofList []) scapeshiftSacrifice).1
def scapeshiftSearch : Instruction := searchLibraryFor (Primitives.Quantity.upToOf Primitives.Amount.groupSize) land
theorem okScapeshiftSearch : Instruction.check scapeshiftSacrificed scapeshiftSearch = [] := by
  decide
def scapeshiftAfterSearch : Bindings := (stepInstruction 512 .postEvent .obj scapeshiftSacrificed scapeshiftSearch).1
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
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ sacrifice (counted anyNumber land),
              searchLibraryFor (Primitives.Quantity.upToOf Primitives.Amount.groupSize) land,
              putOntoBattlefieldTapped (theVerbed (.action "search") .many),
              shuffle ]) ] } }

/-- Bind to Life, Vastlands Scavenger's adventure -/
def bindToLife : Instruction :=
  Primitives.Instruction.sequentially
    [mill (.lit 7) Primitives.NounPhrase.you, move (fromAmong (exactly 1) creature them) battlefield]
theorem okBindToLife : Instruction.check (DRS.ofList []) bindToLife = [] := by decide

/-- Glamdring, Foe-hammer's Gleam of Death -/
def gleamOfDeath : Instruction :=
  Primitives.Instruction.sequentially
    [mill (.lit 6) Primitives.NounPhrase.you, move (allFromAmong (Primitives.Predicate.or [instant, sorcery]) them) hand]
theorem okGleamOfDeath : Instruction.check (DRS.ofList []) gleamOfDeath = [] := by decide

/-- Tezzeret, Master of the Bridge -/
def tezzeretAllArtifacts : Instruction :=
  Primitives.Instruction.sequentially
    [ exile (topSlice (.lit 10)),
      move (allFromAmong artifact (theVerbed (.action "exile") .many (some .card) .thisWay)) battlefield ]
theorem okTezzeretAllArtifacts : Instruction.check (DRS.ofList []) tezzeretAllArtifacts = [] := by decide

def companyContext : Bindings := (stepInstruction 512 .postEvent .obj (DRS.ofList []) (lookAt (topSlice (.lit 6)))).1
def companyDescribedSlice : NounPhrase :=
  fromAmong (upTo 2) (Primitives.Predicate.and [creature, Primitives.Predicate.compare [.stat .manaValue] .atMost (.lit 3)]) them
theorem okCompanyDescribedSlice :
    NounPhrase.checkAt (some .object) (companyContext) companyDescribedSlice = [] := by decide
def companyBareSlice : NounPhrase := someOf (exactly 2) them
theorem okCompanyBareSlice : NounPhrase.checkAt (some .object) (companyContext) companyBareSlice = [] := by
  decide

/-- Lord of the Void -/
def exileTopThenPutFromAmong : Instruction :=
  Primitives.Instruction.sequentially
    [ exile (Primitives.NounPhrase.slice .top (.lit 7) (library (target Primitives.Predicate.opponent))),
      putOntoBattlefieldUnderYourControl (fromAmong (exactly 1) creature them) ]
theorem okExileTopThenPutFromAmong : Instruction.check (DRS.ofList []) exileTopThenPutFromAmong = [] := by decide

/-- Thought Sponge -/
def greatestCardsAnOpponentDrew : Amount :=
  Primitives.Amount.aggregateOver .max Primitives.Predicate.opponent (eventCount (Primitives.Instruction.draw Primitives.Amount.anyAmount) (Primitives.Subject.actor they) soFarThisTurn)
theorem okGreatestCardsAnOpponentDrew : Amount.check (DRS.ofList []) greatestCardsAnOpponentDrew = [] := by
  decide
def greatestCardsAPlayerDiscardedThisWay : Amount :=
  Primitives.Amount.aggregateOver .max Primitives.Predicate.anyPlayer
    (Primitives.Amount.tally (⟨(Primitives.Subject.actor they), (discard ((allOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.inZone hand])))), Primitives.LookbackSpan.thisWay⟩) .events)
theorem okGreatestCardsAPlayerDiscardedThisWay :
    Amount.check (DRS.ofList []) greatestCardsAPlayerDiscardedThisWay = [] := by decide

def eachPlayerBase : NounPhrase := each Primitives.Predicate.anyPlayer
theorem okEachPlayerBase : NounPhrase.checkAt (some .player) (DRS.ofList []) eachPlayerBase = [] := by decide
theorem eachPlayerBindsNoSingular : countOnes .player ((stepNounPhrase 512 (DRS.ofList []) .obj true eachPlayerBase).1) = 0 := by decide
theorem eachPlayerBindsAGroup : countManys .player ((stepNounPhrase 512 (DRS.ofList []) .obj true eachPlayerBase).1) = 1 := by decide

/-- Soul Ransom -/
def soulRansomRansom : Instruction :=
  Primitives.Instruction.sequentially [instruct (controllerOf thisAura) (sacrifice itPermanent), instruct they (Primitives.Instruction.draw (.lit 2))]
theorem okSoulRansomRansom : Instruction.check (DRS.ofList []) soulRansomRansom = [] := by decide
def thisAurasController : NounPhrase := controllerOf thisAura
theorem okThisAurasController : NounPhrase.checkAt (some .player) (DRS.ofList []) thisAurasController = [] := by
  decide
def targetCreaturesController : NounPhrase := controllerOf (target creature)
theorem okTargetCreaturesController :
    NounPhrase.checkAt (some .player) (DRS.ofList []) targetCreaturesController = [] := by decide
theorem possessiveDeicticIsReadableByIt :
    countReach (.atSlot .permanent) .one ((stepNounPhrase 512 (DRS.ofList []) .obj true thisAurasController).1) = 1 := by decide
theorem possessiveDeicticIsNotADemonstrative :
    countReach (.word (.type .enchantment)) .one ((stepNounPhrase 512 (DRS.ofList []) .obj true thisAurasController).1) = 0 := by decide
theorem possessiveDescribedBaseUnchanged :
    countReach (.atSlot .permanent) .one ((stepNounPhrase 512 (DRS.ofList []) .obj true targetCreaturesController).1) = 1 := by decide

/-- Bile Blight -/
def bileBlight : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bile Blight", cost := some [pip .black, pip .black], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (get
            (Primitives.NounPhrase.both (target creature)
              (allOf (Primitives.Predicate.and [ creature, (Primitives.Predicate.shares .cardName .share (that (.type .creature))),
                             Primitives.Predicate.otherThan (that (.type .creature)) ])))
            (Primitives.Delta.down (.lit 3)) (Primitives.Delta.down (.lit 3)) (some untilEndOfTurn)) ] } }

/-- Echoing Ruin -/
def echoingRuin : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Echoing Ruin", cost := some [generic 1, pip .red], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (destroy
            (Primitives.NounPhrase.both (target artifact)
              (allOf (Primitives.Predicate.and [ artifact, (Primitives.Predicate.shares .cardName .share (that (.type .artifact))),
                             Primitives.Predicate.otherThan (that (.type .artifact)) ])))) ] } }

/-- Hijack -/
def hijack : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hijack", cost := some [generic 1, pip .red, pip .red], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ gainControl (target (Primitives.Predicate.or [artifact, creature])) (some untilEndOfTurn) (agent := Primitives.NounPhrase.you),
              untap (theVerbed (.core .gainControl) .one),
              gainHaste (theVerbed (.action "untap") .one) (some untilEndOfTurn) ]) ] } }

/-- Open the Vaults -/
def openTheVaults : Instruction :=
  returnTo
    (allOf (Primitives.Predicate.and [ Primitives.Predicate.isCard, Primitives.Predicate.or [artifact, enchantment],
                   Primitives.Predicate.inZone (graveyardOf (Primitives.NounPhrase.playerGroup .allPlayers)) ]))
    battlefield [Primitives.EntryRider.under (ownerOf them)]
theorem okOpenTheVaults : Instruction.check (DRS.ofList []) openTheVaults = [] := by decide

/-- Codecracker Hound -/
def codecrackerHoundLook : Instruction :=
  Primitives.Instruction.sequentially
    [ lookAt (topSlice (.lit 2)), move (someOf (exactly 1) them) hand,
      move (theOther .object) graveyard ]
theorem okCodecrackerHoundLook : Instruction.check (DRS.ofList []) codecrackerHoundLook = [] := by decide

def oneOfTheTopTwo : NounPhrase := someOf (exactly 1) (Primitives.NounPhrase.slice .top (.lit 2) (library Primitives.NounPhrase.you))
theorem okOneOfTheTopTwo : NounPhrase.checkAt (some .object) (DRS.ofList []) oneOfTheTopTwo = [] := by decide
def oneOfAnUncountedTop : NounPhrase :=
  someOf (exactly 1) (Primitives.NounPhrase.slice .top (countOf creature) (library Primitives.NounPhrase.you))
theorem okOneOfAnUncountedTop : NounPhrase.checkAt (some .object) (DRS.ofList []) oneOfAnUncountedTop = [] := by
  decide
def theArtifactsAmongTheTopFive : NounPhrase :=
  allAmong artifact (Primitives.NounPhrase.slice .top (.lit 5) (library Primitives.NounPhrase.you))
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

def eachPlayerShufflesTheirHandAndGraveyard : Instruction :=
  instruct (each Primitives.Predicate.anyPlayer) (shuffleInto
    (Primitives.NounPhrase.both (allOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.inZone (handOf they)])) (allOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.inZone (graveyardOf they)]))))
theorem okEachPlayerShufflesTheirHandAndGraveyard :
    Instruction.check (DRS.ofList []) eachPlayerShufflesTheirHandAndGraveyard = [] := by decide

/-- Fact or Fiction -/
def factOrFiction : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fact or Fiction", cost := some [generic 3, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ revealCards (topSlice (.lit 5)),
              instruct anOpponent (Primitives.Instruction.separateIntoPiles them 2 []),
              move onePile hand,
              move (theOther .pile) graveyard ]) ] } }

def unsummon : Instruction := move (target creature) hand
theorem okUnsummon : Instruction.check (DRS.ofList []) unsummon = [] := by decide
def grismold : Instruction :=
  instruct (each Primitives.Predicate.anyPlayer) (Primitives.Instruction.create (.lit 1)
    (Primitives.TokenSpec.written (creatureToken 1 1 [.green] [creatureType "Plant"])) [])
theorem okGrismold : Instruction.check (DRS.ofList []) grismold = [] := by decide
/-- Grenzo, Dungeon Warden -/
def grenzoBottomCard : Instruction := move bottomCard graveyard
theorem okGrenzoBottomCard : Instruction.check (DRS.ofList []) grenzoBottomCard = [] := by decide

def chronostutter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chronostutter", cost := some [generic 5, pip .blue], types := [.instant],
      text := [ Primitives.Ability.spell none (Primitives.Instruction.move (target creature) Primitives.EventSource.anywhere (some onTop) .arrival none [Primitives.MoveRider.placement (Primitives.PilePlacement.nthFromEnd (.nth 2))]) ] } }

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
      text := [ Primitives.Ability.spell none (instruct (Primitives.NounPhrase.eachOf (Primitives.NounPhrase.both Primitives.NounPhrase.you (target Primitives.Predicate.opponent))) (Primitives.Instruction.draw (.lit 3))) ]
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
theorem okArcumDagssonSacrifice : Instruction.check (DRS.ofList []) arcumDagssonSacrifice = [] := by decide

/-- Vaevictis Asmadi, the Dire -/
def vaevictisAsmadiTheDire : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vaevictis Asmadi, the Dire",
      cost := some [generic 3, pip .black, pip .red, pip .green],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Elder", creatureType "Dragon"],
      text :=
        [ keyword (.named "flying"),
          whenever (attacks thisCreature)
            (Primitives.Instruction.sequentially
              [ Primitives.Instruction.doForEach (each Primitives.Predicate.anyPlayer)
                  (choose (target (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller they]))),
                instruct (those .player) (sacrifice (those .permanent)),
                Primitives.Instruction.doForEach
                  (each (Primitives.Predicate.and [ Primitives.Predicate.anyPlayer,
                                Primitives.Predicate.did (⟨(Primitives.Subject.actor (relative .player)), (sacrifice ((a permanent))), Primitives.LookbackSpan.thisWay⟩)
                                  ]))
                  (Primitives.Instruction.sequentially
                    [ instruct they (Primitives.Instruction.expose .reveal (Primitives.Exposed.cards (Primitives.NounPhrase.slice .top (.lit 1) (library they)))),
                      Primitives.Instruction.doIf (Primitives.Condition.matches itCard permanentCard) (move itCard battlefield)
                        none ]) ]) ],
      power := stat 6, toughness := stat 6 } }

end Semantics.Cards
