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

def cloudshift : Action :=
  Primitives.Action.sequentially
    [exile (target creatureYouControl), putOntoBattlefieldUnderYourControl (that .card)]
theorem okCloudshift : Action.check (DRS.ofList []) cloudshift = [] := by decide

def bitterDownfall : Action :=
  Primitives.Action.sequentially [destroy (target creature), instruct (controllerOf it) (loseLife (.lit 2))]
theorem okBitterDownfall : Action.check (DRS.ofList []) bitterDownfall = [] := by decide

def suspendedSentence : Action :=
  Primitives.Action.sequentially
    [ destroy (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller anOpponent])),
      instruct (that .player) (loseLife (.lit 3)) ]
theorem okSuspendedSentence : Action.check (DRS.ofList []) suspendedSentence = [] := by decide

def flickeringSpirit : Action :=
  Primitives.Action.sequentially [exile thisCreature, Primitives.Action.move it Primitives.EventSource.anywhere (some battlefield) .arrival none [Primitives.MoveRider.entry (Primitives.EntryRider.under (ownerOf it))]]
theorem okFlickeringSpirit : Action.check (DRS.ofList []) flickeringSpirit = [] := by decide

/-- Bond of Revival -/
def bondOfRevival : Action :=
  Primitives.Action.sequentially
    [ returnToBattlefield (target (Primitives.Predicate.and [creature, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)])),
      gainHaste (theVerbed (.core .return_) .one) (some untilYourNextTurn) ]
theorem okBondOfRevival : Action.check (DRS.ofList []) bondOfRevival = [] := by decide

def vraskasStoneglare : Action :=
  Primitives.Action.sequentially [destroy (target creature), gainLife (Primitives.Amount.statOf (.stat .toughness) it)]
theorem okVraskasStoneglare : Action.check (DRS.ofList []) vraskasStoneglare = [] := by decide

def phthisis : Action :=
  Primitives.Action.sequentially
    [ destroy (target creature),
      instruct (controllerOf it) (loseLife
        (plus (Primitives.Amount.statOf (.stat .power) it) (Primitives.Amount.statOf (.stat .toughness) it))) ]
theorem okPhthisis : Action.check (DRS.ofList []) phthisis = [] := by decide

def foulTongueShriek : Action :=
  Primitives.Action.sequentially
    [ instruct (target Primitives.Predicate.opponent) (loseLife
        (forEach 1 (Primitives.Predicate.and [attacking, creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))),
      gainLife Primitives.Amount.thatMuch ]
theorem okFoulTongueShriek : Action.check (DRS.ofList []) foulTongueShriek = [] := by decide

def phyrexianInfiltrator : Action :=
  Primitives.Action.simultaneously
    [ gainControl thisCreature none (agent := (controllerOf (target creature))),
      gainControl (that (.type .creature)) none (agent := (controllerOf thisCreature)) ]
theorem okPhyrexianInfiltrator : Action.check (DRS.ofList []) phyrexianInfiltrator = [] := by decide

def impulse : Action :=
  Primitives.Action.sequentially
    [ lookAt (topSlice (.lit 4)), move (someOf (exactly 1) them) hand,
      Primitives.Action.move (theRest .object) Primitives.EventSource.anywhere (some onBottom) .arrival none [Primitives.MoveRider.placement (Primitives.PilePlacement.inOrder .anyOrder)] ]
theorem okImpulse : Action.check (DRS.ofList []) impulse = [] := by decide

def anticipate : Action :=
  Primitives.Action.sequentially
    [ lookAt (topSlice (.lit 3)), move (someOf (exactly 1) them) hand,
      Primitives.Action.move (theRest .object) Primitives.EventSource.anywhere (some onBottom) .arrival none [Primitives.MoveRider.placement (Primitives.PilePlacement.inOrder .anyOrder)] ]
theorem okAnticipate : Action.check (DRS.ofList []) anticipate = [] := by decide

def revealFourPartition : Action :=
  Primitives.Action.sequentially
    [ revealCards (topSlice (.lit 4)), move (someOf (exactly 1) (those .card)) hand,
      move (theRest .object) graveyard ]
theorem okRevealFourPartition : Action.check (DRS.ofList []) revealFourPartition = [] := by decide

def exileFourOfThem : Action :=
  Primitives.Action.sequentially
    [ lookAt (topSlice (.lit 8)), exile (someOf (exactly 4) them),
      Primitives.Action.move (theRest .object) Primitives.EventSource.anywhere (some onTop) .arrival none [Primitives.MoveRider.placement (Primitives.PilePlacement.inOrder .anyOrder)] ]
theorem okExileFourOfThem : Action.check (DRS.ofList []) exileFourOfThem = [] := by decide

def sylvanScrying : Action :=
  Primitives.Action.sequentially [searchLibraryFor (exactly 1) land, revealCards it, move it hand, shuffle]
theorem okSylvanScrying : Action.check (DRS.ofList []) sylvanScrying = [] := by decide

def searchToBattlefield : Action :=
  Primitives.Action.sequentially [searchLibraryFor (exactly 1) creature, move it battlefield, shuffle]
theorem okSearchToBattlefield : Action.check (DRS.ofList []) searchToBattlefield = [] := by decide

def glimpseTheUnthinkable : Action := instruct (target Primitives.Predicate.anyPlayer) (mill (.lit 10) they)
theorem okGlimpseTheUnthinkable : Action.check (DRS.ofList []) glimpseTheUnthinkable = [] := by decide

def millThenReadGroup : Action :=
  Primitives.Action.sequentially [mill (.lit 3) Primitives.NounPhrase.you, exile (those .card)]
theorem okMillThenReadGroup : Action.check (DRS.ofList []) millThenReadGroup = [] := by decide

def takeIntoCustody : Action :=
  Primitives.Action.sequentially [Primitives.Action.setStatus .tapped (target creature), Primitives.Action.skipUntap it (.lit 1)]
theorem okTakeIntoCustody : Action.check (DRS.ofList []) takeIntoCustody = [] := by decide

/-- Frenzied Gorespawn -/
def frenziedGorespawnGoad : Action :=
  Primitives.Action.doForEach (each Primitives.Predicate.opponent)
    (Primitives.Action.gainDesignation
      (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller (that .player)])) (.named "goaded")
      none)
theorem okFrenziedGorespawnGoad : Action.check (DRS.ofList []) frenziedGorespawnGoad = [] := by decide

def spaceTimeAnomaly : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Space-Time Anomaly", cost := some [generic 2, pip .white, pip .blue],
      types := [.sorcery],
      text := [ Primitives.Ability.spell none (instruct (target Primitives.Predicate.anyPlayer) (mill (lifeTotalOf Primitives.NounPhrase.you) they)) ] } }

/-- Consecrate // Consume -/
def consume : Ability :=
  Primitives.Ability.spell none (Primitives.Action.sequentially
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
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
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
            (Primitives.Action.counterSpell it)
            (Primitives.Cost.mana [generic 3]))) ] } }

/-- Oust {W} — Sorcery. "Put target creature into its owner's library second from the top.
Its controller gains 3 life." -/
def oust : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Oust", cost := some [pip .white], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ Primitives.Action.move (target creature) Primitives.EventSource.anywhere (some onTop) .arrival none [Primitives.MoveRider.placement (Primitives.PilePlacement.nthFromEnd (.nth 2))],
              instruct (controllerOf it) (gainLife (.lit 3)) ]) ] } }

def riseFromTheGrave : Action :=
  Primitives.Action.sequentially
    [ putOntoBattlefieldUnderYourControl (target (Primitives.Predicate.and [creature, Primitives.Predicate.inZone graveyard])),
      become (that (.type .creature))
        { characteristics := { colors := [.black], subtypes := [creatureType "Zombie"] } } none ]
theorem okRiseFromTheGrave : Action.check (DRS.ofList []) riseFromTheGrave = [] := by decide

def everAfter : Action :=
  Primitives.Action.sequentially
    [ move (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 2)) (Primitives.Predicate.and [creature, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)]))
        battlefield,
      become (those (.type .creature))
        { characteristics := { colors := [.black], subtypes := [creatureType "Zombie"] } } none,
      move Primitives.NounPhrase.this onBottom ]
theorem okEverAfter : Action.check (DRS.ofList []) everAfter = [] := by decide

def martyrsCry : Action :=
  Primitives.Action.sequentially
    [ exile (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.colorIs .white])),
      Primitives.Action.doForEach (theVerbed (.action "exile") .many (some (.type .creature)) .thisWay)
        (instruct (controllerOf it) (Primitives.Action.draw (.lit 1))) ]
theorem okMartyrsCry : Action.check (DRS.ofList []) martyrsCry = [] := by decide

def anotherRound : Action :=
  Primitives.Action.sequentially
    [ exile (counted anyNumber creatureYouControl),
      Primitives.Action.move them Primitives.EventSource.anywhere (some battlefield) .arrival none [Primitives.MoveRider.entry (Primitives.EntryRider.under (ownerOf them))],
      Primitives.Action.repeat_ (Primitives.Repetition.moreTimes (Primitives.Amount.letter .x)) ]
theorem okAnotherRound : Action.check (DRS.ofList []) anotherRound = [] := by decide

/-- Eradicate -/
def eradicateSearch : Action :=
  Primitives.Action.sequentially
    [ exile (target (Primitives.Predicate.and [creature, Primitives.Predicate.not (Primitives.Predicate.colorIs .black)])),
      searchZonesOf (controllerOf (that .card)) anyNumber ((Primitives.Predicate.shares .cardName .share (that .card))),
      exile (theVerbed (.action "search") .many),
      instruct (that .player) shuffle ]
theorem okEradicateSearch : Action.check (DRS.ofList []) eradicateSearch = [] := by decide

/-- Deem Inferior -/
def deemInferior : Action :=
  instruct (ownerOf (target (Primitives.Predicate.and [permanent, Primitives.Predicate.not land]))) (putWith it onTop [Primitives.MoveRider.placement (Primitives.PilePlacement.nthFromEnd (.nth 2)), Primitives.MoveRider.placement Primitives.PilePlacement.orOtherEnd])
theorem okDeemInferior : Action.check (DRS.ofList []) deemInferior = [] := by decide

/-- Write into Being -/
def writeIntoBeingPlacement : Action :=
  Primitives.Action.sequentially
    [ lookAt (topSlice (.lit 2)),
      manifestPlacement (someOf (exactly 1) them),
      Primitives.Action.move (theOther .object) Primitives.EventSource.anywhere (some onTop) .arrival none [Primitives.MoveRider.placement Primitives.PilePlacement.orOtherEnd] ]
theorem okWriteIntoBeingPlacement : Action.check (DRS.ofList []) writeIntoBeingPlacement = [] := by decide

/-- Mystical Tutor -/
def mysticalTutor : Action :=
  Primitives.Action.sequentially
    [searchLibraryFor (exactly 1) instantOrSorcery, revealCards it, shuffle, move it onTop]
theorem okMysticalTutor : Action.check (DRS.ofList []) mysticalTutor = [] := by decide

/-- Demonic Tutor -/
def demonicTutor : Action :=
  Primitives.Action.sequentially [searchLibraryFor (exactly 1) (Primitives.Predicate.and []), move it hand, shuffle]
theorem okDemonicTutor : Action.check (DRS.ofList []) demonicTutor = [] := by decide

/-- Thalia's Lancers -/
def thaliasLancersSearch : Action :=
  offer
    (Primitives.Action.sequentially
      [ searchLibraryFor (exactly 1) (Primitives.Predicate.and [Primitives.Predicate.hasSupertype .legendary]), revealCards it,
        move it hand, shuffle ])
theorem okThaliasLancersSearch : Action.check (DRS.ofList []) thaliasLancersSearch = [] := by decide

def contrabandLivestock : Action :=
  Primitives.Action.sequentially
    [ exile (target creature),
      rollDice 1 20,
      Primitives.Action.applyResultsTable
        [ rollRow (fromTo 1 9)
            (instruct (controllerOf it) (Primitives.Action.create (.lit 1)
              (Primitives.TokenSpec.written (creatureToken 4 4 [.green] [creatureType "Ox"])) [])),
          rollRow (fromTo 10 19)
            (instruct (controllerOf it) (Primitives.Action.create (.lit 1)
              (Primitives.TokenSpec.written (creatureToken 2 2 [.green] [creatureType "Boar"])) [])),
          rollRow (exactly 20)
            (instruct (controllerOf it) (Primitives.Action.create (.lit 1)
              (Primitives.TokenSpec.written (creatureToken 0 1 [.white] [creatureType "Goat"])) [])) ] ]
theorem okContrabandLivestock : Action.check (DRS.ofList []) contrabandLivestock = [] := by decide

/-- Hypnotic Specter -/
def hypnoticSpecterDiscard : Action := discard (aAtRandom (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.inZone (handOf they)]))
theorem okHypnoticSpecterDiscard :
    Action.check (DRS.ofList [{ det := .the, plur := .one, payload := .player false }]) hypnoticSpecterDiscard = [] := by decide

/-- Wyll, Blade of Frontiers -/
def wyllExtraDie : Action :=
  replaceEvent (Primitives.WouldClause.would (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.rollDice Primitives.Amount.anyAmount .anyDie .many Primitives.RollWatch.anyResult))
    (Primitives.Action.sequentially
      [ Primitives.Action.rollDice (plus Primitives.Amount.thatMuch (.lit 1)) .thoseDice .many .anyResult,
        Primitives.Action.ignoreOutcomes (Primitives.IgnoredOutcomes.extreme .lowest) ])
    none
theorem okWyllExtraDie : Action.check (DRS.ofList []) wyllExtraDie = [] := by decide

/-- Vedalken Squirrel-Whacker -/
def vedalkenSquirrelWhackerReroll : Action :=
  replaceEvent (Primitives.WouldClause.would (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.rollDice Primitives.Amount.anyAmount (.sides 6) .many Primitives.RollWatch.anyResult))
    (Primitives.Action.sequentially
      [ Primitives.Action.rollDice Primitives.Amount.thatMuch .thoseDice .many .anyResult,
        offer
          (Primitives.Action.exchange (Primitives.Exchanged.values (Primitives.Amount.theOutcome .rollResult) (Primitives.Amount.statOf (.stat .power) thisCreature))) ])
    none
theorem okVedalkenSquirrelWhackerReroll :
    Action.check (DRS.ofList []) vedalkenSquirrelWhackerReroll = [] := by decide

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
        [ Primitives.Ability.spell none (Primitives.Action.doOnlyIf (Primitives.Action.counterSpell (target spell))
            (costWasPaid (.byKeyword (.named "kicker")) none it) none) ] } }

/-- Celebrate the Harvest -/
def celebrateTheHarvest : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Celebrate the Harvest", cost := some [generic 3, pip .green], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ searchLibraryFor (Primitives.Quantity.upToOf (Primitives.Amount.letter .x)) (Primitives.Predicate.and [land, Primitives.Predicate.hasSupertype .basic]),
              Primitives.Action.define .x (Primitives.Amount.distinctCount (.value .power) (allOf creatureYouControl)),
              putOntoBattlefieldTapped (those .card),
              shuffle ]) ] } }

/-- Boreas Charger's description -/
def opponentWithMoreLands : Predicate :=
  Primitives.Predicate.compareOver Primitives.Predicate.opponent (countOf (Primitives.Predicate.and [land, Primitives.Predicate.hasPossessor .controller they])) .greater
    (countOf (Primitives.Predicate.and [land, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
theorem okOpponentWithMoreLands : Predicate.checkAt .player (DRS.ofList []) opponentWithMoreLands = [] := by
  decide

def bioplasmCardTest : Condition := Primitives.Condition.matches itCard creature
theorem okBioplasmCardTest : Condition.check (nextClause bioplasmAfterExile) bioplasmCardTest = [] := by decide

/-- Blessed Respite -/
def blessedRespiteShuffle : Action :=
  instruct (target Primitives.Predicate.anyPlayer) (shuffleInto (allOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.inZone (graveyardOf they)])))
theorem okBlessedRespiteShuffle : Action.check (DRS.ofList []) blessedRespiteShuffle = [] := by decide

/-- Nekrataal -/
def nekrataalEntry : GameEvent := Primitives.GameEvent.enters thisCreature none
theorem okNekrataalEntry : GameEvent.check (DRS.ofList []) nekrataalEntry = [] := by decide
def nekrataalDestroy : Action :=
  destroy (target (Primitives.Predicate.and [creature, Primitives.Predicate.not artifact, Primitives.Predicate.not (Primitives.Predicate.colorIs .black)]))
theorem okNekrataalDestroy :
    Action.check ((stepGameEvent 512 (DRS.ofList []) nekrataalEntry).1) nekrataalDestroy = [] := by decide
def nekrataalRider : DRS :=
  (stepAction 512 .postEvent .obj ((stepGameEvent 512 (DRS.ofList []) nekrataalEntry).1) nekrataalDestroy).1
theorem nekrataalOneCreatureWord : countReach (.word (.type .creature)) .one nekrataalRider = 1 := by
  decide
theorem nekrataalOneDestroyed : countReach (.verbed (.action "destroy") none .attributive) .one nekrataalRider = 1 := by
  decide

def sequencedRiderBody : Action :=
  Primitives.Action.sequentially [exile (target artifact), destroy (target creature)]
theorem okSequencedRiderBody : Action.check (DRS.ofList []) sequencedRiderBody = [] := by decide
def sequencedRider : DRS := (stepAction 512 .postEvent .obj (DRS.ofList []) sequencedRiderBody).1
theorem sequencedRiderOneDestroyed :
    countReach (.verbed (.action "destroy") none .attributive) .one sequencedRider = 1 := by decide
theorem sequencedRiderOneExiled : countReach (.verbed (.action "exile") none .attributive) .one sequencedRider = 1 := by
  decide

/-- Engulfing Flames -/
def engulfingFlamesBody : Action := instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 1) (target creature) .noncombatOnly)
theorem okEngulfingFlamesBody : Action.check (DRS.ofList []) engulfingFlamesBody = [] := by decide
def engulfingFlamesRider : DRS := (stepAction 512 .postEvent .obj (DRS.ofList []) engulfingFlamesBody).1
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
    countReach (.verbed (.action "exile") (some (.type .creature)) .attributive) .one bioplasmAfterTest
      = 0 := by
  decide
theorem bioplasmTypedCardReadWrites :
    countReach (.verbed (.action "exile") (some (.ofType .card .creature)) .attributive) .one
      bioplasmAfterTest = 1 := by
  decide

/-- Scapeshift -/
def scapeshiftSacrifice : Action := sacrifice (counted anyNumber land)
theorem okScapeshiftSacrifice : Action.check (DRS.ofList []) scapeshiftSacrifice = [] := by decide
def scapeshiftSacrificed : DRS := (stepAction 512 .postEvent .obj (DRS.ofList []) scapeshiftSacrifice).1
def scapeshiftSearch : Action := searchLibraryFor (Primitives.Quantity.upToOf Primitives.Amount.groupSize) land
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
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ sacrifice (counted anyNumber land),
              searchLibraryFor (Primitives.Quantity.upToOf Primitives.Amount.groupSize) land,
              putOntoBattlefieldTapped (theVerbed (.action "search") .many),
              shuffle ]) ] } }

/-- Bind to Life, Vastlands Scavenger's adventure -/
def bindToLife : Action :=
  Primitives.Action.sequentially
    [mill (.lit 7) Primitives.NounPhrase.you, move (fromAmong (exactly 1) creature them) battlefield]
theorem okBindToLife : Action.check (DRS.ofList []) bindToLife = [] := by decide

/-- Glamdring, Foe-hammer's Gleam of Death -/
def gleamOfDeath : Action :=
  Primitives.Action.sequentially
    [mill (.lit 6) Primitives.NounPhrase.you, move (allFromAmong (Primitives.Predicate.or [instant, sorcery]) them) hand]
theorem okGleamOfDeath : Action.check (DRS.ofList []) gleamOfDeath = [] := by decide

/-- Tezzeret, Master of the Bridge -/
def tezzeretAllArtifacts : Action :=
  Primitives.Action.sequentially
    [ exile (topSlice (.lit 10)),
      move (allFromAmong artifact (theVerbed (.action "exile") .many (some .card) .thisWay)) battlefield ]
theorem okTezzeretAllArtifacts : Action.check (DRS.ofList []) tezzeretAllArtifacts = [] := by decide

def companyContext : DRS := (stepAction 512 .postEvent .obj (DRS.ofList []) (lookAt (topSlice (.lit 6)))).1
def companyDescribedSlice : NounPhrase :=
  fromAmong (upTo 2) (Primitives.Predicate.and [creature, Primitives.Predicate.compare [.stat .manaValue] .atMost (.lit 3)]) them
theorem okCompanyDescribedSlice :
    NounPhrase.checkAt (some .object) (companyContext) companyDescribedSlice = [] := by decide
def companyBareSlice : NounPhrase := someOf (exactly 2) them
theorem okCompanyBareSlice : NounPhrase.checkAt (some .object) (companyContext) companyBareSlice = [] := by
  decide

/-- Lord of the Void -/
def exileTopThenPutFromAmong : Action :=
  Primitives.Action.sequentially
    [ exile (Primitives.NounPhrase.slice .top (.lit 7) (library (target Primitives.Predicate.opponent))),
      putOntoBattlefieldUnderYourControl (fromAmong (exactly 1) creature them) ]
theorem okExileTopThenPutFromAmong : Action.check (DRS.ofList []) exileTopThenPutFromAmong = [] := by decide

/-- Thought Sponge -/
def greatestCardsAnOpponentDrew : Amount :=
  Primitives.Amount.aggregateOver .max Primitives.Predicate.opponent (eventCount (Primitives.Action.draw Primitives.Amount.anyAmount) (Primitives.Subject.actor they) soFarThisTurn)
theorem okGreatestCardsAnOpponentDrew : Amount.check (DRS.ofList []) greatestCardsAnOpponentDrew = [] := by
  decide
def greatestCardsAPlayerDiscardedThisWay : Amount :=
  Primitives.Amount.aggregateOver .max Primitives.Predicate.anyPlayer
    (Primitives.Amount.tally (⟨(Primitives.Subject.actor they), (discard ((allOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.inZone hand])))), Primitives.LookbackSpan.thisWay⟩) .events)
theorem okGreatestCardsAPlayerDiscardedThisWay :
    Amount.check (DRS.ofList []) greatestCardsAPlayerDiscardedThisWay = [] := by decide

def eachPlayerBase : NounPhrase := each Primitives.Predicate.anyPlayer
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
  Primitives.Action.sequentially [instruct (controllerOf thisAura) (sacrifice itPermanent), instruct they (Primitives.Action.draw (.lit 2))]
theorem okSoulRansomRansom : Action.check (DRS.ofList []) soulRansomRansom = [] := by decide
def thisAurasController : NounPhrase := controllerOf thisAura
theorem okThisAurasController : NounPhrase.checkAt (some .player) (DRS.ofList []) thisAurasController = [] := by
  decide
def targetCreaturesController : NounPhrase := controllerOf (target creature)
theorem okTargetCreaturesController :
    NounPhrase.checkAt (some .player) (DRS.ofList []) targetCreaturesController = [] := by decide
theorem possessiveDeicticIsReadableByIt :
    countReach (.atSlot .permanent) .one ((stepNounPhrase 512 (DRS.ofList []) .obj true thisAurasController).1) = 1 := by decide
theorem possessiveDeicticDemonstrativeSelectsTheSource :
    let result := stepNounPhrase 512 (DRS.ofList []) .obj true thisAurasController
    let selected := resolveCandidates (.word (.type .enchantment)) .one result.1
    result.2.1 = [1] ∧ selected.map (·.id) = [0] ∧
      selected.all (fun ref => ref.zone == some .battlefield && ref.ty == [.enchantment]) := by decide

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
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ gainControl (target (Primitives.Predicate.or [artifact, creature])) (some untilEndOfTurn) (agent := Primitives.NounPhrase.you),
              untap it,
              gainHaste it (some untilEndOfTurn) ]) ] } }

def hijackDeedDescriptions : Action :=
  Primitives.Action.sequentially
    [ gainControl (target (Primitives.Predicate.or [artifact, creature]))
        (some untilEndOfTurn) (agent := Primitives.NounPhrase.you),
      untap (theVerbed (.core .gainControl) .one),
      gainHaste (theVerbed (.action "untap") .one) (some untilEndOfTurn) ]
theorem okHijackDeedDescriptions :
    Action.check (DRS.ofList []) hijackDeedDescriptions = [] := by decide

/-- Open the Vaults -/
def openTheVaults : Action :=
  returnTo
    (allOf (Primitives.Predicate.and [ Primitives.Predicate.isCard, Primitives.Predicate.or [artifact, enchantment],
                   Primitives.Predicate.inZone (graveyardOf (Primitives.NounPhrase.playerGroup .allPlayers)) ]))
    battlefield [Primitives.EntryRider.under (ownerOf them)]
theorem okOpenTheVaults : Action.check (DRS.ofList []) openTheVaults = [] := by decide

/-- Codecracker Hound -/
def codecrackerHoundLook : Action :=
  Primitives.Action.sequentially
    [ lookAt (topSlice (.lit 2)), move (someOf (exactly 1) them) hand,
      move (theOther .object) graveyard ]
theorem okCodecrackerHoundLook : Action.check (DRS.ofList []) codecrackerHoundLook = [] := by decide

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

def eachPlayerShufflesTheirHandAndGraveyard : Action :=
  instruct (each Primitives.Predicate.anyPlayer) (shuffleInto
    (Primitives.NounPhrase.both (allOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.inZone (handOf they)])) (allOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.inZone (graveyardOf they)]))))
theorem okEachPlayerShufflesTheirHandAndGraveyard :
    Action.check (DRS.ofList []) eachPlayerShufflesTheirHandAndGraveyard = [] := by decide

/-- Fact or Fiction -/
def factOrFiction : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fact or Fiction", cost := some [generic 3, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ revealCards (topSlice (.lit 5)),
              instruct anOpponent (Primitives.Action.separateIntoPiles them 2 []),
              move onePile hand,
              move (theOther .pile) graveyard ]) ] } }

def unsummon : Action := move (target creature) hand
theorem okUnsummon : Action.check (DRS.ofList []) unsummon = [] := by decide
def grismold : Action :=
  instruct (each Primitives.Predicate.anyPlayer) (Primitives.Action.create (.lit 1)
    (Primitives.TokenSpec.written (creatureToken 1 1 [.green] [creatureType "Plant"])) [])
theorem okGrismold : Action.check (DRS.ofList []) grismold = [] := by decide
/-- Grenzo, Dungeon Warden -/
def grenzoBottomCard : Action := move bottomCard graveyard
theorem okGrenzoBottomCard : Action.check (DRS.ofList []) grenzoBottomCard = [] := by decide

def chronostutter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chronostutter", cost := some [generic 5, pip .blue], types := [.instant],
      text := [ Primitives.Ability.spell none (Primitives.Action.move (target creature) Primitives.EventSource.anywhere (some onTop) .arrival none [Primitives.MoveRider.placement (Primitives.PilePlacement.nthFromEnd (.nth 2))]) ] } }

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
      text := [ Primitives.Ability.spell none (instruct (Primitives.NounPhrase.eachOf (Primitives.NounPhrase.both Primitives.NounPhrase.you (target Primitives.Predicate.opponent))) (Primitives.Action.draw (.lit 3))) ]
          } }

/-- Aggressive Instinct -/
def aggressiveInstinct : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aggressive Instinct", cost := some [generic 1, pip .green], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (dealDamageOwnPower (target creatureYouControl)
            (target creatureYouDontControl)) ] } }

/-- Arcum Dagsson -/
def arcumDagssonSacrifice : Action :=
  controllerSacrifices (target (Primitives.Predicate.and [artifact, creature]))
theorem okArcumDagssonSacrifice : Action.check (DRS.ofList []) arcumDagssonSacrifice = [] := by decide

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
            (Primitives.Action.sequentially
              [ Primitives.Action.doForEach (each Primitives.Predicate.anyPlayer)
                  (choose (target (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller they]))),
                instruct (those .player) (sacrifice (those .permanent)),
                Primitives.Action.doForEach
                  (each (Primitives.Predicate.and [ Primitives.Predicate.anyPlayer,
                                Primitives.Predicate.did (⟨(Primitives.Subject.actor (relative .player)), (sacrifice ((a permanent))), Primitives.LookbackSpan.thisWay⟩)
                                  ]))
                  (Primitives.Action.sequentially
                    [ instruct they (Primitives.Action.expose .reveal (Primitives.Exposed.cards (Primitives.NounPhrase.slice .top (.lit 1) (library they)))),
                      Primitives.Action.doIf (Primitives.Condition.matches itCard permanentCard) (move itCard battlefield)
                        none ]) ]) ],
      power := stat 6, toughness := stat 6 } }

end Semantics.Cards
