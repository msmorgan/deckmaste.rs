import Semantics
import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Cards.Description
import Semantics.Cards.Anaphora
import Semantics.Cards.Trigger

/-!
# Semantics.Cards.Damage

Port of `idris/src/Experimental/Cards/Damage.idr`: the printed cards of the Damage family and
the bench items beside them.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

/-- Lightning Bolt -/
def bolt : Action := instruct this_ (dealDamage 3 (target isAnyTarget))
theorem okBolt : Action.check (DRS.ofList []) bolt = [] := by decide
def barrageOfBoulders : Action :=
  instruct this_ (dealDamage 1 (each (and [isCreature, youDontControl])))
theorem okBarrageOfBoulders : Action.check (DRS.ofList []) barrageOfBoulders = [] := by decide
def rabidBite : Action :=
  dealDamageOwnPower (target (and [isCreature, youControl]))
    (target (and [isCreature, youDontControl]))
theorem okRabidBite : Action.check (DRS.ofList []) rabidBite = [] := by decide
def preyUpon : Action := instruct (target (and [isCreature, youControl])) (fight (target (and [isCreature, youDontControl])))
theorem okPreyUpon : Action.check (DRS.ofList []) preyUpon = [] := by decide
def arcTrail : Action :=
  instruct this_ (sequentially
    [ dealDamage 2 (target isAnyTarget), dealDamage 1 (target isAnyOtherTarget) ])
theorem okArcTrail : Action.check (DRS.ofList []) arcTrail = [] := by decide
def deadshot : Action :=
  sequentially
    [ tap (target isCreature),
      dealDamageOwnPower it (target (and [isCreature, isOther])) ]
theorem okDeadshot : Action.check (DRS.ofList []) deadshot = [] := by decide
def immersturmSkullcairn : Ability :=
  activatedOnlyDuring
    (compound [[generic 1, pip .black, pip .red, pip .red], tapSymbol,
      (sacrifice thisLand)])
    (sequentially
      [ instruct this_ (dealDamage 3 (target isPlayer)), discard (a (cardIn (handOf (that
          .player)))) ])
    asSorcery
theorem okImmersturmSkullcairn : Ability.check (DRS.ofList []) immersturmSkullcairn = [] := by decide
def pyriteSpellbomb : Ability :=
  activated (compound [[pip .red], sacrifice thisArtifact])
    (instruct this_ (dealDamage 2 (target isAnyTarget)))
theorem okPyriteSpellbomb : Ability.check (DRS.ofList []) pyriteSpellbomb = [] := by decide
def karplusanYeti : Action :=
  sequentially
    [ dealDamageOwnPower thisCreature (target isCreature),
      dealDamageOwnPower (that creature) thisCreature ]
theorem okKarplusanYeti : Action.check (DRS.ofList []) karplusanYeti = [] := by decide
def suddenDemise : Action :=
  sequentially
    [ choose (a (quality .color)),
      instruct this_ (dealDamage (letter .x) (each (and [isCreature, ofChosen .color]))) ]
theorem okSuddenDemise : Action.check (DRS.ofList []) suddenDemise = [] := by decide
def caseOfTheGatewayExpress : Action :=
  sequentially
    [ choose (target (and [isCreature, youDontControl])),
      instruct (each (and [isCreature, youControl]))
          (dealDamage 1 (that creature)) ]
theorem okCaseOfTheGatewayExpress : Action.check (DRS.ofList []) caseOfTheGatewayExpress = [] := by decide
def arrowsOfJustice : Action :=
  instruct this_ (dealDamage 4 (target (and [isCreature, or [isAttacking, isBlocking]])))
theorem okArrowsOfJustice : Action.check (DRS.ofList []) arrowsOfJustice = [] := by decide
def flamesOfTheRazeBoar : Action :=
  sequentially
    [ instruct this_ (dealDamage 4 (target (and [isCreature, anOpponentControls]))),
      doOnlyIf
        (instruct this_
          (dealDamage 2 (each (and [isCreature, isOther, controlledBy (that player)]))))
        (exists_ (and [isCreature, youControl, hasPower .atLeast 4])) ]
theorem okFlamesOfTheRazeBoar : Action.check (DRS.ofList []) flamesOfTheRazeBoar = [] := by decide
def yawgmothDemon : Action :=
  mayOrElse (sacrifice (a isArtifact)) (sequentially
                                          [tap thisCreature, instruct this_
                                             (dealDamage 2 you)])
theorem okYawgmothDemon : Action.check (DRS.ofList []) yawgmothDemon = [] := by decide
def arcBlade : Action :=
  sequentially
    [ instruct this_ (dealDamage 2 (target isAnyTarget)),
      exileWithCounters this_ 3 timeCounter ]
theorem okArcBlade : Action.check (DRS.ofList []) arcBlade = [] := by decide
def abrade : Action :=
  chooseOne (instruct this_ (dealDamage 3 (target isCreature)))
    (destroy (target isArtifact))
theorem okAbrade : Action.check (DRS.ofList []) abrade = [] := by decide
def nibelheimAflame : Action :=
  sequentially
    [ choose (target (and [isCreature, youControl])),
      dealDamageOwnPower it (each (and [isCreature, isOther])) ]
theorem okNibelheimAflame : Action.check (DRS.ofList []) nibelheimAflame = [] := by decide
def brashTaunter : Action := instruct thisCreature
    (fight (target (and [isCreature, isOther])))
theorem okBrashTaunter : Action.check (DRS.ofList []) brashTaunter = [] := by decide
def ulvenwaldTracker : Action :=
  instruct (target (and [isCreature, youControl])) (fight (target (and [isCreature, isOther])))
theorem okUlvenwaldTracker : Action.check (DRS.ofList []) ulvenwaldTracker = [] := by decide

def botBashingTime : Action :=
  sequentially
    [ instruct this_ (dealDamage 6 (target isCreature)),
      replaceEvent (would (that creature) die)
          (instead (exile it)) thisTurn ]
theorem okBotBashingTime : Action.check (DRS.ofList []) botBashingTime = [] := by decide
def wordsOfWar : Action :=
  replaceNextEvent (would you (draw 1))
      (instead (instruct this_ (dealDamage 2 (target isAnyTarget)))) thisTurn
theorem okWordsOfWar : Action.check (DRS.ofList []) wordsOfWar = [] := by decide

/-- Thunderwave -/
def thunderwave : Spelled := spelled <| singleFacedCard
  { name := "Thunderwave", cost := [generic 2, pip .red, pip .red], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ rollDice 1 (.sides 20),
            applyResultsTable
              [ rollRow (oneThrough 9) (instruct this_ (dealDamage 3 (each isCreature))),
                rollRow (fromTo 10 19)
                  (sequentially
                    [ may (choose (a isCreature)),
                      instruct this_
                        (dealDamage 3 (each (and [isCreature, notChosen]))) ]),
                rollRow (exactly 20)
                  (instruct this_
                    (dealDamage 6 (each (and [isCreature, opponentsControl])))) ] ]) ] }

/-- Fog, Holy Day, Darkness, Root Snare -/
def fog : Action := preventAll everywhere thisTurn (some .combat)
theorem okFog : Action.check (DRS.ofList []) fog = [] := by decide
/-- Indestructible Aura, Shielded Passage -/
def indestructibleAura : Action :=
  preventAll (toRecipient (target isCreature)) thisTurn
theorem okIndestructibleAura : Action.check (DRS.ofList []) indestructibleAura = [] := by decide
def shieldmatesBlessing : Action :=
  preventNext (toRecipient (target isAnyTarget)) 3 thisTurn
theorem okShieldmatesBlessing : Action.check (DRS.ofList []) shieldmatesBlessing = [] := by decide
def moonlitWake : Ability := whenever (does (a isCreature) die) (gainLife 1)
theorem okMoonlitWake : Ability.check (DRS.ofList []) moonlitWake = [] := by decide
def eliteJavelineer : Ability :=
  whenever (does thisCreature block)
      (instruct this_
         (dealDamage 1 (target (and [isCreature, isAttacking]))))
theorem okEliteJavelineer : Ability.check (DRS.ofList []) eliteJavelineer = [] := by decide
/-- Glacial Chasm -/
def glacialChasmShield : Ability :=
  static (damageRule unattributed (toRecipient you) (prevent allDamage) .repeatedly)
theorem okGlacialChasmShield : Ability.check (DRS.ofList []) glacialChasmShield = [] := by decide
def corneredCrook : Ability :=
  when (does thisCreature enter)
    (offerWhen (sacrifice (a isArtifact)) (instruct this_ (dealDamage 3 (target
        isAnyTarget))))
theorem okCorneredCrook : Ability.check (DRS.ofList []) corneredCrook = [] := by decide

def aladdinsRing : Spelled := spelled <| singleFacedCard
  { name := "Aladdin's Ring", cost := [generic 8], types := [.artifact],
    text :=
      [ activated (compound [[generic 8], tapSymbol])
          (instruct thisArtifact (dealDamage 4 (target isAnyTarget))) ] }

def chandrasRevolution : Action :=
  sequentially
    [ instruct this_ (dealDamage 4 (target isCreature)), tap (target isLand),
      skipUntap (that land) 1 ]
theorem okChandrasRevolution : Action.check (DRS.ofList []) chandrasRevolution = [] := by decide
def arbalestElite : Ability :=
  activated (compound [[generic 2, pip .white], tapSymbol])
    (sequentially
      [ instruct thisCreature (dealDamage 3 (target (and [isCreature, or [isAttacking, isBlocking]]))),
        skipUntap thisCreature 1 ])
theorem okArbalestElite : Ability.check (DRS.ofList []) arbalestElite = [] := by decide
def sizzlingBarrage : Action :=
  instruct this_ (dealDamage 4 (target
    (and [isCreature, did which block soFarThisTurn])))
theorem okSizzlingBarrage : Action.check (DRS.ofList []) sizzlingBarrage = [] := by decide
def goadedAttackTrigger : Ability :=
  whenever (does (a (and [isCreature, hasDesignation goaded])) attack)
    (instruct it (dealDamage 1 (controllerOf it)))
theorem okGoadedAttackTrigger : Ability.check (DRS.ofList []) goadedAttackTrigger = [] := by decide

def extraArms : Spelled := spelled <| singleFacedCard
  { name := "Extra Arms", cost := [generic 4, pip .red], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        whenever (does (enchanted creature) attack)
          (instruct it (dealDamage 2 (target isAnyTarget))) ] }

def chainReaction : Spelled := spelled <| singleFacedCard
  { name := "Chain Reaction", cost := [generic 2, pip .red, pip .red], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ instruct this_ (dealDamage (letter .x) (each isCreature)),
            define .x (countOf isCreature) ]) ] }

/-- Black Vise -/
def blackVise : Spelled := spelled <| singleFacedCard
  { name := "Black Vise", cost := [generic 1], types := [.artifact],
    text :=
      [ static (entersChoosingPlayer thisArtifact anOpponent),
        at_ (beginningOfPossessed .upkeep (the isChosenPlayer))
          (sequentially
            [ instruct thisArtifact (dealDamage (letter .x) (that player)),
              define .x (minus (countOf (cardIn (handOf they))) 4) ]) ] }

def harshSustenance : Spelled := spelled <| singleFacedCard
  { name := "Harsh Sustenance", cost := [generic 1, pip .white, pip .black], types := [.instant],
    text :=
      [ spell (sequentially
          [ instruct this_ (dealDamage (letter .x) (target isAnyTarget)),
            gainLife (letter .x),
            define .x (countOf (and [isCreature, youControl])) ]) ] }

/-- Purging Scythe: "If two or more creatures are tied for least toughness, you choose one of
them." A counted existential, whose domain "them" reads (`semantics-v3.md` §1.3, box 2). -/
def purgingScythe : Ability :=
  at_ (beginningOfYour .upkeep)
    (sequentially
      [ instruct thisArtifact (dealDamage 2 (the (and [isCreature, superlative .min (.stat .toughness) isCreature]))),
        doIf
          (thereAre (counted (orMore 2) (and [isCreature, tiedFor .min (.stat .toughness) isCreature])))
          (choose (someOf (exactly 1) them)) ])
theorem okPurgingScythe : Ability.check (DRS.ofList []) purgingScythe = [] := by decide
/-- In *Purging Scythe*, "them" reads the creatures the counted existential introduces (`.count`):
"the number of …" would close its domain, and "the creature with the least toughness" is one. -/
theorem purgingScytheThemReadsTheTiedCreatures :
    let least := the (and [isCreature, superlative .min (.stat .toughness) isCreature])
    let before := (stepAction 512 .postEvent .obj (DRS.ofList [])
      (instruct thisArtifact (dealDamage 2 least))).1
    let tied := thereAre (counted (orMore 2)
      (and [isCreature, tiedFor .min (.stat .toughness) isCreature]))
    let after := (stepCondition 512 (nextClause before) tied).1
    (resolveCandidates .bare .many (nextClause after)).map (·.det) = [.count] := by
  decide

def lionHeart : Spelled := spelled <| singleFacedCard
  { name := "Lion Heart", cost := [generic 4], types := [.artifact],
    subtypes := [equipment],
    text :=
      [ when (does thisEquipment enter)
          (instruct it (dealDamage 2 (target isAnyTarget))),
        static (getsPt (equipped creature) (up 2) (up 1)),
        equip [generic 2] ] }

def lightmineField : Spelled := spelled <| singleFacedCard
  { name := "Lightmine Field", cost := [generic 2, pip .white, pip .white],
    types := [.enchantment],
    text :=
      [ whenever (does (counted (orMore 1) isCreature) attack)
          (instruct thisEnchantment
             (dealDamage (numberOf (those creature)) (eachOf (those creature)))) ] }

def ingeniousArtillerist : Spelled := spelled <| singleFacedCard
  { name := "Ingenious Artillerist", cost := [generic 2, pip .red], types := [.creature],
    subtypes := [human, artificer],
    text :=
      [ whenever (does (counted (orMore 1) (and [isArtifact, youControl])) enter)
          (instruct thisCreature (dealDamage groupSize eachOpponent)) ],
    power := stat 3, toughness := stat 1 }

/-- Inferno Elemental -/
def infernoElemental : Spelled := spelled <| singleFacedCard
  { name := "Inferno Elemental", cost := [generic 4, pip .red, pip .red], types := [.creature],
    subtypes := [elemental],
    text :=
      [ whenever (does thisCreature (block (a isCreature)))
          (or_ := [does (by_ (a isCreature)) (block thisCreature)])
          (instruct thisCreature (dealDamage 3 (that creature))) ],
    power := stat 4, toughness := stat 4 }

/-- Psychic Purge -/
def psychicPurge : Spelled := spelled <| singleFacedCard
  { name := "Psychic Purge", cost := [pip .blue], types := [.sorcery],
    text :=
      [ spell (instruct this_ (dealDamage 1 (target isAnyTarget))),
        when
          (causes
            (bySource (a (and [or [isSpell, abilityHead .anyOnStack],
                               anOpponentControls])))
            (does you (discard this_)))
          (instruct (that player) (loseLife 5)) ] }

/-- Chandra Nalaar -/
def chandraNalaarsX : Ability :=
  activated (loyaltySymbol (down (letter .x))) (instruct this_ (dealDamage (letter .x) (target isCreature)))
theorem okChandraNalaarsX : Ability.check (DRS.ofList []) chandraNalaarsX = [] := by decide

def etherealHaze : Spelled := spelled <| singleFacedCard
  { name := "Ethereal Haze", cost := [pip .white], types := [.instant],
    subtypes := [arcane],
    text := [spell (preventAllBy (allOf isCreature) everywhere thisTurn)] }

def defang : Spelled := spelled <| singleFacedCard
  { name := "Defang", cost := [generic 1, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (damageRule (dealtBy (enchanted creature)) everywhere
          (prevent allDamage) .repeatedly) ] }

def sphereOfPurity : Spelled := spelled <| singleFacedCard
  { name := "Sphere of Purity", cost := [generic 3, pip .white], types := [.enchantment],
    text :=
      [ static (damageRule (dealtBy (a isArtifact)) (toRecipient you)
          (prevent (ofThatDamage 1)) .repeatedly) ] }

def dazzlingReflection : Spelled := spelled <| singleFacedCard
  { name := "Dazzling Reflection", cost := [generic 1, pip .white], types := [.instant],
    text :=
      [ spell (sequentially
          [ gainLife (powerOf (target isCreature)),
            establishThroughout (damageRule (dealtBy (that creature)) everywhere
                                   (prevent allDamage)
                .nextTimeOnly) thisTurn ]) ] }

def thunderstaff : Spelled := spelled <| singleFacedCard
  { name := "Thunderstaff", cost := [generic 3], types := [.artifact],
    text :=
      [ static (onlyWhile
          (damageRule (dealtBy (a isCreature)) (toRecipient you)
            (prevent (ofThatDamage 1)) .repeatedly (some .combat))
          (is thisArtifact isUntapped)),
        activated (compound [[generic 2], tapSymbol])
          (get (allOf (and [isCreature, isAttacking])) (up 1) (up 0)
            untilEndOfTurn) ] }

def gideonAllyOfZendikar : Spelled := spelled <| singleFacedCard
  { name := "Gideon, Ally of Zendikar", cost := [generic 2, pip .white, pip .white],
    supertypes := [.legendary], types := [.planeswalker], subtypes := [gideon],
    text :=
      [ activated (loyaltySymbol (up 1))
          (sequentially
            [ establishThroughout (qualityChange thisPlaneswalker .sets
                  (bundle
                    { characteristics :=
                      { types := [.creature],
                        subtypes := [human, soldier, ally],
                        text := [indestructible], power := stat 5, toughness := stat 5 } }
                    (some .planeswalker))) untilEndOfTurn,
              preventAll (toRecipient it) thisTurn ]),
        activated (loyaltySymbol (up 0))
          (createToken (creatureToken 2 2 [.white] [knight, ally])),
        activated (loyaltySymbol (down 4))
          (getEmblem
            [static (getsPt (allOf (and [isCreature, youControl])) (up 1) (up 1))]) ],
    loyalty := stat 4 }

def turnTheTables : Spelled := spelled <| singleFacedCard
  { name := "Turn the Tables", cost := [generic 3, pip .white, pip .white], types := [.instant],
    text :=
      [ spell (establishThroughout (damageRule unattributed (toRecipient you)
            (redirect allDamage
               (target (and [isCreature, isAttacking]))) .repeatedly (some .combat))
            thisTurn) ] }

def pariah : Spelled := spelled <| singleFacedCard
  { name := "Pariah", cost := [generic 2, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (damageRule unattributed (toRecipient you)
          (redirect allDamage (enchanted creature)) .repeatedly) ] }

def martyrsOfKorlis : Spelled := spelled <| singleFacedCard
  { name := "Martyrs of Korlis", cost := [generic 3, pip .white, pip .white],
    types := [.creature], subtypes := [human],
    text :=
      [ static (onlyWhile
          (damageRule (dealtBy (allOf isArtifact)) (toRecipient you)
            (redirect allDamage thisCreature) .repeatedly)
          (is thisCreature isUntapped)) ],
    power := stat 1, toughness := stat 6 }

def wardOfPiety : Spelled := spelled <| singleFacedCard
  { name := "Ward of Piety", cost := [generic 1, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        activated [generic 1, pip .white]
          (establishThroughout
             (damageRule unattributed (toRecipient (enchanted creature))
              (redirect (theNext 1) (target isAnyTarget)) .repeatedly) thisTurn) ] }

def mirrorwoodTreefolk : Spelled := spelled <| singleFacedCard
  { name := "Mirrorwood Treefolk", cost := [generic 3, pip .green], types := [.creature],
    subtypes := [treefolk],
    text :=
      [ activated [generic 2, pip .red, pip .white]
          (establishThroughout (damageRule unattributed (toRecipient thisCreature)
              (redirect allDamage (target isAnyTarget)) .nextTimeOnly) thisTurn) ],
    power := stat 2, toughness := stat 4 }

def carom : Spelled := spelled <| singleFacedCard
  { name := "Carom", cost := [generic 1, pip .white], types := [.instant],
    text :=
      [ spell (sequentially
          [ establishThroughout (damageRule unattributed (toRecipient (target isCreature))
                (redirect (theNext 1)
                   (target (and [isCreature, isOther]))) .repeatedly) thisTurn,
            draw 1 ]) ] }

def daughterOfAutumn : Spelled := spelled <| singleFacedCard
  { name := "Daughter of Autumn", cost := [generic 2, pip .green, pip .green],
    supertypes := [.legendary], types := [.creature], subtypes := [avatar],
    text :=
      [ activated [pip .white]
          (establishThroughout
             (damageRule unattributed
                (toRecipient (target (and [isCreature, hasColor .white])))
              (redirect (theNext 1) thisCreature) .repeatedly) thisTurn) ],
    power := stat 2, toughness := stat 4 }

def aegisOfHonor : Spelled := spelled <| singleFacedCard
  { name := "Aegis of Honor", cost := [pip .white], types := [.enchantment],
    text :=
      [ activated [generic 1]
          (establishThroughout (damageRule (dealtBy (a isInstantOrSorcery)) (toRecipient you)
              (redirect allDamage (controllerOf it)) .nextTimeOnly) thisTurn) ] }

def candlesGlow : Spelled := spelled <| singleFacedCard
  { name := "Candles' Glow", cost := [generic 1, pip .white], types := [.instant],
    subtypes := [arcane],
    text :=
      [ spell
          (establishThroughout (damageRule unattributed (toRecipient (target isAnyTarget))
            (prevent (theNext 3) (gainLife preventedThisWay))
                .repeatedly) thisTurn) ] }

/-- Inkshield -/
def inkshieldRider : Action :=
  establishThroughout (damageRule unattributed (toRecipient you)
      (prevent allDamage (createToken (creatureToken 2 1 [.white, .black] []) (count := preventedThisWay)))
      .repeatedly) thisTurn
theorem okInkshieldRider : Action.check (DRS.ofList []) inkshieldRider = [] := by decide

def urzasArmor : Spelled := spelled <| singleFacedCard
  { name := "Urza's Armor", cost := [generic 6], types := [.artifact],
    text :=
      [ static (damageRule (dealtBy (a isSource)) (toRecipient you)
          (prevent (ofThatDamage 1)) .repeatedly) ] }

def circleOfProtectionRed : Spelled := spelled <| singleFacedCard
  { name := "Circle of Protection: Red", cost := [generic 1, pip .white], types := [.enchantment],
    text :=
      [ activated [generic 1]
          (establishThroughout
             (damageRule (dealtBy (aYourChoice (and [isSource, hasColor .red])))
                (toRecipient you)
              (prevent allDamage) .nextTimeOnly) thisTurn) ] }

def healingGrace : Spelled := spelled <| singleFacedCard
  { name := "Healing Grace", cost := [pip .white], types := [.instant],
    text :=
      [ spell (sequentially
          [ establishThroughout
              (damageRule (dealtBy (aYourChoice isSource)) (toRecipient (target isAnyTarget))
                (prevent (theNext 3)) .repeatedly) thisTurn,
            gainLife 3 ]) ] }

def reverseDamage : Spelled := spelled <| singleFacedCard
  { name := "Reverse Damage", cost := [generic 1, pip .white, pip .white], types := [.instant],
    text :=
      [ spell
          (establishThroughout (damageRule (dealtBy (aYourChoice isSource)) (toRecipient you)
            (prevent allDamage (gainLife preventedThisWay)) .nextTimeOnly) thisTurn) ] }

def deflectingPalm : Spelled := spelled <| singleFacedCard
  { name := "Deflecting Palm", cost := [pip .red, pip .white], types := [.instant],
    text :=
      [ spell
          (establishThroughout (damageRule (dealtBy (aYourChoice isSource)) (toRecipient you)
            (prevent allDamage
               (some
                  (instruct this_
                     (dealDamage thatMuch (controllerOf it))))) .nextTimeOnly) thisTurn) ] }

def templeAltisaur : Spelled := spelled <| singleFacedCard
  { name := "Temple Altisaur", cost := [generic 4, pip .white], types := [.creature],
    subtypes := [dinosaur],
    text :=
      [ static (damageRule (dealtBy (a isSource))
          (toRecipient (a (and [dinosaur, youControl])))
          (prevent (allBut 1)) .repeatedly) ],
    power := stat 3, toughness := stat 4 }

def darkSphere : Spelled := spelled <| singleFacedCard
  { name := "Dark Sphere", cost := some [], types := [.artifact],
    text :=
      [ activated (compound [tapSymbol, sacrifice thisArtifact])
          (establishThroughout (damageRule (dealtBy (aYourChoice isSource)) (toRecipient you)
              (prevent (half .down)) .nextTimeOnly) thisTurn) ] }

def shadowbane : Spelled := spelled <| singleFacedCard
  { name := "Shadowbane", cost := [generic 1, pip .white], types := [.instant],
    text :=
      [ spell (establishThroughout (damageRule (dealtBy (aYourChoice isSource))
            (toRecipient (andOr [you, allOf (and [isCreature, youControl])]))
            (prevent allDamage
              (doIf (preventedFromSource (and [isSource, hasColor .black]))
                (gainLife preventedThisWay)))
            .nextTimeOnly) thisTurn) ] }

def sphereOfLaw : Spelled := spelled <| singleFacedCard
  { name := "Sphere of Law", cost := [generic 3, pip .white], types := [.enchantment],
    text :=
      [ static (damageRule (dealtBy (a (and [isSource, hasColor .red]))) (toRecipient you)
          (prevent (ofThatDamage 2)) .repeatedly) ] }

def lavaAxe : Spelled := spelled <| singleFacedCard
  { name := "Lava Axe", cost := [generic 4, pip .red], types := [.sorcery],
    text :=
      [ spell (instruct this_ (dealDamage 5 (target (or [isPlaneswalker, isPlayer])))) ] }

def searingFlesh : Spelled := spelled <| singleFacedCard
  { name := "Searing Flesh", cost := [generic 6, pip .red], types := [.sorcery],
    text :=
      [ spell (instruct this_
          (dealDamage 7 (target (or [isPlaneswalker, isOpponent])))) ] }

def onakkeJavelineerBolt : Ability :=
  activated tapSymbol
    (instruct thisCreature (dealDamage 2 (target (or [isBattle, isPlayer]))))
theorem okOnakkeJavelineerBolt : Ability.check (DRS.ofList []) onakkeJavelineerBolt = [] := by decide

/-- Firesong and Sunspeaker -/
def firesongJoinEcho : Action :=
  sequentially
    [ instruct this_ (dealDamage 3 (target (or [isCreature, isPlayer]))),
      instruct this_ (dealDamage 1 thatJoin) ]
theorem okFiresongJoinEcho : Action.check (DRS.ofList []) firesongJoinEcho = [] := by decide

/-- Forcefield -/
def forcefield : Spelled := spelled <| singleFacedCard
  { name := "Forcefield", cost := [generic 3], types := [.artifact],
    text :=
      [ activated [generic 1]
          (establishThroughout
             (damageRule (dealtBy (aYourChoice (and [isCreature, isUnblocked])))
              (toRecipient you) (prevent (allBut 1)) .nextTimeOnly (some .combat))
            thisTurn) ] }

def endure : Spelled := spelled <| singleFacedCard
  { name := "Endure", cost := [generic 3, pip .white, pip .white], types := [.instant],
    text :=
      [ spell (preventAll (toRecipient (youAnd (allOf (and [isPermanent, youControl]))))
            thisTurn) ] }

def harmsWay : Spelled := spelled <| singleFacedCard
  { name := "Harm's Way", cost := [pip .white], types := [.instant],
    text :=
      [ spell (establishThroughout (damageRule (dealtBy (aYourChoice isSource))
            (toRecipient (andOr [you, allOf (and [isPermanent, youControl])]))
            (redirect (theNext 2) (target isAnyTarget)) .repeatedly) thisTurn) ] }

def divineDeflection : Spelled := spelled <| singleFacedCard
  { name := "Divine Deflection", cost := [.variable, pip .white], types := [.instant],
    text :=
      [ spell (establishThroughout (damageRule unattributed
            (toRecipient (andOr [you, allOf (and [isPermanent, youControl])]))
            (prevent (theNext (letter .x)) (instruct this_ (dealDamage thatMuch (target isAnyTarget))))
            .repeatedly) thisTurn) ] }

def glarecasterShield : Ability :=
  activated [generic 5, pip .white]
    (establishThroughout (damageRule unattributed (toRecipient (andOr [thisCreature, you]))
        (redirect allDamage (target isAnyTarget)) .nextTimeOnly) thisTurn)
theorem okGlarecasterShield : Ability.check (DRS.ofList []) glarecasterShield = [] := by decide

def furnaceOfRath : Spelled := spelled <| singleFacedCard
  { name := "Furnace of Rath", cost := [generic 1, pip .red, pip .red, pip .red],
    types := [.enchantment],
    text :=
      [ static (damageRule (dealtBy (a isSource))
          (toRecipient (a (or [isPermanent, isPlayer]))) (scale (multiplied .doubled))
          .repeatedly) ] }

def gratuitousViolence : Spelled := spelled <| singleFacedCard
  { name := "Gratuitous Violence", cost := [generic 2, pip .red, pip .red, pip .red],
    types := [.enchantment],
    text :=
      [ static (damageRule (dealtBy (a (and [isCreature, youControl])))
          (toRecipient (a (or [isPermanent, isPlayer]))) (scale (multiplied .doubled))
          .repeatedly) ] }

def fieryEmancipation : Spelled := spelled <| singleFacedCard
  { name := "Fiery Emancipation", cost := [generic 3, pip .red, pip .red, pip .red],
    types := [.enchantment],
    text :=
      [ static (damageRule (dealtBy (a (and [isSource, youControl])))
          (toRecipient (a (or [isPermanent, isPlayer]))) (scale (multiplied .tripled))
          .repeatedly) ] }

def sulfuricVapors : Spelled := spelled <| singleFacedCard
  { name := "Sulfuric Vapors", cost := [generic 3, pip .red], types := [.enchantment],
    text :=
      [ static (damageRule (dealtBy (a (and [isSpell, hasColor .red])))
          (toRecipient (a (or [isPermanent, isPlayer]))) (scale (shifted (up 1)))
          .repeatedly) ] }

def lashknifeBarrier : Spelled := spelled <| singleFacedCard
  { name := "Lashknife Barrier", cost := [generic 2, pip .white], types := [.enchantment],
    text :=
      [ when (does thisEnchantment enter) (draw 1),
        static (damageRule (dealtBy (a isSource)) (toRecipient (a (and [isCreature, youControl])))
          (scale (shifted (down 1))) .repeatedly) ] }

def ghostsOfTheInnocent : Spelled := spelled <| singleFacedCard
  { name := "Ghosts of the Innocent", cost := [generic 5, pip .white, pip .white],
    types := [.creature], subtypes := [spirit],
    text :=
      [ static (damageRule (dealtBy (a isSource))
          (toRecipient (a (or [isPermanent, isPlayer]))) (scale (halved .down)) .repeatedly) ],
    power := stat 4, toughness := stat 5 }

def fireServant : Spelled := spelled <| singleFacedCard
  { name := "Fire Servant", cost := [generic 3, pip .red, pip .red], types := [.creature],
    subtypes := [elemental],
    text :=
      [ static (damageRule (dealtBy (a (and [isInstantOrSorcery, hasColor .red, youControl])))
          everywhere (scale (multiplied .doubled)) .repeatedly) ],
    power := stat 4, toughness := stat 3 }

def platedPegasus : Spelled := spelled <| singleFacedCard
  { name := "Plated Pegasus", cost := [generic 2, pip .white], types := [.creature],
    subtypes := [pegasus],
    text :=
      [ flash,
        flying,
        static (damageRule (dealtBy (a isSpell)) (toRecipient (a (or [isPermanent, isPlayer])))
          (prevent (ofThatDamage 1)) .repeatedly) ],
    power := stat 1, toughness := stat 2 }

def spitemare : Spelled := spelled <| singleFacedCard
  { name := "Spitemare", cost := [generic 2, hybridPip .red .white, hybridPip .red .white],
    types := [.creature], subtypes := [elemental],
    text :=
      [ whenever (does any (dealDamage (to := thisCreature))) (instruct it (dealDamage thatMuch (target isAnyTarget))) ],
    power := stat 3, toughness := stat 3 }

def grollub : Spelled := spelled <| singleFacedCard
  { name := "Grollub", cost := [generic 2, pip .black], types := [.creature],
    subtypes := [beast],
    text := [whenever (does any (dealDamage (to := thisCreature))) (instruct eachOpponent (gainLife thatMuch))],
    power := stat 3, toughness := stat 3 }

def moggManiac : Spelled := spelled <| singleFacedCard
  { name := "Mogg Maniac", cost := [generic 1, pip .red], types := [.creature],
    subtypes := [goblin],
    text :=
      [ whenever (does any (dealDamage (to := thisCreature)))
          (instruct it
             (dealDamage thatMuch (target (or [isPlaneswalker, isOpponent])))) ],
    power := stat 1, toughness := stat 1 }

def repercussion : Spelled := spelled <| singleFacedCard
  { name := "Repercussion", cost := [generic 1, pip .red, pip .red], types := [.enchantment],
    text :=
      [ whenever (does any (dealDamage (to := a isCreature)))
          (instruct thisEnchantment
             (dealDamage thatMuch (controllerOf (that creature)))) ] }

def spitefulShadows : Spelled := spelled <| singleFacedCard
  { name := "Spiteful Shadows", cost := [generic 1, pip .black], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        whenever (does any (dealDamage (to := enchanted creature)))
          (instruct it (dealDamage thatMuch (controllerOf it))) ] }

def bindingAgony : Spelled := spelled <| singleFacedCard
  { name := "Binding Agony", cost := [generic 1, pip .black], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        whenever (does any (dealDamage (to := enchanted creature)))
          (instruct thisAura
             (dealDamage thatMuch (controllerOf (that creature)))) ] }

def darienKingOfKjeldor : Spelled := spelled <| singleFacedCard
  { name := "Darien, King of Kjeldor", cost := [generic 4, pip .white, pip .white],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, soldier],
    text :=
      [ whenever (does any (dealDamage (to := you)))
          (may (createToken (creatureToken 1 1 [.white] [soldier]) (count := thatMuch))) ],
    power := stat 3, toughness := stat 3 }

def screamingNemesis : Spelled := spelled <| singleFacedCard
  { name := "Screaming Nemesis", cost := [generic 2, pip .red], types := [.creature],
    subtypes := [spirit],
    text :=
      [ haste,
        whenever (does any (dealDamage (to := thisCreature)))
          (sequentially
            [ instruct it (dealDamage thatMuch (target isAnyOtherTarget)),
              doIf (dealtThisWay isPlayer)
                (establishThroughout (cant they (gainLife anyAmount)) restOfGame) ]) ],
    power := stat 3, toughness := stat 3 }

def sonicShrieker : Spelled := spelled <| singleFacedCard
  { name := "Sonic Shrieker", cost := [generic 2, pip .red, pip .white, pip .black],
    types := [.creature], subtypes := [dragon],
    text :=
      [ flying,
        when (does thisCreature enter)
          (sequentially
            [ instruct it (dealDamage 2 (target isAnyTarget)),
              gainLife 2,
              doIf (dealtThisWay isPlayer) (discard (a (cardIn (handOf they))))
                  ]) ],
    power := stat 4, toughness := stat 4 }

def grievousWoundLifeLock : Spelled := spelled <| singleFacedCard
  { name := "Grievous Wound", cost := [generic 3, pip .black, pip .black], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isPlayer,
        static (cant (enchanted player) (gainLife anyAmount)),
        whenever (does any (dealDamage (to := enchanted player)))
          (instruct they (loseLife (half .up (lifeTotalOf they)))) ] }

def cursedScroll : Spelled := spelled <| singleFacedCard
  { name := "Cursed Scroll", cost := [generic 1], types := [.artifact],
    text :=
      [ activated (compound [[generic 3], tapSymbol])
          (sequentially
            [ choose (a (quality .cardName)),
              reveal (aAtRandom (cardIn yourHand)),
              doIf (is (that card) (named chosenName))
                (instruct thisArtifact (dealDamage 2 (target isAnyTarget))) ]) ] }

def magusOfTheScroll : Spelled := spelled <| singleFacedCard
  { name := "Magus of the Scroll", cost := [pip .red], types := [.creature],
    subtypes := [human, wizard],
    text :=
      [ activated (compound [[generic 3], tapSymbol])
          (sequentially
            [ choose (a (quality .cardName)),
              reveal (aAtRandom (cardIn yourHand)),
              doIf (is (that card) (named chosenName))
                (instruct thisCreature (dealDamage 2 (target isAnyTarget))) ]) ],
    power := stat 1, toughness := stat 1 }

/-- Stuffy Doll -/
def stuffyDoll : Spelled := spelled <| singleFacedCard
  { name := "Stuffy Doll", cost := [generic 5], types := [.artifact, .creature],
    subtypes := [construct],
    text :=
      [ indestructible,
        static (entersChoosingPlayer thisCreature (a isPlayer)),
        whenever (does any (dealDamage (to := thisCreature))) (instruct it (dealDamage thatMuch (the isChosenPlayer))),
        activated tapSymbol (instruct thisCreature (dealDamage 1 thisCreature)) ],
    power := stat 0, toughness := stat 1 }

def saheeliRaiPlusOne : Action :=
  sequentially [scry 1, instruct this_ (dealDamage 1 eachOpponent)]
theorem okSaheeliRaiPlusOne : Action.check (DRS.ofList []) saheeliRaiPlusOne = [] := by decide
def sarkhansUnsealingLine : Ability :=
  whenever
    (does you (cast (a (and [isCreature, isSpell,
                or [ hasPower .eq 4, hasPower .eq 5,
                      hasPower .eq 6 ]]))))
    (instruct thisEnchantment (dealDamage 4 (target isAnyTarget)))
theorem okSarkhansUnsealingLine : Ability.check (DRS.ofList []) sarkhansUnsealingLine = [] := by decide
/-- Savage Swipe, both sentences -/
def savageSwipeLine : Action :=
  sequentially
    [ doOnlyIf (get (target (and [isCreature, youControl])) (up 2) (up 2) (some
        untilEndOfTurn))
        (compareAmt (powerOf it) .eq 2),
      instruct it (fight (target (and [isCreature, youDontControl]))) ]
theorem okSavageSwipeLine : Action.check (DRS.ofList []) savageSwipeLine = [] := by decide

def infernoOfTheStarMounts : Spelled := spelled <| singleFacedCard
  { name := "Inferno of the Star Mounts", cost := [generic 4, pip .red, pip .red],
    supertypes := [.legendary], types := [.creature], subtypes := [dragon],
    text :=
      [ static (cant any (counter this_)),
        flying,
        haste,
        activated [pip .red]
          (triggerThisWay (get thisCreature (up 1) (up 0) untilEndOfTurn)
            (stateHolds (compareAmt (powerOf it) .eq 20))
            (instruct it (dealDamage 20 (target isAnyTarget)))) ],
    power := stat 6, toughness := stat 6 }

def incinerate : Spelled := spelled <| singleFacedCard
  { name := "Incinerate", cost := [generic 1, pip .red], types := [.instant],
    text :=
      [ spell (sequentially
          [ instruct this_ (dealDamage 3 (target isAnyTarget)),
            establishThroughout (cant any
                                   (regeneration
                                      (a (and
                                            [ isCreature,
                                              was (dealDamage anyAmount which) thisWay ]))))
              thisTurn ]) ] }

def ashZealot : Spelled := spelled <| singleFacedCard
  { name := "Ash Zealot", cost := [pip .red, pip .red], types := [.creature],
    subtypes := [human, warrior],
    text :=
      [ firstStrike,
        haste,
        whenever (does (a isPlayer) (cast (a
          (and [isSpell, castFrom graveyard]))))
          (instruct thisCreature (dealDamage 3 (that player))) ],
    power := stat 2, toughness := stat 2 }

def galvanicBlastLine : Action :=
  replace (instruct this_ (dealDamage 2 (target isAnyTarget)))
    (doIf (compareAmt (countOf (and [isArtifact, youControl])) .atLeast 3)
      (instead (instruct this_ (dealDamage 4 thatJoin))))
theorem okGalvanicBlastLine : Action.check (DRS.ofList []) galvanicBlastLine = [] := by decide
/-- Furious Reprisal -/
def furiousReprisal : Action :=
  instruct this_ (dealDamage 2 (eachOf (targets (exactly 2) isAnyTarget)))
theorem okFuriousReprisal : Action.check (DRS.ofList []) furiousReprisal = [] := by decide
/-- Bullseye, Death Dealer -/
def bullseyeModalCost : Ability :=
  activated
    (compound [[generic 3], tapSymbol,
      (chooseOne (sacrifice (a isArtifact))
                 (discard (a (and [non isLand, in_ yourHand]))))])
    (instruct this_ (dealDamage 2 (target isAnyTarget)))
theorem okBullseyeModalCost : Ability.check (DRS.ofList []) bullseyeModalCost = [] := by decide

/-- Twinshot Sniper -/
def twinshotSniper : Spelled := spelled <| singleFacedCard
  { name := "Twinshot Sniper", cost := [generic 3, pip .red], types := [.artifact, .creature],
    subtypes := [goblin, archer],
    text :=
      [ reach,
        when (does thisCreature enter)
            (instruct it (dealDamage 2 (target isAnyTarget))),
        channel
          (activated (compound [[generic 1, pip .red], discard this_])
            (instruct it (dealDamage 2 (target isAnyTarget)))) ],
    power := stat 2, toughness := stat 3 }

/-- Quakebringer -/
def quakebringerDamage : Ability :=
  at_ (beginningOfYour .upkeep)
    (if_ := some (or [ is this_ (in_ battlefield),
           and [ is this_ (cardIn yourGraveyard),
                  exists_ (and [isCreature, giant,
                                 youControl]) ] ]))
    (instruct this_ (dealDamage 2 eachOpponent))
theorem okQuakebringerDamage : Ability.check (DRS.ofList []) quakebringerDamage = [] := by decide
/-- Sand Strangler -/
def sandStranglerDamage : Ability :=
  when (does thisCreature enter)
    (if_ := some (or [ exists_ (and [isLand, desert, youControl]),
           exists_ (and [isLand, desert, in_ yourGraveyard]) ]))
    (may (instruct this_ (dealDamage 3 (target isCreature))))
theorem okSandStranglerDamage : Ability.check (DRS.ofList []) sandStranglerDamage = [] := by decide

def brazenDwarf : Spelled := spelled <| singleFacedCard
  { name := "Brazen Dwarf", cost := [generic 1, pip .red], types := [.creature],
    subtypes := [dwarf, shaman],
    text :=
      [ whenever (does you (rollDice anyAmount .anyDie))
          (instruct thisCreature (dealDamage 1 eachOpponent)) ],
    power := stat 1, toughness := stat 3 }

/-- Lavalanche -/
def lavalanche : Action :=
  instruct this_ (simultaneously
    [ dealDamage (letter .x) targetPlayerOrPlaneswalker,
      dealDamage (letter .x) eachCreatureThatSplitControls ])
theorem okLavalanche : Action.check (DRS.ofList []) lavalanche = [] := by decide
/-- Flame Wave -/
def flameWave : Action :=
  instruct this_ (simultaneously
    [ dealDamage 4 targetPlayerOrPlaneswalker,
      dealDamage 4 eachCreatureThatSplitControls ])
theorem okFlameWave : Action.check (DRS.ofList []) flameWave = [] := by decide
/-- Chandra Nalaar's ultimate -/
def chandraNalaarUltimate : Action :=
  simultaneously
    [ instruct this_ (dealDamage 10 targetPlayerOrPlaneswalker),
      instruct this_ (dealDamage 10 eachCreatureThatSplitControls) ]
theorem okChandraNalaarUltimate : Action.check (DRS.ofList []) chandraNalaarUltimate = [] := by decide
/-- Chandra, Pyrogenius's ultimate -/
def chandraPyrogeniusUltimate : Action :=
  simultaneously
    [ instruct this_ (dealDamage 6 targetPlayerOrPlaneswalker),
      instruct this_ (dealDamage 6 eachCreatureThatSplitControls) ]
theorem okChandraPyrogeniusUltimate : Action.check (DRS.ofList []) chandraPyrogeniusUltimate = [] := by decide
/-- Bonfire of the Damned -/
def bonfireOfTheDamned : Action :=
  instruct this_ (simultaneously
    [ dealDamage (letter .x) targetPlayerOrPlaneswalker,
      dealDamage (letter .x) eachCreatureThatSplitControls ])
theorem okBonfireOfTheDamned : Action.check (DRS.ofList []) bonfireOfTheDamned = [] := by decide
/-- Chandra's Fury -/
def chandrasFury : Action :=
  instruct this_ (simultaneously
    [ dealDamage 4 targetPlayerOrPlaneswalker,
      dealDamage 1 eachCreatureThatSplitControls ])
theorem okChandrasFury : Action.check (DRS.ofList []) chandrasFury = [] := by decide
/-- Angrath, Minotaur Pirate's plus -/
def angrathMinotaurPirateBolt : Action :=
  instruct this_ (simultaneously
    [ dealDamage 1 targetOpponentOrPlaneswalker,
      dealDamage 1 eachCreatureThatSplitControls ])
theorem okAngrathMinotaurPirateBolt : Action.check (DRS.ofList []) angrathMinotaurPirateBolt = [] := by decide
def whichOfYouBurnsBrightestBody : Action :=
  simultaneously
    [ dealDamage (letter .x) targetOpponentOrPlaneswalker,
      dealDamage (letter .x) eachCreatureThatSplitControls ]
theorem okWhichOfYouBurnsBrightestBody :
    Action.check (DRS.ofList []) whichOfYouBurnsBrightestBody = [] := by decide
/-- Chandra, Pyromaster's plus -/
def chandraPyromasterBolt : Action :=
  simultaneously
    [ instruct this_ (dealDamage 1 targetPlayerOrPlaneswalker),
      instruct this_ (dealDamage 1
                         (targets (upTo 1)
                            (and
                               [isCreature, controlledBy splitOverPlaneswalker]))) ]
theorem okChandraPyromasterBolt : Action.check (DRS.ofList []) chandraPyromasterBolt = [] := by decide
/-- Ravager of the Fells -/
def ravagerOfTheFellsBolt : Action :=
  simultaneously
    [ instruct thisCreature (dealDamage 2 targetOpponentOrPlaneswalker),
      instruct thisCreature (dealDamage 2
                               (targets (upTo 1)
                                  (and
                                     [isCreature, controlledBy
                                        splitOverPlaneswalker]))) ]
theorem okRavagerOfTheFellsBolt : Action.check (DRS.ofList []) ravagerOfTheFellsBolt = [] := by decide
/-- Soul of Shandalar's battlefield activation -/
def soulOfShandalarBolt : Action :=
  simultaneously
    [ instruct thisCreature (dealDamage 3 targetPlayerOrPlaneswalker),
      instruct thisCreature (dealDamage 3
                               (targets (upTo 1)
                                  (and
                                     [isCreature, controlledBy
                                        splitOverPlaneswalker]))) ]
theorem okSoulOfShandalarBolt : Action.check (DRS.ofList []) soulOfShandalarBolt = [] := by decide
/-- Soul of Shandalar's graveyard activation -/
def soulOfShandalarGraveyardBolt : Action :=
  simultaneously
    [ instruct this_ (dealDamage 3 targetPlayerOrPlaneswalker),
      instruct this_ (dealDamage 3
                         (targets (upTo 1)
                            (and
                               [isCreature, controlledBy splitOverPlaneswalker]))) ]
theorem okSoulOfShandalarGraveyardBolt :
    Action.check (DRS.ofList []) soulOfShandalarGraveyardBolt = [] := by decide
/-- Blightning -/
def blightning : Action :=
  sequentially
    [ instruct this_ (dealDamage 3 targetPlayerOrPlaneswalker),
      discard (counted (exactly 2) (cardIn (handOf splitOverPlaneswalker))) ]
theorem okBlightning : Action.check (DRS.ofList []) blightning = [] := by decide
/-- Rakdos's Return -/
def rakdossReturn : Action :=
  sequentially
    [ instruct this_ (dealDamage (letter .x) targetOpponentOrPlaneswalker),
      discard (counted (exactlyOf (letter .x)) (cardIn (handOf splitOverPlaneswalker))) ]
theorem okRakdossReturn : Action.check (DRS.ofList []) rakdossReturn = [] := by decide
/-- Nicol Bolas, Planeswalker's ultimate -/
def nicolBolasUltimate : Action :=
  sequentially
    [ instruct this_ (dealDamage 7 targetPlayerOrPlaneswalker),
      discard (counted (exactly 7) (cardIn (handOf splitOverPlaneswalker))),
      instruct splitOverPlaneswalker (sacrifice (counted (exactly 7) isPermanent)) ]
theorem okNicolBolasUltimate : Action.check (DRS.ofList []) nicolBolasUltimate = [] := by decide
/-- Pulse of the Forge -/
def pulseOfTheForge : Action :=
  sequentially
    [ instruct this_ (dealDamage 4 targetPlayerOrPlaneswalker),
      doIf (compareAmt (lifeTotalOf splitOverPlaneswalker) .greater (lifeTotalOf you))
        (returnToHand this_) ]
theorem okPulseOfTheForge : Action.check (DRS.ofList []) pulseOfTheForge = [] := by decide
/-- Goblin Lyre's losing arm -/
def goblinLyreLoseFlip : Action :=
  sequentially
    [ instruct thisArtifact (dealDamage (countOf (and [isCreature, youControl])) targetOpponentOrPlaneswalker),
      instruct thisArtifact (dealDamage
                               (countOf
                                  (and
                                     [isCreature, controlledBy
                                        splitOverPlaneswalker])) you) ]
theorem okGoblinLyreLoseFlip : Action.check (DRS.ofList []) goblinLyreLoseFlip = [] := by decide
/-- Quenchable Fire -/
def quenchableFire : Action :=
  sequentially
    [ instruct this_ (dealDamage 3 targetPlayerOrPlaneswalker),
      delayUnlessPaid (beginningOf .upkeep you nextTime) (instruct this_ (dealDamage 3 thatJoin))
        splitOverPlaneswalker [pip .blue] ]
theorem okQuenchableFire : Action.check (DRS.ofList []) quenchableFire = [] := by decide
/-- Searing Blaze, both sentences -/
def searingBlaze : Ability :=
  landfall
    (spell (replace
        (instruct this_ (simultaneously
          [ dealDamage 1 targetPlayerOrPlaneswalker,
            dealDamage 1 (target
                                   (and
                                      [isCreature, controlledBy
                                         splitOverPlaneswalker])) ]))
        (doIf (did (a (and [isLand, youControl])) enter soFarThisTurn)
          (instead
            (instruct this_ (simultaneously
              [ dealDamage 3 thatJoin,
                dealDamage 3 (that creature) ]))))))
theorem okSearingBlaze : Ability.check (DRS.ofList []) searingBlaze = [] := by decide

/-- Heart of Bogardan -/
def heartOfBogardan : Spelled := spelled <| singleFacedCard
  { name := "Heart of Bogardan", cost := [generic 2, pip .red, pip .red], types := [.enchantment],
    text :=
      [ cumulativeUpkeep [generic 2],
        when heartOfBogardanHeader
          (sequentially
            [ simultaneously
                [ instruct this_ (dealDamage (letter .x) targetPlayerOrPlaneswalker),
                  instruct this_ (dealDamage (letter .x) eachCreatureThatSplitControls) ],
              define .x
                (minus (times 2 (countersOn ageCounter thisEnchantment))
                   2) ]) ] }

/-- Burn at the Stake -/
def burnAtTheStake : Spelled := spelled <| singleFacedCard
  { name := "Burn at the Stake", cost := [generic 2, pip .red, pip .red, pip .red],
    types := [.sorcery],
    text :=
      [ static (addedCost
          (tap (counted anyNumber (and [isCreature, youControl, isUntapped])))
          false),
        spell (instruct this_ (dealDamage (times 3 groupSize) (target isAnyTarget))) ] }

/-- Explosive Singularity -/
def explosiveSingularity : Spelled := spelled <| singleFacedCard
  { name := "Explosive Singularity", cost := [generic 8, pip .red, pip .red], types := [.sorcery],
    text :=
      [ static (addedCost
          (tap (counted anyNumber (and [isCreature, youControl, isUntapped])))
          true),
        static (costShift this_ (less (times 1 groupSize))),
        spell (instruct this_ (dealDamage 10 (target isAnyTarget))) ] }

/-- Tyrant of Valakut -/
def tyrantOfValakut : Spelled := spelled <| singleFacedCard
  { name := "Tyrant of Valakut", cost := [generic 5, pip .red, pip .red], types := [.creature],
    subtypes := [dragon],
    text :=
      [ surge [generic 3, pip .red, pip .red],
        flying,
        when (does thisCreature enter) (if_ := some (keywordCostPaid surge
                                                         thisCreature))
          (instruct thisCreature (dealDamage 3 (target isAnyTarget))) ],
    power := stat 5, toughness := stat 4 }

/-- Fall of the Titans -/
def fallOfTheTitansCard : Spelled := spelled <| singleFacedCard
  { name := "Fall of the Titans", cost := [.variable, .variable, pip .red], types := [.instant],
    text :=
      [ surge [.variable, pip .red],
        spell (instruct this_
          (dealDamage (letter .x) (eachOf (targets (upTo 2) isAnyTarget)))) ] }

/-- Tribal Flames -/
def tribalFlames : Spelled := spelled <| singleFacedCard
  { name := "Tribal Flames", cost := [generic 1, pip .red], types := [.sorcery],
    text :=
      [ domain
          (spell (sequentially
            [ instruct this_ (dealDamage (letter .x) (target isAnyTarget)),
              define .x (distinctCount (.subtype .land .basicOnly)
                (allOf (and [isLand, youControl]))) ])) ] }

/-- Explosive Prodigy -/
def explosiveProdigyTrigger : Ability :=
  vivid
    (when (does thisCreature enter)
      (sequentially
        [ instruct it
            (dealDamage (letter .x) (target (and [isCreature, anOpponentControls]))),
          define .x (distinctCount .color (allOf (and [isPermanent, youControl]))) ]))
theorem okExplosiveProdigyTrigger : Ability.check (DRS.ofList []) explosiveProdigyTrigger = [] := by decide
/-- Niv-Mizzet, Guildpact -/
def nivMizzetGuildpactTrigger : Ability :=
  whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := some .combat)))
    (sequentially
      [ instruct thisCreature (dealDamage (letter .x) (target isAnyTarget)),
        instruct (target isPlayer) (draw (letter .x)),
        gainLife (letter .x),
        define .x (distinctCount .colorPair
          (allOf (and [isPermanent, youControl, colorCount .eq 2]))) ])
theorem okNivMizzetGuildpactTrigger : Ability.check (DRS.ofList []) nivMizzetGuildpactTrigger = [] := by decide

/-- Aurelia, the Law Above -/
def aureliaTheLawAbove : Spelled := spelled <| singleFacedCard
  { name := "Aurelia, the Law Above", cost := [generic 3, pip .red, pip .white],
    supertypes := [.legendary], types := [.creature], subtypes := [angel],
    text :=
      [ flying,
        vigilance,
        haste,
        whenever (does (a isPlayer) (attackWith (counted (orMore 3) isCreature)))
            (draw 1),
        whenever (does (a isPlayer) (attackWith (counted (orMore 5) isCreature)))
          (sequentially [instruct this_ (dealDamage 3 eachOpponent), gainLife 3]) ],
    power := stat 4, toughness := stat 4 }

/-- Syr Konrad, the Grim -/
def syrKonradTheGrim : Spelled := spelled <| singleFacedCard
  { name := "Syr Konrad, the Grim", cost := some [generic 3, pip .black, pip .black],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, knight],
    text :=
      [ whenever (does (a (and [isCreature, isOther])) die)
          (or_ :=
            [ does any
                (putInto (a (and [isCard, isCreature])) graveyard
                  (from_ := some (not battlefield))),
            does (a (and [isCreature, in_ yourGraveyard])) (leave
                                                                      (from_ := yourGraveyard)) ])
          (instruct this_ (dealDamage 1 eachOpponent)),
        activated [generic 1, pip .black] (instruct eachPlayer (mill 1)) ],
    power := stat 5, toughness := stat 4 }

/-- Destructive Revelry -/
def destructiveRevelry : Spelled := spelled <| singleFacedCard
  { name := "Destructive Revelry", cost := [pip .red, pip .green], types := [.instant],
    text :=
      [ spell (sequentially
          [ destroy (target (or [isArtifact, isEnchantment])),
            instruct this_ (dealDamage 2 (controllerOf (that permanent))) ]) ] }

/-- Fuming Effigy -/
def fumingEffigy : Spelled := spelled <| singleFacedCard
  { name := "Fuming Effigy", cost := [generic 3, pip .red], types := [.creature],
    subtypes := [spirit],
    text :=
      [ whenever
          (does (counted (orMore 1) (cardIn yourGraveyard))
             (leave (from_ := yourGraveyard)))
          (instruct this_ (dealDamage 1 eachOpponent)) ],
    power := stat 4, toughness := stat 3 }

/-- Breeches, Brazen Plunderer's slice -/
def eachOfThoseOpponentsTopCard : Action :=
  sequentially
    [ instruct this_ (dealDamage 1 eachOpponent),
      exile (slice .top 1 (libraryOf (eachOf (those player)))) ]
theorem okEachOfThoseOpponentsTopCard : Action.check (DRS.ofList []) eachOfThoseOpponentsTopCard = [] := by
  decide

/-- Inquisitor's Flail -/
def inquisitorsFlail : Spelled := spelled <| singleFacedCard
  { name := "Inquisitor's Flail", cost := [generic 2], types := [.artifact],
    subtypes := [equipment],
    text :=
      [ static (damageRule (dealtBy (equipped creature)) everywhere
          (scale (multiplied .doubled)) .repeatedly (some .combat)),
        static (damageRule (dealtBy (a (and [isCreature, isOther])))
          (toRecipient (equipped creature)) (scale (multiplied .doubled))
          .repeatedly (some .combat)),
        equip [generic 2] ] }

/-- Oath of Kaya -/
def oathOfKaya : Spelled := spelled <| singleFacedCard
  { name := "Oath of Kaya", cost := [generic 1, pip .white, pip .black],
    supertypes := [.legendary], types := [.enchantment],
    text :=
      [ when (does this_ enter)
          (sequentially [instruct this_ (dealDamage 3 (target isAnyTarget)), gainLife 3]),
        whenever
          (does anOpponent
             (attackWith (counted (orMore 1) isCreature)
                (a (and [isPlaneswalker, youControl]))))
          (sequentially
             [instruct this_ (dealDamage 2 (that player)), gainLife
                2]) ] }

/-- Frostwielder -/
def frostwielder : Spelled := spelled <| singleFacedCard
  { name := "Frostwielder", cost := [generic 2, pip .red, pip .red], types := [.creature],
    subtypes := [human, shaman],
    text :=
      [ static
          (replace
            (would
              (a (and [ isCreature,
                        did thisCreature (dealDamage anyAmount which) soFarThisTurn ]))
              die)
            (instead (exile it))),
        activated tapSymbol
          (instruct thisCreature (dealDamage 1 (target isAnyTarget))) ],
    power := stat 1, toughness := stat 2 }

/-- Crackling Doom -/
def cracklingDoom : Action :=
  sequentially
    [ instruct this_ (dealDamage 2 eachOpponent),
      instruct eachOpponent (sacrifice
        (a (and [isCreature,
                  superlative .max (.stat .power)
                    (and [isCreature, controlledBy (that player)])]))) ]
theorem okCracklingDoom : Action.check (DRS.ofList []) cracklingDoom = [] := by decide
def theFallen : Ability :=
  at_ (beginningOfYour .upkeep)
    (instruct thisCreature (dealDamage 1 (each (and [or [isOpponent, isPlaneswalker],
                   did thisCreature (dealDamage anyAmount which) thisGame]))))
theorem okTheFallen : Ability.check (DRS.ofList []) theFallen = [] := by decide
/-- Cavalcade of Calamity -/
def cavalcadeOfCalamity : Ability :=
  whenever
    (does (a (and [isCreature, youControl, hasPower .atMost 1])) attack)
    (instruct thisEnchantment (dealDamage 1 (the (and [or [isPlayer, isPlaneswalker],
                  inCombat .attackedBy (that creature)]))))
theorem okCavalcadeOfCalamity : Ability.check (DRS.ofList []) cavalcadeOfCalamity = [] := by decide

/-- Blind Fury -/
def blindFury : Spelled := spelled <| singleFacedCard
  { name := "Blind Fury", cost := [generic 2, pip .red, pip .red], types := [.instant],
    text :=
      [ spell (sequentially
          [ establishThroughout (abilityLoss (allOf isCreature) [written trample]) untilEndOfTurn,
            establishThroughout (damageRule (dealtBy (a isCreature))
                                   (toRecipient (a isCreature))
                (scale (multiplied .doubled)) .repeatedly (some .combat)) thisTurn ]) ] }

/-- Chandra, Awakened Inferno's emblem -/
def chandraAwakenedInfernoEmblem : Action :=
  getEmblem
    [ at_ (beginningOfYour .upkeep)
        (instruct thisEmblem (dealDamage 1 you)) ]
theorem okChandraAwakenedInfernoEmblem :
    Action.check (DRS.ofList []) chandraAwakenedInfernoEmblem = [] := by decide

/-- Keeper of the Flame -/
def keeperOfTheFlame : Spelled := spelled <| singleFacedCard
  { name := "Keeper of the Flame", cost := [pip .red, pip .red], types := [.creature],
    subtypes := [human, wizard],
    text :=
      [ activated (compound [[pip .red], tapSymbol])
          (sequentially
            [ chooseWhile
                (target (and [isOpponent, compare .greater (lifeTotalOf you) (axes := [.playerStat .lifeTotal])]))
                (whileDoing (does you (activate thisAbility))),
              instruct thisCreature (dealDamage 2 (that player)) ]) ],
    power := stat 1, toughness := stat 2 }

def diabolicEdict : Action := instruct (target isPlayer) (sacrifice (aTheirChoice isCreature))
theorem okDiabolicEdict : Action.check (DRS.ofList []) diabolicEdict = [] := by decide
def innocentBlood : Action := instruct eachPlayer (sacrifice (aTheirChoice isCreature))
theorem okInnocentBlood : Action.check (DRS.ofList []) innocentBlood = [] := by decide
def cryOfContrition : Action := discard (a (cardIn (handOf (target isPlayer))))
theorem okCryOfContrition : Action.check (DRS.ofList []) cryOfContrition = [] := by decide
def cyclingCost : Action := discard this_
theorem okCyclingCost : Action.check (DRS.ofList []) cyclingCost = [] := by decide
def raiseTheAlarm : Action :=
  createToken (creatureToken 1 1 [.white] [soldier]) (count := 2)
theorem okRaiseTheAlarm : Action.check (DRS.ofList []) raiseTheAlarm = [] := by decide
def actOfTreason : Action := gainControl (target isCreature) untilEndOfTurn (controller :=
    you)
theorem okActOfTreason : Action.check (DRS.ofList []) actOfTreason = [] := by decide
def wordsOfWorship : Action :=
  replaceNextEvent (would you (draw 1)) (instead (gainLife 5)) thisTurn
theorem okWordsOfWorship : Action.check (DRS.ofList []) wordsOfWorship = [] := by decide
def theLastRoninII : Action :=
  triggerReflexively (mill 4)
    (returnToHand (target (and [isCreature, in_ yourGraveyard])))
theorem okTheLastRoninII : Action.check (DRS.ofList []) theLastRoninII = [] := by decide

def moonlitWakeCard : Spelled := spelled <| singleFacedCard
  { name := "Moonlit Wake", cost := [generic 2, pip .white], types := [.enchantment],
    text := [moonlitWake] }

def laquatussDisdain : Spelled := spelled <| singleFacedCard
  { name := "Laquatus's Disdain", cost := [generic 1, pip .blue], types := [.instant],
    text :=
      [ spell (sequentially
          [ counter (target (and [isSpell, castFrom graveyard])), draw 1 ]) ] }

/-- Stifle -/
def stifle : Spelled := spelled <| singleFacedCard
  { name := "Stifle", cost := [pip .blue], types := [.instant],
    text :=
      [ spell (counter (target (or [abilityHead .anyActivated, abilityHead .anyTriggered]))) ] }

/-- Disallow -/
def disallow : Spelled := spelled <| singleFacedCard
  { name := "Disallow", cost := [generic 1, pip .blue, pip .blue], types := [.instant],
    text :=
      [ spell (counter
            (target (or [isSpell, abilityHead .anyActivated, abilityHead .anyTriggered]))) ] }

/-- Fry -/
def fry : Spelled := spelled <| singleFacedCard
  { name := "Fry", cost := [generic 1, pip .red], types := [.instant],
    text :=
      [ static (cant any (counter thisSpell)),
        spell (instruct this_ (dealDamage 5
                 (target
                    (and
                       [or [isCreature, isPlaneswalker], or
                          [hasColor .white, hasColor .blue]])))) ] }

/-- Termination Facilitator -/
def terminationFacilitator : Spelled := spelled <| singleFacedCard
  { name := "Termination Facilitator", cost := [generic 1, pip .black], types := [.creature],
    subtypes := [human, assassin],
    text :=
      [ activatedOnlyDuring tapSymbol
          (putCounters 1 bountyCounter
            (target (or [isCreature, isPlaneswalker])))
          asSorcery,
        whenever
          (does any
             (dealDamage (to := a (and [or [isCreature, isPlaneswalker], anOpponentControls,
                      hasCounter bountyCounter]))))
          (destroy it) ],
    power := stat 1, toughness := stat 3 }

end Semantics.Cards
