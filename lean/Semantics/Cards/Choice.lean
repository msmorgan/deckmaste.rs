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
  sequentially [draw 2, discard (counted (exactly 2) (cardIn yourHand))]
theorem okCarefulStudy : Action.check (DRS.ofList []) carefulStudy = [] := by decide

def zombieInfestation : Ability :=
  activated (discard (counted (exactly 2) (cardIn yourHand)))
    (createToken (creatureToken 2 2 [.black] [zombie]))
theorem okZombieInfestation : Ability.check (DRS.ofList []) zombieInfestation = [] := by decide

def fulgentDistraction : Action :=
  sequentially
    [ choose (targets (exactly 2) isCreature),
      tap (those creature),
      unattach (allOf (and [ equipment,
                               attachedTo (those creature) ])) ]
theorem okFulgentDistraction : Action.check (DRS.ofList []) fulgentDistraction = [] := by decide

/-- "Choose target creature you control and target creature you don't control. … those
creatures": the fight-and-bite opening (Blizzard Brawl, Joust, Tail Swipe, Malamet Battle Glyph).
The targets are chosen as the spell is cast [CR#601.2c]; "those creatures" reads both. -/
def chooseCreatureYouControlAndCreatureYouDont : Action :=
  sequentially
    [ choose (both (target (and [isCreature, youControl]))
                   (target (and [isCreature, youDontControl]))),
      tap (those creature) ]
theorem okChooseCreatureYouControlAndCreatureYouDont :
    Action.check (DRS.ofList []) chooseCreatureYouControlAndCreatureYouDont = [] := by decide

/-- The opening the fight-and-bite cards share: "Choose target creature you control and target
creature you don't control." -/
def chooseFightPair : Action :=
  choose (both (target (and [isCreature, youControl])) (target (and [isCreature, youDontControl])))

/-- Blizzard Brawl: "Choose target creature you control and target creature you don't control.
If you control three or more snow permanents, the creature you control gets +1/+0 and gains
indestructible until end of turn. Then those creatures fight each other." "The creature you
control" reads back the first target, which its restriction picks out of the two; "those
creatures fight each other" is the two-creature fight of [CR#701.14a]. -/
def blizzardBrawl : Spelled := spelled <| singleFacedCard
  { name := "Blizzard Brawl", cost := [pip .green], supertypes := [.snow], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ chooseFightPair,
            doIf (compareAmt (countOf (and [hasSupertype .snow, isPermanent, youControl]))
                .atLeast 3)
              (establishFor (the (and [isCreature, youControl]))
                [getsPt it (up 1) (up 0), abilityGrant it indestructible] untilEndOfTurn),
            fightEachOther (those creature) ]) ] }

/-- Tail Swipe: "Choose target creature you control and target creature you don't control. If
you cast this spell during your main phase, the creature you control gets +1/+1 until end of
turn. Then those creatures fight each other." A step in which a spell can be cast ends only with
the stack empty [CR#500.2], so the spell resolves in the turn it was cast, and its casting is one
this turn. -/
def tailSwipe : Spelled := spelled <| singleFacedCard
  { name := "Tail Swipe", cost := [pip .green], types := [.instant],
    text :=
      [ spell (sequentially
          [ chooseFightPair,
            doIf (did you (cast thisSpell)
                (within soFarThisTurn (some (duringPart .mainPhase (some you)))))
              (get (the (and [isCreature, youControl])) (up 1) (up 1) untilEndOfTurn),
            fightEachOther (those creature) ]) ] }

/-- Malamet Battle Glyph: "Choose target creature you control and target creature you don't
control. If the creature you control entered this turn, put a +1/+1 counter on it. Then those
creatures fight each other." -/
def malametBattleGlyph : Spelled := spelled <| singleFacedCard
  { name := "Malamet Battle Glyph", cost := [pip .green], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ chooseFightPair,
            doIf (did (the (and [isCreature, youControl])) enter soFarThisTurn)
              (putCounters 1 p1p1Counter it),
            fightEachOther (those creature) ]) ] }

/-- Joust: "Choose target creature you control and target creature you don't control. The
creature you control gets +2/+1 until end of turn if it's a Knight. Then those creatures fight
each other." The "if" is printed after its clause, so its "it" reads that clause's subject, "the
creature you control", though the condition is checked before the creature gets +2/+1
[CR#608.2c]. -/
def joust : Spelled := spelled <| singleFacedCard
  { name := "Joust", cost := [generic 1, pip .red], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ chooseFightPair,
            doOnlyIf (get (the (and [isCreature, youControl])) (up 2) (up 1) untilEndOfTurn)
              (is it (hasSubtype knight)),
            fightEachOther (those creature) ]) ] }
theorem okJoust : joust.card.check = [] := joust.ok

/-- Joust's postposed "if it's a Knight" reads the one creature its clause re-mentions, the
target you control, and not the target you don't control, which its clause does not mention.
The objects resolved are counted: "you", re-mentioned by each "you control", is resolved too
(`mentionYou`). -/
theorem joustConditionReadsTheClauseSubject :
    let out := (stepAction 16 .postEvent .obj (DRS.ofList []) (sequentially
      [ chooseFightPair,
        doOnlyIf (get (the (and [isCreature, youControl])) (up 2) (up 1) untilEndOfTurn)
          (is it (hasSubtype knight)) ])).1
    ((out.resolvedIds.filterMap out.findById?).filter (·.kind == .object)).map
      (·.controlledByYou) = [some true] := by
  decide

/-- "If it's a Knight, the creature you control gets +2/+1 until end of turn.": a preposed "if"
is read before its clause, so its "it" reads neither target over the other, and is refused. -/
theorem badPreposedConditionReadsNoLaterClauseSubject :
    Action.check (DRS.ofList []) (sequentially
      [ chooseFightPair,
        doIf (is it (hasSubtype knight))
          (get (the (and [isCreature, youControl])) (up 2) (up 1) untilEndOfTurn) ]) =
      [.anaphor .bare .one 2] := by decide

/-- Grim Contest: "Choose target creature you control and target creature an opponent controls.
Each of those creatures deals damage equal to its toughness to the other." "The other" is the
current member's counterpart in the group of two the targets make up
(`DRS.distributivePair?`). -/
def grimContest : Spelled := spelled <| singleFacedCard
  { name := "Grim Contest", cost := [generic 1, pip .black, pip .green], types := [.instant],
    text :=
      [ spell (sequentially
          [ choose (both (target (and [isCreature, youControl]))
                         (target (and [isCreature, anOpponentControls]))),
            instruct (eachOf (those creature))
              (dealDamage (toughnessOf it) (theOther .object)) ]) ] }

/-- Beastie Beatdown: "Choose target creature you control and target creature an opponent
controls. / Delirium — If there are four or more card types among cards in your graveyard, put
two +1/+1 counters on the creature you control. / The creature you control deals damage equal to
its power to the creature an opponent controls." Three paragraphs, three spell abilities
[CR#113.2c], of one spell: its targets are announced for the spell as a whole [CR#601.2c] and its
instructions are followed in the order written [CR#608.2c], so "the creature you control" and
"the creature an opponent controls" in the later two read back the first one's targets. -/
def beastieBeatdown : Spelled := spelled <| singleFacedCard
  { name := "Beastie Beatdown", cost := [pip .red, pip .green], types := [.sorcery],
    text :=
      [ spell (choose (both (target (and [isCreature, youControl]))
                            (target (and [isCreature, anOpponentControls])))),
        delirium (spell (doIf
          (compareAmt (distinctCount .cardType (allOf (cardIn yourGraveyard))) .atLeast 4)
          (putCounters 2 p1p1Counter (the (and [isCreature, youControl]))))),
        spell (dealDamageOwnPower (the (and [isCreature, youControl]))
          (the (and [isCreature, anOpponentControls]))) ] }
theorem okBeastieBeatdown : beastieBeatdown.card.check = [] := beastieBeatdown.ok

/-- Duel for Dominance: "Coven — Choose target creature you control and target creature you
don't control. If you control three or more creatures with different powers, put a +1/+1 counter
on the chosen creature you control. Then the chosen creatures fight each other." The targets are
chosen as the spell is cast [CR#601.2c]; "the chosen creature you control" reads back the first
of them, which its restriction picks out of the two, and "the chosen creatures" the pair, which
fight as in [CR#701.14a]. -/
def duelForDominance : Spelled := spelled <| singleFacedCard
  { name := "Duel for Dominance", cost := [generic 1, pip .green], types := [.instant],
    text :=
      [ coven (spell (sequentially
          [ chooseFightPair,
            doIf (compareAmt (distinctCount (.value .power) (allOf (and [isCreature, youControl])))
                .atLeast 3)
              (putCounters 1 p1p1Counter (the (and [isCreature, isChosen, youControl]))),
            fightEachOther (thePlural (and [isCreature, isChosen])) ])) ] }
theorem okDuelForDominance : duelForDominance.card.check = [] := duelForDominance.ok

/-- Stolen Uniform, through its third sentence: "Choose target creature you control and target
Equipment. Gain control of that Equipment until end of turn. Attach it to the chosen creature."
"The chosen creature" reads back the creature target, chosen as the spell is cast [CR#601.2c].
Its last sentence, "When you lose control of that Equipment this turn, if it's attached to a
creature you control, unattach it", waits on `lean-v3-lose-control-event`. -/
def stolenUniformOpening : Action :=
  sequentially
    [ choose (both (target (and [isCreature, youControl])) (target (hasSubtype equipment))),
      gainControl (that (.subtype equipment)) untilEndOfTurn,
      attachTo it (the (and [isCreature, isChosen])) ]
theorem okStolenUniformOpening : Action.check (DRS.ofList []) stolenUniformOpening = [] := by
  decide

/-- Green Sun's Twilight: "Reveal the top X plus one cards of your library. Choose a creature card
and/or a land card from among them. Put those cards into your hand and the rest on the bottom of
your library in a random order. If X is 5 or more, instead put the chosen cards onto the
battlefield or into your hand and the rest on the bottom of your library in a random order."
The cards are chosen as the effect applies [CR#608.2d]. A self-replacement [CR#614.15]: "the
chosen cards" reads back the cards the replaced action read, the ones chosen. -/
def greenSunsTwilight : Spelled := spelled <| singleFacedCard
  { name := "Green Sun's Twilight", cost := [.variable, pip .green], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ reveal (topSlice (plus (letter .x) 1)),
            choose (NounPhrase.andOr
              [ fromAmong (exactly 1) (and [isCreature, isCard]) them,
                fromAmong (exactly 1) (and [isLand, isCard]) them ]),
            replace
              (sequentially
                [ putInto (those .card) hand,
                  putInto (theRest .object) onBottom (riders := [inOrder .randomOrder]) ])
              (doIf (compareAmt (letter .x) .atLeast 5)
                (instead (sequentially
                  [ putInto (thePlural (and [isCard, isChosen]))
                      (Templates.Location.or [battlefield, hand]),
                    putInto (theRest .object) onBottom (riders := [inOrder .randomOrder]) ]))) ])
      ] }
theorem okGreenSunsTwilight : greenSunsTwilight.card.check = [] := greenSunsTwilight.ok

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
  may (sacrifice (a isArtifact)) (destroy (target (or [isArtifact,
      isCreature])))
theorem okDarettisMinusOne : Action.check (DRS.ofList []) darettisMinusOne = [] := by decide

def cheeringFanatic : Spelled := spelled <| singleFacedCard
  { name := "Cheering Fanatic", cost := [generic 1, pip .red], types := [.creature],
    subtypes := [goblin],
    text :=
      [ whenever (does thisCreature attack)
          (sequentially
            [ choose (a (quality .cardName)),
              establishThroughout (costShift (allOf (and [isSpell, ofChosen .cardName]))
                                     (less 1)) thisTurn ]) ],
    power := stat 2, toughness := stat 2 }

def rainOfThorns : Action :=
  chooseModes (orMore 1)
    [destroy (target isArtifact), destroy (target isEnchantment), destroy (target isLand)]
theorem okRainOfThorns : Action.check (DRS.ofList []) rainOfThorns = [] := by decide

/-- *Call Damage Control*: the header "Return those cards from your graveyard to your hand." is
the supplied body, written once; each bullet is the target it completes into a mode. -/
def callDamageControl : Spelled := spelled <| singleFacedCard
  { name := "Call Damage Control", cost := [generic 1, pip .green], types := [.sorcery],
    text :=
      [ spell
          (chooseModesOver (upTo 2)
            [ target (and [isArtifact, isCard]),
              target (and [isCreature, isCard]),
              target (and [isEnchantment, isCard]),
              target (and [isLand, isCard]) ]
            (fun cards => returnToHand cards yourGraveyard)) ] }

/-- *Wild Shape*: the header "Until end of turn, target creature you control has that base power
and toughness, becomes that creature type, and gains that ability." is the supplied body, written
once; each bullet is the creature shape it sets, as one set of the shape's characteristics: its
creature type, its base power and toughness [CR#613.4b] and its ability. -/
def wildShape : Spelled := spelled <| singleFacedCard
  { name := "Wild Shape", cost := [pip .green], types := [.instant],
    text :=
      [ spell
          (chooseModesOver (exactly 1)
            [ { characteristics :=
                  { subtypes := [turtle], text := [hexproof], power := stat 1,
                    toughness := stat 3 } },
              { characteristics :=
                  { subtypes := [spider], text := [reach], power := stat 1,
                    toughness := stat 5 } },
              { characteristics :=
                  { subtypes := [elephant], text := [trample], power := stat 3,
                    toughness := stat 3 } } ]
            (fun shape =>
              establish
                (qualityChange (target (and [isCreature, youControl])) .sets (bundle shape))
                untilEndOfTurn)) ] }

def rankleMasterOfPranks : Action :=
  chooseModes anyNumber
    [ sequentially [instruct eachPlayer (choose (a (cardIn hand))), discard them],
      sequentially [instruct eachPlayer (loseLife 1), instruct (those player)
                      (draw 1)],
      instruct eachPlayer (sacrifice (aTheirChoice isCreature)) ]
theorem okRankleMasterOfPranks : Action.check (DRS.ofList []) rankleMasterOfPranks = [] := by decide

def myrkulsEdict : Action :=
  sequentially [choose anOpponent, instruct (that player) (sacrifice (aTheirChoice isCreature))]
theorem okMyrkulsEdict : Action.check (DRS.ofList []) myrkulsEdict = [] := by decide
def moltingHarpy : Action := doUnless (sacrifice thisCreature) [generic
    2]
theorem okMoltingHarpy : Action.check (DRS.ofList []) moltingHarpy = [] := by decide
def carnophage : Action := doUnless (tap thisCreature) (payLife 1)
theorem okCarnophage : Action.check (DRS.ofList []) carnophage = [] := by decide
def solitaryConfinement : Action :=
  mayOrElse (discard (a (cardIn yourHand))) (sacrifice thisEnchantment)
theorem okSolitaryConfinement : Action.check (DRS.ofList []) solitaryConfinement = [] := by decide

def yasminKhan : Ability :=
  activated tapSymbol
    (sequentially
      [ exile (topSlice 1),
        establishThroughout (can you (play it)) untilYourNextEndStep ])
theorem okYasminKhan : Ability.check (DRS.ofList []) yasminKhan = [] := by decide

/-- Brazen Cannonade -/
def brazenCannonadePermission : Action :=
  sequentially
    [ exile (topSlice 1),
      establishThroughout (can you (play it)) (untilEvent
                                                         (endOf .combat you nextTime)) ]
theorem okBrazenCannonadePermission : Action.check (DRS.ofList []) brazenCannonadePermission = [] := by
  decide

def thousandMoonsCrackshot : Ability :=
  whenever (does thisCreature attack)
    (offerWhen (pay [generic 2, pip .white] .once)
      (tap (target isCreature)))
theorem okThousandMoonsCrackshot : Ability.check (DRS.ofList []) thousandMoonsCrackshot = [] := by decide

def zimoneQuandrixProdigy : Ability :=
  activated (compound [[generic 1], tapSymbol])
    (may (putOntoBattlefieldTapped (a (and [isLand, in_ yourHand]))))
theorem okZimoneQuandrixProdigy : Ability.check (DRS.ofList []) zimoneQuandrixProdigy = [] := by decide

def preeminentCaptain : Ability :=
  whenever (does thisCreature attack)
    (may (putOntoBattlefieldTappedAttacking
      (a (and [soldier, isCreature, in_ yourHand]))))
theorem okPreeminentCaptain : Ability.check (DRS.ofList []) preeminentCaptain = [] := by decide

def skaabRuinator : Ability :=
  static (can you (cast this_ (from_ := yourGraveyard)))
theorem okSkaabRuinator : Ability.check (DRS.ofList []) skaabRuinator = [] := by decide

/-- Raffine's Guidance -/
def raffinesGuidance : Spelled := spelled <| singleFacedCard
  { name := "Raffine's Guidance", cost := [pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (getsPt (enchanted creature) (up 1) (up 1)),
        static (can you
                  (cast this_ (from_ := yourGraveyard)
                     (payment := payingInstead [generic 2, pip .white]))) ] }

/-- Scourge of Nel Toth -/
def scourgeOfNelToth : Spelled := spelled <| singleFacedCard
  { name := "Scourge of Nel Toth", cost := [generic 5, pip .black, pip .black],
    types := [.creature], subtypes := [zombie, dragon],
    text :=
      [ flying,
        static (can you
                  (cast this_ (from_ := yourGraveyard) (payment := payingInstead (compound
              [ [pip .black, pip .black],
                (sacrifice (counted (exactly 2) isCreature)) ])))) ],
    power := stat 6, toughness := stat 6 }

def escapeToTheWilds : Action :=
  sequentially
    [ exile (topSlice 5),
      establishThroughout (can you
                             (play (bare (and [isCard, was (exile which) thisWay]))))
        (untilEvent (endOf .turn you nextTime)) ]
theorem okEscapeToTheWilds : Action.check (DRS.ofList []) escapeToTheWilds = [] := by decide

/-- Muse Vessel -/
def museVesselPlay : Ability :=
  activated [generic 1]
    (sequentially
      [ choose (a exiledWithThisArtifact),
        establishThroughout (can you (play (that card))) thisTurn ])
theorem okMuseVesselPlay : Ability.check (DRS.ofList []) museVesselPlay = [] := by decide

def demonicConsultationChoice : Action := choose (a (quality .cardName))
theorem okDemonicConsultationChoice : Action.check (DRS.ofList []) demonicConsultationChoice = [] := by
  decide
def voidChoice : Action := choose (a (quality .number))
theorem okVoidChoice : Action.check (DRS.ofList []) voidChoice = [] := by decide
def ashnodsBattleGear : Ability := static (mayDeclineUntap thisArtifact you)
theorem okAshnodsBattleGear : Ability.check (DRS.ofList []) ashnodsBattleGear = [] := by decide

/-- Phyrexian Ingester -/
def phyrexianIngesterPump : Ability :=
  static (conjunction [ getsPt thisCreature (up (letter .x)) (up (letter .y)),
      letterDefinition .x
        (powerOf (the (and [isCreature, in_ (exiledWith thisCreature)]))),
      letterDefinition .y (toughnessOf (that card)) ])
theorem okPhyrexianIngesterPump : Ability.check (DRS.ofList []) phyrexianIngesterPump = [] := by decide

/-- Phyrexian Ingester -/
def phyrexianIngester : Spelled := spelled <| singleFacedCard
  { name := "Phyrexian Ingester", cost := [generic 6, pip .blue], types := [.creature],
    subtypes := [phyrexian, beast],
    text :=
      [ imprint
          (when (does thisCreature enter)
            (may (exile (target (and [isCreature, isNontoken]))))),
        phyrexianIngesterPump ],
    power := stat 3, toughness := stat 3 }

def misterFantastic : Spelled := spelled <| singleFacedCard
  { name := "Mister Fantastic, Reed Richards", cost := [generic 3, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, scientist, hero],
    text :=
      [ reach,
        whenever
          (does (counted (orMore 1) (and [isToken, youControl])) enter)
          (may (draw 1)) ],
    power := stat 2, toughness := stat 4 }

def murmursFromBeyond : Spelled := spelled <| singleFacedCard
  { name := "Murmurs from Beyond", cost := [generic 2, pip .blue], types := [.instant],
    subtypes := [arcane],
    text :=
      [ spell (sequentially
          [ reveal (topSlice 3),
            instruct anOpponent (choose (someOf (exactly 1) them)),
            putInto (that card) graveyard,
            putInto (theRest .object) hand ]) ] }

/-- Helica Glider -/
def helicaGlider : Spelled := spelled <| singleFacedCard
  { name := "Helica Glider", cost := [generic 2, pip .white], types := [.creature],
    subtypes := [nightmare, squirrel],
    text :=
      [ static (enters thisCreature
          [withCounters 1 (chosenKind [flyingCounter, firstStrikeCounter]) .fresh]) ],
    power := stat 2, toughness := stat 2 }

/-- Eager Construct -/
def eagerConstruct : Spelled := spelled <| singleFacedCard
  { name := "Eager Construct", cost := [generic 2], types := [.artifact, .creature],
    subtypes := [construct],
    text := [when (does thisCreature enter) (instruct eachPlayer (may (scry 1)))],
    power := stat 2, toughness := stat 2 }

/-- Rune Snag -/
def runeSnag : Spelled := spelled <| singleFacedCard
  { name := "Rune Snag", cost := [generic 1, pip .blue], types := [.instant],
    text :=
      [ spell (instruct (controllerOf (target isSpell)) (doUnless (counter it)
          (compound
            [ [generic 2],
              scaledGenericMana
                (forEach 2
                   (and [named "Rune Snag", in_ graveyard])) ]))) ] }

/-- Tahngarth, First Mate -/
def tahngarthChoosesDefender : Action :=
  choose (a (and [ or [isPlaneswalker, isPlayer],
               inCombat .attackedBy (that player) ]))
theorem okTahngarthChoosesDefender :
    Action.check ((stepGameEvent 512 (DRS.ofList []) tahngarthHeader).1) tahngarthChoosesDefender = [] := by
  decide

def reefShaman : Spelled := spelled <| singleFacedCard
  { name := "Reef Shaman", cost := [pip .blue], types := [.creature],
    subtypes := [merfolk, shaman],
    text :=
      [ activated tapSymbol
          (establishThroughout (qualityChange (target isLand) .sets
              (chosenQuality
                 (ofYourChoice (.subtype .land) basicTypesOnly))) untilEndOfTurn) ],
    power := stat 0, toughness := stat 2 }

def grixisIllusionist : Spelled := spelled <| singleFacedCard
  { name := "Grixis Illusionist", cost := [pip .blue], types := [.creature],
    subtypes := [human, wizard],
    text :=
      [ activated tapSymbol
          (establishThroughout (qualityChange (target (and [isLand, youControl])) .sets
              (chosenQuality
                 (ofYourChoice (.subtype .land) basicTypesOnly))) untilEndOfTurn) ],
    power := stat 1, toughness := stat 1 }

def distantMelody : Spelled := spelled <| singleFacedCard
  { name := "Distant Melody", cost := [generic 3, pip .blue], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ choose (a (quality (.subtype .creature))),
            draw
              (forEach 1
                (and [isPermanent, youControl, ofChosen (.subtype .creature)])) ]) ] }

def cripplingFear : Spelled := spelled <| singleFacedCard
  { name := "Crippling Fear", cost := [generic 2, pip .black, pip .black],
    types := [.sorcery],
    text :=
      [ spell (sequentially
          [ choose (a (quality (.subtype .creature))),
            get (allOf (and [isCreature, not (ofChosen (.subtype .creature))]))
              (down 3) (down 3) untilEndOfTurn ]) ] }

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

def xenograft : Spelled := spelled <| singleFacedCard
  { name := "Xenograft", cost := [generic 4, pip .blue], types := [.enchantment],
    text :=
      [ static (entersChoosing thisEnchantment (.subtype .creature)),
        static (qualityChange (allOf (and [isCreature, youControl])) .adds
          (chosenQuality (ofChosen (.subtype .creature)))) ] }

/-- Convincing Mirage -/
def convincingMirage : Spelled := spelled <| singleFacedCard
  { name := "Convincing Mirage", cost := [generic 1, pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isLand,
        static (entersChoosingFrom thisAura (.subtype .land) basicTypesOnly),
        static (qualityChange (enchanted land) .sets
          (chosenQuality (ofChosen (.subtype .land)))) ] }

/-- Realmwright -/
def realmwright : Spelled := spelled <| singleFacedCard
  { name := "Realmwright", cost := [pip .blue], types := [.creature],
    subtypes := [vedalken, wizard],
    text :=
      [ static (entersChoosingFrom thisCreature (.subtype .land) basicTypesOnly),
        static (qualityChange (allOf (and [isLand, youControl])) .adds
          (chosenQuality (ofChosen (.subtype .land)))) ],
    power := stat 1, toughness := stat 1 }

/-- Adaptive Automaton. "This creature is the chosen type in addition to its other types" is a
Characteristic-Defining Ability: it depends on the choice made as it enters, as Shapeshifter's
power does, and depending on a choice is no condition [CR#604.3a]. Its pump of the other creatures of the chosen type stays a
separate ability, affecting other objects [CR#604.3a]. -/
def adaptiveAutomaton : Spelled := spelled <| singleFacedCard
  { name := "Adaptive Automaton", cost := [generic 3], types := [.artifact, .creature],
    subtypes := [construct],
    text :=
      [ static (entersChoosing thisCreature (.subtype .creature)),
        static (qualityDefinition thisCreature .adds (chosenQuality (ofChosen (.subtype
            .creature)))),
        static (getsPt
          (allOf (and [ isCreature, youControl, isOther,
                         ofChosen (.subtype .creature) ]))
          (up 1) (up 1)) ],
    power := stat 2, toughness := stat 2 }

def mistformDreamer : Spelled := spelled <| singleFacedCard
  { name := "Mistform Dreamer", cost := [generic 2, pip .blue], types := [.creature],
    subtypes := [illusion],
    text :=
      [ flying,
        activated [generic 1]
          (establishThroughout (qualityChange thisCreature .sets
              (chosenQuality (ofYourChoice (.subtype .creature)))) untilEndOfTurn) ],
    power := stat 2, toughness := stat 1 }

def arcaneAdaptation : Spelled := spelled <| singleFacedCard
  { name := "Arcane Adaptation", cost := [generic 2, pip .blue], types := [.enchantment],
    text :=
      [ static (entersChoosing thisEnchantment (.subtype .creature)),
        static (qualityChange (and [
            allOf (and [isCreature, youControl]),
            allOf (and [isCreature, isSpell, youControl]),
            allOf (and [isCard, isCreature, youOwn, not (in_ battlefield)]) ]) .adds
            (chosenQuality (ofChosen (.subtype .creature)))) ] }

def conspiracy : Spelled := spelled <| singleFacedCard
  { name := "Conspiracy", cost := [generic 3, pip .black, pip .black],
    types := [.enchantment],
    text :=
      [ static (entersChoosing thisEnchantment (.subtype .creature)),
        static (qualityChange (and [
            allOf (and [isCreature, youControl]),
            allOf (and [isCreature, isSpell, youControl]),
            allOf (and [isCard, isCreature, youOwn, not (in_ battlefield)]) ]) .sets
            (chosenQuality (ofChosen (.subtype .creature)))) ] }

def declarationOfNaught : Spelled := spelled <| singleFacedCard
  { name := "Declaration of Naught", cost := [pip .blue, pip .blue], types := [.enchantment],
    text :=
      [ static (entersChoosing thisEnchantment .cardName),
        activated [pip .blue] (counter (target (and [isSpell, named chosenName]))) ] }

def imagecrafter : Spelled := spelled <| singleFacedCard
  { name := "Imagecrafter", cost := [pip .blue], types := [.creature],
    subtypes := [human, wizard],
    text :=
      [ activated tapSymbol
          (sequentially
            [ choose (a (qualityFrom (.subtype .creature) (typeOtherThan wall))),
              establishThroughout (qualityChange (target isCreature) .sets
                                     (chosenQuality (ofChosen (.subtype
                    .creature)))) untilEndOfTurn ]) ],
    power := stat 1, toughness := stat 1 }

def unnaturalSelection : Spelled := spelled <| singleFacedCard
  { name := "Unnatural Selection", cost := [generic 1, pip .blue], types := [.enchantment],
    text :=
      [ activated [generic 1]
          (sequentially
            [ choose (a (qualityFrom (.subtype .creature) (typeOtherThan wall))),
              establishThroughout (qualityChange (target isCreature) .sets
                                     (chosenQuality (ofChosen (.subtype
                    .creature)))) untilEndOfTurn ]) ] }

def standardize : Spelled := spelled <| singleFacedCard
  { name := "Standardize", cost := [pip .blue, pip .blue], types := [.instant],
    text :=
      [ spell (sequentially
          [ choose (a (qualityFrom (.subtype .creature) (typeOtherThan wall))),
            establishThroughout (qualityChange (each isCreature) .sets
                                   (chosenQuality (ofChosen (.subtype
                  .creature)))) untilEndOfTurn ]) ] }

def silverquillSilencer : Spelled := spelled <| singleFacedCard
  { name := "Silverquill Silencer", cost := [pip .white, pip .black], types := [.creature],
    subtypes := [human, cleric],
    text :=
      [ static (entersChoosingFrom thisCreature .cardName (nameOfCard (non isLand))),
        whenever (does anOpponent (cast (a (and [isSpell,
          named chosenName]))))
          (sequentially [instruct they (loseLife 3), draw 1]) ],
    power := stat 3, toughness := stat 2 }

def meddlingMage : Spelled := spelled <| singleFacedCard
  { name := "Meddling Mage", cost := [pip .white, pip .blue], types := [.creature],
    subtypes := [human, wizard],
    text :=
      [ static (entersChoosingFrom thisCreature .cardName (nameOfCard (non isLand))),
        static (cant any (cast (allOf (and [isSpell, named chosenName])))) ],
    power := stat 2, toughness := stat 2 }

/-- Nyxathid -/
def nyxathid : Spelled := spelled <| singleFacedCard
  { name := "Nyxathid", cost := [generic 1, pip .black, pip .black], types := [.creature],
    subtypes := [elemental],
    text :=
      [ static (entersChoosingPlayer thisCreature anOpponent),
        static (getsPt thisCreature
          (down (countOf (cardIn (handOf (the isChosenPlayer)))))
          (down (countOf (cardIn (handOf (the isChosenPlayer)))))) ],
    power := stat 7, toughness := stat 7 }

/-- Expel the Interlopers -/
def expelTheInterlopers : Spelled := spelled <| singleFacedCard
  { name := "Expel the Interlopers", cost := [generic 3, pip .white, pip .white],
    types := [.sorcery],
    text :=
      [ spell (sequentially
          [ choose (a (qualityFrom .number (number (fromTo 0 10)))),
            destroy (allOf (and [isCreature, hasPower .atLeast chosenNumber])) ]) ] }

def nevermore : Spelled := spelled <| singleFacedCard
  { name := "Nevermore", cost := [generic 1, pip .white, pip .white], types := [.enchantment],
    text :=
      [ static (entersChoosingFrom thisEnchantment .cardName (nameOfCard (non isLand))),
        static (cant any (cast (allOf (and [isSpell, named chosenName])))) ] }

def necromentiaChoice : Action :=
  choose (a (qualityFrom .cardName (nameOfCard (not (and [hasSupertype .basic, isLand])))))
theorem okNecromentiaChoice : Action.check (DRS.ofList []) necromentiaChoice = [] := by decide

def conjurersBan : Spelled := spelled <| singleFacedCard
  { name := "Conjurer's Ban", cost := [pip .white, pip .black], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ choose (a (quality .cardName)),
            establishThroughout (conjunction
                                   [ cant any (cast (allOf (and [isSpell, named chosenName]))),
                  cant any (play
                              (allOf (and [isLand, named chosenName]))) ]) untilYourNextTurn ]),
        spell (draw 1) ] }

def foundingOfOmashu : Spelled := spelled <| singleFacedCard
  { name := "Founding of Omashu", cost := [generic 2, pip .red], types := [.enchantment],
    subtypes := [saga],
    text :=
      [ when (chapterMark [1])
          (createToken (creatureToken 1 1 [.white] [ally]) (count := 2)),
        when (chapterMark [2])
          (may (discard (a (cardIn yourHand))) (draw 1)),
        when (chapterMark [3])
          (get (allOf (and [isCreature, youControl])) (up 1) (up 0) untilEndOfTurn) ] }

def vedalkenOrrery : Spelled := spelled <| singleFacedCard
  { name := "Vedalken Orrery", cost := [generic 4], types := [.artifact],
    text :=
      [ static (can you (cast (allOf isSpell)) (asThough := asThoughHadFlash them)) ] }

def shimmerMyr : Spelled := spelled <| singleFacedCard
  { name := "Shimmer Myr", cost := [generic 3], types := [.artifact, .creature],
    subtypes := [myr],
    text :=
      [ flash,
        static (can you (cast (allOf (and [isSpell, isArtifact]))) (asThough := asThoughHadFlash them)) ],
    power := stat 2, toughness := stat 2 }

def quickSliver : Spelled := spelled <| singleFacedCard
  { name := "Quick Sliver", cost := [generic 1, pip .green], types := [.creature],
    subtypes := [sliver],
    text :=
      [ flash,
        static (can (a isPlayer) (cast (allOf (and [isSpell, sliver])))
                  (asThough := asThoughHadFlash them)) ],
    power := stat 1, toughness := stat 1 }

def vernalEquinox : Spelled := spelled <| singleFacedCard
  { name := "Vernal Equinox", cost := [generic 3, pip .green], types := [.enchantment],
    text :=
      [ static
          (can (a isPlayer) (cast (allOf (and [isSpell, or [isCreature, isEnchantment]])))
             (asThough := asThoughHadFlash them)) ] }

def borneUponAWind : Spelled := spelled <| singleFacedCard
  { name := "Borne Upon a Wind", cost := [generic 1, pip .blue], types := [.instant],
    text :=
      [ spell
          (establishThroughout
             (can you (cast (allOf isSpell)) (asThough := asThoughHadFlash them)) thisTurn),
        spell (draw 1) ] }

def gisaAndGeralf : Spelled := spelled <| singleFacedCard
  { name := "Gisa and Geralf", cost := [generic 2, pip .blue, pip .black],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, wizard],
    text :=
      [ when (does thisCreature enter) (mill 4),
        static (can you
                  (cast (counted (exactly 1) (and [isSpell, isCreature, zombie]))
                     (from_ := yourGraveyard)) (timing := onEachOfYourTurns)) ],
    power := stat 4, toughness := stat 4 }

/-- "play lands from the top of your library" -/
def playLandsFromTop : StaticSpec :=
  can you (play (allOf isLand) (from_ := onTop))
/-- "cast <p> spells from the top of your library" -/
def castFromTop (p : Predicate) : StaticSpec :=
  can you (cast (allOf p) (from_ := onTop))

/-- Future Sight; Magus of the Future -/
def playLandsAndCastSpellsFromTop : StaticSpec :=
  conjunction [playLandsFromTop, castFromTop isSpell]
theorem okPlayLandsAndCastSpellsFromTop :
    StaticSpec.check (DRS.ofList []) playLandsAndCastSpellsFromTop = [] := by decide
/-- Assemble the Players -/
def castSmallCreatureFromTopOnceEachTurn : StaticSpec :=
  can you
      (cast (counted (exactly 1) (and [isSpell, isCreature, hasPower .atMost 2]))
         (from_ := onTop)) (timing := eachTurn)
theorem okCastSmallCreatureFromTopOnceEachTurn :
    StaticSpec.check (DRS.ofList []) castSmallCreatureFromTopOnceEachTurn = [] := by decide

def courserOfKruphix : Spelled := spelled <| singleFacedCard
  { name := "Courser of Kruphix", cost := [generic 1, pip .green, pip .green],
    types := [.enchantment, .creature], subtypes := [centaur],
    text :=
      [ static (visibility .reveal you topOfLibrary),
        static playLandsFromTop,
        whenever (does (a (and [isLand, youControl])) enter)
          (gainLife 1) ],
    power := stat 2, toughness := stat 4 }

def korlessaScaleSinger : Spelled := spelled <| singleFacedCard
  { name := "Korlessa, Scale Singer", cost := [pip .green, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [dragon, bard],
    text :=
      [ static (visibility .lookAt you topOfLibrary),
        static (castFromTop (and [isSpell, dragon])) ],
    power := stat 1, toughness := stat 4 }

def precognitionField : Spelled := spelled <| singleFacedCard
  { name := "Precognition Field", cost := [generic 3, pip .blue], types := [.enchantment],
    text :=
      [ static (visibility .lookAt you topOfLibrary),
        static (castFromTop (and [isSpell, isInstantOrSorcery])),
        activated [generic 3] (exile (topSlice 1)) ] }

def mysticForge : Spelled := spelled <| singleFacedCard
  { name := "Mystic Forge", cost := [generic 4], types := [.artifact],
    text :=
      [ static (visibility .lookAt you topOfLibrary),
        static (conjunction [castFromTop (and [isSpell, isArtifact]), castFromTop (and
            [isSpell, isColorless])]),
        activated (compound [tapSymbol, payLife 1]) (exile (topSlice 1)) ] }

def exploration : Spelled := spelled <| singleFacedCard
  { name := "Exploration", cost := [pip .green], types := [.enchantment],
    text :=
        [static
           (can you (play (counted (additional (exactly 1)) isLand))
              (timing := onEachOfYourTurns))] }

def oracleOfMulDaya : Spelled := spelled <| singleFacedCard
  { name := "Oracle of Mul Daya", cost := [generic 3, pip .green], types := [.creature],
    subtypes := [elf, shaman],
    text :=
      [ static
          (can you (play (counted (additional (exactly 1)) isLand))
             (timing := onEachOfYourTurns)),
        static (visibility .reveal you topOfLibrary),
        static playLandsFromTop ],
    power := stat 2, toughness := stat 2 }

def azusaLostButSeeking : Spelled := spelled <| singleFacedCard
  { name := "Azusa, Lost but Seeking", cost := [generic 2, pip .green],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, monk],
    text :=
        [static
           (can you (play (counted (additional (exactly 2)) isLand))
              (timing := onEachOfYourTurns))],
    power := stat 1, toughness := stat 2 }

def summerBloom : Spelled := spelled <| singleFacedCard
  { name := "Summer Bloom", cost := [generic 1, pip .green], types := [.sorcery],
    text := [spell
               (establishThroughout
                  (can you (play (counted (additional (upTo 3)) isLand))) thisTurn)] }

def explore : Spelled := spelled <| singleFacedCard
  { name := "Explore", cost := [generic 1, pip .green], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ establishThroughout (can you (play (counted (additional (exactly 1)) isLand)))
              thisTurn,
            draw 1 ]) ] }

def urbanEvolution : Spelled := spelled <| singleFacedCard
  { name := "Urban Evolution", cost := [generic 3, pip .green, pip .blue],
    types := [.sorcery],
    text :=
      [ spell (sequentially
          [ draw 3,
            establishThroughout (can you
                                   (play
                                      (counted (additional (exactly 1))
                                         isLand))) thisTurn ]) ] }

def dryadOfTheIlysianGrove : Spelled := spelled <| singleFacedCard
  { name := "Dryad of the Ilysian Grove", cost := [generic 2, pip .green],
    types := [.enchantment, .creature], subtypes := [nymph, dryad],
    text :=
      [ static
          (can you (play (counted (additional (exactly 1)) isLand))
             (timing := onEachOfYourTurns)),
        static (qualityChange (allOf (and [isLand, youControl])) .adds
          (everyTypeOf .basicLand)) ],
    power := stat 2, toughness := stat 4 }

/-- Ashes of the Fallen -/
def ashesOfTheFallen : Spelled := spelled <| singleFacedCard
  { name := "Ashes of the Fallen", cost := [generic 2], types := [.artifact],
    text :=
      [ static (entersChoosing thisArtifact (.subtype .creature)),
        static (qualityChange (each (and [isCreature, in_ yourGraveyard])) .adds
          (chosenQuality (ofChosen (.subtype .creature)))) ] }

def hellkiteCharger : Spelled := spelled <| singleFacedCard
  { name := "Hellkite Charger", cost := [generic 4, pip .red, pip .red],
    types := [.creature], subtypes := [dragon],
    text :=
      [ flying, haste,
        whenever (does thisCreature attack)
          (may (pay [generic 5, pip .red, pip .red] .once)
            (sequentially
              [ untap (allOf (and [isCreature, isAttacking])),
                additionalPhase .combat none 1 ])) ],
    power := stat 5, toughness := stat 5 }

/-- Foriysian Brigade -/
def foriysianBrigade : Spelled := spelled <| singleFacedCard
  { name := "Foriysian Brigade", cost := [generic 3, pip .white], types := [.creature],
    subtypes := [human, soldier],
    text :=
        [static
           (can thisCreature (block (counted (additional (exactly 1)) isCreature))
              (timing := eachCombat))],
    power := stat 2, toughness := stat 4 }

/-- Two-Headed Giant of Foriys -/
def twoHeadedGiantOfForiys : Spelled := spelled <| singleFacedCard
  { name := "Two-Headed Giant of Foriys", cost := [generic 4, pip .red],
    types := [.creature], subtypes := [giant],
    text := [trample, static
               (can thisCreature (block (counted (additional (exactly 1)) isCreature))
                  (timing := eachCombat))],
    power := stat 4, toughness := stat 4 }

def highGround : Spelled := spelled <| singleFacedCard
  { name := "High Ground", cost := [pip .white], types := [.enchantment],
    text :=
        [static
           (can (each (and [isCreature, youControl]))
              (block (counted (additional (exactly 1)) isCreature))
        (timing := eachCombat))] }

/-- Watcher in the Web -/
def watcherInTheWeb : Spelled := spelled <| singleFacedCard
  { name := "Watcher in the Web", cost := [generic 4, pip .green], types := [.creature],
    subtypes := [spider],
    text := [reach, static
               (can thisCreature (block (counted (additional (exactly 7)) isCreature))
                  (timing := eachCombat))],
    power := stat 2, toughness := stat 5 }

/-- Forgotten Lore -/
def forgottenLoreRepeat : Action :=
  sequentially
    [ instruct (target isOpponent) (choose (a (cardIn yourGraveyard))),
      may (pay [pip .green] .once) (repeat_
          againExcludingChosen) ]
theorem okForgottenLoreRepeat : Action.check (DRS.ofList []) forgottenLoreRepeat = [] := by decide

/-- Leyline of the Meek -/
def leylineOfTheMeek : Spelled := spelled <| singleFacedCard
  { name := "Leyline of the Meek", cost := [generic 2, pip .white, pip .white],
    types := [.enchantment],
    text :=
      [ mayBeginOnBattlefield,
        static (getsPt (allOf (and [isCreature, isToken])) (up 1) (up 1)) ] }

/-- Leyline of Vitality -/
def leylineOfVitality : Spelled := spelled <| singleFacedCard
  { name := "Leyline of Vitality", cost := [generic 2, pip .green, pip .green],
    types := [.enchantment],
    text :=
      [ mayBeginOnBattlefield,
        static (getsPt (allOf (and [isCreature, youControl])) (up 0) (up 1)),
        whenever (does (a (and [isCreature, youControl])) enter) (may (gainLife 1)) ] }

def phyrexianRevoker : Spelled := spelled <| singleFacedCard
  { name := "Phyrexian Revoker", cost := [generic 2], types := [.artifact, .creature],
    subtypes := [phyrexian, horror],
    text :=
      [ static (entersChoosingFrom thisCreature .cardName (nameOfCard (non isLand))),
        static (cant any (activate (allOf (and [ abilityHead .anyActivated,
                         abilityOf (allOf (and [isSource, named chosenName])) ])))) ],
    power := stat 2, toughness := stat 1 }

def voidstoneGargoyle : Spelled := spelled <| singleFacedCard
  { name := "Voidstone Gargoyle", cost := [generic 3, pip .white, pip .white],
    types := [.creature], subtypes := [gargoyle],
    text :=
      [ flying,
        static (entersChoosingFrom thisCreature .cardName (nameOfCard (non isLand))),
        static (cant any (cast (allOf (and [isSpell, named chosenName])))),
        static (cant any (activate (allOf (and [ abilityHead .anyActivated,
                         abilityOf (allOf (and [isSource, named chosenName])) ])))) ],
    power := stat 3, toughness := stat 3 }

def pithingNeedle : Spelled := spelled <| singleFacedCard
  { name := "Pithing Needle", cost := [generic 1], types := [.artifact],
    text :=
      [ static (entersChoosing thisArtifact .cardName),
        static (cant any (activate (allOf (and [ abilityHead .anyActivated,
                         abilityOf (allOf (and [isSource, named chosenName])),
                         not isManaAbility ])))) ] }

/-- Rhystic Study -/
def rhysticStudy : Spelled := spelled <| singleFacedCard
  { name := "Rhystic Study", cost := [generic 2, pip .blue], types := [.enchantment],
    text :=
      [ whenever (does anOpponent (cast (a isSpell)))
          (instruct (that player) (doUnless (instruct you (may (draw 1)))
            [generic 1])) ] }

def gandalfWhiteRider : Spelled := spelled <| singleFacedCard
  { name := "Gandalf, White Rider", cost := [generic 3, pip .white],
    supertypes := [.legendary], types := [.creature],
    subtypes := [avatar, wizard],
    text :=
      [ vigilance,
        whenever (does you (cast (a isSpell)))
          (sequentially
            [ get (each (and [isCreature, youControl])) (up 1) (up 0) untilEndOfTurn,
              scry 1 ]),
        when (does thisCreature die) (may (putInto it (fromTop 5 library))) ],
    power := stat 3, toughness := stat 3 }

def helmOfPossession : Spelled := spelled <| singleFacedCard
  { name := "Helm of Possession", cost := [generic 4], types := [.artifact],
    text :=
      [ static (mayDeclineUntap thisArtifact you),
        activated (compound [[generic 2], tapSymbol, sacrifice (a isCreature)])
          (gainControl (target isCreature)
            (forAsLongAs
              (and [ is thisArtifact youControl,
                      is thisArtifact isTapped ])) (controller := you)) ] }

def override : Spelled := spelled <| singleFacedCard
  { name := "Override", cost := [generic 2, pip .blue], types := [.instant],
    text :=
      [ spell (instruct (controllerOf (target isSpell)) (doUnless (counter it)
          (scaledGenericMana (forEach 1 (and [isArtifact, youControl]))))) ] }

def rakshasasDisdain : Spelled := spelled <| singleFacedCard
  { name := "Rakshasa's Disdain", cost := [generic 2, pip .blue], types := [.instant],
    text :=
      [ spell (instruct (controllerOf (target isSpell)) (doUnless (counter it)
          (scaledGenericMana (forEach 1 (cardIn yourGraveyard))))) ] }

def megatherium : Spelled := spelled <| singleFacedCard
  { name := "Megatherium", cost := [generic 2, pip .green], types := [.creature],
    subtypes := [beast],
    text :=
      [ trample,
        when (does thisCreature enter)
          (doUnless (sacrifice thisCreature)
            (scaledGenericMana (forEach 1 (cardIn yourHand)))) ],
    power := stat 4, toughness := stat 4 }

def killingWave : Action :=
  doForEach (each isCreature)
    (instruct (controllerOf it) (doUnless (instruct they (sacrifice it)) (instruct they (loseLife (letter .x)))))
theorem okKillingWave : Action.check (DRS.ofList []) killingWave = [] := by decide
def fadeAway : Action :=
  doForEach (each isCreature)
    (instruct (controllerOf it) (doUnless (instruct they (sacrifice (a isPermanent))) [generic 1]))
theorem okFadeAway : Action.check (DRS.ofList []) fadeAway = [] := by decide

def tidalFlats : Spelled := spelled <| singleFacedCard
  { name := "Tidal Flats", cost := [pip .blue], types := [.enchantment],
    text :=
      [ activated [pip .blue, pip .blue]
          (doForEach (each (and [isCreature, isAttacking, not (hasKeyword flying)]))
            (instruct (controllerOf it) (doUnless (gain
                (allOf (and [ isCreature, youControl,
                               inCombat .blockerOf (that creature) ]))
                firstStrike untilEndOfTurn) [generic 1]))) ] }

def primalSurge : Spelled := spelled <| singleFacedCard
  { name := "Primal Surge", cost := [generic 8, pip .green, pip .green], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ exile (topSlice 1),
            doIf (is it isPermanentCard)
              (may (putOntoBattlefield it) (repeat_ again))
                  ]) ] }

def cultivatorColossus : Spelled := spelled <| singleFacedCard
  { name := "Cultivator Colossus", cost := [generic 4, pip .green, pip .green, pip .green],
    types := [.creature], subtypes := [plant, beast],
    text :=
      [ trample,
        static (ptDefinition thisCreature .bothEach
          (countOf (and [isLand, youControl]))),
        when (does thisCreature enter)
          (may (putOntoBattlefieldTapped (a (and [isLand, in_ yourHand])))
            (sequentially [draw 1, repeat_ again])) ] }

def zimoneAndDina : Ability :=
  activated
      (compound [tapSymbol, sacrifice (a (and [isCreature, isOther]))])
    (sequentially
      [ draw 1,
        may (putOntoBattlefieldTapped (a (and [isLand, in_ yourHand]))),
        doIf (compareAmt (countOf (and [isLand, youControl])) .atLeast 8)
          (repeat_ (moreTimes 1)) ])
theorem okZimoneAndDina : Ability.check (DRS.ofList []) zimoneAndDina = [] := by decide

def cryptLurker : Spelled := spelled <| singleFacedCard
  { name := "Crypt Lurker", cost := [generic 3, pip .black], types := [.creature],
    subtypes := [horror],
    text :=
      [ when (does thisCreature enter)
          (may
            (chooseOne (sacrifice (a isCreature)) (discard (a (and [isCreature, in_ yourHand]))))
            (draw 1)) ],
    power := stat 3, toughness := stat 4 }

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
      (can you (cast this_ (from_ := yourLibrary)) (timing := whileSearchingYourLibrary))
theorem okPanglacialWurmCast : Ability.check (DRS.ofList []) panglacialWurmCast = [] := by decide

/-- Shah of Naar Isle -/
def shahOfNaarIsle : Spelled := spelled <| singleFacedCard
  { name := "Shah of Naar Isle", cost := some [generic 3, pip .red], types := [.creature],
    subtypes := [efreet],
    text :=
      [ trample,
        echo [generic 0],
        when (does any (pay anyCost .once .paid (some thisCreature) (some echo)))
          (instruct eachOpponent (may (drawCount (upToOf 3)))) ],
    power := stat 6, toughness := stat 6 }

/-- Memory Plunder -/
def memoryPlunder : Spelled := spelled <| singleFacedCard
  { name := "Memory Plunder",
    cost := some [hybridPip .blue .black, hybridPip .blue .black, hybridPip .blue .black,
                  hybridPip .blue .black],
    types := [.instant],
    text :=
      [ spell (establish
          (can you
             (cast (target (and [isInstantOrSorcery, isCard]))
                (from_ := graveyardOf anOpponent) (payment := withoutPaying)))) ] }

/-- Omniscience -/
def omniscience : Spelled := spelled <| singleFacedCard
  { name := "Omniscience", cost := [generic 7, pip .blue, pip .blue, pip .blue],
    types := [.enchantment],
    text :=
      [ static (altCost (allOf (and [isSpell,
          did you (cast which (from_ := yourHand)) thisObject]))) ] }

/-- Tranquil Frillback -/
def tranquilFrillbackOffer : Action := may (pay [pip .green] (.upTo 3))
theorem okTranquilFrillbackOffer : Action.check (DRS.ofList []) tranquilFrillbackOffer = [] := by decide

/-- Caller of the Hunt -/
def callerOfTheHunt : Spelled := spelled <| singleFacedCard
  { name := "Caller of the Hunt", cost := [generic 2, pip .green], types := [.creature],
    subtypes := [human],
    text :=
      [ static (addedCost (choose (a (quality (.subtype .creature)))) false),
        static (ptDefinition thisCreature .bothEach
          (countOf (and [isCreature, ofChosen (.subtype .creature)]))) ] }

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
      search yourLibrary (counted (exactlyOf theDifference) plains),
      reveal (those card),
      putOntoBattlefieldTapped (someOf (exactly 1) (those card)),
      putInto (theRest .object) hand ]
theorem okBoreasChargerSpell : Action.check (DRS.ofList []) boreasChargerSpell = [] := by decide

/-- Sandstone Oracle -/
def sandstoneOracle : Spelled := spelled <| singleFacedCard
  { name := "Sandstone Oracle", cost := [generic 7], types := [.artifact, .creature],
    subtypes := [sphinx],
    text :=
      [ flying,
        when (does thisCreature enter)
          (sequentially
            [ choose anOpponent,
              doIf (compareAmt (countOf (cardIn (handOf (that player)))) .greater
                      (countOf (cardIn yourHand)))
                (draw theDifference) ]) ],
    power := stat 4, toughness := stat 4 }

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
def celestialJudgment : Spelled := spelled <| singleFacedCard
  { name := "Celestial Judgment", cost := [generic 4, pip .white, pip .white],
    types := [.sorcery],
    text :=
      [ spell (sequentially
          [ doForEachKind (.value .power) .number (choose (a (and [isCreature, hasPower .eq chosenNumber]))) (allOf isCreature),
            destroy (each (and [isCreature, notChosen])) ]) ] }

/-- World Queller -/
def worldQuellerChoice : Action :=
  may (choose (a (quality .cardType)))
    (instruct eachPlayer (sacrifice (aTheirChoice (and [isPermanent, ofChosen .cardType]))))
theorem okWorldQuellerChoice : Action.check (DRS.ofList []) worldQuellerChoice = [] := by decide

/-- Moonlit Meditation -/
def moonlitMeditation : Spelled := spelled <| singleFacedCard
  { name := "Moonlit Meditation", cost := [generic 2, pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant (and [or [isArtifact, isCreature], youControl]),
        static (replace
                  (nthOccurrence (.nth 1)
                     (would (by_ you)
                        (createObject anyAmount (token (copyOf (counted (orMore 1) isToken)) [])))
                     (some .turn))
                  (instead (may (create groupSize (copyOf (enchanted permanent)) [])))) ] }

def discardUpToTwoThenDrawThatMany : Action :=
  sequentially [discard (counted (upTo 2) (cardIn yourHand)), draw groupSize]
theorem okDiscardUpToTwoThenDrawThatMany :
    Action.check (DRS.ofList []) discardUpToTwoThenDrawThatMany = [] := by decide
/-- Truce and Temporary Truce -/
def drawUpToTwoThenGainPerShortfall : Action :=
  instruct eachPlayer
    (sequentially
      [ may (drawCount (upToOf 2)),
        instruct (that player) (gainLife (times 2 shortOfCeiling)) ])
theorem okDrawUpToTwoThenGainPerShortfall :
    Action.check (DRS.ofList []) drawUpToTwoThenGainPerShortfall = [] := by decide

/-- Commune with the Gods -/
def communeWithTheGods : Spelled := spelled <| singleFacedCard
  { name := "Commune with the Gods", cost := [generic 1, pip .green], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ reveal (topSlice 5),
            may (putInto (fromAmong (exactly 1) (or [isCreature, isEnchantment]) them) hand),
            putInto (theRest .object) graveyard ]) ] }

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
                (toCast (and [isInstantOrSorcery, in_ (exiledWith thisSaga)])))) ]
theorem okSummonEsperValigarmandaCast :
    StaticSpec.check (DRS.ofList []) summonEsperValigarmandaCast = [] := by decide
/-- Rogue Class's level-3 body -/
def rogueClassLevelThree : StaticSpec :=
  conjunction [ can you (play (allOf (in_ (exiledWith thisClass)))),
      can you (pay anyCost .once)
          (asThough := some
             (spendManaAs none .anyColor (toCast (in_ (exiledWith thisClass))))) ]
theorem okRogueClassLevelThree : StaticSpec.check (DRS.ofList []) rogueClassLevelThree = [] := by decide

/-- Soul Ransom -/
def soulRansom : Spelled := spelled <| singleFacedCard
  { name := "Soul Ransom", cost := [generic 2, pip .blue, pip .black], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (controlGrant you (enchanted creature)),
        activatedBy
          (repeatTimes 2
            (sequentially [choose (a (cardIn hand)), discard (that card)]))
          (sequentially [instruct (controllerOf thisAura) (sacrifice itPermanent), instruct they (draw 2)])
          yourOpponents ] }

/-- Vraska's Scorn -/
def vraskasScorn : Spelled := spelled <| singleFacedCard
  { name := "Vraska's Scorn", cost := [generic 2, pip .black, pip .black], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ instruct (target isOpponent) (loseLife 4),
            may
              (sequentially
                [ searchLibraryOrGraveyard (a (named "Vraska, Scheming Gorgon")),
                  reveal it, putInto it hand ]),
            doIf (did you (search yourLibrary) thisWay) shuffle ]) ] }

/-- Old-Growth Dryads -/
def oldGrowthDryads : Spelled := spelled <| singleFacedCard
  { name := "Old-Growth Dryads", cost := [pip .green], types := [.creature],
    subtypes := [dryad],
    text :=
      [ when (does thisCreature enter)
          (instruct eachOpponent (may
            (sequentially
              [ search (libraryOf they) (a (and [isLand, hasSupertype .basic])),
                putOntoBattlefieldTapped it,
                shuffle ]))) ],
    power := stat 3, toughness := stat 3 }

/-- Verity Circle -/
def verityCircle : Spelled := spelled <| singleFacedCard
  { name := "Verity Circle", cost := [generic 2, pip .blue], types := [.enchantment],
    text :=
      [ whenever
          (does (a (and [isCreature, anOpponentControls])) (become tapped))
          (if_ := is it (not (inCombat .declaredAttacker)))
          (may (draw 1)),
        activated [generic 4, pip .blue]
          (tap (target (and [isCreature, not (hasKeyword flying)]))) ] }

/-- Danitha, New Benalia's Light -/
def danithaNewBenaliasLight : Spelled := spelled <| singleFacedCard
  { name := "Danitha, New Benalia's Light", cost := [generic 1, pip .green, pip .white],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, knight],
    text :=
      [ vigilance, trample, lifelink,
        static (can you (cast (counted (exactly 1) (and [ isSpell, or [ aura,
                                  equipment ] ])) (from_ := some
                                                                yourGraveyard)) (timing := some
                                                                                   onEachOfYourTurns)) ],
    power := stat 2, toughness := stat 2 }

/-- Muldrotha, the Gravetide -/
def muldrothaLandWindow : StaticSpec :=
  can you (play (a isLand) (from_ := yourGraveyard)) (timing := onEachOfYourTurns)
theorem okMuldrothaLandWindow : StaticSpec.check (DRS.ofList []) muldrothaLandWindow = [] := by decide
/-- Nahiri's Lithoforming -/
def nahiriExtraLands : StaticSpec := can you
    (play (counted (additional (exactlyOf (letter .x))) isLand))
theorem okNahiriExtraLands : StaticSpec.check (DRS.ofList [letterB .x]) nahiriExtraLands = [] := by decide

/-- Haakon, Stromgald Scourge -/
def haakonStromgaldScourge : Spelled := spelled <| singleFacedCard
  { name := "Haakon, Stromgald Scourge", cost := some [generic 1, pip .black, pip .black],
    supertypes := [.legendary], types := [.creature],
    subtypes := [zombie, knight],
    text :=
      [ static (can you (cast this_ (from_ := some yourGraveyard))),
        static (cant you (cast this_ (from_ := some (not yourGraveyard)))),
        static (onlyWhile
                  (can you
                     (cast (allOf (and [isSpell, knight]))
                        (from_ := some yourGraveyard))) (is this_ (in_ battlefield))),
        when (does thisCreature die) (loseLife 2) ],
    power := stat 3, toughness := stat 3 }

/-- Apex of Power -/
def apexOfPowerCast : Action :=
  sequentially
    [ exile (topSlice 7),
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
def trueNameNemesis : Spelled := spelled <| singleFacedCard
  { name := "True-Name Nemesis", cost := [generic 1, pip .blue, pip .blue],
    types := [.creature], subtypes := [merfolk, rogue],
    text :=
      [ static (entersChoosingPlayer thisCreature (a isPlayer)),
        protection (objectsOfPlayer (the isChosenPlayer)) ],
    power := stat 3, toughness := stat 1 }

/-- Akiri, Fearless Voyager -/
def akiriUnattachOffer : Ability :=
  activated [pip .white]
    (may
      (unattach (a (and [ equipment,
                            attachedTo (a (and [isCreature, youControl])) ])))
      (tap (that creature)))
theorem okAkiriUnattachOffer : Ability.check (DRS.ofList []) akiriUnattachOffer = [] := by decide
/-- Summoning Materia -/
def summoningMateriaTopCast : Ability :=
  static (onlyWhile (castFromTop (and [isSpell, isCreature]))
    (is thisEquipment (attachedTo (a isCreature))))
theorem okSummoningMateriaTopCast : Ability.check (DRS.ofList []) summoningMateriaTopCast = [] := by decide
/-- Vizier of the Menagerie -/
def vizierOfTheMenagerieSpend : StaticSpec :=
  can you (pay anyCost .once)
      (asThough := spendManaAs none .anyType (some (toCast (and [isCreature, isSpell]))))
theorem okVizierOfTheMenagerieSpend : StaticSpec.check (DRS.ofList []) vizierOfTheMenagerieSpend = [] := by
  decide

/-- Conspicuous Snoop -/
def conspicuousSnoop : Spelled := spelled <| singleFacedCard
  { name := "Conspicuous Snoop", cost := [pip .red, pip .red], types := [.creature],
    subtypes := [goblin, rogue],
    text :=
      [ static (visibility .reveal you topOfLibrary),
        static (can you
                  (play (allOf (and [isSpell, goblin])) (from_ := onTop))),
        static (onlyWhile
          (abilityGrantFrom thisCreature (topSlice 1) [.anyActivated])
          (is (topSlice 1) goblin)) ],
    power := stat 2, toughness := stat 2 }

/-- Ballot Broker -/
def ballotBroker : Spelled := spelled <| singleFacedCard
  { name := "Ballot Broker", cost := [generic 2, pip .white], types := [.creature],
    subtypes := [human, advisor],
    text := [static (can you (vote (times := additional (exactly 1))))],
    power := stat 2, toughness := stat 3 }

/-- Emissary of Grudges -/
def emissaryOfGrudgesReveal : Ability :=
  activatedOnlyOnce (expose .reveal (choiceOf .player))
    (doOnlyIf (chooseNewTargets (target (or [isSpell, abilityHead .anyOnStack])))
      (and
        [ is it (controlledBy (the isChosenPlayer)),
          is it
            (targeting (youOr (a (and [isPermanent, youControl]))) .someTarget) ]))
    oncePerGame
theorem okEmissaryOfGrudgesReveal :
    Ability.check (DRS.ofList [ChoiceSort.binding .player]) emissaryOfGrudgesReveal = [] := by decide

def prosperity : Spelled := spelled <| singleFacedCard
  { name := "Prosperity", cost := [.variable, pip .blue], types := [.sorcery],
    text := [spell (instruct eachPlayer (draw (letter .x)))] }

def collectiveUnconscious : Action := draw (forEach 1 (and [isCreature, youControl]))
theorem okCollectiveUnconscious : Action.check (DRS.ofList []) collectiveUnconscious = [] := by decide
def killiansConfidence : Action :=
  sequentially
    [get (target isCreature) (up 1) (up 1) untilEndOfTurn, draw 1]
theorem okKilliansConfidence : Action.check (DRS.ofList []) killiansConfidence = [] := by decide

def scatheZombies : Spelled := spelled <| singleFacedCard
  { name := "Scathe Zombies", cost := [generic 2, pip .black], types := [.creature],
    subtypes := [zombie], power := stat 2, toughness := stat 2 }

def plainsCard : Spelled := spelled <| singleFacedCard
  { name := "Plains", supertypes := [.basic], types := [.land], subtypes := [plains] }

def islandCard : Spelled := spelled <| singleFacedCard
  { name := "Island", supertypes := [.basic], types := [.land], subtypes := [island] }

def swampCard : Spelled := spelled <| singleFacedCard
  { name := "Swamp", supertypes := [.basic], types := [.land], subtypes := [swamp] }

def mountainCard : Spelled := spelled <| singleFacedCard
  { name := "Mountain", supertypes := [.basic], types := [.land], subtypes := [mountain] }

def forestCard : Spelled := spelled <| singleFacedCard
  { name := "Forest", supertypes := [.basic], types := [.land], subtypes := [forest] }

def snowCoveredForest : Spelled := spelled <| singleFacedCard
  { name := "Snow-Covered Forest", supertypes := [.basic, .snow], types := [.land],
    subtypes := [forest] }

/-- Not Forgotten -/
def notForgottenPlacement : Action :=
  putInto (target (cardIn graveyard)) (or [onTop, onBottom])
theorem okNotForgottenPlacement : Action.check (DRS.ofList []) notForgottenPlacement = [] := by decide

def duneblastCard : Spelled := spelled <| singleFacedCard
  { name := "Duneblast", cost := [generic 4, pip .white, pip .black, pip .green],
    types := [.sorcery], text := [spell duneblast] }

def boreasChargerDifference : Amount := theDifference
theorem okBoreasChargerDifference :
    Amount.check ((stepAction 512 .postEvent .obj (DRS.ofList []) boreasChargerChoice).1) boreasChargerDifference = [] := by decide

/-- Hymn to Tourach -/
def hymnToTourach : Spelled := spelled <| singleFacedCard
  { name := "Hymn to Tourach", cost := [pip .black, pip .black], types := [.sorcery],
    text :=
      [ spell (discard (countedAtRandom (exactly 2) (cardIn (handOf (target
          isPlayer))))) ] }

/-- Caught in the Crossfire -/
def caughtInTheCrossfire : Spelled := spelled <| singleFacedCard
  { name := "Caught in the Crossfire", cost := [pip .red, pip .red], types := [.instant],
    text :=
      [ spell (chooseSpree
          [ ([generic 1],
             instruct this_ (dealDamage 2 (each (and [isCreature, isOutlaw])))),
            ([generic 1],
             instruct this_
                 (dealDamage 2 (each (and [isCreature, non isOutlaw])))) ]) ] }

/-- Requisition Raid -/
def requisitionRaid : Spelled := spelled <| singleFacedCard
  { name := "Requisition Raid", cost := [pip .white], types := [.sorcery],
    text :=
      [ spell (chooseSpree
          [ ([generic 1], destroy (target isArtifact)),
            ([generic 1], destroy (target isEnchantment)),
            ([generic 1],
             putCounters 1 p1p1Counter
               (each (and [isCreature, controlledBy (target isPlayer)]))) ]) ] }

/-- Rustler Rampage -/
def rustlerRampage : Spelled := spelled <| singleFacedCard
  { name := "Rustler Rampage", cost := [pip .white], types := [.instant],
    text :=
      [ spell (chooseSpree
          [ ([generic 1],
             untap
               (allOf (and [isCreature, controlledBy (target isPlayer)]))),
            ([generic 1],
             gain (target isCreature) doubleStrike untilEndOfTurn) ]) ] }

/-- Consuming Tide -/
def consumingTide : Spelled := spelled <| singleFacedCard
  { name := "Consuming Tide", cost := [generic 2, pip .blue, pip .blue], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ instruct eachPlayer (choose
              (a (and [isPermanent, non isLand, controlledBy they]))),
            returnToHand (allOf (and [isPermanent, non isLand, notChosen])),
            doForEach
              (each (compareOver isOpponent (countOf (cardIn (handOf they))) .greater
                (countOf (cardIn yourHand))))
              (draw 1) ]) ] }

/-- Stick Together: a chosen party is four up-to-one choices, not the group the game computes
[CR#700.8d]. -/
def stickTogether : Action :=
  sequentially
    [ instruct eachPlayer (choose
        (counted (upTo 1)
          (and [isCreature, cleric, controlledBy they]))),
      instruct eachPlayer (choose
        (counted (upTo 1)
          (and [isCreature, rogue, controlledBy they]))),
      instruct eachPlayer (choose
        (counted (upTo 1)
          (and [isCreature, warrior, controlledBy they]))),
      instruct eachPlayer (choose
        (counted (upTo 1)
          (and [isCreature, wizard, controlledBy they]))),
      instruct eachPlayer (sacrifice (theRest .object)) ]
theorem okStickTogether : Action.check (DRS.ofList []) stickTogether = [] := by decide

/-- Disciple of Caelus Nin: "starting with you" overrides the default APNAP choice order
[CR#101.1,101.4]. -/
def discipleOfCaelusNin : Spelled := spelled <| singleFacedCard
  { name := "Disciple of Caelus Nin", cost := [generic 4, pip .white], types := [.creature],
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
    power := stat 3, toughness := stat 4 }

/-- Sculpted Sunburst: "if you chose a creature this way" reads back the choice you made earlier
in the spell (a `did` over `choose`);
"not chosen by any player this way" excludes every player's chosen creature. -/
def sculptedSunburst : Spelled := spelled <| singleFacedCard
  { name := "Sculpted Sunburst", cost := [generic 3, pip .white, pip .white],
    types := [.sorcery],
    text :=
      [ spell (sequentially
          [ choose (a (and [isCreature, youControl])),
            instruct eachOpponent (choose
              (a (comparesOwnStat .power (and [isCreature, controlledBy they])
                .atMost (powerOf it)))),
            doIf (did you (choose (a isCreature)) thisWay) (exile (each (and [isCreature, notChosen])))
                ]) ] }

/-- Loxodon Peacekeeper: "If two or more players are tied for lowest life total, you choose one
of them, and that player gains control of this creature." "Them" reads the players the counted
existential introduces (`semantics-v3.md` §1.3, box 2). -/
def loxodonPeacekeeper : Spelled := spelled <| singleFacedCard
  { name := "Loxodon Peacekeeper", cost := [generic 1, pip .white], types := [.creature],
    subtypes := [elephant, soldier],
    text :=
      [ at_ (beginningOfYour .upkeep)
          (sequentially
            [ gainControl thisCreature (controller :=
                the (and [isPlayer, superlative .min (.playerStat .lifeTotal) isPlayer])),
              doIf (thereAre (counted (orMore 2)
                      (and [isPlayer, tiedFor .min (.playerStat .lifeTotal) isPlayer])))
                (sequentially
                  [ choose (someOf (exactly 1) themPlayers),
                    gainControl thisCreature (controller := that player) ]) ]) ],
    power := stat 4, toughness := stat 4 }
theorem okLoxodonPeacekeeper : loxodonPeacekeeper.card.check = [] := loxodonPeacekeeper.ok

/-- In *Loxodon Peacekeeper*, "them" after the counted existential reads the players it
introduced (`.count`), not the player with the lowest life total. -/
theorem loxodonPeacekeeperThemReadsTheTiedPlayers :
    let lowest := the (and [isPlayer, superlative .min (.playerStat .lifeTotal) isPlayer])
    let before := (stepAction 512 .postEvent .obj (DRS.ofList [])
      (gainControl thisCreature (controller := lowest))).1
    let tied := thereAre (counted (orMore 2)
      (and [isPlayer, tiedFor .min (.playerStat .lifeTotal) isPlayer]))
    let after := (stepCondition 512 (nextClause before) tied).1
    (resolveCandidates (.word .player) .many (nextClause after)).map (·.det) = [.count] := by
  decide

/-- "The permanent of `kind` `who` controls with the greatest mana value." -/
private def greatestManaValue (kind : Predicate) (who : NounPhrase) : NounPhrase :=
  the (and [kind, controlledBy who,
            superlative .max (.stat .manaValue) (and [kind, controlledBy who])])

/-- Juxtapose's last sentence: "If two or more permanents a player controls are tied for
greatest, their controller chooses one of them." -/
private def juxtaposeTie : Action :=
  doIf (thereAre (counted (orMore 2)
          (and [isPermanent, controlledBy (a isPlayer),
                tiedFor .max (.stat .manaValue) (and [isPermanent, controlledBy (that player)])])))
    (instruct (controllerOf them) (choose (someOf (exactly 1) them)))

/-- Juxtapose: "You and target player exchange control of the creature you each control with the
greatest mana value. Then exchange control of artifacts the same way. If two or more permanents
a player controls are tied for greatest, their controller chooses one of them." "The same way"
is written out as the second exchange. -/
def juxtapose : Spelled := spelled <| singleFacedCard
  { name := "Juxtapose", cost := [generic 3, pip .blue], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ exchange (controlOf (greatestManaValue isCreature you)
              (greatestManaValue isCreature (target isPlayer))),
            exchange (controlOf (greatestManaValue isArtifact you)
              (greatestManaValue isArtifact (that player))),
            juxtaposeTie ]) ] }
theorem okJuxtapose : juxtapose.card.check = [] := juxtapose.ok

/-- In *Juxtapose*, "them" reads the tied permanents the counted existential introduces
(`.count`), even after an ordinary plural mention of artifacts ("artifacts you control get
+1/+1"): an existential that is no replacement's condition ranks as the nearest mention (owner
ruling 2026-10-22, batch 4). -/
theorem okJuxtaposeThemReadsTheTiedPermanents :
    let tied := thereAre (counted (orMore 2)
      (and [isPermanent, controlledBy (a isPlayer),
            tiedFor .max (.stat .manaValue) (and [isPermanent, controlledBy (that player)])]))
    let artifacts := (stepAction 512 .postEvent .obj (DRS.ofList [])
      (get (bare (and [isArtifact, youControl])) (up 1) (up 1) untilEndOfTurn)).1
    let after := (stepCondition 512 (nextClause artifacts) tied).1
    Action.check (DRS.ofList [])
        (sequentially [get (bare (and [isArtifact, youControl])) (up 1) (up 1) untilEndOfTurn,
          juxtaposeTie]) = [] ∧
      (artifacts.findById? 1).map (·.det) = some .all ∧
      (resolveCandidates .bare .many (nextClause after)).map (·.det) = [.count] := by
  decide

end Semantics.Cards
