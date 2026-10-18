import Semantics
import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Cards.Anaphora
import Semantics.Cards.Trigger

/-!
# Semantics.Cards.Choice

Port of `idris/src/Experimental/Cards/Choice.idr`: the printed cards of the Choice family
(choices, modes, may, permissions to play, chosen qualities) and the bench items beside them.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

def carefulStudy : Action :=
  Primitives.Action.sequentially [Primitives.Action.draw (.lit 2), discard (counted (exactly 2) (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ yourHand]))]
theorem okCarefulStudy : Action.check (DRS.ofList []) carefulStudy = [] := by decide

def zombieInfestation : Ability :=
  activated (Primitives.Cost.perform (discard (counted (exactly 2) (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ yourHand]))))
    (createToken (creatureToken 2 2 [.black] [creatureType "Zombie"]))
theorem okZombieInfestation : Ability.check (DRS.ofList []) zombieInfestation = [] := by decide

def fulgentDistraction : Action :=
  Primitives.Action.sequentially
    [ choose (Primitives.NounPhrase.described (Primitives.DetPhrase.target (exactly 2)) isCreature),
      Primitives.Action.setStatus .tapped (those (.type .creature)),
      Primitives.Action.unattach (allOf (Primitives.Predicate.and [ Primitives.Predicate.hasSubtype (artifactType "Equipment"),
                               Primitives.Predicate.attachedTo (those (.type .creature)) ])) ]
theorem okFulgentDistraction : Action.check (DRS.ofList []) fulgentDistraction = [] := by decide

def continueSpell : Action :=
  Primitives.Action.sequentially
    [ choose (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 4)) (Primitives.Predicate.and [isCreature, Primitives.Predicate.in_ (graveyardOf Primitives.NounPhrase.you)])),
      putInto them battlefield ]
theorem okContinueSpell : Action.check (DRS.ofList []) continueSpell = [] := by decide

def kindredDominance : Action :=
  Primitives.Action.sequentially
    [ choose (a (quality (.subtype .creature))),
      destroy (allOf (Primitives.Predicate.and [isCreature, Primitives.Predicate.not (ofChosen (.subtype .creature))])) ]
theorem okKindredDominance : Action.check (DRS.ofList []) kindredDominance = [] := by decide

def phantomBlade : Ability :=
  when (does (actor thisEquipment) enter)
    (Primitives.Action.sequentially
      [ attachTo it
          (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1)) (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])),
        destroy (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1)) (Primitives.Predicate.and [isCreature, Primitives.Predicate.other])) ])
theorem okPhantomBlade : Ability.check (DRS.ofList []) phantomBlade = [] := by decide

/-- Braids's Frightful Return -/
def braidsFrightfulReturn : Action :=
  Primitives.Action.offer (sacrifice (a isCreature)) (some (Primitives.Action.sequentially [instruct (each (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) (choose (a (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ hand]))), discard them])) none
theorem okBraidsFrightfulReturn : Action.check (DRS.ofList []) braidsFrightfulReturn = [] := by decide
/-- Daretti, Ingenious Iconoclast -/
def darettisMinusOne : Action :=
  Primitives.Action.offer (sacrifice (a isArtifact)) (some (destroy (target (Primitives.Predicate.or [isArtifact,
      isCreature])))) none
theorem okDarettisMinusOne : Action.check (DRS.ofList []) darettisMinusOne = [] := by decide

def cheeringFanatic : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cheering Fanatic", cost := some [generic 1, pip .red], types := [.creature],
      subtypes := [creatureType "Goblin"],
      text :=
        [ whenever (does (actor thisCreature) attack)
            (Primitives.Action.sequentially
              [ choose (a (quality .cardName)),
                Primitives.Action.establish
                  (Primitives.StaticSpec.costShift (allOf (Primitives.Predicate.and [isSpell, ofChosen .cardName])) (Primitives.CostShift.less (.lit 1) none))
                  (some thisTurn) ]) ],
      power := stat 2, toughness := stat 2 } }

def rainOfThorns : Action :=
  chooseModes (atLeast 1)
    [destroy (target isArtifact), destroy (target isEnchantment), destroy (target isLand)]
theorem okRainOfThorns : Action.check (DRS.ofList []) rainOfThorns = [] := by decide

def rankleMasterOfPranks : Action :=
  chooseModes anyNumber
    [ Primitives.Action.sequentially [instruct (each Primitives.Predicate.anyPlayer) (choose (a (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ hand]))), discard them],
      Primitives.Action.sequentially [instruct (each Primitives.Predicate.anyPlayer) (loseLife (.lit 1)), instruct (those .player) (Primitives.Action.draw (.lit 1))],
      instruct (each Primitives.Predicate.anyPlayer) (sacrifice (aTheirChoice isCreature)) ]
theorem okRankleMasterOfPranks : Action.check (DRS.ofList []) rankleMasterOfPranks = [] := by decide

def myrkulsEdict : Action :=
  Primitives.Action.sequentially [choose (a (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)), instruct (that .player) (sacrifice (aTheirChoice isCreature))]
theorem okMyrkulsEdict : Action.check (DRS.ofList []) myrkulsEdict = [] := by decide
def moltingHarpy : Action := doUnless (sacrifice thisCreature) (Primitives.Cost.mana [generic
    2])
theorem okMoltingHarpy : Action.check (DRS.ofList []) moltingHarpy = [] := by decide
def carnophage : Action := doUnless (Primitives.Action.setStatus .tapped thisCreature) (payLife 1)
theorem okCarnophage : Action.check (DRS.ofList []) carnophage = [] := by decide
def solitaryConfinement : Action :=
  Primitives.Action.offer (discard (a (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ yourHand]))) none (some (sacrifice thisEnchantment))
theorem okSolitaryConfinement : Action.check (DRS.ofList []) solitaryConfinement = [] := by decide

def yasminKhan : Ability :=
  activated Primitives.Cost.tapSymbol
    (Primitives.Action.sequentially
      [ exile (topSlice (.lit 1)),
        Primitives.Action.establish
          (can (actor you) (play it))
          (some untilYourNextEndStep) ])
theorem okYasminKhan : Ability.check (DRS.ofList []) yasminKhan = [] := by decide

/-- Brazen Cannonade -/
def brazenCannonadePermission : Action :=
  Primitives.Action.sequentially
    [ exile (topSlice (.lit 1)),
      Primitives.Action.establish
        (can (actor you) (play it))
        (some (untilEvent (Primitives.GameEvent.endOf .combat (some Primitives.NounPhrase.you) (Primitives.TurnRef.next none)))) ]
theorem okBrazenCannonadePermission : Action.check (DRS.ofList []) brazenCannonadePermission = [] := by
  decide

def thousandMoonsCrackshot : Ability :=
  whenever (does (actor thisCreature) attack)
    (offerWhen (Primitives.Action.pay (Primitives.Cost.mana [generic 2, pip .white]) .once .paid none none)
      (Primitives.Action.setStatus .tapped (target isCreature)))
theorem okThousandMoonsCrackshot : Ability.check (DRS.ofList []) thousandMoonsCrackshot = [] := by decide

def zimoneQuandrixProdigy : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 1], Primitives.Cost.tapSymbol])
    (may (putOntoBattlefieldTapped (a (Primitives.Predicate.and [isLand, Primitives.Predicate.in_ (handOf Primitives.NounPhrase.you)]))))
theorem okZimoneQuandrixProdigy : Ability.check (DRS.ofList []) zimoneQuandrixProdigy = [] := by decide

def preeminentCaptain : Ability :=
  whenever (does (actor thisCreature) attack)
    (may (putOntoBattlefieldTappedAttacking
      (a (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Soldier"), isCreature, Primitives.Predicate.in_ (handOf Primitives.NounPhrase.you)]))))
theorem okPreeminentCaptain : Ability.check (DRS.ofList []) preeminentCaptain = [] := by decide

def skaabRuinator : Ability :=
  Primitives.Ability.static (can (actor you) (cast Primitives.NounPhrase.this (from_ := some (graveyardOf Primitives.NounPhrase.you))))
theorem okSkaabRuinator : Ability.check (DRS.ofList []) skaabRuinator = [] := by decide

/-- Raffine's Guidance -/
def raffinesGuidance : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Raffine's Guidance", cost := some [pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") isCreature,
          Primitives.Ability.static (getsPt (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))),
          Primitives.Ability.static (can (actor you) (cast Primitives.NounPhrase.this (from_ := some (graveyardOf Primitives.NounPhrase.you)) (payment := Primitives.PlayPayment.payingInstead (Primitives.Cost.mana [generic 2, pip .white])))) ] } }

/-- Scourge of Nel Toth -/
def scourgeOfNelToth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Scourge of Nel Toth", cost := some [generic 5, pip .black, pip .black],
      types := [.creature], subtypes := [creatureType "Zombie", creatureType "Dragon"],
      text :=
        [ keyword (.named "flying"),
          Primitives.Ability.static (can (actor you) (cast Primitives.NounPhrase.this (from_ := some (graveyardOf Primitives.NounPhrase.you)) (payment := Primitives.PlayPayment.payingInstead (Primitives.Cost.compound
                [ Primitives.Cost.mana [pip .black, pip .black],
                  Primitives.Cost.perform (sacrifice (counted (exactly 2) isCreature)) ])))) ],
      power := stat 6, toughness := stat 6 } }

def escapeToTheWilds : Action :=
  Primitives.Action.sequentially
    [ exile (topSlice (.lit 5)),
      Primitives.Action.establish
        (can (actor you) (play (theVerbed (.action "exile") .many (some .card) .thisWay)))
        (some (untilEvent (Primitives.GameEvent.endOf .turn (some Primitives.NounPhrase.you) (Primitives.TurnRef.next none)))) ]
theorem okEscapeToTheWilds : Action.check (DRS.ofList []) escapeToTheWilds = [] := by decide

/-- Muse Vessel -/
def museVesselPlay : Ability :=
  activated (Primitives.Cost.mana [generic 1])
    (Primitives.Action.sequentially
      [ choose (a exiledWithThisArtifact),
        Primitives.Action.establish
          (can (actor you) (play (that .card)))
          (some thisTurn) ])
theorem okMuseVesselPlay : Ability.check (DRS.ofList []) museVesselPlay = [] := by decide

def demonicConsultationChoice : Action := choose (a (quality .cardName))
theorem okDemonicConsultationChoice : Action.check (DRS.ofList []) demonicConsultationChoice = [] := by
  decide
def voidChoice : Action := choose (a (quality .number))
theorem okVoidChoice : Action.check (DRS.ofList []) voidChoice = [] := by decide
def ashnodsBattleGear : Ability := Primitives.Ability.static (mayDeclineUntap thisArtifact (some Primitives.NounPhrase.you))
theorem okAshnodsBattleGear : Ability.check (DRS.ofList []) ashnodsBattleGear = [] := by decide

/-- Phyrexian Ingester -/
def phyrexianIngesterPump : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.conjunction none
    [ getsPt thisCreature (Primitives.Delta.up (Primitives.Amount.letter .x)) (Primitives.Delta.up (Primitives.Amount.letter .y)),
      Primitives.StaticSpec.letterDefinition .x
        (Primitives.Amount.statOf (.stat .power) (the (Primitives.Predicate.and [isCreature, Primitives.Predicate.in_ (Primitives.Location.exiledWith thisCreature)]))),
      Primitives.StaticSpec.letterDefinition .y (Primitives.Amount.statOf (.stat .toughness) (that .card)) ])
theorem okPhyrexianIngesterPump : Ability.check (DRS.ofList []) phyrexianIngesterPump = [] := by decide

/-- Phyrexian Ingester -/
def phyrexianIngester : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Phyrexian Ingester", cost := some [generic 6, pip .blue], types := [.creature],
      subtypes := [creatureType "Phyrexian", creatureType "Beast"],
      text :=
        [ abilityWord "imprint"
            (when (does (actor thisCreature) enter)
              (may (exile (target (Primitives.Predicate.and [isCreature, isNontoken]))))),
          phyrexianIngesterPump ],
      power := stat 3, toughness := stat 3 } }

def misterFantastic : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mister Fantastic, Reed Richards", cost := some [generic 3, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Scientist", creatureType "Hero"],
      text :=
        [ keyword (.named "reach"),
          whenever
            (does (actor (counted (atLeast 1) (Primitives.Predicate.and [Primitives.Predicate.isToken, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))) enter)
            (may (Primitives.Action.draw (.lit 1))) ],
      power := stat 2, toughness := stat 4 } }

def murmursFromBeyond : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Murmurs from Beyond", cost := some [generic 2, pip .blue], types := [.instant],
      subtypes := [spellType "Arcane"],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ reveal (topSlice (.lit 3)),
              instruct (a (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) (choose (someOf (exactly 1) them)),
              putInto (that .card) graveyard,
              putInto (theRest .object) hand ]) ] } }

/-- Helica Glider -/
def helicaGlider : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Helica Glider", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [creatureType "Nightmare", creatureType "Squirrel"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.entryRider thisCreature
            (Primitives.MoveRider.withCounters (.lit 1) (Primitives.CounterKindSource.chosen [flyingCounter, .named "firstStrikeCounter"]) .fresh)) ],
      power := stat 2, toughness := stat 2 } }

/-- Eager Construct -/
def eagerConstruct : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Eager Construct", cost := some [generic 2], types := [.artifact, .creature],
      subtypes := [creatureType "Construct"],
      text := [when (does (actor thisCreature) enter) (instruct (each Primitives.Predicate.anyPlayer) (may (scry (.lit 1))))],
      power := stat 2, toughness := stat 2 } }

/-- Rune Snag -/
def runeSnag : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rune Snag", cost := some [generic 1, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (instruct (controllerOf (target isSpell)) (doUnless (counter it)
            (Primitives.Cost.compound
              [ Primitives.Cost.mana [generic 2],
                scaledGenericMana
                  (times (.lit 2)
                    (countOf (Primitives.Predicate.and [Primitives.Predicate.named (Primitives.NameSource.printed "Rune Snag"), Primitives.Predicate.in_ graveyard]))) ]))) ] } }

/-- Tahngarth, First Mate -/
def tahngarthChoosesDefender : Action :=
  Primitives.Action.choose none
    (a (Primitives.Predicate.and [ Primitives.Predicate.or [Primitives.Predicate.hasType .planeswalker, Primitives.Predicate.anyPlayer],
               Primitives.Predicate.inCombat .attackedBy (some (that .player)) ]))
    .openly none
theorem okTahngarthChoosesDefender :
    Action.check ((stepGameEvent 512 (DRS.ofList []) tahngarthHeader).1) tahngarthChoosesDefender = [] := by
  decide

def reefShaman : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Reef Shaman", cost := some [pip .blue], types := [.creature],
      subtypes := [creatureType "Merfolk", creatureType "Shaman"],
      text :=
        [ activated Primitives.Cost.tapSymbol
            (Primitives.Action.establish
              (Primitives.StaticSpec.qualityChange (target isLand) .sets
                (Primitives.QualityPayload.chosenQuality (Primitives.Predicate.ofYourChoice (.subtype .land) (some Primitives.ChoiceDomain.basicTypesOnly))))
              (some untilEndOfTurn)) ],
      power := stat 0, toughness := stat 2 } }

def grixisIllusionist : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grixis Illusionist", cost := some [pip .blue], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ activated Primitives.Cost.tapSymbol
            (Primitives.Action.establish
              (Primitives.StaticSpec.qualityChange (target (Primitives.Predicate.and [isLand, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .sets
                (Primitives.QualityPayload.chosenQuality (Primitives.Predicate.ofYourChoice (.subtype .land) (some Primitives.ChoiceDomain.basicTypesOnly))))
              (some untilEndOfTurn)) ],
      power := stat 1, toughness := stat 1 } }

def distantMelody : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Distant Melody", cost := some [generic 3, pip .blue], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ choose (a (quality (.subtype .creature))),
              Primitives.Action.draw
                (forEach (.lit 1)
                  (Primitives.Predicate.and [isPermanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, ofChosen (.subtype .creature)])) ]) ] } }

def cripplingFear : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crippling Fear", cost := some [generic 2, pip .black, pip .black],
      types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ choose (a (quality (.subtype .creature))),
              get (allOf (Primitives.Predicate.and [isCreature, Primitives.Predicate.not (ofChosen (.subtype .creature))]))
                (Primitives.Delta.down (.lit 3)) (Primitives.Delta.down (.lit 3)) (some untilEndOfTurn) ]) ] } }

/-- Koh, the Face Stealer -/
def kohChooser : Ability :=
  activated (payLife 1) (choose (a (Primitives.Predicate.and [isCreature, Primitives.Predicate.in_ (Primitives.Location.exiledWith Primitives.NounPhrase.this)])))
theorem okKohChooser : Ability.check (DRS.ofList []) kohChooser = [] := by decide
/-- Forgotten Lore -/
def forgottenLoreChoice : Action :=
  instruct (target (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) (choose (a (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (graveyardOf Primitives.NounPhrase.you)])))
theorem okForgottenLoreChoice : Action.check (DRS.ofList []) forgottenLoreChoice = [] := by decide

/-- Psychic Paper minus its three-way coordination -/
def psychicPaperChoiceAndReads : List Ability :=
  [ Primitives.Ability.static (Primitives.StaticSpec.conjunction none
      [ attachChoosing thisEquipment .cardName,
        attachChoosing thisEquipment (.subtype .creature) ]),
    Primitives.Ability.static (Primitives.StaticSpec.conjunction none
      [ Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .equipped (.type .creature)) .sets
          (Primitives.QualityPayload.chosenQuality (ofTheLastChosen .cardName)),
        Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .equipped (.type .creature)) .sets
          (Primitives.QualityPayload.chosenQuality (ofTheLastChosen (.subtype .creature))) ]) ]
theorem okPsychicPaperChoiceAndReads : Ability.checkText (DRS.ofList []) psychicPaperChoiceAndReads = [] := by
  decide

def xenograft : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Xenograft", cost := some [generic 4, pip .blue], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (entersChoosing thisEnchantment (.subtype .creature)),
          Primitives.Ability.static (Primitives.StaticSpec.qualityChange (allOf (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .adds
            (Primitives.QualityPayload.chosenQuality (ofChosen (.subtype .creature)))) ] } }

/-- Convincing Mirage -/
def convincingMirage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Convincing Mirage", cost := some [generic 1, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") isLand,
          Primitives.Ability.static (entersChoosingFrom thisAura (.subtype .land) Primitives.ChoiceDomain.basicTypesOnly),
          Primitives.Ability.static (Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .enchanted (.type .land)) .sets
            (Primitives.QualityPayload.chosenQuality (ofChosen (.subtype .land)))) ] } }

/-- Realmwright -/
def realmwright : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Realmwright", cost := some [pip .blue], types := [.creature],
      subtypes := [creatureType "Vedalken", creatureType "Wizard"],
      text :=
        [ Primitives.Ability.static (entersChoosingFrom thisCreature (.subtype .land) Primitives.ChoiceDomain.basicTypesOnly),
          Primitives.Ability.static (Primitives.StaticSpec.qualityChange (allOf (Primitives.Predicate.and [isLand, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .adds
            (Primitives.QualityPayload.chosenQuality (ofChosen (.subtype .land)))) ],
      power := stat 1, toughness := stat 1 } }

def adaptiveAutomaton : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Adaptive Automaton", cost := some [generic 3], types := [.artifact, .creature],
      subtypes := [creatureType "Construct"],
      text :=
        [ Primitives.Ability.static (entersChoosing thisCreature (.subtype .creature)),
          Primitives.Ability.static (Primitives.StaticSpec.qualityChange thisCreature .adds (Primitives.QualityPayload.chosenQuality (ofChosen (.subtype
              .creature)))),
          Primitives.Ability.static (getsPt
            (allOf (Primitives.Predicate.and [ isCreature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.otherThan thisCreature,
                           ofChosen (.subtype .creature) ]))
            (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))) ],
      power := stat 2, toughness := stat 2 } }

def mistformDreamer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mistform Dreamer", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [creatureType "Illusion"],
      text :=
        [ keyword (.named "flying"),
          activated (Primitives.Cost.mana [generic 1])
            (Primitives.Action.establish
              (Primitives.StaticSpec.qualityChange thisCreature .sets
                (Primitives.QualityPayload.chosenQuality (Primitives.Predicate.ofYourChoice (.subtype .creature) none)))
              (some untilEndOfTurn)) ],
      power := stat 2, toughness := stat 1 } }

def arcaneAdaptation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Arcane Adaptation", cost := some [generic 2, pip .blue], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (entersChoosing thisEnchantment (.subtype .creature)),
          Primitives.Ability.static (Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.and [
              allOf (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]),
              allOf (Primitives.Predicate.and [isCreature, isSpell, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]),
              allOf (Primitives.Predicate.and [Primitives.Predicate.isCard, isCreature, Primitives.Predicate.hasPossessor .owner Primitives.NounPhrase.you, Primitives.Predicate.not (Primitives.Predicate.in_ battlefield)]) ]) .adds
              (Primitives.QualityPayload.chosenQuality (ofChosen (.subtype .creature)))) ] } }

def conspiracy : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Conspiracy", cost := some [generic 3, pip .black, pip .black],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.static (entersChoosing thisEnchantment (.subtype .creature)),
          Primitives.Ability.static (Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.and [
              allOf (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]),
              allOf (Primitives.Predicate.and [isCreature, isSpell, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]),
              allOf (Primitives.Predicate.and [Primitives.Predicate.isCard, isCreature, Primitives.Predicate.hasPossessor .owner Primitives.NounPhrase.you, Primitives.Predicate.not (Primitives.Predicate.in_ battlefield)]) ]) .sets
              (Primitives.QualityPayload.chosenQuality (ofChosen (.subtype .creature)))) ] } }

def declarationOfNaught : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Declaration of Naught", cost := some [pip .blue, pip .blue], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (entersChoosing thisEnchantment .cardName),
          activated (Primitives.Cost.mana [pip .blue]) (counter (target (Primitives.Predicate.and [isSpell, Primitives.Predicate.named Primitives.NameSource.chosen]))) ] } }

def imagecrafter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Imagecrafter", cost := some [pip .blue], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ activated Primitives.Cost.tapSymbol
            (Primitives.Action.sequentially
              [ choose (a (qualityFrom (.subtype .creature) (Primitives.ChoiceDomain.typeOtherThan (creatureType "Wall")))),
                Primitives.Action.establish
                  (Primitives.StaticSpec.qualityChange (target isCreature) .sets (Primitives.QualityPayload.chosenQuality (ofChosen (.subtype
                      .creature))))
                  (some untilEndOfTurn) ]) ],
      power := stat 1, toughness := stat 1 } }

def unnaturalSelection : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unnatural Selection", cost := some [generic 1, pip .blue], types := [.enchantment],
      text :=
        [ activated (Primitives.Cost.mana [generic 1])
            (Primitives.Action.sequentially
              [ choose (a (qualityFrom (.subtype .creature) (Primitives.ChoiceDomain.typeOtherThan (creatureType "Wall")))),
                Primitives.Action.establish
                  (Primitives.StaticSpec.qualityChange (target isCreature) .sets (Primitives.QualityPayload.chosenQuality (ofChosen (.subtype
                      .creature))))
                  (some untilEndOfTurn) ]) ] } }

def standardize : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Standardize", cost := some [pip .blue, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ choose (a (qualityFrom (.subtype .creature) (Primitives.ChoiceDomain.typeOtherThan (creatureType "Wall")))),
              Primitives.Action.establish
                (Primitives.StaticSpec.qualityChange (each isCreature) .sets (Primitives.QualityPayload.chosenQuality (ofChosen (.subtype
                    .creature))))
                (some untilEndOfTurn) ]) ] } }

def silverquillSilencer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Silverquill Silencer", cost := some [pip .white, pip .black], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ Primitives.Ability.static (entersChoosingFrom thisCreature .cardName (Primitives.ChoiceDomain.nameOfCard (Primitives.Predicate.not isLand))),
          whenever (Primitives.GameEvent.does (Primitives.Subject.actor anOpponent) (Primitives.Action.procedure .cast ((a (Primitives.Predicate.and [isSpell,
            Primitives.Predicate.named Primitives.NameSource.chosen]))) Primitives.PlayPayment.itsOwnCost none none none))
            (Primitives.Action.sequentially [instruct they (loseLife (.lit 3)), Primitives.Action.draw (.lit 1)]) ],
      power := stat 3, toughness := stat 2 } }

def meddlingMage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Meddling Mage", cost := some [pip .white, pip .blue], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ Primitives.Ability.static (entersChoosingFrom thisCreature .cardName (Primitives.ChoiceDomain.nameOfCard (Primitives.Predicate.not isLand))),
          Primitives.Ability.static (cant Primitives.Subject.any (cast (allOf (Primitives.Predicate.and [isSpell, Primitives.Predicate.named Primitives.NameSource.chosen])))) ],
      power := stat 2, toughness := stat 2 } }

/-- Nyxathid -/
def nyxathid : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nyxathid", cost := some [generic 1, pip .black, pip .black], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ Primitives.Ability.static (entersChoosingPlayer thisCreature (some (Primitives.ChoiceDomain.players (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)))),
          Primitives.Ability.static (getsPt thisCreature
            (Primitives.Delta.down (countOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (handOf (the isChosenPlayer))])))
            (Primitives.Delta.down (countOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (handOf (the isChosenPlayer))])))) ],
      power := stat 7, toughness := stat 7 } }

/-- Expel the Interlopers -/
def expelTheInterlopers : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Expel the Interlopers", cost := some [generic 3, pip .white, pip .white],
      types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ choose (a (qualityFrom .number (Primitives.ChoiceDomain.number (fromTo 0 10)))),
              destroy (allOf (Primitives.Predicate.and [isCreature, Primitives.Predicate.compare [.stat .power] .atLeast chosenNumber])) ]) ] } }

def nevermore : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nevermore", cost := some [generic 1, pip .white, pip .white], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (entersChoosingFrom thisEnchantment .cardName (Primitives.ChoiceDomain.nameOfCard (Primitives.Predicate.not isLand))),
          Primitives.Ability.static (cant Primitives.Subject.any (cast (allOf (Primitives.Predicate.and [isSpell, Primitives.Predicate.named Primitives.NameSource.chosen])))) ] } }

def necromentiaChoice : Action :=
  choose (a (qualityFrom .cardName (Primitives.ChoiceDomain.nameOfCard (Primitives.Predicate.not (Primitives.Predicate.and [Primitives.Predicate.hasSupertype .basic, isLand])))))
theorem okNecromentiaChoice : Action.check (DRS.ofList []) necromentiaChoice = [] := by decide

def conjurersBan : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Conjurer's Ban", cost := some [pip .white, pip .black], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ choose (a (quality .cardName)),
              Primitives.Action.establish
                (Primitives.StaticSpec.conjunction none
                  [ cant Primitives.Subject.any (cast (allOf (Primitives.Predicate.and [isSpell, Primitives.Predicate.named Primitives.NameSource.chosen]))),
                    cant Primitives.Subject.any (play (allOf (Primitives.Predicate.and [isLand, Primitives.Predicate.named Primitives.NameSource.chosen]))) ])
                (some untilYourNextTurn) ]),
          Primitives.Ability.spell none (Primitives.Action.draw (.lit 1)) ] } }

def foundingOfOmashu : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Founding of Omashu", cost := some [generic 2, pip .red], types := [.enchantment],
      subtypes := [enchantmentType "Saga"],
      text :=
        [ when (Primitives.GameEvent.chapterMark [1]) (createToken (creatureToken 1 1 [.white] [creatureType "Ally"]) (count := (.lit 2))),
          when (Primitives.GameEvent.chapterMark [2])
            (Primitives.Action.offer (discard (a (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ yourHand]))) (some (Primitives.Action.draw (.lit 1))) none),
          when (Primitives.GameEvent.chapterMark [3])
            (get (allOf (and [isCreature, youControl])) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn)) ] }
                }

/-- "as though they had flash" -/
def asThoughFlash : Option AsThough :=
  some (Primitives.AsThough.condition (Primitives.Condition.matches them (Primitives.Predicate.hasKeyword (.the (.named "flash")))))

def vedalkenOrrery : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vedalken Orrery", cost := some [generic 4], types := [.artifact],
      text :=
        [ Primitives.Ability.static (can (actor you) (cast (allOf isSpell)) (asThough := asThoughFlash)) ] } }

def shimmerMyr : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shimmer Myr", cost := some [generic 3], types := [.artifact, .creature],
      subtypes := [creatureType "Myr"],
      text :=
        [ keyword (.named "flash"),
          Primitives.Ability.static (can (actor you) (cast (allOf (Primitives.Predicate.and [isSpell, isArtifact]))) (asThough := asThoughFlash)) ],
      power := stat 2, toughness := stat 2 } }

def quickSliver : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Quick Sliver", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Sliver"],
      text :=
        [ keyword (.named "flash"),
          Primitives.Ability.static (can (actor (a Primitives.Predicate.anyPlayer)) (cast (allOf (Primitives.Predicate.and [isSpell, Primitives.Predicate.hasSubtype (creatureType "Sliver")]))) (asThough := asThoughFlash)) ],
      power := stat 1, toughness := stat 1 } }

def vernalEquinox : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vernal Equinox", cost := some [generic 3, pip .green], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (can (actor (a Primitives.Predicate.anyPlayer)) (cast (allOf (Primitives.Predicate.and [isSpell, Primitives.Predicate.or [isCreature, isEnchantment]]))) (asThough := asThoughFlash)) ] } }

def borneUponAWind : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Borne Upon a Wind", cost := some [generic 1, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.establish
            (can (actor you) (cast (allOf isSpell)) (asThough := asThoughFlash))
            (some thisTurn)),
          Primitives.Ability.spell none (Primitives.Action.draw (.lit 1)) ] } }

def gisaAndGeralf : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gisa and Geralf", cost := some [generic 2, pip .blue, pip .black],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ when (does (actor thisCreature) enter) (mill (.lit 4) Primitives.NounPhrase.you),
          Primitives.Ability.static (can (actor you) (cast (counted (exactly 1) (Primitives.Predicate.and [isSpell, isCreature, Primitives.Predicate.hasSubtype (creatureType "Zombie")])) (from_ := some (graveyardOf Primitives.NounPhrase.you))) (timing := some onEachOfYourTurns)) ],
      power := stat 4, toughness := stat 4 } }

/-- "play lands from the top of your library" -/
def playLandsFromTop : StaticSpec :=
  can (actor you) (play (allOf isLand) (from_ := some onTop))
/-- "cast <p> spells from the top of your library" -/
def castFromTop (p : Predicate) : StaticSpec :=
  can (actor you) (cast (allOf p) (from_ := some onTop))

/-- Future Sight; Magus of the Future -/
def playLandsAndCastSpellsFromTop : StaticSpec :=
  Primitives.StaticSpec.conjunction none [playLandsFromTop, castFromTop isSpell]
theorem okPlayLandsAndCastSpellsFromTop :
    StaticSpec.check (DRS.ofList []) playLandsAndCastSpellsFromTop = [] := by decide
/-- Assemble the Players -/
def castSmallCreatureFromTopOnceEachTurn : StaticSpec :=
  can (actor you) (cast (counted (exactly 1) (Primitives.Predicate.and [isSpell, isCreature, Primitives.Predicate.compare [.stat .power] .atMost (.lit 2)])) (from_ := some onTop)) (timing := some (Primitives.Timing.inPart .turn none (Primitives.TurnRef.each none)))
theorem okCastSmallCreatureFromTopOnceEachTurn :
    StaticSpec.check (DRS.ofList []) castSmallCreatureFromTopOnceEachTurn = [] := by decide

def courserOfKruphix : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Courser of Kruphix", cost := some [generic 1, pip .green, pip .green],
      types := [.enchantment, .creature], subtypes := [creatureType "Centaur"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.visibility .reveal Primitives.NounPhrase.you Primitives.VisibleThing.topOfLibrary),
          Primitives.Ability.static playLandsFromTop,
          whenever (does (actor (a (Primitives.Predicate.and [isLand, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))) enter)
            (gainLife (.lit 1)) ],
      power := stat 2, toughness := stat 4 } }

def korlessaScaleSinger : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Korlessa, Scale Singer", cost := some [pip .green, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Dragon", creatureType "Bard"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.visibility .lookAt Primitives.NounPhrase.you Primitives.VisibleThing.topOfLibrary),
          Primitives.Ability.static (castFromTop (Primitives.Predicate.and [isSpell, Primitives.Predicate.hasSubtype (creatureType "Dragon")])) ],
      power := stat 1, toughness := stat 4 } }

def precognitionField : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Precognition Field", cost := some [generic 3, pip .blue], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.visibility .lookAt Primitives.NounPhrase.you Primitives.VisibleThing.topOfLibrary),
          Primitives.Ability.static (castFromTop (Primitives.Predicate.and [isSpell, isInstantOrSorcery])),
          activated (Primitives.Cost.mana [generic 3]) (exile (Primitives.NounPhrase.slice .top (.lit 1) (libraryOf Primitives.NounPhrase.you))) ] } }

def mysticForge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mystic Forge", cost := some [generic 4], types := [.artifact],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.visibility .lookAt Primitives.NounPhrase.you Primitives.VisibleThing.topOfLibrary),
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none [castFromTop (Primitives.Predicate.and [isSpell, isArtifact]), castFromTop (Primitives.Predicate.and
              [isSpell, isColorless])]),
          activated (Primitives.Cost.compound [Primitives.Cost.tapSymbol, payLife 1]) (exile (Primitives.NounPhrase.slice .top (.lit 1) (libraryOf Primitives.NounPhrase.you))) ] } }

def exploration : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Exploration", cost := some [pip .green], types := [.enchantment],
      text := [Primitives.Ability.static (can (actor you) (play (counted (additional (exactly 1)) isLand)) (timing := some onEachOfYourTurns))] } }

def oracleOfMulDaya : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Oracle of Mul Daya", cost := some [generic 3, pip .green], types := [.creature],
      subtypes := [creatureType "Elf", creatureType "Shaman"],
      text :=
        [ Primitives.Ability.static (can (actor you) (play (counted (additional (exactly 1)) isLand)) (timing := some onEachOfYourTurns)),
          Primitives.Ability.static (Primitives.StaticSpec.visibility .reveal Primitives.NounPhrase.you Primitives.VisibleThing.topOfLibrary),
          Primitives.Ability.static playLandsFromTop ],
      power := stat 2, toughness := stat 2 } }

def azusaLostButSeeking : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Azusa, Lost but Seeking", cost := some [generic 2, pip .green],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Monk"],
      text := [Primitives.Ability.static (can (actor you) (play (counted (additional (exactly 2)) isLand)) (timing := some onEachOfYourTurns))],
      power := stat 1, toughness := stat 2 } }

def summerBloom : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Summer Bloom", cost := some [generic 1, pip .green], types := [.sorcery],
      text := [Primitives.Ability.spell none (Primitives.Action.establish (can (actor you) (play (counted (additional (upTo 3)) isLand))) (some thisTurn))] } }

def explore : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Explore", cost := some [generic 1, pip .green], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ Primitives.Action.establish (can (actor you) (play (counted (additional (exactly 1)) isLand))) (some thisTurn),
              Primitives.Action.draw (.lit 1) ]) ] } }

def urbanEvolution : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Urban Evolution", cost := some [generic 3, pip .green, pip .blue],
      types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ Primitives.Action.draw (.lit 3),
              Primitives.Action.establish (can (actor you) (play (counted (additional (exactly 1)) isLand))) (some thisTurn) ]) ] } }

def dryadOfTheIlysianGrove : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dryad of the Ilysian Grove", cost := some [generic 2, pip .green],
      types := [.enchantment, .creature], subtypes := [creatureType "Nymph", creatureType "Dryad"],
      text :=
        [ Primitives.Ability.static (can (actor you) (play (counted (additional (exactly 1)) isLand)) (timing := some onEachOfYourTurns)),
          Primitives.Ability.static (Primitives.StaticSpec.qualityChange (allOf (Primitives.Predicate.and [isLand, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .adds
            (Primitives.QualityPayload.everyTypeOf .basicLand)) ],
      power := stat 2, toughness := stat 4 } }

/-- Ashes of the Fallen -/
def ashesOfTheFallen : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ashes of the Fallen", cost := some [generic 2], types := [.artifact],
      text :=
        [ Primitives.Ability.static (entersChoosing thisArtifact (.subtype .creature)),
          Primitives.Ability.static (Primitives.StaticSpec.qualityChange (each (Primitives.Predicate.and [isCreature, Primitives.Predicate.in_ (graveyardOf Primitives.NounPhrase.you)])) .adds
            (Primitives.QualityPayload.chosenQuality (ofChosen (.subtype .creature)))) ] } }

def hellkiteCharger : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hellkite Charger", cost := some [generic 4, pip .red, pip .red],
      types := [.creature], subtypes := [creatureType "Dragon"],
      text :=
        [ keyword (.named "flying"), keyword (.named "haste"),
          whenever (does (actor thisCreature) attack)
            (Primitives.Action.offer (Primitives.Action.pay (Primitives.Cost.mana [generic 5, pip .red, pip .red]) .once .paid none none)
              (some (Primitives.Action.sequentially
                [ Primitives.Action.setStatus .untapped (allOf (Primitives.Predicate.and [isCreature, isAttacking])),
                  additionalPhase .combat none (.lit 1) ]))
              none) ],
      power := stat 5, toughness := stat 5 } }

/-- Foriysian Brigade -/
def foriysianBrigade : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Foriysian Brigade", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text := [Primitives.Ability.static (can (actor thisCreature) (block (counted (additional (exactly 1)) isCreature)) (timing := some eachCombat))],
      power := stat 2, toughness := stat 4 } }

/-- Two-Headed Giant of Foriys -/
def twoHeadedGiantOfForiys : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Two-Headed Giant of Foriys", cost := some [generic 4, pip .red],
      types := [.creature], subtypes := [creatureType "Giant"],
      text := [keyword (.named "trample"), Primitives.Ability.static (can (actor thisCreature) (block (counted (additional (exactly 1)) isCreature)) (timing := some eachCombat))],
      power := stat 4, toughness := stat 4 } }

def highGround : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "High Ground", cost := some [pip .white], types := [.enchantment],
      text := [Primitives.Ability.static (can (actor (each (and [isCreature, youControl]))) (block (counted (additional (exactly 1)) isCreature)) (timing := some eachCombat))] } }

/-- Watcher in the Web -/
def watcherInTheWeb : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Watcher in the Web", cost := some [generic 4, pip .green], types := [.creature],
      subtypes := [creatureType "Spider"],
      text := [keyword (.named "reach"), Primitives.Ability.static (can (actor thisCreature) (block (counted (additional (exactly 7)) isCreature)) (timing := some eachCombat))],
      power := stat 2, toughness := stat 5 } }

/-- Forgotten Lore -/
def forgottenLoreRepeat : Action :=
  Primitives.Action.sequentially
    [ instruct (target (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) (Primitives.Action.choose none (a (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (graveyardOf Primitives.NounPhrase.you)])) .openly none),
      Primitives.Action.offer (Primitives.Action.pay (Primitives.Cost.mana [pip .green]) .once .paid none none) (some (Primitives.Action.repeat_
          Primitives.Repetition.againExcludingChosen)) none ]
theorem okForgottenLoreRepeat : Action.check (DRS.ofList []) forgottenLoreRepeat = [] := by decide

/-- Leyline of the Meek -/
def leylineOfTheMeek : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Leyline of the Meek", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.mayBeginOnBattlefield,
          Primitives.Ability.static (getsPt (allOf (Primitives.Predicate.and [isCreature, Primitives.Predicate.isToken])) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))) ] } }

/-- Leyline of Vitality -/
def leylineOfVitality : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Leyline of Vitality", cost := some [generic 2, pip .green, pip .green],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.mayBeginOnBattlefield,
          Primitives.Ability.static (getsPt (allOf (and [isCreature, youControl])) (Primitives.Delta.up (.lit 0)) (Primitives.Delta.up (.lit 1))),
          whenever (does (actor (a (and [isCreature, youControl]))) enter) (may (gainLife (.lit 1))) ] } }

def phyrexianRevoker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Phyrexian Revoker", cost := some [generic 2], types := [.artifact, .creature],
      subtypes := [creatureType "Phyrexian", creatureType "Horror"],
      text :=
        [ Primitives.Ability.static (entersChoosingFrom thisCreature .cardName (Primitives.ChoiceDomain.nameOfCard (Primitives.Predicate.not isLand))),
          Primitives.Ability.static (cant Primitives.Subject.any (activate (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .anyActivated,
                           Primitives.Predicate.abilityOf (allOf (Primitives.Predicate.and [isSource, Primitives.Predicate.named Primitives.NameSource.chosen])) ])))) ],
      power := stat 2, toughness := stat 1 } }

def voidstoneGargoyle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Voidstone Gargoyle", cost := some [generic 3, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Gargoyle"],
      text :=
        [ keyword (.named "flying"),
          Primitives.Ability.static (entersChoosingFrom thisCreature .cardName (Primitives.ChoiceDomain.nameOfCard (Primitives.Predicate.not isLand))),
          Primitives.Ability.static (cant Primitives.Subject.any (cast (allOf (Primitives.Predicate.and [isSpell, Primitives.Predicate.named Primitives.NameSource.chosen])))),
          Primitives.Ability.static (cant Primitives.Subject.any (activate (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .anyActivated,
                           Primitives.Predicate.abilityOf (allOf (Primitives.Predicate.and [isSource, Primitives.Predicate.named Primitives.NameSource.chosen])) ])))) ],
      power := stat 3, toughness := stat 3 } }

def pithingNeedle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pithing Needle", cost := some [generic 1], types := [.artifact],
      text :=
        [ Primitives.Ability.static (entersChoosing thisArtifact .cardName),
          Primitives.Ability.static (cant Primitives.Subject.any (activate (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .anyActivated,
                           Primitives.Predicate.abilityOf (allOf (Primitives.Predicate.and [isSource, Primitives.Predicate.named Primitives.NameSource.chosen])),
                           Primitives.Predicate.not Primitives.Predicate.isManaAbility ])))) ] } }

/-- Rhystic Study -/
def rhysticStudy : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rhystic Study", cost := some [generic 2, pip .blue], types := [.enchantment],
      text :=
        [ whenever (Primitives.GameEvent.does (Primitives.Subject.actor anOpponent) (Primitives.Action.procedure .cast ((a isSpell)) Primitives.PlayPayment.itsOwnCost none none none))
            (instruct (that .player) (doUnless (instruct Primitives.NounPhrase.you (may (Primitives.Action.draw (.lit 1))))
              (Primitives.Cost.mana [generic 1]))) ] } }

def gandalfWhiteRider : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gandalf, White Rider", cost := some [generic 3, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Avatar", creatureType "Wizard"],
      text :=
        [ keyword (.named "vigilance"),
          whenever (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.procedure .cast ((a isSpell)) Primitives.PlayPayment.itsOwnCost none none none))
            (Primitives.Action.sequentially
              [ get (each (and [isCreature, youControl])) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn),
                scry (.lit 1) ]),
          when (does (actor thisCreature) die) (may (Primitives.Action.move it Primitives.Location.anyLocation (some (Primitives.Location.fromEnd .top (.nth 5) (Primitives.Location.zone .library none))) .arrival none [])) ],
      power := stat 3, toughness := stat 3 } }

def helmOfPossession : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Helm of Possession", cost := some [generic 4], types := [.artifact],
      text :=
        [ Primitives.Ability.static (mayDeclineUntap thisArtifact (some Primitives.NounPhrase.you)),
          activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2], Primitives.Cost.tapSymbol, Primitives.Cost.perform (sacrifice (a isCreature))])
            (gainControl (target isCreature)
              (some (Primitives.Duration.forAsLongAs
                (Primitives.Condition.and [ Primitives.Condition.matches thisArtifact (Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you),
                        Primitives.Condition.matches thisArtifact isTapped ]))) (controller := Primitives.NounPhrase.you)) ] } }

def override : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Override", cost := some [generic 2, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (instruct (controllerOf (target isSpell)) (doUnless (counter it)
            (scaledGenericMana (forEach (.lit 1) (Primitives.Predicate.and [isArtifact, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))))) ] } }

def rakshasasDisdain : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rakshasa's Disdain", cost := some [generic 2, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (instruct (controllerOf (target isSpell)) (doUnless (counter it)
            (scaledGenericMana (forEach (.lit 1) (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (graveyardOf Primitives.NounPhrase.you)]))))) ] } }

def megatherium : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Megatherium", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Beast"],
      text :=
        [ keyword (.named "trample"),
          when (does (actor thisCreature) enter)
            (doUnless (sacrifice thisCreature)
              (scaledGenericMana (forEach (.lit 1) (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (handOf Primitives.NounPhrase.you)])))) ],
      power := stat 4, toughness := stat 4 } }

def killingWave : Action :=
  Primitives.Action.doForEach (each isCreature)
    (instruct (controllerOf it) (doUnless (instruct they (sacrifice it)) (Primitives.Cost.perform (instruct they (loseLife (Primitives.Amount.letter .x))))))
theorem okKillingWave : Action.check (DRS.ofList []) killingWave = [] := by decide
def fadeAway : Action :=
  Primitives.Action.doForEach (each isCreature)
    (instruct (controllerOf it) (doUnless (instruct they (sacrifice (a isPermanent))) (Primitives.Cost.mana [generic 1])))
theorem okFadeAway : Action.check (DRS.ofList []) fadeAway = [] := by decide

def tidalFlats : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tidal Flats", cost := some [pip .blue], types := [.enchantment],
      text :=
        [ activated (Primitives.Cost.mana [pip .blue, pip .blue])
            (Primitives.Action.doForEach (each (Primitives.Predicate.and [isCreature, isAttacking, Primitives.Predicate.not (Primitives.Predicate.hasKeyword (.the (.named "flying")))]))
              (instruct (controllerOf it) (Primitives.Action.offer (Primitives.Action.pay (Primitives.Cost.mana [generic 1]) .once .paid none none) none
                (some (gain
                  (allOf (Primitives.Predicate.and [ isCreature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                                 Primitives.Predicate.inCombat .blockerOf (some (that (.type .creature))) ]))
                  (keyword (.named "firstStrike")) (some untilEndOfTurn)))))) ] }
                      }

def primalSurge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Primal Surge", cost := some [generic 8, pip .green, pip .green], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ exile (topSlice (.lit 1)),
              Primitives.Action.doIf (Primitives.Condition.matches it isPermanentCard)
                (Primitives.Action.offer (putOntoBattlefield it) (some (Primitives.Action.repeat_ Primitives.Repetition.again)) none) none
                    ]) ] } }

def cultivatorColossus : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cultivator Colossus", cost := some [generic 4, pip .green, pip .green, pip .green],
      types := [.creature], subtypes := [creatureType "Plant", creatureType "Beast"],
      text :=
        [ keyword (.named "trample"),
          Primitives.Ability.static (Primitives.StaticSpec.ptDefinition thisCreature .bothEach
            (countOf (Primitives.Predicate.and [isLand, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))),
          when (does (actor thisCreature) enter)
            (Primitives.Action.offer (putOntoBattlefieldTapped (a (Primitives.Predicate.and [isLand, Primitives.Predicate.in_ (handOf Primitives.NounPhrase.you)])))
              (some (Primitives.Action.sequentially [Primitives.Action.draw (.lit 1), Primitives.Action.repeat_ Primitives.Repetition.again])) none) ] } }

def zimoneAndDina : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.tapSymbol, Primitives.Cost.perform (sacrifice (a ((and [isCreature, otherThan thisCreature]))))])
    (Primitives.Action.sequentially
      [ Primitives.Action.draw (.lit 1),
        may (putOntoBattlefieldTapped (a (Primitives.Predicate.and [isLand, Primitives.Predicate.in_ (handOf Primitives.NounPhrase.you)]))),
        Primitives.Action.doIf (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [isLand, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .atLeast (.lit
            8))
          (Primitives.Action.repeat_ (Primitives.Repetition.moreTimes (.lit 1))) none ])
theorem okZimoneAndDina : Ability.check (DRS.ofList []) zimoneAndDina = [] := by decide

def cryptLurker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crypt Lurker", cost := some [generic 3, pip .black], types := [.creature],
      subtypes := [creatureType "Horror"],
      text :=
        [ when (does (actor thisCreature) enter)
            (Primitives.Action.offer
              (chooseModes (exactly 1)
                [ sacrifice (a isCreature),
                  discard (a (Primitives.Predicate.and [isCreature, Primitives.Predicate.in_ yourHand])) ])
              (some (Primitives.Action.draw (.lit 1))) none) ],
      power := stat 3, toughness := stat 4 } }

/-- Memoricide -/
def memoricideSearch : Action :=
  Primitives.Action.sequentially
    [ choose (a (qualityFrom .cardName (Primitives.ChoiceDomain.nameOfCard (Primitives.Predicate.not isLand)))),
      searchZonesOf (target Primitives.Predicate.anyPlayer) (exactly 1) (Primitives.Predicate.named Primitives.NameSource.chosen),
      instruct (that .player) shuffle ]
theorem okMemoricideSearch : Action.check (DRS.ofList []) memoricideSearch = [] := by decide
/-- Lost Hours -/
def lostHoursPlacement : Action :=
  Primitives.Action.sequentially
    [ instruct (target Primitives.Predicate.anyPlayer) (revealHand),
      choose (a (Primitives.Predicate.and [Primitives.Predicate.not isLand, Primitives.Predicate.in_ (handOf they)])),
      instruct (that .player) (put it (Primitives.Location.fromEnd .top (.nth 3) (Primitives.Location.zone .library none))) ]
theorem okLostHoursPlacement : Action.check (DRS.ofList []) lostHoursPlacement = [] := by decide
/-- Aether Gust -/
def aetherGustPlacement : Action :=
  Primitives.Action.sequentially
    [ choose (target (Primitives.Predicate.and [isPermanent, Primitives.Predicate.colorIs .red])),
      instruct (ownerOf it) (put it (Primitives.Location.or [onTop, onBottom])) ]
theorem okAetherGustPlacement : Action.check (DRS.ofList []) aetherGustPlacement = [] := by decide
/-- Fastbond -/
def fastbondLands : Ability := Primitives.Ability.static (can (actor you) (play (counted (additional anyNumber) isLand)))
theorem okFastbondLands : Ability.check (DRS.ofList []) fastbondLands = [] := by decide
/-- Panglacial Wurm -/
def panglacialWurmCast : Ability :=
  Primitives.Ability.static (can (actor you) (cast Primitives.NounPhrase.this (from_ := some yourLibrary)) (timing := some whileSearchingYourLibrary))
theorem okPanglacialWurmCast : Ability.check (DRS.ofList []) panglacialWurmCast = [] := by decide

/-- Shah of Naar Isle -/
def shahOfNaarIsle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shah of Naar Isle", cost := some [generic 3, pip .red], types := [.creature],
      subtypes := [creatureType "Efreet"],
      text :=
        [ keyword (.named "trample"),
          keywordCosting (.named "echo") (Primitives.Cost.mana [generic 0]),
          when (Primitives.GameEvent.does Primitives.Subject.any (Primitives.Action.pay Primitives.Cost.anyCost .once .paid (some thisCreature) (some (.named "echo"))))
            (instruct (each (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) (may (Primitives.Action.draw (Primitives.Amount.upTo (.lit 3))))) ],
      power := stat 6, toughness := stat 6 } }

/-- Memory Plunder -/
def memoryPlunder : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Memory Plunder",
      cost := some [hybridPip .blue .black, hybridPip .blue .black, hybridPip .blue .black,
                    hybridPip .blue .black],
      types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.establish
            (can (actor you) (cast (target (Primitives.Predicate.and [isInstantOrSorcery, Primitives.Predicate.isCard])) (from_ := some (graveyardOf anOpponent)) (payment := Primitives.PlayPayment.withoutPaying)))
            none) ] } }

/-- Omniscience -/
def omniscience : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Omniscience", cost := some [generic 7, pip .blue, pip .blue, pip .blue],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.static (can (actor you) (cast (allOf isSpell) (from_ := some (handOf Primitives.NounPhrase.you)) (payment := Primitives.PlayPayment.withoutPaying))) ] } }

/-- Tranquil Frillback -/
def tranquilFrillbackOffer : Action := may (Primitives.Action.pay (Primitives.Cost.mana [pip .green]) (.upTo 3) .paid none none)
theorem okTranquilFrillbackOffer : Action.check (DRS.ofList []) tranquilFrillbackOffer = [] := by decide

/-- Caller of the Hunt -/
def callerOfTheHunt : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Caller of the Hunt", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Human"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.addedCost (Primitives.Cost.perform (choose (a (quality (.subtype .creature))))) false .once),
          Primitives.Ability.static (Primitives.StaticSpec.ptDefinition thisCreature .bothEach
            (countOf (Primitives.Predicate.and [isCreature, ofChosen (.subtype .creature)]))) ] } }

/-- Duneblast -/
def duneblast : Action :=
  Primitives.Action.sequentially [choose (counted (upTo 1) isCreature), destroy (theRest .object)]
theorem okDuneblast : Action.check (DRS.ofList []) duneblast = [] := by decide
/-- Boreas Charger -/
def boreasChargerChoice : Action := choose (a opponentWithMoreLands)
theorem okBoreasChargerChoice : Action.check (DRS.ofList []) boreasChargerChoice = [] := by decide
/-- Boreas Charger's spell text -/
def boreasChargerSpell : Action :=
  Primitives.Action.sequentially
    [ choose (a opponentWithMoreLands),
      searchLibraryFor (Primitives.Quantity.exactlyOf Primitives.Amount.theDifference) (Primitives.Predicate.hasSubtype (landType "Plains")),
      reveal (those .card),
      putOntoBattlefieldTapped (someOf (exactly 1) (those .card)),
      putInto (theRest .object) hand ]
theorem okBoreasChargerSpell : Action.check (DRS.ofList []) boreasChargerSpell = [] := by decide

/-- Sandstone Oracle -/
def sandstoneOracle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sandstone Oracle", cost := some [generic 7], types := [.artifact, .creature],
      subtypes := [creatureType "Sphinx"],
      text :=
        [ keyword (.named "flying"),
          when (does (actor thisCreature) enter)
            (Primitives.Action.sequentially
              [ choose anOpponent,
                Primitives.Action.doIf (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (handOf (that .player))])) .greater
                        (countOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (handOf Primitives.NounPhrase.you)])))
                  (Primitives.Action.draw Primitives.Amount.theDifference) none ]) ],
      power := stat 4, toughness := stat 4 } }

/-- Slithermuse -/
def slithermuseTrigger : Ability :=
  when (does (actor thisCreature) leave)
    (Primitives.Action.sequentially
      [ choose anOpponent,
        Primitives.Action.doIf (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (handOf (that .player))])) .greater
                (countOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (handOf Primitives.NounPhrase.you)])))
          (Primitives.Action.draw Primitives.Amount.theDifference) none ])
theorem okSlithermuseTrigger : Ability.check (DRS.ofList []) slithermuseTrigger = [] := by decide

/-- Celestial Judgment -/
def celestialJudgment : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Celestial Judgment", cost := some [generic 4, pip .white, pip .white],
      types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ Primitives.Action.doForEachKind (.value .power) (some (allOf isCreature)) .number
                (choose (a (Primitives.Predicate.and [isCreature, Primitives.Predicate.compare [.stat .power] .eq chosenNumber]))),
              destroy (each (Primitives.Predicate.and [isCreature, Primitives.Predicate.notChosen])) ]) ] } }

/-- World Queller -/
def worldQuellerChoice : Action :=
  Primitives.Action.offer (choose (a (quality .cardType)))
    (some (instruct (each Primitives.Predicate.anyPlayer) (sacrifice (aTheirChoice (Primitives.Predicate.and [isPermanent, ofChosen .cardType]))))) none
theorem okWorldQuellerChoice : Action.check (DRS.ofList []) worldQuellerChoice = [] := by decide

/-- Moonlit Meditation -/
def moonlitMeditation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Moonlit Meditation", cost := some [generic 2, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") (Primitives.Predicate.and [Primitives.Predicate.or [isArtifact, isCreature], Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]),
          Primitives.Ability.static (Primitives.StaticSpec.instead (Primitives.WouldClause.would (Primitives.Subject.by Primitives.NounPhrase.you) (Primitives.Action.createObject Primitives.Amount.anyAmount (Primitives.CreationSpec.token (Primitives.TokenSpec.copyOf (counted (atLeast 1) Primitives.Predicate.isToken) []) []) false none)) (may (create Primitives.Amount.groupSize (Primitives.TokenSpec.copyOf (Primitives.NounPhrase.attachHost .enchanted .permanent) []) [])) none .repeatedly (some oncePerTurn)) ] } }

def discardUpToTwoThenDrawThatMany : Action :=
  Primitives.Action.sequentially [discard (counted (upTo 2) (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ yourHand])), Primitives.Action.draw Primitives.Amount.groupSize]
theorem okDiscardUpToTwoThenDrawThatMany :
    Action.check (DRS.ofList []) discardUpToTwoThenDrawThatMany = [] := by decide
/-- Truce and Temporary Truce -/
def drawUpToTwoThenGainPerShortfall : Action :=
  instruct (each Primitives.Predicate.anyPlayer)
    (Primitives.Action.sequentially
      [ may (Primitives.Action.draw (Primitives.Amount.upTo (.lit 2))),
        instruct (that .player) (gainLife (times (.lit 2) shortOfCeiling)) ])
theorem okDrawUpToTwoThenGainPerShortfall :
    Action.check (DRS.ofList []) drawUpToTwoThenGainPerShortfall = [] := by decide

/-- Commune with the Gods -/
def communeWithTheGods : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Commune with the Gods", cost := some [generic 1, pip .green], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ reveal (topSlice (.lit 5)),
              may (putInto (fromAmong (exactly 1) (Primitives.Predicate.or [isCreature, isEnchantment]) them) hand),
              putInto (theRest .object) graveyard ]) ] } }

/-- Nautiloid Ship -/
def nautiloidShipTrigger : Ability :=
  whenever (does (actor thisVehicle) (dealDamage (to := a Primitives.Predicate.anyPlayer) (kind := .combatOnly)))
    (may (putOntoBattlefieldUnderYourControl (a (Primitives.Predicate.and [isCreature, Primitives.Predicate.in_ (Primitives.Location.exiledWith thisVehicle)]))))
theorem okNautiloidShipTrigger : Ability.check (DRS.ofList []) nautiloidShipTrigger = [] := by decide

/-- Summon: Esper Valigarmanda's II/III/IV body, in part -/
def summonEsperValigarmandaCast : StaticSpec :=
  Primitives.StaticSpec.conjunction none
    [ can (actor you) (cast (a (Primitives.Predicate.and [isInstantOrSorcery, Primitives.Predicate.in_ (Primitives.Location.exiledWith thisSaga)]))),
      can (actor you) (pay anyCost .once) (asThough := some (Primitives.AsThough.mana none .anyType (some (Primitives.SpendPurpose.toCast (Primitives.Predicate.and [isInstantOrSorcery, Primitives.Predicate.in_ (Primitives.Location.exiledWith thisSaga)]))))) ]
theorem okSummonEsperValigarmandaCast :
    StaticSpec.check (DRS.ofList []) summonEsperValigarmandaCast = [] := by decide
/-- Rogue Class's level-3 body -/
def rogueClassLevelThree : StaticSpec :=
  Primitives.StaticSpec.conjunction none
    [ can (actor you) (play (allOf (Primitives.Predicate.in_ (Primitives.Location.exiledWith thisClass)))),
      can (actor you) (pay anyCost .once) (asThough := some (Primitives.AsThough.mana none .anyColor (some (Primitives.SpendPurpose.toCast (Primitives.Predicate.in_ (Primitives.Location.exiledWith thisClass)))))) ]
theorem okRogueClassLevelThree : StaticSpec.check (DRS.ofList []) rogueClassLevelThree = [] := by decide

/-- Soul Ransom -/
def soulRansom : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Soul Ransom", cost := some [generic 2, pip .blue, pip .black], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") isCreature,
          Primitives.Ability.static (Primitives.StaticSpec.controlGrant Primitives.NounPhrase.you (Primitives.NounPhrase.attachHost .enchanted (.type .creature))),
          activatedBy
            (Primitives.Cost.perform (Primitives.Action.repeatTimes (.lit 2)
              (Primitives.Action.sequentially [choose (a (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ hand])), discard (that .card)])))
            (Primitives.Action.sequentially [instruct (controllerOf thisAura) (sacrifice itPermanent), instruct they (Primitives.Action.draw (.lit 2))])
            (allOf (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) ] } }

/-- Vraska's Scorn -/
def vraskasScorn : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vraska's Scorn", cost := some [generic 2, pip .black, pip .black], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ instruct (target (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) (loseLife (.lit 4)),
              may
                (Primitives.Action.sequentially
                  [ searchLibraryOrGraveyard (Primitives.Predicate.named (Primitives.NameSource.printed "Vraska, Scheming Gorgon")),
                    reveal it, putInto it hand ]),
              Primitives.Action.doIf (happenedThisWay (Primitives.Action.search (Primitives.Location.anyLocation) anyNumber (Primitives.Predicate.and [])) (Primitives.Subject.actor Primitives.NounPhrase.you)) shuffle
                none ]) ] } }

/-- Old-Growth Dryads -/
def oldGrowthDryads : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Old-Growth Dryads", cost := some [pip .green], types := [.creature],
      subtypes := [creatureType "Dryad"],
      text :=
        [ when (does (actor thisCreature) enter)
            (instruct (each (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) (may
              (Primitives.Action.sequentially
                [ Primitives.Action.search (Primitives.Location.zone .library (some they)) (exactly 1) (Primitives.Predicate.and [isLand, Primitives.Predicate.hasSupertype .basic]),
                  putOntoBattlefieldTapped (theVerbed (.action "search") .one),
                  shuffle ]))) ],
      power := stat 3, toughness := stat 3 } }

/-- Verity Circle -/
def verityCircle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Verity Circle", cost := some [generic 2, pip .blue], types := [.enchantment],
      text :=
        [ triggeredIf
            (Primitives.GameEvent.does Primitives.Subject.any (Primitives.Action.setStatus .tapped (a (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller anOpponent]))))
            (Primitives.Condition.matches it (Primitives.Predicate.not (Primitives.Predicate.inCombat .declaredAttacker none)))
            (may (Primitives.Action.draw (.lit 1))),
          activated (Primitives.Cost.mana [generic 4, pip .blue])
            (tap (target (Primitives.Predicate.and [isCreature, Primitives.Predicate.not (Primitives.Predicate.hasKeyword (.the (.named "flying")))]))) ] } }

/-- Danitha, New Benalia's Light -/
def danithaNewBenaliasLight : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Danitha, New Benalia's Light", cost := some [generic 1, pip .green, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Knight"],
      text :=
        [ keyword (.named "vigilance"), keyword (.named "trample"), keyword (.named "lifelink"),
          Primitives.Ability.static (can (actor you) (cast (counted (exactly 1) (Primitives.Predicate.and [ isSpell, Primitives.Predicate.or [ Primitives.Predicate.hasSubtype (enchantmentType "Aura"),
                                    Primitives.Predicate.hasSubtype (artifactType "Equipment") ] ])) (from_ := some (graveyardOf Primitives.NounPhrase.you))) (timing := some onEachOfYourTurns)) ],
      power := stat 2, toughness := stat 2 } }

/-- Muldrotha, the Gravetide -/
def muldrothaLandWindow : StaticSpec :=
  can (actor you) (play (a isLand) (from_ := some (graveyardOf Primitives.NounPhrase.you))) (timing := some onEachOfYourTurns)
theorem okMuldrothaLandWindow : StaticSpec.check (DRS.ofList []) muldrothaLandWindow = [] := by decide
/-- Nahiri's Lithoforming -/
def nahiriExtraLands : StaticSpec := can (actor you) (play (counted (additional (Primitives.Quantity.exactlyOf (Primitives.Amount.letter .x))) isLand))
theorem okNahiriExtraLands : StaticSpec.check (DRS.ofList [letterB .x]) nahiriExtraLands = [] := by decide

/-- Haakon, Stromgald Scourge -/
def haakonStromgaldScourge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Haakon, Stromgald Scourge", cost := some [generic 1, pip .black, pip .black],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Zombie", creatureType "Knight"],
      text :=
        [ Primitives.Ability.static (can (actor you) (cast Primitives.NounPhrase.this (from_ := some (graveyardOf Primitives.NounPhrase.you)))),
          Primitives.Ability.static (Primitives.StaticSpec.cant (Primitives.Subject.actor Primitives.NounPhrase.you)
            (Primitives.Action.procedure .cast Primitives.NounPhrase.this Primitives.PlayPayment.itsOwnCost none
              (some (Primitives.Location.not (graveyardOf Primitives.NounPhrase.you))) none) {}),
          Primitives.Ability.static (Primitives.StaticSpec.conditional
            (can (actor you) (cast (allOf (Primitives.Predicate.and [isSpell, Primitives.Predicate.hasSubtype (creatureType "Knight")])) (from_ := some (graveyardOf Primitives.NounPhrase.you))))
            (Primitives.Condition.matches Primitives.NounPhrase.this (Primitives.Predicate.in_ battlefield)) .asLongAs),
          when (does (actor thisCreature) die) (loseLife (.lit 2)) ],
      power := stat 3, toughness := stat 3 } }

/-- Apex of Power -/
def apexOfPowerCast : Action :=
  Primitives.Action.sequentially
    [ exile (Primitives.NounPhrase.slice .top (.lit 7) (libraryOf Primitives.NounPhrase.you)),
      Primitives.Action.establish
        (can (actor you) (cast (fromAmong anyNumber isSpell them)))
        (some thisTurn) ]
theorem okApexOfPowerCast : Action.check (DRS.ofList []) apexOfPowerCast = [] := by decide
/-- Blight Herder -/
def blightHerderCast : Action :=
  may
    (putInto (counted (exactly 2)
            (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.hasPossessor .owner (allOf (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)), Primitives.Predicate.in_ exileZone]))
      graveyard)
theorem okBlightHerderCast : Action.check (DRS.ofList []) blightHerderCast = [] := by decide

/-- True-Name Nemesis -/
def trueNameNemesis : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "True-Name Nemesis", cost := some [generic 1, pip .blue, pip .blue],
      types := [.creature], subtypes := [creatureType "Merfolk", creatureType "Rogue"],
      text :=
        [ Primitives.Ability.static (entersChoosingPlayer thisCreature none),
          keywordQuality (.named "protection") isChosenPlayer ],
      power := stat 3, toughness := stat 1 } }

/-- Akiri, Fearless Voyager -/
def akiriUnattachOffer : Ability :=
  activated (Primitives.Cost.mana [pip .white])
    (Primitives.Action.offer
      (Primitives.Action.unattach (a (Primitives.Predicate.and [ Primitives.Predicate.hasSubtype (artifactType "Equipment"),
                            Primitives.Predicate.attachedTo (a (and [isCreature, youControl])) ])))
      (some (Primitives.Action.setStatus .tapped (that (.type .creature)))) none)
theorem okAkiriUnattachOffer : Ability.check (DRS.ofList []) akiriUnattachOffer = [] := by decide
/-- Summoning Materia -/
def summoningMateriaTopCast : Ability :=
  Primitives.Ability.static (onlyWhile (castFromTop (Primitives.Predicate.and [isSpell, isCreature]))
    (Primitives.Condition.matches thisEquipment (Primitives.Predicate.attachedTo (a isCreature))))
theorem okSummoningMateriaTopCast : Ability.check (DRS.ofList []) summoningMateriaTopCast = [] := by decide
/-- Vizier of the Menagerie -/
def vizierOfTheMenagerieSpend : StaticSpec :=
  can (actor you) (pay anyCost .once) (asThough := some (Primitives.AsThough.mana none .anyType (some (Primitives.SpendPurpose.toCast (Primitives.Predicate.and [isCreature, isSpell])))))
theorem okVizierOfTheMenagerieSpend : StaticSpec.check (DRS.ofList []) vizierOfTheMenagerieSpend = [] := by
  decide

/-- Conspicuous Snoop -/
def conspicuousSnoop : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Conspicuous Snoop", cost := some [pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Goblin", creatureType "Rogue"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.visibility .reveal Primitives.NounPhrase.you Primitives.VisibleThing.topOfLibrary),
          Primitives.Ability.static (can (actor you) (play (allOf (Primitives.Predicate.and [isSpell, Primitives.Predicate.hasSubtype (creatureType "Goblin")])) (from_ := some onTop))),
          Primitives.Ability.static (onlyWhile
            (Primitives.StaticSpec.abilityGrantFrom thisCreature [.anyActivated] (topSlice (.lit 1)) none)
            (Primitives.Condition.matches (topSlice (.lit 1)) (Primitives.Predicate.hasSubtype (creatureType "Goblin")))) ],
      power := stat 2, toughness := stat 2 } }

/-- Ballot Broker -/
def ballotBroker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ballot Broker", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Advisor"],
      text := [Primitives.Ability.static (can (actor you) (vote (times := some (additional (exactly 1)))))],
      power := stat 2, toughness := stat 3 } }

/-- Emissary of Grudges -/
def emissaryOfGrudgesReveal : Ability :=
  activatedOnlyOnce (Primitives.Cost.perform (Primitives.Action.expose .reveal (Primitives.Exposed.choice .player)))
    (Primitives.Action.doOnlyIf (Primitives.Action.chooseNewTargets (target (Primitives.Predicate.or [isSpell, Primitives.Predicate.abilityHead .anyOnStack])))
      (Primitives.Condition.and
        [ Primitives.Condition.matches it (Primitives.Predicate.hasPossessor .controller (the isChosenPlayer)),
          Primitives.Condition.matches it
            (Primitives.Predicate.targets (youOr (a (Primitives.Predicate.and [isPermanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))) .someTarget) ])
      none)
    oncePerGame
theorem okEmissaryOfGrudgesReveal :
    Ability.check (DRS.ofList [ChoiceSort.binding .player]) emissaryOfGrudgesReveal = [] := by decide

def prosperity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Prosperity", cost := some [.variable, pip .blue], types := [.sorcery],
      text := [Primitives.Ability.spell none (instruct (each Primitives.Predicate.anyPlayer) (Primitives.Action.draw (Primitives.Amount.letter .x)))] } }

def collectiveUnconscious : Action := Primitives.Action.draw (forEach (.lit 1) (and [isCreature, youControl]))
theorem okCollectiveUnconscious : Action.check (DRS.ofList []) collectiveUnconscious = [] := by decide
def killiansConfidence : Action :=
  Primitives.Action.sequentially
    [get (target isCreature) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn), Primitives.Action.draw (.lit 1)]
theorem okKilliansConfidence : Action.check (DRS.ofList []) killiansConfidence = [] := by decide

def scatheZombies : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Scathe Zombies", cost := some [generic 2, pip .black], types := [.creature],
      subtypes := [creatureType "Zombie"], power := stat 2, toughness := stat 2 } }

/-- A basic land card of the named type. -/
def basicLand (name : String) : Card :=
  .singleFaced
    { characteristics :=
      { name, supertypes := [.basic], types := [.land], subtypes := [landType name] } }
def basicLandCards : List Card :=
  ["Plains", "Island", "Swamp", "Mountain", "Forest"].map basicLand
theorem okBasicLandCards : basicLandCards.flatMap Card.check = [] := by decide

def snowCoveredForest : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Snow-Covered Forest", supertypes := [.basic, .snow], types := [.land],
      subtypes := [landType "Forest"] } }

/-- Not Forgotten -/
def notForgottenPlacement : Action :=
  Primitives.Action.move (target (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ graveyard])) Primitives.Location.anyLocation (some (Primitives.Location.or [onTop, onBottom])) .arrival none []
theorem okNotForgottenPlacement : Action.check (DRS.ofList []) notForgottenPlacement = [] := by decide

def duneblastCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Duneblast", cost := some [generic 4, pip .white, pip .black, pip .green],
      types := [.sorcery], text := [Primitives.Ability.spell none duneblast] } }

def boreasChargerDifference : Amount := Primitives.Amount.theDifference
theorem okBoreasChargerDifference :
    Amount.check ((stepAction 512 .postEvent .obj (DRS.ofList []) boreasChargerChoice).1) boreasChargerDifference = [] := by decide

/-- Hymn to Tourach -/
def hymnToTourach : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hymn to Tourach", cost := some [pip .black, pip .black], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (discard (countedAtRandom (exactly 2) (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (handOf (target
            Primitives.Predicate.anyPlayer))]))) ] } }

/-- Caught in the Crossfire -/
def caughtInTheCrossfire : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Caught in the Crossfire", cost := some [pip .red, pip .red], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (chooseSpree
            [ (some (Primitives.Cost.mana [generic 1]),
               instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 2) (each (Primitives.Predicate.and [isCreature, isOutlaw])) .noncombatOnly)),
              (some (Primitives.Cost.mana [generic 1]),
               instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 2) (each (Primitives.Predicate.and [isCreature, Primitives.Predicate.not isOutlaw])) .noncombatOnly)) ]) ] } }

/-- Requisition Raid -/
def requisitionRaid : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Requisition Raid", cost := some [pip .white], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (chooseSpree
            [ (some (Primitives.Cost.mana [generic 1]), destroy (target isArtifact)),
              (some (Primitives.Cost.mana [generic 1]), destroy (target isEnchantment)),
              (some (Primitives.Cost.mana [generic 1]),
               Primitives.Action.putCounters (.lit 1) (Primitives.CounterKindSource.printed p1p1Counter)
                 (each (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller (target Primitives.Predicate.anyPlayer)])) .many true) ]) ] } }

/-- Rustler Rampage -/
def rustlerRampage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rustler Rampage", cost := some [pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (chooseSpree
            [ (some (Primitives.Cost.mana [generic 1]),
               Primitives.Action.setStatus .untapped
                 (allOf (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller (target Primitives.Predicate.anyPlayer)]))),
              (some (Primitives.Cost.mana [generic 1]),
               gain (target isCreature) (keyword (.named "doubleStrike")) (some untilEndOfTurn)) ]) ] } }

/-- Consuming Tide -/
def consumingTide : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Consuming Tide", cost := some [generic 2, pip .blue, pip .blue], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ instruct (each Primitives.Predicate.anyPlayer) (choose
                (a (Primitives.Predicate.and [isPermanent, Primitives.Predicate.not isLand, Primitives.Predicate.hasPossessor .controller they]))),
              returnTo (allOf (Primitives.Predicate.and [isPermanent, Primitives.Predicate.not isLand, Primitives.Predicate.notChosen])) hand [],
              Primitives.Action.doForEach
                (each (Primitives.Predicate.compareOver (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you) (countOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (handOf they)])) .greater
                  (countOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (handOf Primitives.NounPhrase.you)]))))
                (Primitives.Action.draw (.lit 1)) ]) ] } }

/-- Stick Together: a chosen party is four up-to-one choices, not the group the game computes
[CR#700.8d]. -/
def stickTogether : Action :=
  Primitives.Action.sequentially
    [ instruct (each Primitives.Predicate.anyPlayer) (choose
        (counted (upTo 1)
          (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasSubtype (creatureType "Cleric"), Primitives.Predicate.hasPossessor .controller they]))),
      instruct (each Primitives.Predicate.anyPlayer) (choose
        (counted (upTo 1)
          (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasSubtype (creatureType "Rogue"), Primitives.Predicate.hasPossessor .controller they]))),
      instruct (each Primitives.Predicate.anyPlayer) (choose
        (counted (upTo 1)
          (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasSubtype (creatureType "Warrior"), Primitives.Predicate.hasPossessor .controller they]))),
      instruct (each Primitives.Predicate.anyPlayer) (choose
        (counted (upTo 1)
          (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasSubtype (creatureType "Wizard"), Primitives.Predicate.hasPossessor .controller they]))),
      instruct (each Primitives.Predicate.anyPlayer) (sacrifice (theRest .object)) ]
theorem okStickTogether : Action.check (DRS.ofList []) stickTogether = [] := by decide

/-- Disciple of Caelus Nin: "starting with you" overrides the default APNAP choice order
[CR#101.1,101.4]. -/
def discipleOfCaelusNin : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Disciple of Caelus Nin", cost := some [generic 4, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ when (does (actor thisCreature) enter)
            (Primitives.Action.sequentially
              [ instruct (each Primitives.Predicate.anyPlayer) (Primitives.Action.choose (some Primitives.NounPhrase.you)
                  (counted (upTo 5) (Primitives.Predicate.and [isPermanent, Primitives.Predicate.hasPossessor .controller they])) .openly none),
                Primitives.Action.setStatus .phasedOut
                  (allOf (Primitives.Predicate.and [isPermanent, Primitives.Predicate.otherThan thisCreature, Primitives.Predicate.notChosen])) ]),
          Primitives.Ability.static (Primitives.StaticSpec.cant Primitives.Subject.any (Primitives.Action.setStatus .phasedIn (allOf isPermanent)) {}) ],
      power := stat 3, toughness := stat 4 } }

/-- Sculpted Sunburst: "if you chose a creature this way" reads the earlier choice;
"not chosen by any player this way" excludes every player's chosen creature. -/
def sculptedSunburst : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sculpted Sunburst", cost := some [generic 3, pip .white, pip .white],
      types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ choose (a (and [isCreature, youControl])),
              instruct (each (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) (choose
                (a (comparesOwnStat .power (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller they])
                  .atMost (Primitives.Amount.statOf (.stat .power) it)))),
              Primitives.Action.doIf (Primitives.Condition.choseThisWay Primitives.NounPhrase.you isCreature) (exile (each (Primitives.Predicate.and [isCreature, Primitives.Predicate.notChosen]))) none
                  ]) ] } }

end Semantics.Cards
