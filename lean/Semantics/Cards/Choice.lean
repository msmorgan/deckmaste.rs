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
  sequentially [draw (.lit 2), discard (counted (exactly 2) (cardIn yourHand))]
theorem okCarefulStudy : Action.check (DRS.ofList []) carefulStudy = [] := by decide

def zombieInfestation : Ability :=
  activated (perform (discard (counted (exactly 2) (cardIn yourHand))))
    (createToken (creatureToken 2 2 [.black] [zombie]))
theorem okZombieInfestation : Ability.check (DRS.ofList []) zombieInfestation = [] := by decide

def fulgentDistraction : Action :=
  sequentially
    [ choose (targets (exactly 2) isCreature),
      tap (those creature),
      unattach (allOf (and [ hasSubtype equipment,
                               attachedTo (those creature) ])) ]
theorem okFulgentDistraction : Action.check (DRS.ofList []) fulgentDistraction = [] := by decide

def continueSpell : Action :=
  sequentially
    [ choose (targets (upTo 4) (and [isCreature, in_ yourGraveyard])),
      returnToBattlefield them ]
theorem okContinueSpell : Action.check (DRS.ofList []) continueSpell = [] := by decide

def kindredDominance : Action :=
  sequentially
    [ choose (a (quality (.subtype .creature))),
      destroy (allOf (and [isCreature, not (ofChosen (.subtype .creature))])) ]
theorem okKindredDominance : Action.check (DRS.ofList []) kindredDominance = [] := by decide

def phantomBlade : Ability :=
  when (does thisEquipment enter)
    (sequentially
      [ attachTo it
          (targets (upTo 1) (and [isCreature, youControl])),
        destroy (targets (upTo 1) (and [isCreature, isOther])) ])
theorem okPhantomBlade : Ability.check (DRS.ofList []) phantomBlade = [] := by decide

/-- Braids's Frightful Return -/
def braidsFrightfulReturn : Action :=
  may (sacrifice (a isCreature)) (some
                                    (sequentially
                                       [instruct eachOpponent (choose (a (cardIn hand))), discard
                                          them]))
theorem okBraidsFrightfulReturn : Action.check (DRS.ofList []) braidsFrightfulReturn = [] := by decide
/-- Daretti, Ingenious Iconoclast -/
def darettisMinusOne : Action :=
  may (sacrifice (a isArtifact)) (some (destroy (target (or [isArtifact,
      isCreature]))))
theorem okDarettisMinusOne : Action.check (DRS.ofList []) darettisMinusOne = [] := by decide

def cheeringFanatic : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cheering Fanatic", cost := some [generic 1, pip .red], types := [.creature],
      subtypes := [goblin],
      text :=
        [ whenever (does thisCreature attack)
            (sequentially
              [ choose (a (quality .cardName)),
                establishThroughout (costShift (allOf (and [isSpell, ofChosen .cardName]))
                                       (less (.lit 1))) thisTurn ]) ],
      power := stat 2, toughness := stat 2 } }

def rainOfThorns : Action :=
  chooseModes (atLeast 1)
    [destroy (target isArtifact), destroy (target isEnchantment), destroy (target isLand)]
theorem okRainOfThorns : Action.check (DRS.ofList []) rainOfThorns = [] := by decide

def rankleMasterOfPranks : Action :=
  chooseModes anyNumber
    [ sequentially [instruct eachPlayer (choose (a (cardIn hand))), discard them],
      sequentially [instruct eachPlayer (loseLife (.lit 1)), instruct (those player)
                      (draw (.lit 1))],
      instruct eachPlayer (sacrifice (aTheirChoice isCreature)) ]
theorem okRankleMasterOfPranks : Action.check (DRS.ofList []) rankleMasterOfPranks = [] := by decide

def myrkulsEdict : Action :=
  sequentially [choose anOpponent, instruct (that player) (sacrifice (aTheirChoice isCreature))]
theorem okMyrkulsEdict : Action.check (DRS.ofList []) myrkulsEdict = [] := by decide
def moltingHarpy : Action := doUnless (sacrifice thisCreature) (mana [generic
    2])
theorem okMoltingHarpy : Action.check (DRS.ofList []) moltingHarpy = [] := by decide
def carnophage : Action := doUnless (tap thisCreature) (payLife 1)
theorem okCarnophage : Action.check (DRS.ofList []) carnophage = [] := by decide
def solitaryConfinement : Action :=
  mayOrElse (discard (a (cardIn yourHand))) (sacrifice thisEnchantment)
theorem okSolitaryConfinement : Action.check (DRS.ofList []) solitaryConfinement = [] := by decide

def yasminKhan : Ability :=
  activated tapSymbol
    (sequentially
      [ exile (topSlice (.lit 1)),
        establishThroughout (can you (play it)) untilYourNextEndStep ])
theorem okYasminKhan : Ability.check (DRS.ofList []) yasminKhan = [] := by decide

/-- Brazen Cannonade -/
def brazenCannonadePermission : Action :=
  sequentially
    [ exile (topSlice (.lit 1)),
      establishThroughout (can you (play it)) (untilEvent
                                                         (endOf .combat (some you) nextTime)) ]
theorem okBrazenCannonadePermission : Action.check (DRS.ofList []) brazenCannonadePermission = [] := by
  decide

def thousandMoonsCrackshot : Ability :=
  whenever (does thisCreature attack)
    (offerWhen (pay (mana [generic 2, pip .white]) .once)
      (tap (target isCreature)))
theorem okThousandMoonsCrackshot : Ability.check (DRS.ofList []) thousandMoonsCrackshot = [] := by decide

def zimoneQuandrixProdigy : Ability :=
  activated (compound [mana [generic 1], tapSymbol])
    (may (putOntoBattlefieldTapped (a (and [isLand, in_ yourHand]))))
theorem okZimoneQuandrixProdigy : Ability.check (DRS.ofList []) zimoneQuandrixProdigy = [] := by decide

def preeminentCaptain : Ability :=
  whenever (does thisCreature attack)
    (may (putOntoBattlefieldTappedAttacking
      (a (and [hasSubtype soldier, isCreature, in_ yourHand]))))
theorem okPreeminentCaptain : Ability.check (DRS.ofList []) preeminentCaptain = [] := by decide

def skaabRuinator : Ability :=
  static (can you (cast this_ (from_ := some yourGraveyard)))
theorem okSkaabRuinator : Ability.check (DRS.ofList []) skaabRuinator = [] := by decide

/-- Raffine's Guidance -/
def raffinesGuidance : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Raffine's Guidance", cost := some [pip .white], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isCreature,
          static (getsPt (enchanted creature) (up (.lit 1)) (up (.lit 1))),
          static (can you
                    (cast this_ (from_ := some yourGraveyard)
                       (payment := payingInstead (mana [generic 2, pip .white])))) ] } }

/-- Scourge of Nel Toth -/
def scourgeOfNelToth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Scourge of Nel Toth", cost := some [generic 5, pip .black, pip .black],
      types := [.creature], subtypes := [zombie, dragon],
      text :=
        [ flying,
          static (can you
                    (cast this_ (from_ := some yourGraveyard) (payment := payingInstead (compound
                [ mana [pip .black, pip .black],
                  perform (sacrifice (counted (exactly 2) isCreature)) ])))) ],
      power := stat 6, toughness := stat 6 } }

def escapeToTheWilds : Action :=
  sequentially
    [ exile (topSlice (.lit 5)),
      establishThroughout (can you
                             (play (bare (and [isCard, was (exile which) thisWay]))))
        (untilEvent (endOf .turn (some you) nextTime)) ]
theorem okEscapeToTheWilds : Action.check (DRS.ofList []) escapeToTheWilds = [] := by decide

/-- Muse Vessel -/
def museVesselPlay : Ability :=
  activated (mana [generic 1])
    (sequentially
      [ choose (a exiledWithThisArtifact),
        establishThroughout (can you (play (that card))) thisTurn ])
theorem okMuseVesselPlay : Ability.check (DRS.ofList []) museVesselPlay = [] := by decide

def demonicConsultationChoice : Action := choose (a (quality .cardName))
theorem okDemonicConsultationChoice : Action.check (DRS.ofList []) demonicConsultationChoice = [] := by
  decide
def voidChoice : Action := choose (a (quality .number))
theorem okVoidChoice : Action.check (DRS.ofList []) voidChoice = [] := by decide
def ashnodsBattleGear : Ability := static (mayDeclineUntap thisArtifact (some you))
theorem okAshnodsBattleGear : Ability.check (DRS.ofList []) ashnodsBattleGear = [] := by decide

/-- Phyrexian Ingester -/
def phyrexianIngesterPump : Ability :=
  static (conjunction [ getsPt thisCreature (up (letter .x)) (up (letter .y)),
      letterDefinition .x
        (powerOf (the (and [isCreature, in_ (exiledWith thisCreature)]))),
      letterDefinition .y (toughnessOf (that card)) ])
theorem okPhyrexianIngesterPump : Ability.check (DRS.ofList []) phyrexianIngesterPump = [] := by decide

/-- Phyrexian Ingester -/
def phyrexianIngester : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Phyrexian Ingester", cost := some [generic 6, pip .blue], types := [.creature],
      subtypes := [phyrexian, beast],
      text :=
        [ imprint
            (when (does thisCreature enter)
              (may (exile (target (and [isCreature, isNontoken]))))),
          phyrexianIngesterPump ],
      power := stat 3, toughness := stat 3 } }

def misterFantastic : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mister Fantastic, Reed Richards", cost := some [generic 3, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [human, scientist, hero],
      text :=
        [ reach,
          whenever
            (does (counted (atLeast 1) (and [isToken, youControl])) enter)
            (may (draw (.lit 1))) ],
      power := stat 2, toughness := stat 4 } }

def murmursFromBeyond : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Murmurs from Beyond", cost := some [generic 2, pip .blue], types := [.instant],
      subtypes := [arcane],
      text :=
        [ spell (sequentially
            [ reveal (topSlice (.lit 3)),
              instruct anOpponent (choose (someOf (exactly 1) them)),
              putInto (that card) graveyard,
              putInto (theRest .object) hand ]) ] } }

/-- Helica Glider -/
def helicaGlider : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Helica Glider", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [nightmare, squirrel],
      text :=
        [ static (enters thisCreature
            [withCounters (.lit 1) (chosenKind [flyingCounter, firstStrikeCounter]) .fresh]) ],
      power := stat 2, toughness := stat 2 } }

/-- Eager Construct -/
def eagerConstruct : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Eager Construct", cost := some [generic 2], types := [.artifact, .creature],
      subtypes := [construct],
      text := [when (does thisCreature enter) (instruct eachPlayer (may (scry (.lit 1))))],
      power := stat 2, toughness := stat 2 } }

/-- Rune Snag -/
def runeSnag : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rune Snag", cost := some [generic 1, pip .blue], types := [.instant],
      text :=
        [ spell (instruct (controllerOf (target isSpell)) (doUnless (counter it)
            (compound
              [ mana [generic 2],
                scaledGenericMana
                  (forEach (.lit 2)
                     (and [named "Rune Snag", in_ graveyard])) ]))) ] } }

/-- Tahngarth, First Mate -/
def tahngarthChoosesDefender : Action :=
  choose (a (and [ or [isPlaneswalker, isPlayer],
               inCombat .attackedBy (some (that player)) ]))
theorem okTahngarthChoosesDefender :
    Action.check ((stepGameEvent 512 (DRS.ofList []) tahngarthHeader).1) tahngarthChoosesDefender = [] := by
  decide

def reefShaman : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Reef Shaman", cost := some [pip .blue], types := [.creature],
      subtypes := [merfolk, shaman],
      text :=
        [ activated tapSymbol
            (establishThroughout (qualityChange (target isLand) .sets
                (chosenQuality
                   (ofYourChoice (.subtype .land) (some basicTypesOnly)))) untilEndOfTurn) ],
      power := stat 0, toughness := stat 2 } }

def grixisIllusionist : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grixis Illusionist", cost := some [pip .blue], types := [.creature],
      subtypes := [human, wizard],
      text :=
        [ activated tapSymbol
            (establishThroughout (qualityChange (target (and [isLand, youControl])) .sets
                (chosenQuality
                   (ofYourChoice (.subtype .land) (some basicTypesOnly)))) untilEndOfTurn) ],
      power := stat 1, toughness := stat 1 } }

def distantMelody : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Distant Melody", cost := some [generic 3, pip .blue], types := [.sorcery],
      text :=
        [ spell (sequentially
            [ choose (a (quality (.subtype .creature))),
              draw
                (forEach (.lit 1)
                  (and [isPermanent, youControl, ofChosen (.subtype .creature)])) ]) ] } }

def cripplingFear : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crippling Fear", cost := some [generic 2, pip .black, pip .black],
      types := [.sorcery],
      text :=
        [ spell (sequentially
            [ choose (a (quality (.subtype .creature))),
              get (allOf (and [isCreature, not (ofChosen (.subtype .creature))]))
                (down (.lit 3)) (down (.lit 3)) (some untilEndOfTurn) ]) ] } }

/-- Koh, the Face Stealer -/
def kohChooser : Ability :=
  activated (payLife 1) (choose (a (and [isCreature, in_ (exiledWith this_)])))
theorem okKohChooser : Ability.check (DRS.ofList []) kohChooser = [] := by decide
/-- Forgotten Lore -/
def forgottenLoreChoice : Action :=
  instruct (target isOpponent) (choose (a (cardIn yourGraveyard)))
theorem okForgottenLoreChoice : Action.check (DRS.ofList []) forgottenLoreChoice = [] := by decide

/-- Psychic Paper minus its three-way coordination -/
def psychicPaperChoiceAndReads : List Ability :=
  [ static (conjunction [ attachChoosing thisEquipment .cardName,
        attachChoosing thisEquipment (.subtype .creature) ]),
    static (conjunction [ qualityChange (equipped creature) .sets
          (chosenQuality (ofTheLastChosen .cardName)),
        qualityChange (equipped creature) .sets
          (chosenQuality (ofTheLastChosen (.subtype .creature))) ]) ]
theorem okPsychicPaperChoiceAndReads : Ability.checkText (DRS.ofList []) psychicPaperChoiceAndReads = [] := by
  decide

def xenograft : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Xenograft", cost := some [generic 4, pip .blue], types := [.enchantment],
      text :=
        [ static (entersChoosing thisEnchantment (.subtype .creature)),
          static (qualityChange (allOf (and [isCreature, youControl])) .adds
            (chosenQuality (ofChosen (.subtype .creature)))) ] } }

/-- Convincing Mirage -/
def convincingMirage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Convincing Mirage", cost := some [generic 1, pip .blue], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isLand,
          static (entersChoosingFrom thisAura (.subtype .land) basicTypesOnly),
          static (qualityChange (enchanted land) .sets
            (chosenQuality (ofChosen (.subtype .land)))) ] } }

/-- Realmwright -/
def realmwright : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Realmwright", cost := some [pip .blue], types := [.creature],
      subtypes := [vedalken, wizard],
      text :=
        [ static (entersChoosingFrom thisCreature (.subtype .land) basicTypesOnly),
          static (qualityChange (allOf (and [isLand, youControl])) .adds
            (chosenQuality (ofChosen (.subtype .land)))) ],
      power := stat 1, toughness := stat 1 } }

/-- Adaptive Automaton. "This creature is the chosen type in addition to its other types" is a
Characteristic-Defining Ability: it depends on the choice made as it enters, as Shapeshifter's
power does, and depending on a choice is no condition [CR#604.3a]. Its pump of the other creatures of the chosen type stays a
separate ability, affecting other objects [CR#604.3a]. -/
def adaptiveAutomaton : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Adaptive Automaton", cost := some [generic 3], types := [.artifact, .creature],
      subtypes := [construct],
      text :=
        [ static (entersChoosing thisCreature (.subtype .creature)),
          static (qualityDefinition thisCreature .adds (chosenQuality (ofChosen (.subtype
              .creature)))),
          static (getsPt
            (allOf (and [ isCreature, youControl, otherThan thisCreature,
                           ofChosen (.subtype .creature) ]))
            (up (.lit 1)) (up (.lit 1))) ],
      power := stat 2, toughness := stat 2 } }

def mistformDreamer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mistform Dreamer", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [illusion],
      text :=
        [ flying,
          activated (mana [generic 1])
            (establishThroughout (qualityChange thisCreature .sets
                (chosenQuality (ofYourChoice (.subtype .creature)))) untilEndOfTurn) ],
      power := stat 2, toughness := stat 1 } }

def arcaneAdaptation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Arcane Adaptation", cost := some [generic 2, pip .blue], types := [.enchantment],
      text :=
        [ static (entersChoosing thisEnchantment (.subtype .creature)),
          static (qualityChange (and [
              allOf (and [isCreature, youControl]),
              allOf (and [isCreature, isSpell, youControl]),
              allOf (and [isCard, isCreature, youOwn, not (in_ battlefield)]) ]) .adds
              (chosenQuality (ofChosen (.subtype .creature)))) ] } }

def conspiracy : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Conspiracy", cost := some [generic 3, pip .black, pip .black],
      types := [.enchantment],
      text :=
        [ static (entersChoosing thisEnchantment (.subtype .creature)),
          static (qualityChange (and [
              allOf (and [isCreature, youControl]),
              allOf (and [isCreature, isSpell, youControl]),
              allOf (and [isCard, isCreature, youOwn, not (in_ battlefield)]) ]) .sets
              (chosenQuality (ofChosen (.subtype .creature)))) ] } }

def declarationOfNaught : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Declaration of Naught", cost := some [pip .blue, pip .blue], types := [.enchantment],
      text :=
        [ static (entersChoosing thisEnchantment .cardName),
          activated (mana [pip .blue]) (counter (target (and [isSpell, named chosenName]))) ] } }

def imagecrafter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Imagecrafter", cost := some [pip .blue], types := [.creature],
      subtypes := [human, wizard],
      text :=
        [ activated tapSymbol
            (sequentially
              [ choose (a (qualityFrom (.subtype .creature) (typeOtherThan wall))),
                establishThroughout (qualityChange (target isCreature) .sets
                                       (chosenQuality (ofChosen (.subtype
                      .creature)))) untilEndOfTurn ]) ],
      power := stat 1, toughness := stat 1 } }

def unnaturalSelection : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unnatural Selection", cost := some [generic 1, pip .blue], types := [.enchantment],
      text :=
        [ activated (mana [generic 1])
            (sequentially
              [ choose (a (qualityFrom (.subtype .creature) (typeOtherThan wall))),
                establishThroughout (qualityChange (target isCreature) .sets
                                       (chosenQuality (ofChosen (.subtype
                      .creature)))) untilEndOfTurn ]) ] } }

def standardize : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Standardize", cost := some [pip .blue, pip .blue], types := [.instant],
      text :=
        [ spell (sequentially
            [ choose (a (qualityFrom (.subtype .creature) (typeOtherThan wall))),
              establishThroughout (qualityChange (each isCreature) .sets
                                     (chosenQuality (ofChosen (.subtype
                    .creature)))) untilEndOfTurn ]) ] } }

def silverquillSilencer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Silverquill Silencer", cost := some [pip .white, pip .black], types := [.creature],
      subtypes := [human, cleric],
      text :=
        [ static (entersChoosingFrom thisCreature .cardName (nameOfCard (non isLand))),
          whenever (does anOpponent (cast (a (and [isSpell,
            named chosenName]))))
            (sequentially [instruct they (loseLife (.lit 3)), draw (.lit 1)]) ],
      power := stat 3, toughness := stat 2 } }

def meddlingMage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Meddling Mage", cost := some [pip .white, pip .blue], types := [.creature],
      subtypes := [human, wizard],
      text :=
        [ static (entersChoosingFrom thisCreature .cardName (nameOfCard (non isLand))),
          static (cant any (cast (allOf (and [isSpell, named chosenName])))) ],
      power := stat 2, toughness := stat 2 } }

/-- Nyxathid -/
def nyxathid : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nyxathid", cost := some [generic 1, pip .black, pip .black], types := [.creature],
      subtypes := [elemental],
      text :=
        [ static (entersChoosingPlayer thisCreature anOpponent),
          static (getsPt thisCreature
            (down (countOf (cardIn (handOf (the isChosenPlayer)))))
            (down (countOf (cardIn (handOf (the isChosenPlayer)))))) ],
      power := stat 7, toughness := stat 7 } }

/-- Expel the Interlopers -/
def expelTheInterlopers : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Expel the Interlopers", cost := some [generic 3, pip .white, pip .white],
      types := [.sorcery],
      text :=
        [ spell (sequentially
            [ choose (a (qualityFrom .number (number (fromTo 0 10)))),
              destroy (allOf (and [isCreature, hasPower .atLeast chosenNumber])) ]) ] } }

def nevermore : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nevermore", cost := some [generic 1, pip .white, pip .white], types := [.enchantment],
      text :=
        [ static (entersChoosingFrom thisEnchantment .cardName (nameOfCard (non isLand))),
          static (cant any (cast (allOf (and [isSpell, named chosenName])))) ] } }

def necromentiaChoice : Action :=
  choose (a (qualityFrom .cardName (nameOfCard (not (and [hasSupertype .basic, isLand])))))
theorem okNecromentiaChoice : Action.check (DRS.ofList []) necromentiaChoice = [] := by decide

def conjurersBan : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Conjurer's Ban", cost := some [pip .white, pip .black], types := [.sorcery],
      text :=
        [ spell (sequentially
            [ choose (a (quality .cardName)),
              establishThroughout (conjunction
                                     [ cant any (cast (allOf (and [isSpell, named chosenName]))),
                    cant any (play
                                (allOf (and [isLand, named chosenName]))) ]) untilYourNextTurn ]),
          spell (draw (.lit 1)) ] } }

def foundingOfOmashu : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Founding of Omashu", cost := some [generic 2, pip .red], types := [.enchantment],
      subtypes := [saga],
      text :=
        [ when (chapterMark [1])
            (createToken (creatureToken 1 1 [.white] [ally]) (count := .lit 2)),
          when (chapterMark [2])
            (may (discard (a (cardIn yourHand))) (some (draw (.lit 1)))),
          when (chapterMark [3])
            (get (allOf (and [isCreature, youControl])) (up (.lit 1)) (up (.lit 0)) (some untilEndOfTurn)) ] }
                }

/-- "as though they had flash" -/
def asThoughFlash : Option AsThough :=
  some (holds (is them (hasKeyword flash)))

def vedalkenOrrery : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vedalken Orrery", cost := some [generic 4], types := [.artifact],
      text :=
        [ static (can you (cast (allOf isSpell)) (asThough := asThoughFlash)) ] } }

def shimmerMyr : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shimmer Myr", cost := some [generic 3], types := [.artifact, .creature],
      subtypes := [myr],
      text :=
        [ flash,
          static (can you (cast (allOf (and [isSpell, isArtifact]))) (asThough := asThoughFlash)) ],
      power := stat 2, toughness := stat 2 } }

def quickSliver : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Quick Sliver", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [sliver],
      text :=
        [ flash,
          static (can (a isPlayer) (cast (allOf (and [isSpell, hasSubtype sliver])))
                    (asThough := asThoughFlash)) ],
      power := stat 1, toughness := stat 1 } }

def vernalEquinox : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vernal Equinox", cost := some [generic 3, pip .green], types := [.enchantment],
      text :=
        [ static
            (can (a isPlayer) (cast (allOf (and [isSpell, or [isCreature, isEnchantment]])))
               (asThough := asThoughFlash)) ] } }

def borneUponAWind : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Borne Upon a Wind", cost := some [generic 1, pip .blue], types := [.instant],
      text :=
        [ spell
            (establishThroughout
               (can you (cast (allOf isSpell)) (asThough := asThoughFlash)) thisTurn),
          spell (draw (.lit 1)) ] } }

def gisaAndGeralf : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gisa and Geralf", cost := some [generic 2, pip .blue, pip .black],
      supertypes := [.legendary], types := [.creature],
      subtypes := [human, wizard],
      text :=
        [ when (does thisCreature enter) (mill (.lit 4)),
          static (can you
                    (cast (counted (exactly 1) (and [isSpell, isCreature, hasSubtype zombie]))
                       (from_ := some yourGraveyard)) (timing := some onEachOfYourTurns)) ],
      power := stat 4, toughness := stat 4 } }

/-- "play lands from the top of your library" -/
def playLandsFromTop : StaticSpec :=
  can you (play (allOf isLand) (from_ := some onTop))
/-- "cast <p> spells from the top of your library" -/
def castFromTop (p : Predicate) : StaticSpec :=
  can you (cast (allOf p) (from_ := some onTop))

/-- Future Sight; Magus of the Future -/
def playLandsAndCastSpellsFromTop : StaticSpec :=
  conjunction [playLandsFromTop, castFromTop isSpell]
theorem okPlayLandsAndCastSpellsFromTop :
    StaticSpec.check (DRS.ofList []) playLandsAndCastSpellsFromTop = [] := by decide
/-- Assemble the Players -/
def castSmallCreatureFromTopOnceEachTurn : StaticSpec :=
  can you
      (cast (counted (exactly 1) (and [isSpell, isCreature, hasPower .atMost (.lit 2)]))
         (from_ := some onTop)) (timing := some eachTurn)
theorem okCastSmallCreatureFromTopOnceEachTurn :
    StaticSpec.check (DRS.ofList []) castSmallCreatureFromTopOnceEachTurn = [] := by decide

def courserOfKruphix : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Courser of Kruphix", cost := some [generic 1, pip .green, pip .green],
      types := [.enchantment, .creature], subtypes := [centaur],
      text :=
        [ static (visibility .reveal you topOfLibrary),
          static playLandsFromTop,
          whenever (does (a (and [isLand, youControl])) enter)
            (gainLife (.lit 1)) ],
      power := stat 2, toughness := stat 4 } }

def korlessaScaleSinger : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Korlessa, Scale Singer", cost := some [pip .green, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [dragon, bard],
      text :=
        [ static (visibility .lookAt you topOfLibrary),
          static (castFromTop (and [isSpell, hasSubtype dragon])) ],
      power := stat 1, toughness := stat 4 } }

def precognitionField : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Precognition Field", cost := some [generic 3, pip .blue], types := [.enchantment],
      text :=
        [ static (visibility .lookAt you topOfLibrary),
          static (castFromTop (and [isSpell, isInstantOrSorcery])),
          activated (mana [generic 3]) (exile (topSlice (.lit 1))) ] } }

def mysticForge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mystic Forge", cost := some [generic 4], types := [.artifact],
      text :=
        [ static (visibility .lookAt you topOfLibrary),
          static (conjunction [castFromTop (and [isSpell, isArtifact]), castFromTop (and
              [isSpell, isColorless])]),
          activated (compound [tapSymbol, payLife 1]) (exile (topSlice (.lit 1))) ] } }

def exploration : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Exploration", cost := some [pip .green], types := [.enchantment],
      text :=
          [static
             (can you (play (counted (additional (exactly 1)) isLand))
                (timing := some onEachOfYourTurns))] } }

def oracleOfMulDaya : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Oracle of Mul Daya", cost := some [generic 3, pip .green], types := [.creature],
      subtypes := [elf, shaman],
      text :=
        [ static
            (can you (play (counted (additional (exactly 1)) isLand))
               (timing := some onEachOfYourTurns)),
          static (visibility .reveal you topOfLibrary),
          static playLandsFromTop ],
      power := stat 2, toughness := stat 2 } }

def azusaLostButSeeking : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Azusa, Lost but Seeking", cost := some [generic 2, pip .green],
      supertypes := [.legendary], types := [.creature],
      subtypes := [human, monk],
      text :=
          [static
             (can you (play (counted (additional (exactly 2)) isLand))
                (timing := some onEachOfYourTurns))],
      power := stat 1, toughness := stat 2 } }

def summerBloom : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Summer Bloom", cost := some [generic 1, pip .green], types := [.sorcery],
      text := [spell
                 (establishThroughout
                    (can you (play (counted (additional (upTo 3)) isLand))) thisTurn)] } }

def explore : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Explore", cost := some [generic 1, pip .green], types := [.sorcery],
      text :=
        [ spell (sequentially
            [ establishThroughout (can you (play (counted (additional (exactly 1)) isLand)))
                thisTurn,
              draw (.lit 1) ]) ] } }

def urbanEvolution : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Urban Evolution", cost := some [generic 3, pip .green, pip .blue],
      types := [.sorcery],
      text :=
        [ spell (sequentially
            [ draw (.lit 3),
              establishThroughout (can you
                                     (play
                                        (counted (additional (exactly 1))
                                           isLand))) thisTurn ]) ] } }

def dryadOfTheIlysianGrove : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dryad of the Ilysian Grove", cost := some [generic 2, pip .green],
      types := [.enchantment, .creature], subtypes := [nymph, dryad],
      text :=
        [ static
            (can you (play (counted (additional (exactly 1)) isLand))
               (timing := some onEachOfYourTurns)),
          static (qualityChange (allOf (and [isLand, youControl])) .adds
            (everyTypeOf .basicLand)) ],
      power := stat 2, toughness := stat 4 } }

/-- Ashes of the Fallen -/
def ashesOfTheFallen : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ashes of the Fallen", cost := some [generic 2], types := [.artifact],
      text :=
        [ static (entersChoosing thisArtifact (.subtype .creature)),
          static (qualityChange (each (and [isCreature, in_ yourGraveyard])) .adds
            (chosenQuality (ofChosen (.subtype .creature)))) ] } }

def hellkiteCharger : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hellkite Charger", cost := some [generic 4, pip .red, pip .red],
      types := [.creature], subtypes := [dragon],
      text :=
        [ flying, haste,
          whenever (does thisCreature attack)
            (may (pay (mana [generic 5, pip .red, pip .red]) .once)
              (some (sequentially
                [ untap (allOf (and [isCreature, isAttacking])),
                  additionalPhase .combat none (.lit 1) ]))) ],
      power := stat 5, toughness := stat 5 } }

/-- Foriysian Brigade -/
def foriysianBrigade : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Foriysian Brigade", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [human, soldier],
      text :=
          [static
             (can thisCreature (block (counted (additional (exactly 1)) isCreature))
                (timing := some eachCombat))],
      power := stat 2, toughness := stat 4 } }

/-- Two-Headed Giant of Foriys -/
def twoHeadedGiantOfForiys : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Two-Headed Giant of Foriys", cost := some [generic 4, pip .red],
      types := [.creature], subtypes := [giant],
      text := [trample, static
                 (can thisCreature (block (counted (additional (exactly 1)) isCreature))
                    (timing := some eachCombat))],
      power := stat 4, toughness := stat 4 } }

def highGround : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "High Ground", cost := some [pip .white], types := [.enchantment],
      text :=
          [static
             (can (each (and [isCreature, youControl]))
                (block (counted (additional (exactly 1)) isCreature))
          (timing := some eachCombat))] } }

/-- Watcher in the Web -/
def watcherInTheWeb : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Watcher in the Web", cost := some [generic 4, pip .green], types := [.creature],
      subtypes := [spider],
      text := [reach, static
                 (can thisCreature (block (counted (additional (exactly 7)) isCreature))
                    (timing := some eachCombat))],
      power := stat 2, toughness := stat 5 } }

/-- Forgotten Lore -/
def forgottenLoreRepeat : Action :=
  sequentially
    [ instruct (target isOpponent) (choose (a (cardIn yourGraveyard))),
      may (pay (mana [pip .green]) .once) (some (repeat_
          againExcludingChosen)) ]
theorem okForgottenLoreRepeat : Action.check (DRS.ofList []) forgottenLoreRepeat = [] := by decide

/-- Leyline of the Meek -/
def leylineOfTheMeek : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Leyline of the Meek", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ mayBeginOnBattlefield,
          static (getsPt (allOf (and [isCreature, isToken])) (up (.lit 1)) (up (.lit 1))) ] } }

/-- Leyline of Vitality -/
def leylineOfVitality : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Leyline of Vitality", cost := some [generic 2, pip .green, pip .green],
      types := [.enchantment],
      text :=
        [ mayBeginOnBattlefield,
          static (getsPt (allOf (and [isCreature, youControl])) (up (.lit 0)) (up (.lit 1))),
          whenever (does (a (and [isCreature, youControl])) enter) (may (gainLife (.lit 1))) ] } }

def phyrexianRevoker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Phyrexian Revoker", cost := some [generic 2], types := [.artifact, .creature],
      subtypes := [phyrexian, horror],
      text :=
        [ static (entersChoosingFrom thisCreature .cardName (nameOfCard (non isLand))),
          static (cant any (activate (allOf (and [ abilityHead .anyActivated,
                           abilityOf (allOf (and [isSource, named chosenName])) ])))) ],
      power := stat 2, toughness := stat 1 } }

def voidstoneGargoyle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Voidstone Gargoyle", cost := some [generic 3, pip .white, pip .white],
      types := [.creature], subtypes := [gargoyle],
      text :=
        [ flying,
          static (entersChoosingFrom thisCreature .cardName (nameOfCard (non isLand))),
          static (cant any (cast (allOf (and [isSpell, named chosenName])))),
          static (cant any (activate (allOf (and [ abilityHead .anyActivated,
                           abilityOf (allOf (and [isSource, named chosenName])) ])))) ],
      power := stat 3, toughness := stat 3 } }

def pithingNeedle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pithing Needle", cost := some [generic 1], types := [.artifact],
      text :=
        [ static (entersChoosing thisArtifact .cardName),
          static (cant any (activate (allOf (and [ abilityHead .anyActivated,
                           abilityOf (allOf (and [isSource, named chosenName])),
                           not isManaAbility ])))) ] } }

/-- Rhystic Study -/
def rhysticStudy : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rhystic Study", cost := some [generic 2, pip .blue], types := [.enchantment],
      text :=
        [ whenever (does anOpponent (cast (a isSpell)))
            (instruct (that player) (doUnless (instruct you (may (draw (.lit 1))))
              (mana [generic 1]))) ] } }

def gandalfWhiteRider : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gandalf, White Rider", cost := some [generic 3, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [avatar, wizard],
      text :=
        [ vigilance,
          whenever (does you (cast (a isSpell)))
            (sequentially
              [ get (each (and [isCreature, youControl])) (up (.lit 1)) (up (.lit 0)) (some untilEndOfTurn),
                scry (.lit 1) ]),
          when (does thisCreature die) (may (putInto it (fromTop 5 library))) ],
      power := stat 3, toughness := stat 3 } }

def helmOfPossession : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Helm of Possession", cost := some [generic 4], types := [.artifact],
      text :=
        [ static (mayDeclineUntap thisArtifact (some you)),
          activated (compound [mana [generic 2], tapSymbol, perform (sacrifice (a isCreature))])
            (gainControl (target isCreature)
              (some (forAsLongAs
                (and [ is thisArtifact youControl,
                        is thisArtifact isTapped ]))) (controller := you)) ] } }

def override : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Override", cost := some [generic 2, pip .blue], types := [.instant],
      text :=
        [ spell (instruct (controllerOf (target isSpell)) (doUnless (counter it)
            (scaledGenericMana (forEach (.lit 1) (and [isArtifact, youControl]))))) ] } }

def rakshasasDisdain : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rakshasa's Disdain", cost := some [generic 2, pip .blue], types := [.instant],
      text :=
        [ spell (instruct (controllerOf (target isSpell)) (doUnless (counter it)
            (scaledGenericMana (forEach (.lit 1) (cardIn yourGraveyard))))) ] } }

def megatherium : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Megatherium", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [beast],
      text :=
        [ trample,
          when (does thisCreature enter)
            (doUnless (sacrifice thisCreature)
              (scaledGenericMana (forEach (.lit 1) (cardIn yourHand)))) ],
      power := stat 4, toughness := stat 4 } }

def killingWave : Action :=
  doForEach (each isCreature)
    (instruct (controllerOf it) (doUnless (instruct they (sacrifice it)) (perform (instruct they (loseLife (letter .x))))))
theorem okKillingWave : Action.check (DRS.ofList []) killingWave = [] := by decide
def fadeAway : Action :=
  doForEach (each isCreature)
    (instruct (controllerOf it) (doUnless (instruct they (sacrifice (a isPermanent))) (mana [generic 1])))
theorem okFadeAway : Action.check (DRS.ofList []) fadeAway = [] := by decide

def tidalFlats : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tidal Flats", cost := some [pip .blue], types := [.enchantment],
      text :=
        [ activated (mana [pip .blue, pip .blue])
            (doForEach (each (and [isCreature, isAttacking, not (hasKeyword flying)]))
              (instruct (controllerOf it) (doUnless (gain
                  (allOf (and [ isCreature, youControl,
                                 inCombat .blockerOf (some (that creature)) ]))
                  firstStrike (some untilEndOfTurn)) (mana [generic 1])))) ] }
                      }

def primalSurge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Primal Surge", cost := some [generic 8, pip .green, pip .green], types := [.sorcery],
      text :=
        [ spell (sequentially
            [ exile (topSlice (.lit 1)),
              doIf (is it isPermanentCard)
                (may (putOntoBattlefield it) (some (repeat_ again)))
                    ]) ] } }

def cultivatorColossus : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cultivator Colossus", cost := some [generic 4, pip .green, pip .green, pip .green],
      types := [.creature], subtypes := [plant, beast],
      text :=
        [ trample,
          static (ptDefinition thisCreature .bothEach
            (countOf (and [isLand, youControl]))),
          when (does thisCreature enter)
            (may (putOntoBattlefieldTapped (a (and [isLand, in_ yourHand])))
              (some (sequentially [draw (.lit 1), repeat_ again]))) ] } }

def zimoneAndDina : Ability :=
  activated
      (compound [tapSymbol, perform (sacrifice (a (and [isCreature, otherThan thisCreature])))])
    (sequentially
      [ draw (.lit 1),
        may (putOntoBattlefieldTapped (a (and [isLand, in_ yourHand]))),
        doIf (compareAmt (countOf (and [isLand, youControl])) .atLeast (.lit
            8))
          (repeat_ (moreTimes (.lit 1))) ])
theorem okZimoneAndDina : Ability.check (DRS.ofList []) zimoneAndDina = [] := by decide

def cryptLurker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crypt Lurker", cost := some [generic 3, pip .black], types := [.creature],
      subtypes := [horror],
      text :=
        [ when (does thisCreature enter)
            (may
              (chooseOne (sacrifice (a isCreature)) (discard (a (and [isCreature, in_ yourHand]))))
              (some (draw (.lit 1)))) ],
      power := stat 3, toughness := stat 4 } }

/-- Memoricide -/
def memoricideSearch : Action :=
  sequentially
    [ choose (a (qualityFrom .cardName (nameOfCard (non isLand)))),
      searchZonesOf (target isPlayer) (counted anyNumber (named chosenName)),
      instruct (that player) shuffle ]
theorem okMemoricideSearch : Action.check (DRS.ofList []) memoricideSearch = [] := by decide
/-- Lost Hours -/
def lostHoursPlacement : Action :=
  sequentially
    [ instruct (target isPlayer) revealHand,
      choose (a (and [non isLand, in_ (handOf they)])),
      instruct (that player) (put it (fromTop 3 library)) ]
theorem okLostHoursPlacement : Action.check (DRS.ofList []) lostHoursPlacement = [] := by decide
/-- Aether Gust -/
def aetherGustPlacement : Action :=
  sequentially
    [ choose (target (and [isPermanent, hasColor .red])),
      instruct (ownerOf it) (put it (or [onTop, onBottom])) ]
theorem okAetherGustPlacement : Action.check (DRS.ofList []) aetherGustPlacement = [] := by decide
/-- Fastbond -/
def fastbondLands : Ability := static (can you (play (counted (additional anyNumber) isLand)))
theorem okFastbondLands : Ability.check (DRS.ofList []) fastbondLands = [] := by decide
/-- Panglacial Wurm -/
def panglacialWurmCast : Ability :=
  static
      (can you (cast this_ (from_ := some yourLibrary)) (timing := some whileSearchingYourLibrary))
theorem okPanglacialWurmCast : Ability.check (DRS.ofList []) panglacialWurmCast = [] := by decide

/-- Shah of Naar Isle -/
def shahOfNaarIsle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shah of Naar Isle", cost := some [generic 3, pip .red], types := [.creature],
      subtypes := [efreet],
      text :=
        [ trample,
          echo (mana [generic 0]),
          when (does any (pay anyCost .once .paid (some thisCreature) (some echo)))
            (instruct eachOpponent (may (drawCount (upToOf (.lit 3))))) ],
      power := stat 6, toughness := stat 6 } }

/-- Memory Plunder -/
def memoryPlunder : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Memory Plunder",
      cost := some [hybridPip .blue .black, hybridPip .blue .black, hybridPip .blue .black,
                    hybridPip .blue .black],
      types := [.instant],
      text :=
        [ spell (establish
            (can you
               (cast (target (and [isInstantOrSorcery, isCard]))
                  (from_ := some (graveyardOf anOpponent)) (payment := withoutPaying)))) ] } }

/-- Omniscience -/
def omniscience : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Omniscience", cost := some [generic 7, pip .blue, pip .blue, pip .blue],
      types := [.enchantment],
      text :=
        [ static
            (can you
               (cast (allOf isSpell) (from_ := some yourHand) (payment := withoutPaying))) ] } }

/-- Tranquil Frillback -/
def tranquilFrillbackOffer : Action := may (pay (mana [pip .green]) (.upTo 3))
theorem okTranquilFrillbackOffer : Action.check (DRS.ofList []) tranquilFrillbackOffer = [] := by decide

/-- Caller of the Hunt -/
def callerOfTheHunt : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Caller of the Hunt", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [human],
      text :=
        [ static (addedCost (perform (choose (a (quality (.subtype .creature))))) false),
          static (ptDefinition thisCreature .bothEach
            (countOf (and [isCreature, ofChosen (.subtype .creature)]))) ] } }

/-- Duneblast -/
def duneblast : Action :=
  sequentially [choose (counted (upTo 1) isCreature), destroy (theRest .object)]
theorem okDuneblast : Action.check (DRS.ofList []) duneblast = [] := by decide
/-- Boreas Charger -/
def boreasChargerChoice : Action := choose (a opponentWithMoreLands)
theorem okBoreasChargerChoice : Action.check (DRS.ofList []) boreasChargerChoice = [] := by decide
/-- Boreas Charger's spell text -/
def boreasChargerSpell : Action :=
  sequentially
    [ choose (a opponentWithMoreLands),
      search yourLibrary (counted (exactlyOf theDifference) (hasSubtype plains)),
      reveal (those card),
      putOntoBattlefieldTapped (someOf (exactly 1) (those card)),
      putInto (theRest .object) hand ]
theorem okBoreasChargerSpell : Action.check (DRS.ofList []) boreasChargerSpell = [] := by decide

/-- Sandstone Oracle -/
def sandstoneOracle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sandstone Oracle", cost := some [generic 7], types := [.artifact, .creature],
      subtypes := [sphinx],
      text :=
        [ flying,
          when (does thisCreature enter)
            (sequentially
              [ choose anOpponent,
                doIf (compareAmt (countOf (cardIn (handOf (that player)))) .greater
                        (countOf (cardIn yourHand)))
                  (draw theDifference) ]) ],
      power := stat 4, toughness := stat 4 } }

/-- Slithermuse -/
def slithermuseTrigger : Ability :=
  when (does thisCreature leave)
    (sequentially
      [ choose anOpponent,
        doIf (compareAmt (countOf (cardIn (handOf (that player)))) .greater
                (countOf (cardIn yourHand)))
          (draw theDifference) ])
theorem okSlithermuseTrigger : Ability.check (DRS.ofList []) slithermuseTrigger = [] := by decide

/-- Celestial Judgment -/
def celestialJudgment : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Celestial Judgment", cost := some [generic 4, pip .white, pip .white],
      types := [.sorcery],
      text :=
        [ spell (sequentially
            [ doForEachKind (.value .power) .number (choose (a (and [isCreature, hasPower .eq chosenNumber]))) (some (allOf isCreature)),
              destroy (each (and [isCreature, notChosen])) ]) ] } }

/-- World Queller -/
def worldQuellerChoice : Action :=
  may (choose (a (quality .cardType)))
    (some (instruct eachPlayer (sacrifice (aTheirChoice (and [isPermanent, ofChosen .cardType])))))
theorem okWorldQuellerChoice : Action.check (DRS.ofList []) worldQuellerChoice = [] := by decide

/-- Moonlit Meditation -/
def moonlitMeditation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Moonlit Meditation", cost := some [generic 2, pip .blue], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant (and [or [isArtifact, isCreature], youControl]),
          static (replace
                    (would (by_ you)
                       (createObject anyAmount (token (copyOf (counted (atLeast 1) isToken)) [])
                          false)) (instead (may
                                     (create groupSize (copyOf (enchanted permanent))
                                        []))) none .repeatedly (some oncePerTurn)) ] } }

def discardUpToTwoThenDrawThatMany : Action :=
  sequentially [discard (counted (upTo 2) (cardIn yourHand)), draw groupSize]
theorem okDiscardUpToTwoThenDrawThatMany :
    Action.check (DRS.ofList []) discardUpToTwoThenDrawThatMany = [] := by decide
/-- Truce and Temporary Truce -/
def drawUpToTwoThenGainPerShortfall : Action :=
  instruct eachPlayer
    (sequentially
      [ may (drawCount (upToOf (.lit 2))),
        instruct (that player) (gainLife (times (.lit 2) shortOfCeiling)) ])
theorem okDrawUpToTwoThenGainPerShortfall :
    Action.check (DRS.ofList []) drawUpToTwoThenGainPerShortfall = [] := by decide

/-- Commune with the Gods -/
def communeWithTheGods : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Commune with the Gods", cost := some [generic 1, pip .green], types := [.sorcery],
      text :=
        [ spell (sequentially
            [ reveal (topSlice (.lit 5)),
              may (putInto (fromAmong (exactly 1) (or [isCreature, isEnchantment]) them) hand),
              putInto (theRest .object) graveyard ]) ] } }

/-- Nautiloid Ship -/
def nautiloidShipTrigger : Ability :=
  whenever (does thisVehicle (dealDamage (to := a isPlayer) (kind := some .combat)))
    (may (putOntoBattlefieldUnderYourControl (a (and [isCreature, in_ (exiledWith thisVehicle)]))))
theorem okNautiloidShipTrigger : Ability.check (DRS.ofList []) nautiloidShipTrigger = [] := by decide

/-- Summon: Esper Valigarmanda's II/III/IV body, in part -/
def summonEsperValigarmandaCast : StaticSpec :=
  conjunction [ can you (cast (a (and [isInstantOrSorcery, in_ (exiledWith thisSaga)]))),
      can you (pay anyCost .once)
          (asThough := some
             (spendManaAs none .anyType
                (some (toCast (and [isInstantOrSorcery, in_ (exiledWith thisSaga)]))))) ]
theorem okSummonEsperValigarmandaCast :
    StaticSpec.check (DRS.ofList []) summonEsperValigarmandaCast = [] := by decide
/-- Rogue Class's level-3 body -/
def rogueClassLevelThree : StaticSpec :=
  conjunction [ can you (play (allOf (in_ (exiledWith thisClass)))),
      can you (pay anyCost .once)
          (asThough := some
             (spendManaAs none .anyColor (some (toCast (in_ (exiledWith thisClass)))))) ]
theorem okRogueClassLevelThree : StaticSpec.check (DRS.ofList []) rogueClassLevelThree = [] := by decide

/-- Soul Ransom -/
def soulRansom : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Soul Ransom", cost := some [generic 2, pip .blue, pip .black], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isCreature,
          static (controlGrant you (enchanted creature)),
          activatedBy
            (perform (repeatTimes (.lit 2)
              (sequentially [choose (a (cardIn hand)), discard (that card)])))
            (sequentially [instruct (controllerOf thisAura) (sacrifice itPermanent), instruct they (draw (.lit 2))])
            yourOpponents ] } }

/-- Vraska's Scorn -/
def vraskasScorn : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vraska's Scorn", cost := some [generic 2, pip .black, pip .black], types := [.sorcery],
      text :=
        [ spell (sequentially
            [ instruct (target isOpponent) (loseLife (.lit 4)),
              may
                (sequentially
                  [ searchLibraryOrGraveyard (a (named "Vraska, Scheming Gorgon")),
                    reveal it, putInto it hand ]),
              doIf (did you (search yourLibrary) thisWay) shuffle ]) ] } }

/-- Old-Growth Dryads -/
def oldGrowthDryads : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Old-Growth Dryads", cost := some [pip .green], types := [.creature],
      subtypes := [dryad],
      text :=
        [ when (does thisCreature enter)
            (instruct eachOpponent (may
              (sequentially
                [ search (libraryOf they) (a (and [isLand, hasSupertype .basic])),
                  putOntoBattlefieldTapped it,
                  shuffle ]))) ],
      power := stat 3, toughness := stat 3 } }

/-- Verity Circle -/
def verityCircle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Verity Circle", cost := some [generic 2, pip .blue], types := [.enchantment],
      text :=
        [ whenever
            (does (a (and [isCreature, anOpponentControls])) (become tapped))
            (if_ := some (is it (not (inCombat .declaredAttacker))))
            (may (draw (.lit 1))),
          activated (mana [generic 4, pip .blue])
            (tap (target (and [isCreature, not (hasKeyword flying)]))) ] } }

/-- Danitha, New Benalia's Light -/
def danithaNewBenaliasLight : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Danitha, New Benalia's Light", cost := some [generic 1, pip .green, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [human, knight],
      text :=
        [ vigilance, trample, lifelink,
          static (can you (cast (counted (exactly 1) (and [ isSpell, or [ hasSubtype aura,
                                    hasSubtype equipment ] ])) (from_ := some
                                                                  yourGraveyard)) (timing := some
                                                                                     onEachOfYourTurns)) ],
      power := stat 2, toughness := stat 2 } }

/-- Muldrotha, the Gravetide -/
def muldrothaLandWindow : StaticSpec :=
  can you (play (a isLand) (from_ := some yourGraveyard)) (timing := some onEachOfYourTurns)
theorem okMuldrothaLandWindow : StaticSpec.check (DRS.ofList []) muldrothaLandWindow = [] := by decide
/-- Nahiri's Lithoforming -/
def nahiriExtraLands : StaticSpec := can you
    (play (counted (additional (exactlyOf (letter .x))) isLand))
theorem okNahiriExtraLands : StaticSpec.check (DRS.ofList [letterB .x]) nahiriExtraLands = [] := by decide

/-- Haakon, Stromgald Scourge -/
def haakonStromgaldScourge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Haakon, Stromgald Scourge", cost := some [generic 1, pip .black, pip .black],
      supertypes := [.legendary], types := [.creature],
      subtypes := [zombie, knight],
      text :=
        [ static (can you (cast this_ (from_ := some yourGraveyard))),
          static (cant you (cast this_ (from_ := some (not yourGraveyard)))),
          static (onlyWhile
                    (can you
                       (cast (allOf (and [isSpell, hasSubtype knight]))
                          (from_ := some yourGraveyard))) (is this_ (in_ battlefield))),
          when (does thisCreature die) (loseLife (.lit 2)) ],
      power := stat 3, toughness := stat 3 } }

/-- Apex of Power -/
def apexOfPowerCast : Action :=
  sequentially
    [ exile (topSlice (.lit 7)),
      establishThroughout (can you (cast (fromAmong anyNumber isSpell them))) thisTurn ]
theorem okApexOfPowerCast : Action.check (DRS.ofList []) apexOfPowerCast = [] := by decide
/-- Blight Herder -/
def blightHerderCast : Action :=
  may
    (putInto (counted (exactly 2)
            (and [isCard, opponentsOwn, in_ exileZone]))
      graveyard)
theorem okBlightHerderCast : Action.check (DRS.ofList []) blightHerderCast = [] := by decide

/-- True-Name Nemesis -/
def trueNameNemesis : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "True-Name Nemesis", cost := some [generic 1, pip .blue, pip .blue],
      types := [.creature], subtypes := [merfolk, rogue],
      text :=
        [ static (entersChoosingPlayer thisCreature (a isPlayer)),
          protection isChosenPlayer ],
      power := stat 3, toughness := stat 1 } }

/-- Akiri, Fearless Voyager -/
def akiriUnattachOffer : Ability :=
  activated (mana [pip .white])
    (may
      (unattach (a (and [ hasSubtype equipment,
                            attachedTo (a (and [isCreature, youControl])) ])))
      (some (tap (that creature))))
theorem okAkiriUnattachOffer : Ability.check (DRS.ofList []) akiriUnattachOffer = [] := by decide
/-- Summoning Materia -/
def summoningMateriaTopCast : Ability :=
  static (onlyWhile (castFromTop (and [isSpell, isCreature]))
    (is thisEquipment (attachedTo (a isCreature))))
theorem okSummoningMateriaTopCast : Ability.check (DRS.ofList []) summoningMateriaTopCast = [] := by decide
/-- Vizier of the Menagerie -/
def vizierOfTheMenagerieSpend : StaticSpec :=
  can you (pay anyCost .once)
      (asThough := some (spendManaAs none .anyType (some (toCast (and [isCreature, isSpell])))))
theorem okVizierOfTheMenagerieSpend : StaticSpec.check (DRS.ofList []) vizierOfTheMenagerieSpend = [] := by
  decide

/-- Conspicuous Snoop -/
def conspicuousSnoop : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Conspicuous Snoop", cost := some [pip .red, pip .red], types := [.creature],
      subtypes := [goblin, rogue],
      text :=
        [ static (visibility .reveal you topOfLibrary),
          static (can you
                    (play (allOf (and [isSpell, hasSubtype goblin])) (from_ := some onTop))),
          static (onlyWhile
            (abilityGrantFrom thisCreature (topSlice (.lit 1)) [.anyActivated])
            (is (topSlice (.lit 1)) (hasSubtype goblin))) ],
      power := stat 2, toughness := stat 2 } }

/-- Ballot Broker -/
def ballotBroker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ballot Broker", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [human, advisor],
      text := [static (can you (vote (times := some (additional (exactly 1)))))],
      power := stat 2, toughness := stat 3 } }

/-- Emissary of Grudges -/
def emissaryOfGrudgesReveal : Ability :=
  activatedOnlyOnce (perform (expose .reveal (choiceOf .player)))
    (doOnlyIf (chooseNewTargets (target (or [isSpell, abilityHead .anyOnStack])))
      (and
        [ is it (controlledBy (the isChosenPlayer)),
          is it
            (targeting (youOr (a (and [isPermanent, youControl]))) .someTarget) ]))
    oncePerGame
theorem okEmissaryOfGrudgesReveal :
    Ability.check (DRS.ofList [ChoiceSort.binding .player]) emissaryOfGrudgesReveal = [] := by decide

def prosperity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Prosperity", cost := some [.variable, pip .blue], types := [.sorcery],
      text := [spell (instruct eachPlayer (draw (letter .x)))] } }

def collectiveUnconscious : Action := draw (forEach (.lit 1) (and [isCreature, youControl]))
theorem okCollectiveUnconscious : Action.check (DRS.ofList []) collectiveUnconscious = [] := by decide
def killiansConfidence : Action :=
  sequentially
    [get (target isCreature) (up (.lit 1)) (up (.lit 1)) (some untilEndOfTurn), draw (.lit 1)]
theorem okKilliansConfidence : Action.check (DRS.ofList []) killiansConfidence = [] := by decide

def scatheZombies : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Scathe Zombies", cost := some [generic 2, pip .black], types := [.creature],
      subtypes := [zombie], power := stat 2, toughness := stat 2 } }

def plainsCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Plains", supertypes := [.basic], types := [.land], subtypes := [plains] } }

def islandCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Island", supertypes := [.basic], types := [.land], subtypes := [island] } }

def swampCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Swamp", supertypes := [.basic], types := [.land], subtypes := [swamp] } }

def mountainCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mountain", supertypes := [.basic], types := [.land], subtypes := [mountain] } }

def forestCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Forest", supertypes := [.basic], types := [.land], subtypes := [forest] } }

def snowCoveredForest : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Snow-Covered Forest", supertypes := [.basic, .snow], types := [.land],
      subtypes := [forest] } }

/-- Not Forgotten -/
def notForgottenPlacement : Action :=
  putInto (target (cardIn graveyard)) (or [onTop, onBottom])
theorem okNotForgottenPlacement : Action.check (DRS.ofList []) notForgottenPlacement = [] := by decide

def duneblastCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Duneblast", cost := some [generic 4, pip .white, pip .black, pip .green],
      types := [.sorcery], text := [spell duneblast] } }

def boreasChargerDifference : Amount := theDifference
theorem okBoreasChargerDifference :
    Amount.check ((stepAction 512 .postEvent .obj (DRS.ofList []) boreasChargerChoice).1) boreasChargerDifference = [] := by decide

/-- Hymn to Tourach -/
def hymnToTourach : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hymn to Tourach", cost := some [pip .black, pip .black], types := [.sorcery],
      text :=
        [ spell (discard (countedAtRandom (exactly 2) (cardIn (handOf (target
            isPlayer))))) ] } }

/-- Caught in the Crossfire -/
def caughtInTheCrossfire : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Caught in the Crossfire", cost := some [pip .red, pip .red], types := [.instant],
      text :=
        [ spell (chooseSpree
            [ (some (mana [generic 1]),
               instruct this_ (dealDamage (.lit 2) (each (and [isCreature, isOutlaw])))),
              (some (mana [generic 1]),
               instruct this_
                   (dealDamage (.lit 2) (each (and [isCreature, non isOutlaw])))) ]) ] } }

/-- Requisition Raid -/
def requisitionRaid : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Requisition Raid", cost := some [pip .white], types := [.sorcery],
      text :=
        [ spell (chooseSpree
            [ (some (mana [generic 1]), destroy (target isArtifact)),
              (some (mana [generic 1]), destroy (target isEnchantment)),
              (some (mana [generic 1]),
               putCounters (.lit 1) p1p1Counter
                 (each (and [isCreature, controlledBy (target isPlayer)])) .many) ]) ] } }

/-- Rustler Rampage -/
def rustlerRampage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rustler Rampage", cost := some [pip .white], types := [.instant],
      text :=
        [ spell (chooseSpree
            [ (some (mana [generic 1]),
               untap
                 (allOf (and [isCreature, controlledBy (target isPlayer)]))),
              (some (mana [generic 1]),
               gain (target isCreature) doubleStrike (some untilEndOfTurn)) ]) ] } }

/-- Consuming Tide -/
def consumingTide : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Consuming Tide", cost := some [generic 2, pip .blue, pip .blue], types := [.sorcery],
      text :=
        [ spell (sequentially
            [ instruct eachPlayer (choose
                (a (and [isPermanent, non isLand, controlledBy they]))),
              returnToHand (allOf (and [isPermanent, non isLand, notChosen])),
              doForEach
                (each (compareOver isOpponent (countOf (cardIn (handOf they))) .greater
                  (countOf (cardIn yourHand))))
                (draw (.lit 1)) ]) ] } }

/-- Stick Together: a chosen party is four up-to-one choices, not the group the game computes
[CR#700.8d]. -/
def stickTogether : Action :=
  sequentially
    [ instruct eachPlayer (choose
        (counted (upTo 1)
          (and [isCreature, hasSubtype cleric, controlledBy they]))),
      instruct eachPlayer (choose
        (counted (upTo 1)
          (and [isCreature, hasSubtype rogue, controlledBy they]))),
      instruct eachPlayer (choose
        (counted (upTo 1)
          (and [isCreature, hasSubtype warrior, controlledBy they]))),
      instruct eachPlayer (choose
        (counted (upTo 1)
          (and [isCreature, hasSubtype wizard, controlledBy they]))),
      instruct eachPlayer (sacrifice (theRest .object)) ]
theorem okStickTogether : Action.check (DRS.ofList []) stickTogether = [] := by decide

/-- Disciple of Caelus Nin: "starting with you" overrides the default APNAP choice order
[CR#101.1,101.4]. -/
def discipleOfCaelusNin : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Disciple of Caelus Nin", cost := some [generic 4, pip .white], types := [.creature],
      subtypes := [human, wizard],
      text :=
        [ when (does thisCreature enter)
            (sequentially
              [ instruct eachPlayer
                  (chooseStartingWith you
                     (counted (upTo 5) (and [isPermanent, controlledBy they]))),
                instruct (allOf (and [isPermanent, otherThan thisCreature, notChosen]))
                  phaseOut ]),
          static (cant (bare isPermanent) phaseIn) ],
      power := stat 3, toughness := stat 4 } }

/-- Sculpted Sunburst: "if you chose a creature this way" reads the earlier choice;
"not chosen by any player this way" excludes every player's chosen creature. -/
def sculptedSunburst : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sculpted Sunburst", cost := some [generic 3, pip .white, pip .white],
      types := [.sorcery],
      text :=
        [ spell (sequentially
            [ choose (a (and [isCreature, youControl])),
              instruct eachOpponent (choose
                (a (comparesOwnStat .power (and [isCreature, controlledBy they])
                  .atMost (powerOf it)))),
              doIf (choseThisWay you isCreature) (exile (each (and [isCreature, notChosen])))
                  ]) ] } }

end Semantics.Cards
