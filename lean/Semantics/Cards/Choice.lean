import Semantics
import Semantics.Macros
import Semantics.Check.Card
import Semantics.Cards.Anaphora
import Semantics.Cards.Trigger

/-!
# Semantics.Cards.Choice

Port of `idris/src/Experimental/Cards/Choice.idr`: the printed cards of the Choice family
(choices, modes, may, permissions to play, chosen qualities) and the bench items beside them.
-/

open Semantics Semantics.Macros

namespace Semantics.Cards

def carefulStudy : Instruction :=
  Primitives.Instruction.sequentially [Primitives.Instruction.draw (.lit 2) (agent := Primitives.NounPhrase.you), discard (counted (exactly 2) (Primitives.Predicate.inZone hand)) (agent :=
      Primitives.NounPhrase.you)]
theorem okCarefulStudy : Instruction.check [] carefulStudy = [] := by decide

def zombieInfestation : Ability :=
  activated (Primitives.Cost.perform (discard (counted (exactly 2) (Primitives.Predicate.inZone hand)) (agent := Primitives.NounPhrase.you)))
    (create (.lit 1) (creatureToken 2 2 [.black] [creatureType "Zombie"]))
theorem okZombieInfestation : Ability.check [] zombieInfestation = [] := by decide

def fulgentDistraction : Instruction :=
  Primitives.Instruction.sequentially
    [ choose (Primitives.NounPhrase.described (Primitives.DetPhrase.target (exactly 2)) creature),
      Primitives.Instruction.setStatus .tapped (those (.type .creature)),
      Primitives.Instruction.unattach (allOf (Primitives.Predicate.and [ Primitives.Predicate.hasSubtype (artifactType "Equipment"),
                               Primitives.Predicate.attachedTo (those (.type .creature)) ])) ]
theorem okFulgentDistraction : Instruction.check [] fulgentDistraction = [] := by decide

def continueSpell : Instruction :=
  Primitives.Instruction.sequentially
    [ choose (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 4)) (Primitives.Predicate.and [creature, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)])),
      move them battlefield ]
theorem okContinueSpell : Instruction.check [] continueSpell = [] := by decide

def kindredDominance : Instruction :=
  Primitives.Instruction.sequentially
    [ choose (a (quality (.subtype .creature))),
      destroy (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.not (ofChosen (.subtype .creature))])) ]
theorem okKindredDominance : Instruction.check [] kindredDominance = [] := by decide

def phantomBlade : Ability :=
  when (Primitives.GameEvent.enters thisEquipment none)
    (Primitives.Instruction.sequentially
      [ Primitives.Instruction.attachTo it
          (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1)) (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])),
        destroy (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1)) (Primitives.Predicate.and [creature, Primitives.Predicate.other])) ])
theorem okPhantomBlade : Ability.check [] phantomBlade = [] := by decide

/-- Braids's Frightful Return -/
def braidsFrightfulReturn : Instruction :=
  Primitives.Instruction.offer (sacrifice (a creature) (agent := Primitives.NounPhrase.you)) (some (discard (a (Primitives.Predicate.inZone hand)) (agent := (each
      Primitives.Predicate.opponent)))) none (agent := Primitives.NounPhrase.you)
theorem okBraidsFrightfulReturn : Instruction.check [] braidsFrightfulReturn = [] := by decide
/-- Daretti, Ingenious Iconoclast -/
def darettisMinusOne : Instruction :=
  Primitives.Instruction.offer (sacrifice (a artifact) (agent := Primitives.NounPhrase.you)) (some (destroy (target (Primitives.Predicate.or [artifact,
      creature])))) none (agent := Primitives.NounPhrase.you)
theorem okDarettisMinusOne : Instruction.check [] darettisMinusOne = [] := by decide

def cheeringFanatic : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cheering Fanatic", cost := some [generic 1, pip .red], types := [.creature],
      subtypes := [creatureType "Goblin"],
      text :=
        [ whenever (attacks thisCreature)
            (Primitives.Instruction.sequentially
              [ choose (a (quality .cardName)),
                Primitives.Instruction.establish
                  (Primitives.StaticSpec.costShift (allOf (Primitives.Predicate.and [spell, ofChosen .cardName])) (Primitives.CostShift.less (.lit 1) none))
                  (some Primitives.Duration.thisTurn) ]) ],
      power := stat 2, toughness := stat 2 } }

def rainOfThorns : Instruction :=
  chooseModes (atLeast 1)
    [destroy (target artifact), destroy (target enchantment), destroy (target land)]
theorem okRainOfThorns : Instruction.check [] rainOfThorns = [] := by decide

def rankleMasterOfPranks : Instruction :=
  chooseModes anyNumber
    [ discard (a (Primitives.Predicate.inZone hand)) (agent := (each Primitives.Predicate.anyPlayer)),
      Primitives.Instruction.sequentially [loseLife (.lit 1) (agent := (each Primitives.Predicate.anyPlayer)), Primitives.Instruction.draw (.lit 1) (agent := (those
          .player))],
      sacrifice (aTheirChoice creature) (agent := (each Primitives.Predicate.anyPlayer)) ]
theorem okRankleMasterOfPranks : Instruction.check [] rankleMasterOfPranks = [] := by decide

def myrkulsEdict : Instruction :=
  Primitives.Instruction.sequentially [choose (a Primitives.Predicate.opponent), sacrifice (aTheirChoice creature) (agent := (that .player))]
theorem okMyrkulsEdict : Instruction.check [] myrkulsEdict = [] := by decide
def moltingHarpy : Instruction := doUnless (sacrifice thisCreature (agent := Primitives.NounPhrase.you)) (Primitives.Cost.mana [generic
    2]) (agent := Primitives.NounPhrase.you)
theorem okMoltingHarpy : Instruction.check [] moltingHarpy = [] := by decide
def carnophage : Instruction := doUnless (Primitives.Instruction.setStatus .tapped thisCreature) (payLife Primitives.NounPhrase.you 1) (agent
    := Primitives.NounPhrase.you)
theorem okCarnophage : Instruction.check [] carnophage = [] := by decide
def solitaryConfinement : Instruction :=
  Primitives.Instruction.offer (discard (a (Primitives.Predicate.inZone hand)) (agent := Primitives.NounPhrase.you)) none (some (sacrifice thisEnchantment (agent
      := Primitives.NounPhrase.you))) (agent := Primitives.NounPhrase.you)
theorem okSolitaryConfinement : Instruction.check [] solitaryConfinement = [] := by decide

def yasminKhan : Ability :=
  activated Primitives.Cost.tapSymbol
    (Primitives.Instruction.sequentially
      [ exile (topSlice (.lit 1)),
        Primitives.Instruction.establish
          (mayPlayDeed (.action "Play") Primitives.NounPhrase.you it none (Primitives.DeonticRider.play none none none false Primitives.PlayPayment.itsOwnCost))
          (some untilYourNextEndStep) ])
theorem okYasminKhan : Ability.check [] yasminKhan = [] := by decide

/-- Brazen Cannonade -/
def brazenCannonadePermission : Instruction :=
  Primitives.Instruction.sequentially
    [ exile (topSlice (.lit 1)),
      Primitives.Instruction.establish
        (mayPlayDeed (.action "Play") Primitives.NounPhrase.you it none (Primitives.DeonticRider.play none none none false Primitives.PlayPayment.itsOwnCost))
        (some (Primitives.Duration.until_ (Primitives.DurationEnd.endOf .combat (some Primitives.NounPhrase.you)))) ]
theorem okBrazenCannonadePermission : Instruction.check [] brazenCannonadePermission = [] := by
  decide

def thousandMoonsCrackshot : Ability :=
  whenever (attacks thisCreature)
    (offerWhen (Primitives.Instruction.pay (Primitives.Cost.mana [generic 2, pip .white]) .once (agent := Primitives.NounPhrase.you))
      (Primitives.Instruction.setStatus .tapped (target creature)) (agent := Primitives.NounPhrase.you))
theorem okThousandMoonsCrackshot : Ability.check [] thousandMoonsCrackshot = [] := by decide

def zimoneQuandrixProdigy : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 1], Primitives.Cost.tapSymbol])
    (offer (putOntoBattlefieldTapped (a (Primitives.Predicate.and [land, Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you)]))) (agent := Primitives.NounPhrase.you))
theorem okZimoneQuandrixProdigy : Ability.check [] zimoneQuandrixProdigy = [] := by decide

def preeminentCaptain : Ability :=
  whenever (attacks thisCreature)
    (offer (putOntoBattlefieldTappedAttacking
      (a (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Soldier"), creature, Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you)]))) (agent :=
          Primitives.NounPhrase.you))
theorem okPreeminentCaptain : Ability.check [] preeminentCaptain = [] := by decide

def skaabRuinator : Ability :=
  Primitives.Ability.static (mayPlayDeed (.action "Cast") Primitives.NounPhrase.you Primitives.NounPhrase.this none
    (Primitives.DeonticRider.play (some (graveyardOf Primitives.NounPhrase.you)) none none false Primitives.PlayPayment.itsOwnCost))
theorem okSkaabRuinator : Ability.check [] skaabRuinator = [] := by decide

/-- Raffine's Guidance -/
def raffinesGuidance : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Raffine's Guidance", cost := some [pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (getsPt (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))),
          Primitives.Ability.static (mayPlayDeed (.action "Cast") Primitives.NounPhrase.you Primitives.NounPhrase.this none
            (Primitives.DeonticRider.play (some (graveyardOf Primitives.NounPhrase.you)) none none false
              (Primitives.PlayPayment.payingInstead (Primitives.Cost.mana [generic 2, pip .white])))) ] } }

/-- Scourge of Nel Toth -/
def scourgeOfNelToth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Scourge of Nel Toth", cost := some [generic 5, pip .black, pip .black],
      types := [.creature], subtypes := [creatureType "Zombie", creatureType "Dragon"],
      text :=
        [ keyword "Flying",
          Primitives.Ability.static (mayPlayDeed (.action "Cast") Primitives.NounPhrase.you Primitives.NounPhrase.this none
            (Primitives.DeonticRider.play (some (graveyardOf Primitives.NounPhrase.you)) none none false
              (Primitives.PlayPayment.payingInstead (Primitives.Cost.compound
                [ Primitives.Cost.mana [pip .black, pip .black],
                  Primitives.Cost.perform (sacrifice (counted (exactly 2) creature) (agent := Primitives.NounPhrase.you)) ])))) ],
      power := stat 6, toughness := stat 6 } }

def escapeToTheWilds : Instruction :=
  Primitives.Instruction.sequentially
    [ exile (topSlice (.lit 5)),
      Primitives.Instruction.establish
        (mayPlayDeed (.action "Play") Primitives.NounPhrase.you (theVerbed (.action "Exile") .card .thisWay .many) none
          (Primitives.DeonticRider.play none none none false Primitives.PlayPayment.itsOwnCost))
        (some (Primitives.Duration.until_ (Primitives.DurationEnd.endOf .turn (some Primitives.NounPhrase.you)))) ]
theorem okEscapeToTheWilds : Instruction.check [] escapeToTheWilds = [] := by decide

/-- Muse Vessel -/
def museVesselPlay : Ability :=
  activated (Primitives.Cost.mana [generic 1])
    (Primitives.Instruction.sequentially
      [ choose (a exiledWithThisArtifact),
        Primitives.Instruction.establish
          (mayPlayDeed (.action "Play") Primitives.NounPhrase.you (that .card) none
            (Primitives.DeonticRider.play none none none false Primitives.PlayPayment.itsOwnCost))
          (some Primitives.Duration.thisTurn) ])
theorem okMuseVesselPlay : Ability.check [] museVesselPlay = [] := by decide

def demonicConsultationChoice : Instruction := choose (a (quality .cardName))
theorem okDemonicConsultationChoice : Instruction.check [] demonicConsultationChoice = [] := by
  decide
def voidChoice : Instruction := choose (a (quality .number))
theorem okVoidChoice : Instruction.check [] voidChoice = [] := by decide
def ashnodsBattleGear : Ability := Primitives.Ability.static (mayDeclineUntap thisArtifact (some Primitives.NounPhrase.you))
theorem okAshnodsBattleGear : Ability.check [] ashnodsBattleGear = [] := by decide

/-- Phyrexian Ingester -/
def phyrexianIngesterPump : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.conjunction none
    [ Primitives.StaticSpec.modification thisCreature .power (Primitives.Delta.up (Primitives.Amount.letter .x)),
      Primitives.StaticSpec.modification thisCreature .toughness (Primitives.Delta.up (Primitives.Amount.letter .y)),
      Primitives.StaticSpec.letterDefinition .x
        (Primitives.Amount.statOf (.stat .power) (the (Primitives.Predicate.and [creature, Primitives.Predicate.exiledWith thisCreature]))),
      Primitives.StaticSpec.letterDefinition .y (Primitives.Amount.statOf (.stat .toughness) (that .card)) ])
theorem okPhyrexianIngesterPump : Ability.check [] phyrexianIngesterPump = [] := by decide

/-- Phyrexian Ingester -/
def phyrexianIngester : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Phyrexian Ingester", cost := some [generic 6, pip .blue], types := [.creature],
      subtypes := [creatureType "Phyrexian", creatureType "Beast"],
      text :=
        [ abilityWord "imprint"
            (when (Primitives.GameEvent.enters thisCreature none)
              (offer (exile (target (Primitives.Predicate.and [creature, nontoken]))) (agent := Primitives.NounPhrase.you))),
          phyrexianIngesterPump ],
      power := stat 3, toughness := stat 3 } }

def misterFantastic : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mister Fantastic, Reed Richards", cost := some [generic 3, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Scientist", creatureType "Hero"],
      text :=
        [ keyword "Reach",
          whenever
            (Primitives.GameEvent.enters (counted (atLeast 1) (Primitives.Predicate.and [Primitives.Predicate.isToken, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) none)
            (offer (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)) (agent := Primitives.NounPhrase.you)) ],
      power := stat 2, toughness := stat 4 } }

def murmursFromBeyond : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Murmurs from Beyond", cost := some [generic 2, pip .blue], types := [.instant],
      subtypes := [spellType "Arcane"],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ revealCards (topSlice (.lit 3)),
              choose (someOf (exactly 1) them) (agent := some (a Primitives.Predicate.opponent)),
              move (that .card) graveyard,
              move (theRest .object) hand ]) ] } }

/-- Helica Glider -/
def helicaGlider : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Helica Glider", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [creatureType "Nightmare", creatureType "Squirrel"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.entryRider thisCreature
            (Primitives.TokenRider.withCounters (.lit 1) (Primitives.CounterKindSource.chosen [flyingCounter, .keyword "FirstStrike"]) .fresh)) ],
      power := stat 2, toughness := stat 2 } }

/-- Eager Construct -/
def eagerConstruct : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Eager Construct", cost := some [generic 2], types := [.artifact, .creature],
      subtypes := [creatureType "Construct"],
      text := [when (Primitives.GameEvent.enters thisCreature none) (offer (scry (.lit 1) (agent := they)) (agent :=
          (each Primitives.Predicate.anyPlayer)))],
      power := stat 2, toughness := stat 2 } }

/-- Rune Snag -/
def runeSnag : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rune Snag", cost := some [generic 1, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (doUnless (Primitives.Instruction.counterSpell it)
            (Primitives.Cost.compound
              [ Primitives.Cost.mana [generic 2],
                scaledMana .generic
                  (times (.lit 2)
                    (countOf (Primitives.Predicate.and [Primitives.Predicate.named (Primitives.NameSource.printed "Rune Snag"), Primitives.Predicate.inZone graveyard]))) ]) (agent
                        := (controllerOf (target spell)))) ] } }

/-- Tahngarth, First Mate -/
def tahngarthChoosesDefender : Instruction :=
  Primitives.Instruction.choose none
    (a (Primitives.Predicate.and [ Primitives.Predicate.or [Primitives.Predicate.hasType .planeswalker, Primitives.Predicate.anyPlayer],
               Primitives.Predicate.inCombat .attackedBy (some (that .player)) ]))
    .openly none (agent := none)
theorem okTahngarthChoosesDefender :
    Instruction.check (GameEvent.intro [] tahngarthHeader) tahngarthChoosesDefender = [] := by
  decide

def reefShaman : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Reef Shaman", cost := some [pip .blue], types := [.creature],
      subtypes := [creatureType "Merfolk", creatureType "Shaman"],
      text :=
        [ activated Primitives.Cost.tapSymbol
            (Primitives.Instruction.establish
              (Primitives.StaticSpec.qualityChange (target land) .sets
                (Primitives.QualityPayload.chosenQuality (Primitives.Predicate.ofYourChoice (.subtype .land) (some Primitives.ChoiceDomain.basicTypesOnly))))
              (some untilEndOfTurn)) ],
      power := stat 0, toughness := stat 2 } }

def grixisIllusionist : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grixis Illusionist", cost := some [pip .blue], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ activated Primitives.Cost.tapSymbol
            (Primitives.Instruction.establish
              (Primitives.StaticSpec.qualityChange (target (Primitives.Predicate.and [land, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .sets
                (Primitives.QualityPayload.chosenQuality (Primitives.Predicate.ofYourChoice (.subtype .land) (some Primitives.ChoiceDomain.basicTypesOnly))))
              (some untilEndOfTurn)) ],
      power := stat 1, toughness := stat 1 } }

def distantMelody : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Distant Melody", cost := some [generic 3, pip .blue], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ choose (a (quality (.subtype .creature))),
              Primitives.Instruction.draw
                (forEach 1
                  (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, ofChosen (.subtype .creature)]))
                      (agent := Primitives.NounPhrase.you) ]) ] } }

def cripplingFear : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crippling Fear", cost := some [generic 2, pip .black, pip .black],
      types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ choose (a (quality (.subtype .creature))),
              get (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.not (ofChosen (.subtype .creature))]))
                (Primitives.Delta.down (.lit 3)) (Primitives.Delta.down (.lit 3)) (some untilEndOfTurn) ]) ] } }

/-- Koh, the Face Stealer -/
def kohChooser : Ability :=
  activated (payLife Primitives.NounPhrase.you 1) (choose (a (Primitives.Predicate.and [creature, Primitives.Predicate.exiledWith Primitives.NounPhrase.this])))
theorem okKohChooser : Ability.check [] kohChooser = [] := by decide
/-- Forgotten Lore -/
def forgottenLoreChoice : Instruction :=
  choose (a (Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you))) (agent := some (target Primitives.Predicate.opponent))
theorem okForgottenLoreChoice : Instruction.check [] forgottenLoreChoice = [] := by decide

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
theorem okPsychicPaperChoiceAndReads : Ability.checkText [] psychicPaperChoiceAndReads = [] := by
  decide

def xenograft : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Xenograft", cost := some [generic 4, pip .blue], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (entersChoosing thisEnchantment (.subtype .creature)),
          Primitives.Ability.static (Primitives.StaticSpec.qualityChange (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .adds
            (Primitives.QualityPayload.chosenQuality (ofChosen (.subtype .creature)))) ] } }

/-- Convincing Mirage -/
def convincingMirage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Convincing Mirage", cost := some [generic 1, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" land,
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
          Primitives.Ability.static (Primitives.StaticSpec.qualityChange (allOf (Primitives.Predicate.and [land, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .adds
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
            (allOf (Primitives.Predicate.and [ creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.otherThan thisCreature,
                           ofChosen (.subtype .creature) ]))
            (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))) ],
      power := stat 2, toughness := stat 2 } }

def mistformDreamer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mistform Dreamer", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [creatureType "Illusion"],
      text :=
        [ keyword "Flying",
          activated (Primitives.Cost.mana [generic 1])
            (Primitives.Instruction.establish
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
              allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]),
              allOf (Primitives.Predicate.and [creature, spell, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]),
              allOf (Primitives.Predicate.and [Primitives.Predicate.isCard, creature, Primitives.Predicate.hasPossessor .owner Primitives.NounPhrase.you, Primitives.Predicate.not (Primitives.Predicate.inZone battlefield)]) ]) .adds
              (Primitives.QualityPayload.chosenQuality (ofChosen (.subtype .creature)))) ] } }

def conspiracy : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Conspiracy", cost := some [generic 3, pip .black, pip .black],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.static (entersChoosing thisEnchantment (.subtype .creature)),
          Primitives.Ability.static (Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.and [
              allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]),
              allOf (Primitives.Predicate.and [creature, spell, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]),
              allOf (Primitives.Predicate.and [Primitives.Predicate.isCard, creature, Primitives.Predicate.hasPossessor .owner Primitives.NounPhrase.you, Primitives.Predicate.not (Primitives.Predicate.inZone battlefield)]) ]) .sets
              (Primitives.QualityPayload.chosenQuality (ofChosen (.subtype .creature)))) ] } }

def declarationOfNaught : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Declaration of Naught", cost := some [pip .blue, pip .blue], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (entersChoosing thisEnchantment .cardName),
          activated (Primitives.Cost.mana [pip .blue]) (Primitives.Instruction.counterSpell (target (Primitives.Predicate.and [spell, Primitives.Predicate.named Primitives.NameSource.chosen]))) ] } }

def imagecrafter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Imagecrafter", cost := some [pip .blue], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ activated Primitives.Cost.tapSymbol
            (Primitives.Instruction.sequentially
              [ choose (a (qualityFrom (.subtype .creature) (Primitives.ChoiceDomain.typeOtherThan (creatureType "Wall")))),
                Primitives.Instruction.establish
                  (Primitives.StaticSpec.qualityChange (target creature) .sets (Primitives.QualityPayload.chosenQuality (ofChosen (.subtype
                      .creature))))
                  (some untilEndOfTurn) ]) ],
      power := stat 1, toughness := stat 1 } }

def unnaturalSelection : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unnatural Selection", cost := some [generic 1, pip .blue], types := [.enchantment],
      text :=
        [ activated (Primitives.Cost.mana [generic 1])
            (Primitives.Instruction.sequentially
              [ choose (a (qualityFrom (.subtype .creature) (Primitives.ChoiceDomain.typeOtherThan (creatureType "Wall")))),
                Primitives.Instruction.establish
                  (Primitives.StaticSpec.qualityChange (target creature) .sets (Primitives.QualityPayload.chosenQuality (ofChosen (.subtype
                      .creature))))
                  (some untilEndOfTurn) ]) ] } }

def standardize : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Standardize", cost := some [pip .blue, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ choose (a (qualityFrom (.subtype .creature) (Primitives.ChoiceDomain.typeOtherThan (creatureType "Wall")))),
              Primitives.Instruction.establish
                (Primitives.StaticSpec.qualityChange (each creature) .sets (Primitives.QualityPayload.chosenQuality (ofChosen (.subtype
                    .creature))))
                (some untilEndOfTurn) ]) ] } }

def silverquillSilencer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Silverquill Silencer", cost := some [pip .white, pip .black], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ Primitives.Ability.static (entersChoosingFrom thisCreature .cardName (Primitives.ChoiceDomain.nameOfCard (Primitives.Predicate.not land))),
          whenever (Primitives.GameEvent.casts anOpponent (some (a (Primitives.Predicate.and [spell,
            Primitives.Predicate.named Primitives.NameSource.chosen]))) none)
            (Primitives.Instruction.sequentially [loseLife (.lit 3) (agent := they), Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)]) ],
      power := stat 3, toughness := stat 2 } }

def meddlingMage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Meddling Mage", cost := some [pip .white, pip .blue], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ Primitives.Ability.static (entersChoosingFrom thisCreature .cardName (Primitives.ChoiceDomain.nameOfCard (Primitives.Predicate.not land))),
          Primitives.Ability.static (objectCant (.action "Cast") (allOf (Primitives.Predicate.and [spell, Primitives.Predicate.named Primitives.NameSource.chosen]))) ],
      power := stat 2, toughness := stat 2 } }

/-- Nyxathid -/
def nyxathid : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nyxathid", cost := some [generic 1, pip .black, pip .black], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ Primitives.Ability.static (entersChoosingPlayer thisCreature (some (Primitives.ChoiceDomain.players Primitives.Predicate.opponent))),
          Primitives.Ability.static (getsPt thisCreature
            (Primitives.Delta.down (countOf (Primitives.Predicate.inZone (handOf (the chosenPlayer)))))
            (Primitives.Delta.down (countOf (Primitives.Predicate.inZone (handOf (the chosenPlayer)))))) ],
      power := stat 7, toughness := stat 7 } }

/-- Expel the Interlopers -/
def expelTheInterlopers : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Expel the Interlopers", cost := some [generic 3, pip .white, pip .white],
      types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ choose (a (qualityFrom .number (Primitives.ChoiceDomain.number (fromTo 0 10)))),
              destroy (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.compare [.stat .power] .atLeast chosenNumber])) ]) ] } }

def nevermore : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nevermore", cost := some [generic 1, pip .white, pip .white], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (entersChoosingFrom thisEnchantment .cardName (Primitives.ChoiceDomain.nameOfCard (Primitives.Predicate.not land))),
          Primitives.Ability.static (objectCant (.action "Cast") (allOf (Primitives.Predicate.and [spell, Primitives.Predicate.named Primitives.NameSource.chosen]))) ] } }

def necromentiaChoice : Instruction :=
  choose (a (qualityFrom .cardName (Primitives.ChoiceDomain.nameOfCard (Primitives.Predicate.not (Primitives.Predicate.and [Primitives.Predicate.hasSupertype .basic, land])))))
theorem okNecromentiaChoice : Instruction.check [] necromentiaChoice = [] := by decide

def conjurersBan : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Conjurer's Ban", cost := some [pip .white, pip .black], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ choose (a (quality .cardName)),
              Primitives.Instruction.establish
                (Primitives.StaticSpec.conjunction none
                  [ objectCant (.action "Cast") (allOf (Primitives.Predicate.and [spell, Primitives.Predicate.named Primitives.NameSource.chosen])),
                    objectCant (.action "Play") (allOf (Primitives.Predicate.and [land, Primitives.Predicate.named Primitives.NameSource.chosen])) ])
                (some untilYourNextTurn) ]),
          Primitives.Ability.spell none (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)) ] } }

def foundingOfOmashu : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Founding of Omashu", cost := some [generic 2, pip .red], types := [.enchantment],
      subtypes := [enchantmentType "Saga"],
      text :=
        [ when (Primitives.GameEvent.chapterMark [1]) (create (.lit 2) (creatureToken 1 1 [.white] [creatureType "Ally"])),
          when (Primitives.GameEvent.chapterMark [2])
            (Primitives.Instruction.offer (discard (a (Primitives.Predicate.inZone hand)) (agent := Primitives.NounPhrase.you)) (some (Primitives.Instruction.draw (.lit 1) (agent :=
                Primitives.NounPhrase.you))) none (agent := Primitives.NounPhrase.you)),
          when (Primitives.GameEvent.chapterMark [3])
            (get (allOf creatureYouControl) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn)) ] }
                }

/-- "as though it had flash" -/
def asThoughFlash : Option AsThough := some (Primitives.AsThough.of (Primitives.Predicate.hasKeyword (.the "Flash")))

def vedalkenOrrery : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vedalken Orrery", cost := some [generic 4], types := [.artifact],
      text :=
        [ Primitives.Ability.static (mayPlayDeed (.action "Cast") Primitives.NounPhrase.you (allOf spell) asThoughFlash
            (Primitives.DeonticRider.play none none none false Primitives.PlayPayment.itsOwnCost)) ] } }

def shimmerMyr : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shimmer Myr", cost := some [generic 3], types := [.artifact, .creature],
      subtypes := [creatureType "Myr"],
      text :=
        [ keyword "Flash",
          Primitives.Ability.static (mayPlayDeed (.action "Cast") Primitives.NounPhrase.you (allOf (Primitives.Predicate.and [spell, artifact])) asThoughFlash
            (Primitives.DeonticRider.play none none none false Primitives.PlayPayment.itsOwnCost)) ],
      power := stat 2, toughness := stat 2 } }

def quickSliver : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Quick Sliver", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Sliver"],
      text :=
        [ keyword "Flash",
          Primitives.Ability.static (mayPlayDeed (.action "Cast") (a Primitives.Predicate.anyPlayer)
            (allOf (Primitives.Predicate.and [spell, Primitives.Predicate.hasSubtype (creatureType "Sliver")])) asThoughFlash
            (Primitives.DeonticRider.play none none none false Primitives.PlayPayment.itsOwnCost)) ],
      power := stat 1, toughness := stat 1 } }

def vernalEquinox : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vernal Equinox", cost := some [generic 3, pip .green], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (mayPlayDeed (.action "Cast") (a Primitives.Predicate.anyPlayer)
            (allOf (Primitives.Predicate.and [spell, Primitives.Predicate.or [creature, enchantment]])) asThoughFlash
            (Primitives.DeonticRider.play none none none false Primitives.PlayPayment.itsOwnCost)) ] } }

def borneUponAWind : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Borne Upon a Wind", cost := some [generic 1, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.establish
            (mayPlayDeed (.action "Cast") Primitives.NounPhrase.you (allOf spell) asThoughFlash
              (Primitives.DeonticRider.play none none none false Primitives.PlayPayment.itsOwnCost))
            (some Primitives.Duration.thisTurn)),
          Primitives.Ability.spell none (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)) ] } }

def gisaAndGeralf : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gisa and Geralf", cost := some [generic 2, pip .blue, pip .black],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ when (Primitives.GameEvent.enters thisCreature none) (mill (.lit 4) Primitives.NounPhrase.you (agent := Primitives.NounPhrase.you)),
          Primitives.Ability.static (mayPlayDeed (.action "Cast") Primitives.NounPhrase.you
            (a (Primitives.Predicate.and [spell, creature, Primitives.Predicate.hasSubtype (creatureType "Zombie")])) none
            (Primitives.DeonticRider.play (some (graveyardOf Primitives.NounPhrase.you)) (some .onceEachYourTurn) none false Primitives.PlayPayment.itsOwnCost)) ],
      power := stat 4, toughness := stat 4 } }

/-- "play lands from the top of your library" -/
def playLandsFromTop : StaticSpec :=
  mayPlayDeed (.action "Play") Primitives.NounPhrase.you (allOf land) none (Primitives.DeonticRider.play (some onTop) none none false Primitives.PlayPayment.itsOwnCost)
/-- "cast <p> spells from the top of your library" -/
def castFromTop (p : Predicate) : StaticSpec :=
  mayPlayDeed (.action "Cast") Primitives.NounPhrase.you (allOf p) none (Primitives.DeonticRider.play (some onTop) none none false Primitives.PlayPayment.itsOwnCost)

/-- Future Sight; Magus of the Future -/
def playLandsAndCastSpellsFromTop : StaticSpec :=
  Primitives.StaticSpec.conjunction none [playLandsFromTop, castFromTop spell]
theorem okPlayLandsAndCastSpellsFromTop :
    StaticSpec.check [] playLandsAndCastSpellsFromTop = [] := by decide
/-- Assemble the Players -/
def castSmallCreatureFromTopOnceEachTurn : StaticSpec :=
  mayPlayDeed (.action "Cast") Primitives.NounPhrase.you
    (a (Primitives.Predicate.and [spell, creature, Primitives.Predicate.compare [.stat .power] .atMost (.lit 2)])) none
    (Primitives.DeonticRider.play (some onTop) (some .onceEachTurn) none false Primitives.PlayPayment.itsOwnCost)
theorem okCastSmallCreatureFromTopOnceEachTurn :
    StaticSpec.check [] castSmallCreatureFromTopOnceEachTurn = [] := by decide

def courserOfKruphix : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Courser of Kruphix", cost := some [generic 1, pip .green, pip .green],
      types := [.enchantment, .creature], subtypes := [creatureType "Centaur"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.visibility .reveal Primitives.NounPhrase.you Primitives.VisibleThing.topOfLibrary),
          Primitives.Ability.static playLandsFromTop,
          whenever (Primitives.GameEvent.enters (a (Primitives.Predicate.and [land, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) none)
            (gainLife (.lit 1) (agent := Primitives.NounPhrase.you)) ],
      power := stat 2, toughness := stat 4 } }

def korlessaScaleSinger : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Korlessa, Scale Singer", cost := some [pip .green, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Dragon", creatureType "Bard"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.visibility .lookAt Primitives.NounPhrase.you Primitives.VisibleThing.topOfLibrary),
          Primitives.Ability.static (castFromTop (Primitives.Predicate.and [spell, Primitives.Predicate.hasSubtype (creatureType "Dragon")])) ],
      power := stat 1, toughness := stat 4 } }

def precognitionField : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Precognition Field", cost := some [generic 3, pip .blue], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.visibility .lookAt Primitives.NounPhrase.you Primitives.VisibleThing.topOfLibrary),
          Primitives.Ability.static (castFromTop (Primitives.Predicate.and [spell, instantOrSorcery])),
          activated (Primitives.Cost.mana [generic 3]) (exile (Primitives.NounPhrase.librarySlice .top (.lit 1) Primitives.NounPhrase.you)) ] } }

def mysticForge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mystic Forge", cost := some [generic 4], types := [.artifact],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.visibility .lookAt Primitives.NounPhrase.you Primitives.VisibleThing.topOfLibrary),
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none [castFromTop (Primitives.Predicate.and [spell, artifact]), castFromTop (Primitives.Predicate.and
              [spell, colorless])]),
          activated (Primitives.Cost.compound [Primitives.Cost.tapSymbol, payLife Primitives.NounPhrase.you 1]) (exile (Primitives.NounPhrase.librarySlice .top (.lit 1) Primitives.NounPhrase.you)) ] } }

def exploration : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Exploration", cost := some [pip .green], types := [.enchantment],
      text := [Primitives.Ability.static (mayPlayAdditionalLands Primitives.NounPhrase.you (exactly 1))] } }

def oracleOfMulDaya : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Oracle of Mul Daya", cost := some [generic 3, pip .green], types := [.creature],
      subtypes := [creatureType "Elf", creatureType "Shaman"],
      text :=
        [ Primitives.Ability.static (mayPlayAdditionalLands Primitives.NounPhrase.you (exactly 1)),
          Primitives.Ability.static (Primitives.StaticSpec.visibility .reveal Primitives.NounPhrase.you Primitives.VisibleThing.topOfLibrary),
          Primitives.Ability.static playLandsFromTop ],
      power := stat 2, toughness := stat 2 } }

def azusaLostButSeeking : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Azusa, Lost but Seeking", cost := some [generic 2, pip .green],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Monk"],
      text := [Primitives.Ability.static (mayPlayAdditionalLands Primitives.NounPhrase.you (exactly 2))],
      power := stat 1, toughness := stat 2 } }

def summerBloom : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Summer Bloom", cost := some [generic 1, pip .green], types := [.sorcery],
      text := [Primitives.Ability.spell none (Primitives.Instruction.establish (mayPlayAdditionalLands Primitives.NounPhrase.you (upTo 3)) (some Primitives.Duration.thisTurn))] } }

def explore : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Explore", cost := some [generic 1, pip .green], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ Primitives.Instruction.establish (mayPlayAdditionalLands Primitives.NounPhrase.you (exactly 1)) (some Primitives.Duration.thisTurn),
              Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you) ]) ] } }

def urbanEvolution : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Urban Evolution", cost := some [generic 3, pip .green, pip .blue],
      types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ Primitives.Instruction.draw (.lit 3) (agent := Primitives.NounPhrase.you),
              Primitives.Instruction.establish (mayPlayAdditionalLands Primitives.NounPhrase.you (exactly 1)) (some Primitives.Duration.thisTurn) ]) ] } }

def dryadOfTheIlysianGrove : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dryad of the Ilysian Grove", cost := some [generic 2, pip .green],
      types := [.enchantment, .creature], subtypes := [creatureType "Nymph", creatureType "Dryad"],
      text :=
        [ Primitives.Ability.static (mayPlayAdditionalLands Primitives.NounPhrase.you (exactly 1)),
          Primitives.Ability.static (Primitives.StaticSpec.qualityChange (allOf (Primitives.Predicate.and [land, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .adds
            (Primitives.QualityPayload.everyTypeOf .basicLand)) ],
      power := stat 2, toughness := stat 4 } }

/-- Ashes of the Fallen -/
def ashesOfTheFallen : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ashes of the Fallen", cost := some [generic 2], types := [.artifact],
      text :=
        [ Primitives.Ability.static (entersChoosing thisArtifact (.subtype .creature)),
          Primitives.Ability.static (Primitives.StaticSpec.qualityChange (each (Primitives.Predicate.and [creature, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)])) .adds
            (Primitives.QualityPayload.chosenQuality (ofChosen (.subtype .creature)))) ] } }

def hellkiteCharger : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hellkite Charger", cost := some [generic 4, pip .red, pip .red],
      types := [.creature], subtypes := [creatureType "Dragon"],
      text :=
        [ keyword "Flying", keyword "Haste",
          whenever (attacks thisCreature)
            (Primitives.Instruction.offer (Primitives.Instruction.pay (Primitives.Cost.mana [generic 5, pip .red, pip .red]) .once (agent := Primitives.NounPhrase.you))
              (some (Primitives.Instruction.sequentially
                [ Primitives.Instruction.setStatus .untapped (allOf (Primitives.Predicate.and [creature, attacking])),
                  addPart .combat none (.lit 1) ]))
              none (agent := Primitives.NounPhrase.you)) ],
      power := stat 5, toughness := stat 5 } }

/-- Foriysian Brigade -/
def foriysianBrigade : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Foriysian Brigade", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text := [Primitives.Ability.static (mayBlockAdditional thisCreature (exactly 1))],
      power := stat 2, toughness := stat 4 } }

/-- Two-Headed Giant of Foriys -/
def twoHeadedGiantOfForiys : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Two-Headed Giant of Foriys", cost := some [generic 4, pip .red],
      types := [.creature], subtypes := [creatureType "Giant"],
      text := [keyword "Trample", Primitives.Ability.static (mayBlockAdditional thisCreature (exactly 1))],
      power := stat 4, toughness := stat 4 } }

def highGround : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "High Ground", cost := some [pip .white], types := [.enchantment],
      text := [Primitives.Ability.static (mayBlockAdditional (each creatureYouControl) (exactly 1))] } }

/-- Watcher in the Web -/
def watcherInTheWeb : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Watcher in the Web", cost := some [generic 4, pip .green], types := [.creature],
      subtypes := [creatureType "Spider"],
      text := [keyword "Reach", Primitives.Ability.static (mayBlockAdditional thisCreature (exactly 7))],
      power := stat 2, toughness := stat 5 } }

/-- Forgotten Lore -/
def forgottenLoreRepeat : Instruction :=
  Primitives.Instruction.sequentially
    [ Primitives.Instruction.choose none (a (Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you))) .openly none (agent := (some (target
        Primitives.Predicate.opponent))),
      Primitives.Instruction.offer (Primitives.Instruction.pay (Primitives.Cost.mana [pip .green]) .once (agent := Primitives.NounPhrase.you)) (some (Primitives.Instruction.repeat_
          Primitives.Repetition.againExcludingChosen)) none (agent := Primitives.NounPhrase.you) ]
theorem okForgottenLoreRepeat : Instruction.check [] forgottenLoreRepeat = [] := by decide

/-- Leyline of the Meek -/
def leylineOfTheMeek : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Leyline of the Meek", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.mayBeginOnBattlefield,
          Primitives.Ability.static (getsPt (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.isToken])) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))) ] } }

/-- Leyline of Vitality -/
def leylineOfVitality : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Leyline of Vitality", cost := some [generic 2, pip .green, pip .green],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.mayBeginOnBattlefield,
          Primitives.Ability.static (getsPt (allOf creatureYouControl) (Primitives.Delta.up (.lit 0)) (Primitives.Delta.up (.lit 1))),
          whenever (Primitives.GameEvent.enters (a creatureYouControl) none) (offer (gainLife (.lit 1) (agent := Primitives.NounPhrase.you))
              (agent := Primitives.NounPhrase.you)) ] } }

def phyrexianRevoker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Phyrexian Revoker", cost := some [generic 2], types := [.artifact, .creature],
      subtypes := [creatureType "Phyrexian", creatureType "Horror"],
      text :=
        [ Primitives.Ability.static (entersChoosingFrom thisCreature .cardName (Primitives.ChoiceDomain.nameOfCard (Primitives.Predicate.not land))),
          Primitives.Ability.static (objectCant (.action "Activate")
            (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .anyActivated,
                           Primitives.Predicate.abilityOf (allOf (Primitives.Predicate.and [source, Primitives.Predicate.named Primitives.NameSource.chosen])) ]))) ],
      power := stat 2, toughness := stat 1 } }

def voidstoneGargoyle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Voidstone Gargoyle", cost := some [generic 3, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Gargoyle"],
      text :=
        [ keyword "Flying",
          Primitives.Ability.static (entersChoosingFrom thisCreature .cardName (Primitives.ChoiceDomain.nameOfCard (Primitives.Predicate.not land))),
          Primitives.Ability.static (objectCant (.action "Cast") (allOf (Primitives.Predicate.and [spell, Primitives.Predicate.named Primitives.NameSource.chosen]))),
          Primitives.Ability.static (objectCant (.action "Activate")
            (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .anyActivated,
                           Primitives.Predicate.abilityOf (allOf (Primitives.Predicate.and [source, Primitives.Predicate.named Primitives.NameSource.chosen])) ]))) ],
      power := stat 3, toughness := stat 3 } }

def pithingNeedle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pithing Needle", cost := some [generic 1], types := [.artifact],
      text :=
        [ Primitives.Ability.static (entersChoosing thisArtifact .cardName),
          Primitives.Ability.static (objectCant (.action "Activate")
            (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .anyActivated,
                           Primitives.Predicate.abilityOf (allOf (Primitives.Predicate.and [source, Primitives.Predicate.named Primitives.NameSource.chosen])),
                           Primitives.Predicate.not Primitives.Predicate.isManaAbility ]))) ] } }

/-- Rhystic Study -/
def rhysticStudy : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rhystic Study", cost := some [generic 2, pip .blue], types := [.enchantment],
      text :=
        [ whenever (Primitives.GameEvent.casts anOpponent (some (a spell)) none)
            (doUnless (offer (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)) (agent := Primitives.NounPhrase.you)) (Primitives.Cost.mana [generic 1])
                (agent := (that .player))) ] } }

def gandalfWhiteRider : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gandalf, White Rider", cost := some [generic 3, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Avatar", creatureType "Wizard"],
      text :=
        [ keyword "Vigilance",
          whenever (Primitives.GameEvent.casts Primitives.NounPhrase.you (some (a spell)) none)
            (Primitives.Instruction.sequentially
              [ get (each creatureYouControl) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn),
                scry (.lit 1) (agent := Primitives.NounPhrase.you) ]),
          when (Primitives.GameEvent.dies thisCreature) (offer (move it (nthFromTop (.nth 5))) (agent := Primitives.NounPhrase.you)) ],
      power := stat 3, toughness := stat 3 } }

def helmOfPossession : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Helm of Possession", cost := some [generic 4], types := [.artifact],
      text :=
        [ Primitives.Ability.static (mayDeclineUntap thisArtifact (some Primitives.NounPhrase.you)),
          activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2], Primitives.Cost.tapSymbol, Primitives.Cost.perform (sacrifice (a creature)
              (agent := Primitives.NounPhrase.you))])
            (gainControl (target creature)
              (some (Primitives.Duration.forAsLongAs
                (Primitives.Condition.and [ Primitives.Condition.matches thisArtifact (Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you),
                        Primitives.Condition.matches thisArtifact tapped ]))) (agent := Primitives.NounPhrase.you)) ] } }

def override : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Override", cost := some [generic 2, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (doUnless (Primitives.Instruction.counterSpell it)
            (scaledMana .generic (forEach 1 (Primitives.Predicate.and [artifact, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
                (agent := (controllerOf (target spell)))) ] } }

def rakshasasDisdain : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rakshasa's Disdain", cost := some [generic 2, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (doUnless (Primitives.Instruction.counterSpell it)
            (scaledMana .generic (forEach 1 (Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)))) (agent := (controllerOf
                (target spell)))) ] } }

def megatherium : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Megatherium", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Beast"],
      text :=
        [ keyword "Trample",
          when (Primitives.GameEvent.enters thisCreature none)
            (doUnless (sacrifice thisCreature (agent := Primitives.NounPhrase.you))
              (scaledMana .generic (forEach 1 (Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you)))) (agent := Primitives.NounPhrase.you)) ],
      power := stat 4, toughness := stat 4 } }

def killingWave : Instruction :=
  Primitives.Instruction.doForEach (each creature)
    (doUnless (sacrifice it (agent := they)) (Primitives.Cost.perform (loseLife (Primitives.Amount.letter .x) (agent := they)))
        (agent := (controllerOf it)))
theorem okKillingWave : Instruction.check [] killingWave = [] := by decide
def fadeAway : Instruction :=
  Primitives.Instruction.doForEach (each creature)
    (doUnless (sacrifice (a permanent) (agent := they)) (Primitives.Cost.mana [generic 1]) (agent := (controllerOf
        it)))
theorem okFadeAway : Instruction.check [] fadeAway = [] := by decide

def tidalFlats : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tidal Flats", cost := some [pip .blue], types := [.enchantment],
      text :=
        [ activated (Primitives.Cost.mana [pip .blue, pip .blue])
            (Primitives.Instruction.doForEach (each (Primitives.Predicate.and [creature, attacking, Primitives.Predicate.not (Primitives.Predicate.hasKeyword (.the "Flying"))]))
              (Primitives.Instruction.offer (Primitives.Instruction.pay (Primitives.Cost.mana [generic 1]) .once (agent := they)) none
                (some (gain
                  (allOf (Primitives.Predicate.and [ creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                                 Primitives.Predicate.inCombat .blockerOf (some (that (.type .creature))) ]))
                  (keyword "FirstStrike") (some untilEndOfTurn))) (agent := (controllerOf it)))) ] }
                      }

def primalSurge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Primal Surge", cost := some [generic 8, pip .green, pip .green], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ exile (topSlice (.lit 1)),
              Primitives.Instruction.doIf (itsA permanentCard)
                (Primitives.Instruction.offer (putOntoBattlefield it) (some (Primitives.Instruction.repeat_ Primitives.Repetition.again)) none (agent := Primitives.NounPhrase.you)) none
                    ]) ] } }

def cultivatorColossus : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cultivator Colossus", cost := some [generic 4, pip .green, pip .green, pip .green],
      types := [.creature], subtypes := [creatureType "Plant", creatureType "Beast"],
      text :=
        [ keyword "Trample",
          Primitives.Ability.static (Primitives.StaticSpec.ptDefinition thisCreature .bothEach
            (countOf (Primitives.Predicate.and [land, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))),
          when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Instruction.offer (putOntoBattlefieldTapped (a (Primitives.Predicate.and [land, Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you)])))
              (some (Primitives.Instruction.sequentially [Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you), Primitives.Instruction.repeat_ Primitives.Repetition.again])) none (agent :=
                  Primitives.NounPhrase.you)) ] } }

def zimoneAndDina : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.tapSymbol, Primitives.Cost.perform (sacrifice (a (otherCreature thisCreature)) (agent :=
      Primitives.NounPhrase.you))])
    (Primitives.Instruction.sequentially
      [ Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you),
        offer (putOntoBattlefieldTapped (a (Primitives.Predicate.and [land, Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you)]))) (agent := Primitives.NounPhrase.you),
        Primitives.Instruction.doIf (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [land, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .atLeast (.lit
            8))
          (Primitives.Instruction.repeat_ (Primitives.Repetition.moreTimes (.lit 1))) none ])
theorem okZimoneAndDina : Ability.check [] zimoneAndDina = [] := by decide

def cryptLurker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crypt Lurker", cost := some [generic 3, pip .black], types := [.creature],
      subtypes := [creatureType "Horror"],
      text :=
        [ when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Instruction.offer
              (chooseModes (exactly 1)
                [ sacrifice (a creature) (agent := Primitives.NounPhrase.you),
                  discard (a (Primitives.Predicate.and [creature, Primitives.Predicate.inZone hand])) (agent := Primitives.NounPhrase.you) ])
              (some (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you))) none (agent := Primitives.NounPhrase.you)) ],
      power := stat 3, toughness := stat 4 } }

/-- Memoricide -/
def memoricideSearch : Instruction :=
  Primitives.Instruction.sequentially
    [ choose (a (qualityFrom .cardName (Primitives.ChoiceDomain.nameOfCard (Primitives.Predicate.not land)))),
      searchZonesOf (target Primitives.Predicate.anyPlayer) (exactly 1) (Primitives.Predicate.named Primitives.NameSource.chosen),
      Primitives.Instruction.shuffle (agent := (that .player)) ]
theorem okMemoricideSearch : Instruction.check [] memoricideSearch = [] := by decide
/-- Lost Hours -/
def lostHoursPlacement : Instruction :=
  Primitives.Instruction.sequentially
    [ revealTheirHand (agent := (target Primitives.Predicate.anyPlayer)),
      choose (a (Primitives.Predicate.and [Primitives.Predicate.not land, Primitives.Predicate.inZone (handOf they)])),
      put it (nthFromTop (.nth 3)) (agent := (that .player)) ]
theorem okLostHoursPlacement : Instruction.check [] lostHoursPlacement = [] := by decide
/-- Aether Gust -/
def aetherGustPlacement : Instruction :=
  Primitives.Instruction.sequentially
    [ choose (target (Primitives.Predicate.and [permanent, Primitives.Predicate.colorIs .red])),
      put it (choiceOfTopOrBottom they) (agent := (ownerOf it)) ]
theorem okAetherGustPlacement : Instruction.check [] aetherGustPlacement = [] := by decide
/-- Fastbond -/
def fastbondLands : Ability := Primitives.Ability.static (mayPlayAdditionalLands Primitives.NounPhrase.you anyNumber)
theorem okFastbondLands : Ability.check [] fastbondLands = [] := by decide
/-- Panglacial Wurm -/
def panglacialWurmCast : Ability :=
  Primitives.Ability.static (mayPlayDeed (.action "Cast") Primitives.NounPhrase.you Primitives.NounPhrase.this none
    (Primitives.DeonticRider.play (some yourLibrary) none (some .whileSearchingLibrary) false Primitives.PlayPayment.itsOwnCost))
theorem okPanglacialWurmCast : Ability.check [] panglacialWurmCast = [] := by decide

/-- Shah of Naar Isle -/
def shahOfNaarIsle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shah of Naar Isle", cost := some [generic 3, pip .red], types := [.creature],
      subtypes := [creatureType "Efreet"],
      text :=
        [ keyword "Trample",
          keywordCosting "Echo" (Primitives.Cost.mana [generic 0]),
          when (Primitives.GameEvent.paysCost none .paid thisCreature "Echo")
            (offer (Primitives.Instruction.draw (Primitives.Amount.upTo (.lit 3)) (agent := they)) (agent := (each Primitives.Predicate.opponent))) ],
      power := stat 6, toughness := stat 6 } }

/-- Memory Plunder -/
def memoryPlunder : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Memory Plunder",
      cost := some [hybridPip .blue .black, hybridPip .blue .black, hybridPip .blue .black,
                    hybridPip .blue .black],
      types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.establish
            (mayPlayDeed (.action "Cast") Primitives.NounPhrase.you (target (Primitives.Predicate.and [instantOrSorcery, Primitives.Predicate.isCard])) none
              (Primitives.DeonticRider.play (some (graveyardOf anOpponent)) none none false Primitives.PlayPayment.withoutPaying))
            none) ] } }

/-- Omniscience -/
def omniscience : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Omniscience", cost := some [generic 7, pip .blue, pip .blue, pip .blue],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.static (mayPlayDeed (.action "Cast") Primitives.NounPhrase.you (allOf spell) none
            (Primitives.DeonticRider.play (some (handOf Primitives.NounPhrase.you)) none none false Primitives.PlayPayment.withoutPaying)) ] } }

/-- Tranquil Frillback -/
def tranquilFrillbackOffer : Instruction := offer (Primitives.Instruction.pay (Primitives.Cost.mana [pip .green]) (.upTo 3) (agent :=
    Primitives.NounPhrase.you)) (agent := Primitives.NounPhrase.you)
theorem okTranquilFrillbackOffer : Instruction.check [] tranquilFrillbackOffer = [] := by decide

/-- Caller of the Hunt -/
def callerOfTheHunt : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Caller of the Hunt", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Human"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.addedCost (Primitives.Cost.perform (choose (a (quality (.subtype .creature))))) false),
          Primitives.Ability.static (Primitives.StaticSpec.ptDefinition thisCreature .bothEach
            (countOf (Primitives.Predicate.and [creature, ofChosen (.subtype .creature)]))) ] } }

/-- Duneblast -/
def duneblast : Instruction :=
  Primitives.Instruction.sequentially [choose (counted (upTo 1) creature), destroy (theRest .object)]
theorem okDuneblast : Instruction.check [] duneblast = [] := by decide
/-- Boreas Charger -/
def boreasChargerChoice : Instruction := choose (a opponentWithMoreLands)
theorem okBoreasChargerChoice : Instruction.check [] boreasChargerChoice = [] := by decide
/-- Boreas Charger's spell text -/
def boreasChargerSpell : Instruction :=
  Primitives.Instruction.sequentially
    [ choose (a opponentWithMoreLands),
      searchLibraryFor (Primitives.Quantity.exactlyOf Primitives.Amount.theDifference) (Primitives.Predicate.hasSubtype (landType "Plains")),
      revealCards (those .card),
      putOntoBattlefieldTapped (someOf (exactly 1) (those .card)),
      move (theRest .object) hand ]
theorem okBoreasChargerSpell : Instruction.check [] boreasChargerSpell = [] := by decide

/-- Sandstone Oracle -/
def sandstoneOracle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sandstone Oracle", cost := some [generic 7], types := [.artifact, .creature],
      subtypes := [creatureType "Sphinx"],
      text :=
        [ keyword "Flying",
          when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Instruction.sequentially
              [ choose anOpponent,
                Primitives.Instruction.doIf (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.inZone (handOf (that .player)))) .greater
                        (countOf (Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you))))
                  (Primitives.Instruction.draw Primitives.Amount.theDifference (agent := Primitives.NounPhrase.you)) none ]) ],
      power := stat 4, toughness := stat 4 } }

/-- Slithermuse -/
def slithermuseTrigger : Ability :=
  when (leavesBattlefield thisCreature)
    (Primitives.Instruction.sequentially
      [ choose anOpponent,
        Primitives.Instruction.doIf (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.inZone (handOf (that .player)))) .greater
                (countOf (Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you))))
          (Primitives.Instruction.draw Primitives.Amount.theDifference (agent := Primitives.NounPhrase.you)) none ])
theorem okSlithermuseTrigger : Ability.check [] slithermuseTrigger = [] := by decide

/-- Celestial Judgment -/
def celestialJudgment : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Celestial Judgment", cost := some [generic 4, pip .white, pip .white],
      types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ Primitives.Instruction.doForEachKind (.value .power) (some (allOf creature)) .number
                (choose (a (Primitives.Predicate.and [creature, Primitives.Predicate.compare [.stat .power] .eq chosenNumber]))),
              destroy (each (Primitives.Predicate.and [creature, Primitives.Predicate.notChosen])) ]) ] } }

/-- World Queller -/
def worldQuellerChoice : Instruction :=
  Primitives.Instruction.offer (choose (a (quality .cardType)))
    (some (sacrifice (aTheirChoice (Primitives.Predicate.and [permanent, ofChosen .cardType])) (agent := (each
        Primitives.Predicate.anyPlayer)))) none (agent := Primitives.NounPhrase.you)
theorem okWorldQuellerChoice : Instruction.check [] worldQuellerChoice = [] := by decide

/-- Moonlit Meditation -/
def moonlitMeditation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Moonlit Meditation", cost := some [generic 2, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" (Primitives.Predicate.and [Primitives.Predicate.or [artifact, creature], Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]),
          Primitives.Ability.static (Primitives.StaticSpec.replacement (Primitives.GameEvent.tokensCreated (counted (atLeast 1) Primitives.Predicate.isToken) false (some Primitives.NounPhrase.you)
              none)
            [] none
            (offer (Primitives.Instruction.create Primitives.Amount.groupSize (Primitives.TokenSpec.copyOf (Primitives.NounPhrase.attachHost .enchanted .permanent) []) [] (agent :=
                Primitives.NounPhrase.you)) (agent := Primitives.NounPhrase.you))
            .repeatedly (some Primitives.UsageLimit.oncePerTurn)) ] } }

def discardUpToTwoThenDrawThatMany : Instruction :=
  Primitives.Instruction.sequentially [discard (counted (upTo 2) (Primitives.Predicate.inZone hand)) (agent := Primitives.NounPhrase.you), Primitives.Instruction.draw Primitives.Amount.groupSize (agent :=
      Primitives.NounPhrase.you)]
theorem okDiscardUpToTwoThenDrawThatMany :
    Instruction.check [] discardUpToTwoThenDrawThatMany = [] := by decide
/-- Truce and Temporary Truce -/
def drawUpToTwoThenGainPerShortfall : Instruction :=
  Primitives.Instruction.sequentially
    [ offer (Primitives.Instruction.draw (Primitives.Amount.upTo (.lit 2)) (agent := they)) (agent := (each Primitives.Predicate.anyPlayer)),
      gainLife (times (.lit 2) shortOfCeiling) (agent := they) ]
theorem okDrawUpToTwoThenGainPerShortfall :
    Instruction.check [] drawUpToTwoThenGainPerShortfall = [] := by decide

/-- Commune with the Gods -/
def communeWithTheGods : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Commune with the Gods", cost := some [generic 1, pip .green], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ revealCards (topSlice (.lit 5)),
              offer (move (fromAmong (exactly 1) (Primitives.Predicate.or [creature, enchantment]) them) hand) (agent :=
                  Primitives.NounPhrase.you),
              move (theRest .object) graveyard ]) ] } }

/-- Nautiloid Ship -/
def nautiloidShipTrigger : Ability :=
  whenever (dealsCombatDamage thisVehicle (a Primitives.Predicate.anyPlayer))
    (offer (putOntoBattlefieldUnderYourControl (a (Primitives.Predicate.and [creature, Primitives.Predicate.exiledWith thisVehicle])))
        (agent := Primitives.NounPhrase.you))
theorem okNautiloidShipTrigger : Ability.check [] nautiloidShipTrigger = [] := by decide

/-- Summon: Esper Valigarmanda's II/III/IV body, in part -/
def summonEsperValigarmandaCast : StaticSpec :=
  Primitives.StaticSpec.conjunction none
    [ mayPlayDeed (.action "Cast") Primitives.NounPhrase.you (a (Primitives.Predicate.and [instantOrSorcery, Primitives.Predicate.exiledWith thisSaga])) none
        (Primitives.DeonticRider.play none none none false Primitives.PlayPayment.itsOwnCost),
      maySpendAsThough Primitives.NounPhrase.you none .anyType
        (some (Primitives.SpendPurpose.toCast (Primitives.Predicate.and [instantOrSorcery, Primitives.Predicate.exiledWith thisSaga]))) ]
theorem okSummonEsperValigarmandaCast :
    StaticSpec.check [] summonEsperValigarmandaCast = [] := by decide
/-- Rogue Class's level-3 body -/
def rogueClassLevelThree : StaticSpec :=
  Primitives.StaticSpec.conjunction none
    [ mayPlayDeed (.action "Play") Primitives.NounPhrase.you (allOf (Primitives.Predicate.exiledWith thisClass)) none
        (Primitives.DeonticRider.play none none none false Primitives.PlayPayment.itsOwnCost),
      maySpendAsThough Primitives.NounPhrase.you none .anyColor (some (Primitives.SpendPurpose.toCast (Primitives.Predicate.exiledWith thisClass))) ]
theorem okRogueClassLevelThree : StaticSpec.check [] rogueClassLevelThree = [] := by decide

/-- Soul Ransom -/
def soulRansom : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Soul Ransom", cost := some [generic 2, pip .blue, pip .black], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (Primitives.StaticSpec.controlGrant Primitives.NounPhrase.you (Primitives.NounPhrase.attachHost .enchanted (.type .creature))),
          activatedBy
            (Primitives.Cost.perform (Primitives.Instruction.repeatTimes (.lit 2)
              (Primitives.Instruction.sequentially [choose (a (Primitives.Predicate.inZone hand)), discard (that .card) (agent := Primitives.NounPhrase.you)])))
            (Primitives.Instruction.sequentially [sacrificeIt (agent := (controllerOf thisAura)), Primitives.Instruction.draw (.lit 2) (agent :=
                they)])
            (Primitives.NounPhrase.playerGroup .yourOpponents) ] } }

/-- Vraska's Scorn -/
def vraskasScorn : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vraska's Scorn", cost := some [generic 2, pip .black, pip .black], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ loseLife (.lit 4) (agent := (target Primitives.Predicate.opponent)),
              offer
                (Primitives.Instruction.sequentially
                  [ searchLibraryOrGraveyard (Primitives.Predicate.named (Primitives.NameSource.printed "Vraska, Scheming Gorgon")),
                    revealCards it, move it hand ]) (agent := Primitives.NounPhrase.you),
              Primitives.Instruction.doIf (happened (Primitives.GameEvent.verbedEvent (some
                (relative .player)) (.action "Search") none none (some yourLibrary))
                Primitives.NounPhrase.you .thisWay) shuffle
                none ]) ] } }

/-- Old-Growth Dryads -/
def oldGrowthDryads : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Old-Growth Dryads", cost := some [pip .green], types := [.creature],
      subtypes := [creatureType "Dryad"],
      text :=
        [ when (Primitives.GameEvent.enters thisCreature none)
            (offer
              (Primitives.Instruction.sequentially
                [ searchTheirLibraryFor (Primitives.Predicate.and [land, Primitives.Predicate.hasSupertype .basic]) (agent := they),
                  putOntoBattlefieldTapped foundCard,
                  Primitives.Instruction.shuffle (agent := they) ]) (agent := (each Primitives.Predicate.opponent))) ],
      power := stat 3, toughness := stat 3 } }

/-- Verity Circle -/
def verityCircle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Verity Circle", cost := some [generic 2, pip .blue], types := [.enchantment],
      text :=
        [ triggeredIf
            (Primitives.GameEvent.statusEvent (a (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller anOpponent])) .tapped)
            (Primitives.Condition.matches it (Primitives.Predicate.not (Primitives.Predicate.inCombat .declaredAttacker none)))
            (offer (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)) (agent := Primitives.NounPhrase.you)),
          activated (Primitives.Cost.mana [generic 4, pip .blue])
            (tap (target (Primitives.Predicate.and [creature, Primitives.Predicate.not (Primitives.Predicate.hasKeyword (.the "Flying"))]))) ] } }

/-- Danitha, New Benalia's Light -/
def danithaNewBenaliasLight : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Danitha, New Benalia's Light", cost := some [generic 1, pip .green, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Knight"],
      text :=
        [ keyword "Vigilance", keyword "Trample", keyword "Lifelink",
          Primitives.Ability.static (mayPlayDeed (.action "Cast") Primitives.NounPhrase.you
            (a (Primitives.Predicate.and [ spell, Primitives.Predicate.or [ Primitives.Predicate.hasSubtype (enchantmentType "Aura"),
                                    Primitives.Predicate.hasSubtype (artifactType "Equipment") ] ]))
            none (Primitives.DeonticRider.play (some (graveyardOf Primitives.NounPhrase.you)) (some .onceEachYourTurn) none false Primitives.PlayPayment.itsOwnCost)) ],
      power := stat 2, toughness := stat 2 } }

/-- Muldrotha, the Gravetide -/
def muldrothaLandWindow : StaticSpec :=
  mayPlayDeed (.action "Play") Primitives.NounPhrase.you (a land) none
    (Primitives.DeonticRider.play (some (graveyardOf Primitives.NounPhrase.you)) none (some .duringEachOfYourTurns) false Primitives.PlayPayment.itsOwnCost)
theorem okMuldrothaLandWindow : StaticSpec.check [] muldrothaLandWindow = [] := by decide
/-- Nahiri's Lithoforming -/
def nahiriExtraLands : StaticSpec := mayPlayAdditionalLands Primitives.NounPhrase.you (Primitives.Quantity.exactlyOf (Primitives.Amount.letter .x))
theorem okNahiriExtraLands : StaticSpec.check [letterB .x] nahiriExtraLands = [] := by decide

/-- Haakon, Stromgald Scourge -/
def haakonStromgaldScourge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Haakon, Stromgald Scourge", cost := some [generic 1, pip .black, pip .black],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Zombie", creatureType "Knight"],
      text :=
        [ Primitives.Ability.static (mayPlayDeed (.action "Cast") Primitives.NounPhrase.you Primitives.NounPhrase.this none
            (Primitives.DeonticRider.play (some (graveyardOf Primitives.NounPhrase.you)) none none true Primitives.PlayPayment.itsOwnCost)),
          Primitives.Ability.static (Primitives.StaticSpec.conditional
            (mayPlayDeed (.action "Cast") Primitives.NounPhrase.you
              (allOf (Primitives.Predicate.and [spell, Primitives.Predicate.hasSubtype (creatureType "Knight")])) none
              (Primitives.DeonticRider.play (some (graveyardOf Primitives.NounPhrase.you)) none none false Primitives.PlayPayment.itsOwnCost))
            (Primitives.Condition.matches Primitives.NounPhrase.this (Primitives.Predicate.inZone battlefield)) .asLongAs),
          when (Primitives.GameEvent.dies thisCreature) (loseLife (.lit 2) (agent := Primitives.NounPhrase.you)) ],
      power := stat 3, toughness := stat 3 } }

/-- Apex of Power -/
def apexOfPowerCast : Instruction :=
  Primitives.Instruction.sequentially
    [ exile (Primitives.NounPhrase.librarySlice .top (.lit 7) Primitives.NounPhrase.you),
      Primitives.Instruction.establish
        (mayPlayDeed (.action "Cast") Primitives.NounPhrase.you (fromAmong anyNumber spell them) none
          (Primitives.DeonticRider.play none none none false Primitives.PlayPayment.itsOwnCost))
        (some Primitives.Duration.thisTurn) ]
theorem okApexOfPowerCast : Instruction.check [] apexOfPowerCast = [] := by decide
/-- Blight Herder -/
def blightHerderCast : Instruction :=
  offer
    (move (counted (exactly 2)
            (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.hasPossessor .owner (Primitives.NounPhrase.playerGroup .yourOpponents), Primitives.Predicate.inZone exileZone]))
      graveyard) (agent := Primitives.NounPhrase.you)
theorem okBlightHerderCast : Instruction.check [] blightHerderCast = [] := by decide

/-- True-Name Nemesis -/
def trueNameNemesis : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "True-Name Nemesis", cost := some [generic 1, pip .blue, pip .blue],
      types := [.creature], subtypes := [creatureType "Merfolk", creatureType "Rogue"],
      text :=
        [ Primitives.Ability.static (entersChoosingPlayer thisCreature none),
          keywordQuality "Protection" chosenPlayer ],
      power := stat 3, toughness := stat 1 } }

/-- Akiri, Fearless Voyager -/
def akiriUnattachOffer : Ability :=
  activated (Primitives.Cost.mana [pip .white])
    (Primitives.Instruction.offer
      (Primitives.Instruction.unattach (a (Primitives.Predicate.and [ Primitives.Predicate.hasSubtype (artifactType "Equipment"),
                            Primitives.Predicate.attachedTo (a creatureYouControl) ])))
      (some (Primitives.Instruction.setStatus .tapped (that (.type .creature)))) none (agent := Primitives.NounPhrase.you))
theorem okAkiriUnattachOffer : Ability.check [] akiriUnattachOffer = [] := by decide
/-- Summoning Materia -/
def summoningMateriaTopCast : Ability :=
  Primitives.Ability.static (onlyWhile (castFromTop (Primitives.Predicate.and [spell, creature]))
    (Primitives.Condition.matches thisEquipment (Primitives.Predicate.attachedTo (a creature))))
theorem okSummoningMateriaTopCast : Ability.check [] summoningMateriaTopCast = [] := by decide
/-- Vizier of the Menagerie -/
def vizierOfTheMenagerieSpend : StaticSpec :=
  maySpendAsThough Primitives.NounPhrase.you none .anyType (some (Primitives.SpendPurpose.toCast (Primitives.Predicate.and [creature, spell])))
theorem okVizierOfTheMenagerieSpend : StaticSpec.check [] vizierOfTheMenagerieSpend = [] := by
  decide

/-- Conspicuous Snoop -/
def conspicuousSnoop : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Conspicuous Snoop", cost := some [pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Goblin", creatureType "Rogue"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.visibility .reveal Primitives.NounPhrase.you Primitives.VisibleThing.topOfLibrary),
          Primitives.Ability.static (mayPlayDeed (.action "Play") Primitives.NounPhrase.you
            (allOf (Primitives.Predicate.and [spell, Primitives.Predicate.hasSubtype (creatureType "Goblin")])) none
            (Primitives.DeonticRider.play (some onTop) none none false Primitives.PlayPayment.itsOwnCost)),
          Primitives.Ability.static (onlyWhile
            (Primitives.StaticSpec.abilityGrantFrom thisCreature [.anyActivated] (topSlice (.lit 1)) none)
            (Primitives.Condition.matches (topSlice (.lit 1)) (Primitives.Predicate.hasSubtype (creatureType "Goblin")))) ],
      power := stat 2, toughness := stat 2 } }

/-- Ballot Broker -/
def ballotBroker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ballot Broker", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Advisor"],
      text := [Primitives.Ability.static (mayVoteAdditional Primitives.NounPhrase.you (exactly 1))],
      power := stat 2, toughness := stat 3 } }

/-- Emissary of Grudges -/
def emissaryOfGrudgesReveal : Ability :=
  activatedOnlyOnce (Primitives.Cost.perform (Primitives.Instruction.expose .reveal (Primitives.Exposed.choice .player) (agent := Primitives.NounPhrase.you)))
    (Primitives.Instruction.doOnlyIf (Primitives.Instruction.chooseNewTargets (target (Primitives.Predicate.or [spell, Primitives.Predicate.abilityHead .anyOnStack])))
      (Primitives.Condition.and
        [ Primitives.Condition.matches it (Primitives.Predicate.hasPossessor .controller (the chosenPlayer)),
          Primitives.Condition.matches it
            (Primitives.Predicate.targets (youOr (a (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))) .someTarget) ])
      none)
    Primitives.UsageLimit.oncePerGame
theorem okEmissaryOfGrudgesReveal :
    Ability.check [ChoiceSort.binding .player] emissaryOfGrudgesReveal = [] := by decide

def prosperity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Prosperity", cost := some [.variable, pip .blue], types := [.sorcery],
      text := [Primitives.Ability.spell none (Primitives.Instruction.draw (Primitives.Amount.letter .x) (agent := (each Primitives.Predicate.anyPlayer)))] } }

def collectiveUnconscious : Instruction := Primitives.Instruction.draw (forEach 1 creatureYouControl) (agent := Primitives.NounPhrase.you)
theorem okCollectiveUnconscious : Instruction.check [] collectiveUnconscious = [] := by decide
def killiansConfidence : Instruction :=
  Primitives.Instruction.sequentially
    [get (target creature) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn), Primitives.Instruction.draw (.lit 1)
        (agent := Primitives.NounPhrase.you)]
theorem okKilliansConfidence : Instruction.check [] killiansConfidence = [] := by decide

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
def notForgottenPlacement : Instruction :=
  move (target (Primitives.Predicate.inZone graveyard)) (choiceOfTopOrBottom Primitives.NounPhrase.you)
theorem okNotForgottenPlacement : Instruction.check [] notForgottenPlacement = [] := by decide

def duneblastCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Duneblast", cost := some [generic 4, pip .white, pip .black, pip .green],
      types := [.sorcery], text := [Primitives.Ability.spell none duneblast] } }

def boreasChargerDifference : Amount := Primitives.Amount.theDifference
theorem okBoreasChargerDifference :
    Amount.check (Instruction.intro [] boreasChargerChoice) boreasChargerDifference = [] := by decide

/-- Hymn to Tourach -/
def hymnToTourach : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hymn to Tourach", cost := some [pip .black, pip .black], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (discard (countedAtRandom (exactly 2) (Primitives.Predicate.inZone hand)) (agent := (target
            Primitives.Predicate.anyPlayer))) ] } }

/-- Caught in the Crossfire -/
def caughtInTheCrossfire : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Caught in the Crossfire", cost := some [pip .red, pip .red], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (chooseSpree
            [ (some (Primitives.Cost.mana [generic 1]),
               Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 2) (each (Primitives.Predicate.and [creature, outlaw]))),
              (some (Primitives.Cost.mana [generic 1]),
               Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 2) (each (Primitives.Predicate.and [creature, Primitives.Predicate.not outlaw]))) ]) ] } }

/-- Requisition Raid -/
def requisitionRaid : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Requisition Raid", cost := some [pip .white], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (chooseSpree
            [ (some (Primitives.Cost.mana [generic 1]), destroy (target artifact)),
              (some (Primitives.Cost.mana [generic 1]), destroy (target enchantment)),
              (some (Primitives.Cost.mana [generic 1]),
               Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne)
                 (each (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller (target Primitives.Predicate.anyPlayer)]))) ]) ] } }

/-- Rustler Rampage -/
def rustlerRampage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rustler Rampage", cost := some [pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (chooseSpree
            [ (some (Primitives.Cost.mana [generic 1]),
               Primitives.Instruction.setStatus .untapped
                 (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller (target Primitives.Predicate.anyPlayer)]))),
              (some (Primitives.Cost.mana [generic 1]),
               gain (target creature) (keyword "DoubleStrike") (some untilEndOfTurn)) ]) ] } }

/-- Consuming Tide -/
def consumingTide : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Consuming Tide", cost := some [generic 2, pip .blue, pip .blue], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ choose
                (a (Primitives.Predicate.and [permanent, Primitives.Predicate.not land, Primitives.Predicate.hasPossessor .controller they])) (agent := some
                    (each Primitives.Predicate.anyPlayer)),
              returnTo (allOf (Primitives.Predicate.and [permanent, Primitives.Predicate.not land, Primitives.Predicate.notChosen])) hand [],
              Primitives.Instruction.doForEach
                (each (Primitives.Predicate.compareOver Primitives.Predicate.opponent (countOf (Primitives.Predicate.inZone (handOf they))) .greater
                  (countOf (Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you)))))
                (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)) ]) ] } }

/-- Stick Together: a chosen party is four up-to-one choices, not the group the game computes
[CR#700.8d]. -/
def stickTogether : Instruction :=
  Primitives.Instruction.sequentially
    [ choose
        (counted (upTo 1)
          (Primitives.Predicate.and [creature, Primitives.Predicate.hasSubtype (creatureType "Cleric"), Primitives.Predicate.hasPossessor .controller they]))
              (agent := some (each Primitives.Predicate.anyPlayer)),
      choose
        (counted (upTo 1)
          (Primitives.Predicate.and [creature, Primitives.Predicate.hasSubtype (creatureType "Rogue"), Primitives.Predicate.hasPossessor .controller they]))
              (agent := some (each Primitives.Predicate.anyPlayer)),
      choose
        (counted (upTo 1)
          (Primitives.Predicate.and [creature, Primitives.Predicate.hasSubtype (creatureType "Warrior"), Primitives.Predicate.hasPossessor .controller they]))
              (agent := some (each Primitives.Predicate.anyPlayer)),
      choose
        (counted (upTo 1)
          (Primitives.Predicate.and [creature, Primitives.Predicate.hasSubtype (creatureType "Wizard"), Primitives.Predicate.hasPossessor .controller they]))
              (agent := some (each Primitives.Predicate.anyPlayer)),
      sacrifice (theRest .object) (agent := (each Primitives.Predicate.anyPlayer)) ]
theorem okStickTogether : Instruction.check [] stickTogether = [] := by decide

/-- Disciple of Caelus Nin: "starting with you" fixes the order the players choose in
[CR#101.4]. -/
def discipleOfCaelusNin : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Disciple of Caelus Nin", cost := some [generic 4, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Instruction.sequentially
              [ Primitives.Instruction.choose (some Primitives.NounPhrase.you)
                  (counted (upTo 5) (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller they])) .openly none
                      (agent := (some (each Primitives.Predicate.anyPlayer))),
                Primitives.Instruction.setStatus .phasedOut
                  (allOf (Primitives.Predicate.and [permanent, Primitives.Predicate.otherThan thisCreature, Primitives.Predicate.notChosen])) ]),
          Primitives.Ability.static (Primitives.StaticSpec.deonticRule (allOf permanent) Primitives.Compulsion.forbid [.ofAbility "Phasing"] .agent none
              Primitives.DeonticPatient.noPatient
            none Primitives.DeonticRider.noRider) ],
      power := stat 3, toughness := stat 4 } }

/-- Sculpted Sunburst: "if you chose a creature this way" reads the standing choices back, the
same manner "not chosen ... this way" excludes [CR#101.4]. -/
def sculptedSunburst : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sculpted Sunburst", cost := some [generic 3, pip .white, pip .white],
      types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ choose (a creatureYouControl),
              choose
                (a (comparesOwnStat .power (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller they])
                  .atMost (Primitives.Amount.statOf (.stat .power) it))) (agent := some (each Primitives.Predicate.opponent)),
              Primitives.Instruction.doIf (Primitives.Condition.choseThisWay Primitives.NounPhrase.you creature) (exile (each (Primitives.Predicate.and [creature, Primitives.Predicate.notChosen]))) none
                  ]) ] } }

end Semantics.Cards
