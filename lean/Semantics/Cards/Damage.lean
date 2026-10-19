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
def bolt : Action := dealDamage (.lit 3) (target isAnyTarget) .noncombatOnly
theorem okBolt : Action.check (DRS.ofList []) bolt = [] := by decide
def barrageOfBoulders : Action := dealDamage (.lit 1) (each (and [isCreature, youDontControl])) .noncombatOnly
theorem okBarrageOfBoulders : Action.check (DRS.ofList []) barrageOfBoulders = [] := by decide
def rabidBite : Action :=
  instruct (target (and [isCreature, youControl])) (dealDamage (powerOf it)
                                                      (target
                                                         (and
                                                            [isCreature,
                                                             youDontControl])) .noncombatOnly)
theorem okRabidBite : Action.check (DRS.ofList []) rabidBite = [] := by decide
def preyUpon : Action := instruct (target (and [isCreature, youControl])) (fight (target (and [isCreature, youDontControl])))
theorem okPreyUpon : Action.check (DRS.ofList []) preyUpon = [] := by decide
def arcTrail : Action :=
  sequentially
    [ dealDamage (.lit 2) (target isAnyTarget) .noncombatOnly, dealDamage (.lit 1) (target isAnyOtherTarget) .noncombatOnly ]
theorem okArcTrail : Action.check (DRS.ofList []) arcTrail = [] := by decide
def deadshot : Action :=
  sequentially
    [ setStatus .tapped (target isCreature),
      instruct it (dealDamage (powerOf it) (target (and [isCreature, isOther])) .noncombatOnly) ]
theorem okDeadshot : Action.check (DRS.ofList []) deadshot = [] := by decide
def immersturmSkullcairn : Ability :=
  activatedOnlyDuring
    (compound [mana [generic 1, pip .black, pip .red, pip .red], tapSymbol,
      perform (sacrifice thisLand)])
    (sequentially
      [ dealDamage (.lit 3) (target isPlayer) .noncombatOnly, discard (a (cardIn (handOf (that
          .player)))) ])
    asSorcery
theorem okImmersturmSkullcairn : Ability.check (DRS.ofList []) immersturmSkullcairn = [] := by decide
def pyriteSpellbomb : Ability :=
  activated (compound [mana [pip .red], perform (sacrifice thisArtifact)])
    (dealDamage (.lit 2) (target isAnyTarget) .noncombatOnly)
theorem okPyriteSpellbomb : Ability.check (DRS.ofList []) pyriteSpellbomb = [] := by decide
def karplusanYeti : Action :=
  sequentially
    [ dealDamageOwnPower thisCreature (target isCreature),
      instruct (that creature) (dealDamage (powerOf it) thisCreature .noncombatOnly) ]
theorem okKarplusanYeti : Action.check (DRS.ofList []) karplusanYeti = [] := by decide
def suddenDemise : Action :=
  sequentially
    [ choose (a (quality .color)),
      dealDamage (letter .x) (each (and [isCreature, ofChosen .color])) .noncombatOnly ]
theorem okSuddenDemise : Action.check (DRS.ofList []) suddenDemise = [] := by decide
def caseOfTheGatewayExpress : Action :=
  sequentially
    [ choose (target (and [isCreature, youDontControl])),
      instruct (each (and [isCreature, youControl])) (dealDamage (.lit 1) (that creature) .noncombatOnly) ]
theorem okCaseOfTheGatewayExpress : Action.check (DRS.ofList []) caseOfTheGatewayExpress = [] := by decide
def arrowsOfJustice : Action :=
  dealDamage (.lit 4) (target (and [isCreature, or [isAttacking, isBlocking]])) .noncombatOnly
theorem okArrowsOfJustice : Action.check (DRS.ofList []) arrowsOfJustice = [] := by decide
def flamesOfTheRazeBoar : Action :=
  sequentially
    [ dealDamage (.lit 4) (target (and [isCreature, controlledBy anOpponent])) .noncombatOnly,
      doOnlyIf
        (dealDamage (.lit 2) (each (and [isCreature, isOther, controlledBy (that player)]))
           .noncombatOnly)
        (exists_ (and [isCreature, youControl, hasPower .atLeast (.lit 4)])) ]
theorem okFlamesOfTheRazeBoar : Action.check (DRS.ofList []) flamesOfTheRazeBoar = [] := by decide
def yawgmothDemon : Action :=
  mayOrElse (sacrifice (a isArtifact)) (sequentially
                                          [setStatus .tapped thisCreature, instruct «this»
                                             (dealDamage (.lit 2) you .noncombatOnly)])
theorem okYawgmothDemon : Action.check (DRS.ofList []) yawgmothDemon = [] := by decide
def arcBlade : Action :=
  sequentially
    [ dealDamage (.lit 2) (target isAnyTarget) .noncombatOnly, exileWithCounters «this» (.lit 3) (.named "timeCounter") ]
theorem okArcBlade : Action.check (DRS.ofList []) arcBlade = [] := by decide
def abrade : Action :=
  chooseOne (dealDamage (.lit 3) (target isCreature) .noncombatOnly) (destroy (target isArtifact))
theorem okAbrade : Action.check (DRS.ofList []) abrade = [] := by decide
def nibelheimAflame : Action :=
  sequentially
    [ choose (target (and [isCreature, youControl])),
      instruct it (dealDamage (powerOf it) (each (and [isCreature, otherThan it]))
                     .noncombatOnly) ]
theorem okNibelheimAflame : Action.check (DRS.ofList []) nibelheimAflame = [] := by decide
def brashTaunter : Action := instruct thisCreature (fight (target (and [isCreature, otherThan thisCreature])))
theorem okBrashTaunter : Action.check (DRS.ofList []) brashTaunter = [] := by decide
def ulvenwaldTracker : Action :=
  instruct (target (and [isCreature, youControl])) (fight (target (and [isCreature, isOther])))
theorem okUlvenwaldTracker : Action.check (DRS.ofList []) ulvenwaldTracker = [] := by decide

def botBashingTime : Action :=
  sequentially
    [ dealDamage (.lit 6) (target isCreature) .noncombatOnly,
      replaceEvent (would any (move (that creature) battlefield (some graveyard) .departure)) (exile it) (some thisTurn) ]
theorem okBotBashingTime : Action.check (DRS.ofList []) botBashingTime = [] := by decide
def wordsOfWar : Action :=
  replaceNextEvent (would you (draw anyAmount)) (instruct «this» (dealDamage (.lit 2) (target isAnyTarget) .noncombatOnly)) (some thisTurn)
theorem okWordsOfWar : Action.check (DRS.ofList []) wordsOfWar = [] := by decide

/-- Thunderwave -/
def thunderwave : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thunderwave", cost := some [generic 2, pip .red, pip .red], types := [.sorcery],
      text :=
        [ spell (sequentially
            [ rollDice (.lit 1) (.sides 20),
              applyResultsTable
                [ rollRow (oneThrough 9) (dealDamage (.lit 3) (each isCreature) .noncombatOnly),
                  rollRow (fromTo 10 19)
                    (sequentially
                      [ may (choose (a isCreature)),
                        dealDamage (.lit 3) (each (and [isCreature, notChosen])) .noncombatOnly ]),
                  rollRow (exactly 20)
                    (dealDamage (.lit 6) (each (and [isCreature, opponentsControl]))
                       .noncombatOnly) ] ]) ] } }

/-- Fog, Holy Day, Darkness, Root Snare -/
def fog : Action := preventAll .combatOnly everywhere (some thisTurn)
theorem okFog : Action.check (DRS.ofList []) fog = [] := by decide
/-- Indestructible Aura, Shielded Passage -/
def indestructibleAura : Action :=
  preventAll .any (toRecipient (target isCreature)) (some thisTurn)
theorem okIndestructibleAura : Action.check (DRS.ofList []) indestructibleAura = [] := by decide
def shieldmatesBlessing : Action :=
  preventNext .any (toRecipient (target isAnyTarget)) (.lit 3) (some thisTurn)
theorem okShieldmatesBlessing : Action.check (DRS.ofList []) shieldmatesBlessing = [] := by decide
def moonlitWake : Ability := whenever (does (a isCreature) die) (gainLife (.lit 1))
theorem okMoonlitWake : Ability.check (DRS.ofList []) moonlitWake = [] := by decide
def eliteJavelineer : Ability :=
  whenever (does thisCreature block) (instruct «this» (dealDamage (.lit 1) (target (and [isCreature, isAttacking])) .noncombatOnly))
theorem okEliteJavelineer : Ability.check (DRS.ofList []) eliteJavelineer = [] := by decide
/-- Glacial Chasm -/
def glacialChasmShield : Ability :=
  static (damageRule .any unattributed (toRecipient you) (prevent allDamage) .repeatedly)
theorem okGlacialChasmShield : Ability.check (DRS.ofList []) glacialChasmShield = [] := by decide
def corneredCrook : Ability :=
  when (does thisCreature enter)
    (offerWhen (sacrifice (a isArtifact)) (instruct «this» (dealDamage (.lit 3) (target
        isAnyTarget) .noncombatOnly)))
theorem okCorneredCrook : Ability.check (DRS.ofList []) corneredCrook = [] := by decide

def aladdinsRing : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aladdin's Ring", cost := some [generic 8], types := [.artifact],
      text :=
        [ activated (compound [mana [generic 8], tapSymbol])
            (instruct thisArtifact (dealDamage (.lit 4) (target isAnyTarget) .noncombatOnly)) ] } }

def chandrasRevolution : Action :=
  sequentially
    [ dealDamage (.lit 4) (target isCreature) .noncombatOnly, setStatus .tapped (target isLand),
      skipUntap (that land) (.lit 1) ]
theorem okChandrasRevolution : Action.check (DRS.ofList []) chandrasRevolution = [] := by decide
def arbalestElite : Ability :=
  activated (compound [mana [generic 2, pip .white], tapSymbol])
    (sequentially
      [ instruct thisCreature (dealDamage (.lit 3) (target (and [isCreature, or [isAttacking, isBlocking]])) .noncombatOnly),
        skipUntap thisCreature (.lit 1) ])
theorem okArbalestElite : Ability.check (DRS.ofList []) arbalestElite = [] := by decide
def sizzlingBarrage : Action :=
  dealDamage (.lit 4) (target
    (and [isCreature, happenedTo (asMarker .permanent (relative .object)) block soFarThisTurn])) .noncombatOnly
theorem okSizzlingBarrage : Action.check (DRS.ofList []) sizzlingBarrage = [] := by decide
def goadedAttackTrigger : Ability :=
  whenever (does (a (and [isCreature, hasDesignation (.named "goaded")])) attack)
    (instruct it (dealDamage (.lit 1) (controllerOf it) .noncombatOnly))
theorem okGoadedAttackTrigger : Ability.check (DRS.ofList []) goadedAttackTrigger = [] := by decide

def extraArms : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Extra Arms", cost := some [generic 4, pip .red], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ keywordSubject (.named "enchant") isCreature,
          whenever (does (enchanted creature) attack)
            (instruct it (dealDamage (.lit 2) (target isAnyTarget) .noncombatOnly)) ] } }

def chainReaction : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chain Reaction", cost := some [generic 2, pip .red, pip .red], types := [.sorcery],
      text :=
        [ spell (sequentially
            [ dealDamage (letter .x) (each isCreature) .noncombatOnly, define .x (countOf isCreature) ]) ] } }

/-- Black Vise -/
def blackVise : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Black Vise", cost := some [generic 1], types := [.artifact],
      text :=
        [ static (entersChoosingPlayer thisArtifact (some (players isOpponent))),
          at_ (beginningOfPossessed .upkeep (the isChosenPlayer))
            (sequentially
              [ instruct thisArtifact (dealDamage (letter .x) (that player) .noncombatOnly),
                define .x (minus (countOf (cardIn (handOf they))) (.lit 4)) ]) ] } }

def harshSustenance : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Harsh Sustenance", cost := some [generic 1, pip .white, pip .black], types := [.instant],
      text :=
        [ spell (sequentially
            [ dealDamage (letter .x) (target isAnyTarget) .noncombatOnly,
              gainLife (letter .x),
              define .x (countOf (and [isCreature, youControl])) ]) ] } }

/-- Purging Scythe -/
def purgingScythe : Ability :=
  at_ (beginningOfYour .upkeep)
    (sequentially
      [ instruct thisArtifact (dealDamage (.lit 2) (the (and [isCreature, superlative .min (.stat .toughness) isCreature])) .noncombatOnly),
        doIf
          (compareAmt (countOf (and [isCreature, superlative .min (.stat .toughness) isCreature]))
            .atLeast (.lit 2))
          (choose (someOf (exactly 1) them)) ])
theorem okPurgingScythe : Ability.check (DRS.ofList []) purgingScythe = [] := by decide

def lionHeart : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lion Heart", cost := some [generic 4], types := [.artifact],
      subtypes := [equipment],
      text :=
        [ when (does thisEquipment enter) (instruct it (dealDamage (.lit 2) (target isAnyTarget) .noncombatOnly)),
          static (getsPt (equipped creature) (up (.lit 2)) (up (.lit 1))),
          keywordCosting (.named "equip") (mana [generic 2]) ] } }

def lightmineField : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lightmine Field", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ whenever (does (counted (atLeast 1) isCreature) attack)
            (instruct thisEnchantment (dealDamage (numberOf (those creature)) (eachOf (those creature)) .noncombatOnly)) ] } }

def ingeniousArtillerist : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ingenious Artillerist", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [human, artificer],
      text :=
        [ whenever (does (counted (atLeast 1) (and [isArtifact, youControl])) enter)
            (instruct thisCreature (dealDamage groupSize eachOpponent .noncombatOnly)) ],
      power := stat 3, toughness := stat 1 } }

/-- Inferno Elemental -/
def infernoElemental : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Inferno Elemental", cost := some [generic 4, pip .red, pip .red], types := [.creature],
      subtypes := [elemental],
      text :=
        [ triggeredOr (does thisCreature (block (a isCreature)))
            [does (by_ (a isCreature)) (block thisCreature)]
            (instruct thisCreature (dealDamage (.lit 3) (that creature) .noncombatOnly)) ],
      power := stat 4, toughness := stat 4 } }

/-- Psychic Purge -/
def psychicPurge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Psychic Purge", cost := some [pip .blue], types := [.sorcery],
      text :=
        [ spell (dealDamage (.lit 1) (target isAnyTarget) .noncombatOnly),
          when
            (causes
              (bySource (a (and [or [isSpell, abilityHead .anyOnStack],
                                 controlledBy anOpponent])))
              (does you (discard «this»)))
            (instruct (that player) (loseLife (.lit 5))) ] } }

/-- Chandra Nalaar -/
def chandraNalaarsX : Ability :=
  activated (loyaltySymbol .downX) (instruct «this» (dealDamage (letter .x) (target isCreature) .noncombatOnly))
theorem okChandraNalaarsX : Ability.check (DRS.ofList []) chandraNalaarsX = [] := by decide

def etherealHaze : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ethereal Haze", cost := some [pip .white], types := [.instant],
      subtypes := [arcane],
      text := [spell (preventAllBy .any (allOf isCreature) everywhere (some thisTurn))] } }

def defang : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Defang", cost := some [generic 1, pip .white], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ keywordSubject (.named "enchant") isCreature,
          static (damageRule .any (dealtBy (enchanted creature)) everywhere
            (prevent allDamage) .repeatedly) ] } }

def sphereOfPurity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sphere of Purity", cost := some [generic 3, pip .white], types := [.enchantment],
      text :=
        [ static (damageRule .any (dealtBy (a isArtifact)) (toRecipient you)
            (prevent (ofThatDamage (.lit 1))) .repeatedly) ] } }

def dazzlingReflection : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dazzling Reflection", cost := some [generic 1, pip .white], types := [.instant],
      text :=
        [ spell (sequentially
            [ gainLife (powerOf (target isCreature)),
              establishThroughout (damageRule .any (dealtBy (that creature)) everywhere
                                     (prevent allDamage)
                  .nextTimeOnly) thisTurn ]) ] } }

def thunderstaff : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thunderstaff", cost := some [generic 3], types := [.artifact],
      text :=
        [ static (onlyWhile
            (damageRule .combatOnly (dealtBy (a isCreature)) (toRecipient you)
              (prevent (ofThatDamage (.lit 1))) .repeatedly)
            (matches_ thisArtifact isUntapped)),
          activated (compound [mana [generic 2], tapSymbol])
            (get (allOf (and [isCreature, isAttacking])) (up (.lit 1)) (up (.lit 0))
              (some untilEndOfTurn)) ] } }

def gideonAllyOfZendikar : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gideon, Ally of Zendikar", cost := some [generic 2, pip .white, pip .white],
      supertypes := [.legendary], types := [.planeswalker], subtypes := [gideon],
      text :=
        [ activated (loyaltySymbol (.up 1))
            (sequentially
              [ establishThroughout (qualityChange thisPlaneswalker .sets
                    (bundle
                      { characteristics :=
                        { types := [.creature],
                          subtypes := [human, soldier, ally],
                          text := [indestructible], power := stat 5, toughness := stat 5 } }
                      (some .planeswalker))) untilEndOfTurn,
                preventAll .any (toRecipient it) (some thisTurn) ]),
          activated (loyaltySymbol .zero)
            (createToken (creatureToken 2 2 [.white] [knight, ally])),
          activated (loyaltySymbol (.down 4))
            (getEmblem
              [static (getsPt (allOf (and [isCreature, youControl])) (up (.lit 1)) (up (.lit 1)))]) ],
      loyalty := stat 4 } }

def turnTheTables : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Turn the Tables", cost := some [generic 3, pip .white, pip .white], types := [.instant],
      text :=
        [ spell (establishThroughout (damageRule .combatOnly unattributed (toRecipient you)
              (redirect allDamage
                 (target (and [isCreature, isAttacking]))) .repeatedly) thisTurn) ] } }

def pariah : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pariah", cost := some [generic 2, pip .white], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ keywordSubject (.named "enchant") isCreature,
          static (damageRule .any unattributed (toRecipient you)
            (redirect allDamage (enchanted creature)) .repeatedly) ] } }

def martyrsOfKorlis : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Martyrs of Korlis", cost := some [generic 3, pip .white, pip .white],
      types := [.creature], subtypes := [human],
      text :=
        [ static (onlyWhile
            (damageRule .any (dealtBy (allOf isArtifact)) (toRecipient you)
              (redirect allDamage thisCreature) .repeatedly)
            (matches_ thisCreature isUntapped)) ],
      power := stat 1, toughness := stat 6 } }

def wardOfPiety : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ward of Piety", cost := some [generic 1, pip .white], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ keywordSubject (.named "enchant") isCreature,
          activated (mana [generic 1, pip .white])
            (establishThroughout
               (damageRule .any unattributed (toRecipient (enchanted creature))
                (redirect (theNext (.lit 1)) (target isAnyTarget)) .repeatedly) thisTurn) ] } }

def mirrorwoodTreefolk : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mirrorwood Treefolk", cost := some [generic 3, pip .green], types := [.creature],
      subtypes := [treefolk],
      text :=
        [ activated (mana [generic 2, pip .red, pip .white])
            (establishThroughout (damageRule .any unattributed (toRecipient thisCreature)
                (redirect allDamage (target isAnyTarget)) .nextTimeOnly) thisTurn) ],
      power := stat 2, toughness := stat 4 } }

def carom : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Carom", cost := some [generic 1, pip .white], types := [.instant],
      text :=
        [ spell (sequentially
            [ establishThroughout (damageRule .any unattributed (toRecipient (target isCreature))
                  (redirect (theNext (.lit 1))
                     (target (and [isCreature, isOther]))) .repeatedly) thisTurn,
              draw (.lit 1) ]) ] } }

def daughterOfAutumn : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Daughter of Autumn", cost := some [generic 2, pip .green, pip .green],
      supertypes := [.legendary], types := [.creature], subtypes := [avatar],
      text :=
        [ activated (mana [pip .white])
            (establishThroughout
               (damageRule .any unattributed
                  (toRecipient (target (and [isCreature, hasColor .white])))
                (redirect (theNext (.lit 1)) thisCreature) .repeatedly) thisTurn) ],
      power := stat 2, toughness := stat 4 } }

def aegisOfHonor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aegis of Honor", cost := some [pip .white], types := [.enchantment],
      text :=
        [ activated (mana [generic 1])
            (establishThroughout (damageRule .any (dealtBy (a isInstantOrSorcery)) (toRecipient you)
                (redirect allDamage (controllerOf it)) .nextTimeOnly) thisTurn) ] } }

def candlesGlow : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Candles' Glow", cost := some [generic 1, pip .white], types := [.instant],
      subtypes := [arcane],
      text :=
        [ spell
            (establishThroughout (damageRule .any unattributed (toRecipient (target isAnyTarget))
              (prevent (theNext (.lit 3)) (some (gainLife preventedThisWay)))
                  .repeatedly) thisTurn) ] } }

/-- Inkshield -/
def inkshieldRider : Action :=
  establishThroughout (damageRule .any unattributed (toRecipient you)
      (prevent allDamage (some (createToken (creatureToken 2 1 [.white, .black] []) (count := preventedThisWay))))
      .repeatedly) thisTurn
theorem okInkshieldRider : Action.check (DRS.ofList []) inkshieldRider = [] := by decide

def urzasArmor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Urza's Armor", cost := some [generic 6], types := [.artifact],
      text :=
        [ static (damageRule .any (dealtBy (a isSource)) (toRecipient you)
            (prevent (ofThatDamage (.lit 1))) .repeatedly) ] } }

def circleOfProtectionRed : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Circle of Protection: Red", cost := some [generic 1, pip .white], types := [.enchantment],
      text :=
        [ activated (mana [generic 1])
            (establishThroughout
               (damageRule .any (dealtBy (aYourChoice (and [isSource, hasColor .red])))
                  (toRecipient you)
                (prevent allDamage) .nextTimeOnly) thisTurn) ] } }

def healingGrace : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Healing Grace", cost := some [pip .white], types := [.instant],
      text :=
        [ spell (sequentially
            [ establishThroughout
                (damageRule .any (dealtBy (aYourChoice isSource)) (toRecipient (target isAnyTarget))
                  (prevent (theNext (.lit 3))) .repeatedly) thisTurn,
              gainLife (.lit 3) ]) ] } }

def reverseDamage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Reverse Damage", cost := some [generic 1, pip .white, pip .white], types := [.instant],
      text :=
        [ spell
            (establishThroughout (damageRule .any (dealtBy (aYourChoice isSource)) (toRecipient you)
              (prevent allDamage (some (gainLife preventedThisWay))) .nextTimeOnly) thisTurn) ] } }

def deflectingPalm : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Deflecting Palm", cost := some [pip .red, pip .white], types := [.instant],
      text :=
        [ spell
            (establishThroughout (damageRule .any (dealtBy (aYourChoice isSource)) (toRecipient you)
              (prevent allDamage
                 (some
                    (instruct «this»
                       (dealDamage thatMuch (controllerOf it)
                          .noncombatOnly)))) .nextTimeOnly) thisTurn) ] } }

def templeAltisaur : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Temple Altisaur", cost := some [generic 4, pip .white], types := [.creature],
      subtypes := [dinosaur],
      text :=
        [ static (damageRule .any (dealtBy (a isSource))
            (toRecipient (a (and [hasSubtype dinosaur, youControl])))
            (prevent (allBut (.lit 1))) .repeatedly) ],
      power := stat 3, toughness := stat 4 } }

def darkSphere : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dark Sphere", cost := some [], types := [.artifact],
      text :=
        [ activated (compound [tapSymbol, perform (sacrifice thisArtifact)])
            (establishThroughout (damageRule .any (dealtBy (aYourChoice isSource)) (toRecipient you)
                (prevent (half .down)) .nextTimeOnly) thisTurn) ] } }

def shadowbane : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shadowbane", cost := some [generic 1, pip .white], types := [.instant],
      text :=
        [ spell (establishThroughout (damageRule .any (dealtBy (aYourChoice isSource))
              (toRecipient (youAnd (allOf (and [isCreature, youControl]))))
              (prevent allDamage
                (some (doIf (preventedFromSource (and [isSource, hasColor .black]))
                  (gainLife preventedThisWay))))
              .nextTimeOnly) thisTurn) ] } }

def sphereOfLaw : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sphere of Law", cost := some [generic 3, pip .white], types := [.enchantment],
      text :=
        [ static (damageRule .any (dealtBy (a (and [isSource, hasColor .red]))) (toRecipient you)
            (prevent (ofThatDamage (.lit 2))) .repeatedly) ] } }

def lavaAxe : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lava Axe", cost := some [generic 4, pip .red], types := [.sorcery],
      text :=
        [ spell (dealDamage (.lit 5) (target (or [isPlaneswalker, isPlayer])) .noncombatOnly) ] } }

def searingFlesh : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Searing Flesh", cost := some [generic 6, pip .red], types := [.sorcery],
      text :=
        [ spell (dealDamage (.lit 7) (target (or [isPlaneswalker, isOpponent])) .noncombatOnly)
            ] } }

def onakkeJavelineerBolt : Ability :=
  activated tapSymbol
    (instruct thisCreature (dealDamage (.lit 2) (target (or [isBattle, isPlayer])) .noncombatOnly))
theorem okOnakkeJavelineerBolt : Ability.check (DRS.ofList []) onakkeJavelineerBolt = [] := by decide

/-- Firesong and Sunspeaker -/
def firesongJoinEcho : Action :=
  sequentially
    [ instruct «this» (dealDamage (.lit 3) (target (or [isCreature, isPlayer])) .noncombatOnly),
      instruct «this» (dealDamage (.lit 1) thatJoin .noncombatOnly) ]
theorem okFiresongJoinEcho : Action.check (DRS.ofList []) firesongJoinEcho = [] := by decide

/-- Forcefield -/
def forcefield : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Forcefield", cost := some [generic 3], types := [.artifact],
      text :=
        [ activated (mana [generic 1])
            (establishThroughout
               (damageRule .combatOnly (dealtBy (aYourChoice (and [isCreature, isUnblocked])))
                (toRecipient you) (prevent (allBut (.lit 1))) .nextTimeOnly) thisTurn) ] } }

def endure : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Endure", cost := some [generic 3, pip .white, pip .white], types := [.instant],
      text :=
        [ spell (preventAll .any
              (toRecipient (youAnd (allOf (and [isPermanent, youControl]))))
              (some thisTurn)) ] } }

def harmsWay : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Harm's Way", cost := some [pip .white], types := [.instant],
      text :=
        [ spell (establishThroughout (damageRule .any (dealtBy (aYourChoice isSource))
              (toRecipient (youAnd (allOf (and [isPermanent, youControl]))))
              (redirect (theNext (.lit 2)) (target isAnyTarget)) .repeatedly) thisTurn) ] } }

def divineDeflection : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Divine Deflection", cost := some [.variable, pip .white], types := [.instant],
      text :=
        [ spell (establishThroughout (damageRule .any unattributed
              (toRecipient (youAnd (allOf (and [isPermanent, youControl]))))
              (prevent (theNext (letter .x)) (some (instruct «this» (dealDamage thatMuch (target isAnyTarget) .noncombatOnly))))
              .repeatedly) thisTurn) ] } }

def glarecasterShield : Ability :=
  activated (mana [generic 5, pip .white])
    (establishThroughout (damageRule .any unattributed (toRecipient (youAnd thisCreature))
        (redirect allDamage (target isAnyTarget)) .nextTimeOnly) thisTurn)
theorem okGlarecasterShield : Ability.check (DRS.ofList []) glarecasterShield = [] := by decide

def furnaceOfRath : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Furnace of Rath", cost := some [generic 1, pip .red, pip .red, pip .red],
      types := [.enchantment],
      text :=
        [ static (damageRule .any (dealtBy (a isSource))
            (toRecipient (a (or [isPermanent, isPlayer]))) (scale (multiplied .doubled))
            .repeatedly) ] } }

def gratuitousViolence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gratuitous Violence", cost := some [generic 2, pip .red, pip .red, pip .red],
      types := [.enchantment],
      text :=
        [ static (damageRule .any (dealtBy (a (and [isCreature, youControl])))
            (toRecipient (a (or [isPermanent, isPlayer]))) (scale (multiplied .doubled))
            .repeatedly) ] } }

def fieryEmancipation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fiery Emancipation", cost := some [generic 3, pip .red, pip .red, pip .red],
      types := [.enchantment],
      text :=
        [ static (damageRule .any (dealtBy (a (and [isSource, youControl])))
            (toRecipient (a (or [isPermanent, isPlayer]))) (scale (multiplied .tripled))
            .repeatedly) ] } }

def sulfuricVapors : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sulfuric Vapors", cost := some [generic 3, pip .red], types := [.enchantment],
      text :=
        [ static (damageRule .any (dealtBy (a (and [isSpell, hasColor .red])))
            (toRecipient (a (or [isPermanent, isPlayer]))) (scale (shifted .up (.lit 1)))
            .repeatedly) ] } }

def lashknifeBarrier : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lashknife Barrier", cost := some [generic 2, pip .white], types := [.enchantment],
      text :=
        [ when (does thisEnchantment enter) (draw (.lit 1)),
          static (damageRule .any (dealtBy (a isSource)) (toRecipient (a (and [isCreature, youControl])))
            (scale (shifted .down (.lit 1))) .repeatedly) ] } }

def ghostsOfTheInnocent : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ghosts of the Innocent", cost := some [generic 5, pip .white, pip .white],
      types := [.creature], subtypes := [spirit],
      text :=
        [ static (damageRule .any (dealtBy (a isSource))
            (toRecipient (a (or [isPermanent, isPlayer]))) (scale (halved .down)) .repeatedly) ],
      power := stat 4, toughness := stat 5 } }

def fireServant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fire Servant", cost := some [generic 3, pip .red, pip .red], types := [.creature],
      subtypes := [elemental],
      text :=
        [ static (damageRule .any
            (dealtBy (a (and [isInstantOrSorcery, hasColor .red, youControl])))
            everywhere (scale (multiplied .doubled)) .repeatedly) ],
      power := stat 4, toughness := stat 3 } }

def platedPegasus : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Plated Pegasus", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [pegasus],
      text :=
        [ flash,
          flying,
          static (damageRule .any (dealtBy (a isSpell)) (toRecipient (a (or [isPermanent, isPlayer])))
            (prevent (ofThatDamage (.lit 1))) .repeatedly) ],
      power := stat 1, toughness := stat 2 } }

def spitemare : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Spitemare", cost := some [generic 2, hybridPip .red .white, hybridPip .red .white],
      types := [.creature], subtypes := [elemental],
      text :=
        [ whenever (does any (dealDamage (to := thisCreature))) (instruct it (dealDamage thatMuch (target isAnyTarget) .noncombatOnly)) ],
      power := stat 3, toughness := stat 3 } }

def grollub : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grollub", cost := some [generic 2, pip .black], types := [.creature],
      subtypes := [beast],
      text := [whenever (does any (dealDamage (to := thisCreature))) (instruct eachOpponent (gainLife thatMuch))],
      power := stat 3, toughness := stat 3 } }

def moggManiac : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mogg Maniac", cost := some [generic 1, pip .red], types := [.creature],
      subtypes := [goblin],
      text :=
        [ whenever (does any (dealDamage (to := thisCreature)))
            (instruct it
               (dealDamage thatMuch (target (or [isPlaneswalker, isOpponent])) .noncombatOnly)) ],
      power := stat 1, toughness := stat 1 } }

def repercussion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Repercussion", cost := some [generic 1, pip .red, pip .red], types := [.enchantment],
      text :=
        [ whenever (does any (dealDamage (to := a isCreature)))
            (instruct thisEnchantment (dealDamage thatMuch (controllerOf (that creature)) .noncombatOnly)) ] } }

def spitefulShadows : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Spiteful Shadows", cost := some [generic 1, pip .black], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ keywordSubject (.named "enchant") isCreature,
          whenever (does any (dealDamage (to := enchanted creature)))
            (instruct it (dealDamage thatMuch (controllerOf it) .noncombatOnly)) ] } }

def bindingAgony : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Binding Agony", cost := some [generic 1, pip .black], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ keywordSubject (.named "enchant") isCreature,
          whenever (does any (dealDamage (to := enchanted creature)))
            (instruct thisAura (dealDamage thatMuch (controllerOf (that creature)) .noncombatOnly)) ] } }

def darienKingOfKjeldor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Darien, King of Kjeldor", cost := some [generic 4, pip .white, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [human, soldier],
      text :=
        [ whenever (does any (dealDamage (to := you)))
            (may (createToken (creatureToken 1 1 [.white] [soldier]) (count := thatMuch))) ],
      power := stat 3, toughness := stat 3 } }

def screamingNemesis : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Screaming Nemesis", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [spirit],
      text :=
        [ haste,
          whenever (does any (dealDamage (to := thisCreature)))
            (sequentially
              [ instruct it (dealDamage thatMuch (target (and [isAnyTarget, otherThan «this»])) .noncombatOnly),
                doIf (dealtThisWay isPlayer)
                  (establishThroughout (cant they (gainLife anyAmount)) restOfGame) ]) ],
      power := stat 3, toughness := stat 3 } }

def sonicShrieker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sonic Shrieker", cost := some [generic 2, pip .red, pip .white, pip .black],
      types := [.creature], subtypes := [dragon],
      text :=
        [ flying,
          when (does thisCreature enter)
            (sequentially
              [ instruct it (dealDamage (.lit 2) (target isAnyTarget) .noncombatOnly),
                gainLife (.lit 2),
                doIf (dealtThisWay isPlayer) (discard (a (cardIn (handOf they))))
                    ]) ],
      power := stat 4, toughness := stat 4 } }

def grievousWoundLifeLock : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grievous Wound", cost := some [generic 3, pip .black, pip .black], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ keywordSubject (.named "enchant") isPlayer,
          static (cant (enchanted player) (gainLife anyAmount)),
          whenever (does any (dealDamage (to := enchanted player)))
            (instruct they (loseLife (half .up (lifeTotalOf they)))) ] } }

def cursedScroll : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cursed Scroll", cost := some [generic 1], types := [.artifact],
      text :=
        [ activated (compound [mana [generic 3], tapSymbol])
            (sequentially
              [ choose (a (quality .cardName)),
                reveal (aAtRandom (cardIn yourHand)),
                doIf (matches_ (that card) (named chosenName))
                  (instruct thisArtifact (dealDamage (.lit 2) (target isAnyTarget) .noncombatOnly)) ]) ] } }

def magusOfTheScroll : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Magus of the Scroll", cost := some [pip .red], types := [.creature],
      subtypes := [human, wizard],
      text :=
        [ activated (compound [mana [generic 3], tapSymbol])
            (sequentially
              [ choose (a (quality .cardName)),
                reveal (aAtRandom (cardIn yourHand)),
                doIf (matches_ (that card) (named chosenName))
                  (instruct thisCreature (dealDamage (.lit 2) (target isAnyTarget) .noncombatOnly)) ]) ],
      power := stat 1, toughness := stat 1 } }

/-- Stuffy Doll -/
def stuffyDoll : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stuffy Doll", cost := some [generic 5], types := [.artifact, .creature],
      subtypes := [construct],
      text :=
        [ indestructible,
          static (entersChoosingPlayer thisCreature none),
          whenever (does any (dealDamage (to := thisCreature))) (instruct it (dealDamage thatMuch (the isChosenPlayer) .noncombatOnly)),
          activated tapSymbol (instruct thisCreature (dealDamage (.lit 1) thisCreature .noncombatOnly)) ],
      power := stat 0, toughness := stat 1 } }

def saheeliRaiPlusOne : Action :=
  sequentially [scry (.lit 1), instruct «this» (dealDamage (.lit 1) eachOpponent .noncombatOnly)]
theorem okSaheeliRaiPlusOne : Action.check (DRS.ofList []) saheeliRaiPlusOne = [] := by decide
def sarkhansUnsealingLine : Ability :=
  whenever
    (does you (cast (a (and [isCreature, isSpell,
                or [ hasPower .eq (.lit 4), hasPower .eq (.lit 5),
                      hasPower .eq (.lit 6) ]]))))
    (instruct thisEnchantment (dealDamage (.lit 4) (target isAnyTarget) .noncombatOnly))
theorem okSarkhansUnsealingLine : Ability.check (DRS.ofList []) sarkhansUnsealingLine = [] := by decide
/-- Savage Swipe, both sentences -/
def savageSwipeLine : Action :=
  sequentially
    [ doOnlyIf (get (target (and [isCreature, youControl])) (up (.lit 2)) (up (.lit 2)) (some
        untilEndOfTurn))
        (compareAmt (powerOf it) .eq (.lit 2)),
      instruct it (fight (target (and [isCreature, youDontControl]))) ]
theorem okSavageSwipeLine : Action.check (DRS.ofList []) savageSwipeLine = [] := by decide

def infernoOfTheStarMounts : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Inferno of the Star Mounts", cost := some [generic 4, pip .red, pip .red],
      supertypes := [.legendary], types := [.creature], subtypes := [dragon],
      text :=
        [ static (cant any (counter «this»)),
          flying,
          haste,
          activated (mana [pip .red])
            (triggerThisWay (get thisCreature (up (.lit 1)) (up (.lit 0)) (some untilEndOfTurn))
              (statBecomes it .power (.lit 20))
              (instruct it (dealDamage (.lit 20) (target isAnyTarget) .noncombatOnly))) ],
      power := stat 6, toughness := stat 6 } }

def incinerate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Incinerate", cost := some [generic 1, pip .red], types := [.instant],
      text :=
        [ spell (sequentially
            [ dealDamage (.lit 3) (target isAnyTarget) .noncombatOnly,
              establishThroughout (cant any
                                     (regeneration
                                        (a (and [isCreature, did (⟨any, (dealDamage anyAmount (asMarker .permanent (relative .object))), thisWay⟩)])))) thisTurn ]) ] } }

def ashZealot : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ash Zealot", cost := some [pip .red, pip .red], types := [.creature],
      subtypes := [human, warrior],
      text :=
        [ firstStrike,
          haste,
          whenever (does (a isPlayer) (cast (a
            (and [isSpell, castFrom graveyard]))))
            (instruct thisCreature (dealDamage (.lit 3) (that player) .noncombatOnly)) ],
      power := stat 2, toughness := stat 2 } }

def galvanicBlastLine : Action :=
  replace (dealDamage (.lit 2) (target isAnyTarget) .noncombatOnly)
    (doOnlyIf (dealDamage (.lit 4) thatJoin .noncombatOnly)
      (compareAmt (countOf (and [isArtifact, youControl])) .atLeast (.lit 3)))
theorem okGalvanicBlastLine : Action.check (DRS.ofList []) galvanicBlastLine = [] := by decide
/-- Furious Reprisal -/
def furiousReprisal : Action :=
  dealDamage (.lit 2) (eachOf (targets (exactly 2) isAnyTarget)) .noncombatOnly
theorem okFuriousReprisal : Action.check (DRS.ofList []) furiousReprisal = [] := by decide
/-- Bullseye, Death Dealer -/
def bullseyeModalCost : Ability :=
  activated
    (compound [mana [generic 3], tapSymbol,
      perform (chooseOne (sacrifice (a isArtifact))
                 (discard (a (and [non isLand, in_ yourHand]))))])
    (instruct «this» (dealDamage (.lit 2) (target isAnyTarget) .noncombatOnly))
theorem okBullseyeModalCost : Ability.check (DRS.ofList []) bullseyeModalCost = [] := by decide

/-- Twinshot Sniper -/
def twinshotSniper : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Twinshot Sniper", cost := some [generic 3, pip .red], types := [.artifact, .creature],
      subtypes := [goblin, archer],
      text :=
        [ reach,
          when (does thisCreature enter) (instruct it (dealDamage (.lit 2) (target isAnyTarget) .noncombatOnly)),
          abilityWord "channel"
            (activated (compound [mana [generic 1, pip .red], perform (discard «this»)])
              (instruct it (dealDamage (.lit 2) (target isAnyTarget) .noncombatOnly))) ],
      power := stat 2, toughness := stat 3 } }

/-- Quakebringer -/
def quakebringerDamage : Ability :=
  triggeredIf (beginningOfYour .upkeep)
    (or [ matches_ «this» (in_ battlefield),
           and [ matches_ «this» (cardIn yourGraveyard),
                  exists_ (and [isCreature, hasSubtype giant,
                                 youControl]) ] ])
    (instruct «this» (dealDamage (.lit 2) eachOpponent .noncombatOnly))
theorem okQuakebringerDamage : Ability.check (DRS.ofList []) quakebringerDamage = [] := by decide
/-- Sand Strangler -/
def sandStranglerDamage : Ability :=
  triggeredIf (does thisCreature enter)
    (or [ exists_ (and [isLand, hasSubtype desert, youControl]),
           exists_ (and [isLand, hasSubtype desert, in_ yourGraveyard]) ])
    (may (instruct «this» (dealDamage (.lit 3) (target isCreature) .noncombatOnly)))
theorem okSandStranglerDamage : Ability.check (DRS.ofList []) sandStranglerDamage = [] := by decide

def brazenDwarf : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Brazen Dwarf", cost := some [generic 1, pip .red], types := [.creature],
      subtypes := [dwarf, shaman],
      text :=
        [ whenever (does you (rollDice anyAmount .anyDie .many))
            (instruct thisCreature (dealDamage (.lit 1) eachOpponent .noncombatOnly)) ],
      power := stat 1, toughness := stat 3 } }

/-- Lavalanche -/
def lavalanche : Action :=
  simultaneously
    [ dealDamage (letter .x) targetPlayerOrPlaneswalker .noncombatOnly,
      dealDamage (letter .x) eachCreatureThatSplitControls .noncombatOnly ]
theorem okLavalanche : Action.check (DRS.ofList []) lavalanche = [] := by decide
/-- Flame Wave -/
def flameWave : Action :=
  simultaneously
    [ dealDamage (.lit 4) targetPlayerOrPlaneswalker .noncombatOnly,
      dealDamage (.lit 4) eachCreatureThatSplitControls .noncombatOnly ]
theorem okFlameWave : Action.check (DRS.ofList []) flameWave = [] := by decide
/-- Chandra Nalaar's ultimate -/
def chandraNalaarUltimate : Action :=
  simultaneously
    [ instruct «this» (dealDamage (.lit 10) targetPlayerOrPlaneswalker .noncombatOnly),
      instruct «this» (dealDamage (.lit 10) eachCreatureThatSplitControls .noncombatOnly) ]
theorem okChandraNalaarUltimate : Action.check (DRS.ofList []) chandraNalaarUltimate = [] := by decide
/-- Chandra, Pyrogenius's ultimate -/
def chandraPyrogeniusUltimate : Action :=
  simultaneously
    [ instruct «this» (dealDamage (.lit 6) targetPlayerOrPlaneswalker .noncombatOnly),
      instruct «this» (dealDamage (.lit 6) eachCreatureThatSplitControls .noncombatOnly) ]
theorem okChandraPyrogeniusUltimate : Action.check (DRS.ofList []) chandraPyrogeniusUltimate = [] := by decide
/-- Bonfire of the Damned -/
def bonfireOfTheDamned : Action :=
  simultaneously
    [ dealDamage (letter .x) targetPlayerOrPlaneswalker .noncombatOnly,
      dealDamage (letter .x) eachCreatureThatSplitControls .noncombatOnly ]
theorem okBonfireOfTheDamned : Action.check (DRS.ofList []) bonfireOfTheDamned = [] := by decide
/-- Chandra's Fury -/
def chandrasFury : Action :=
  simultaneously
    [ dealDamage (.lit 4) targetPlayerOrPlaneswalker .noncombatOnly,
      dealDamage (.lit 1) eachCreatureThatSplitControls .noncombatOnly ]
theorem okChandrasFury : Action.check (DRS.ofList []) chandrasFury = [] := by decide
/-- Angrath, Minotaur Pirate's plus -/
def angrathMinotaurPirateBolt : Action :=
  simultaneously
    [ instruct «this» (dealDamage (.lit 1) targetOpponentOrPlaneswalker .noncombatOnly),
      instruct «this» (dealDamage (.lit 1) eachCreatureThatSplitControls .noncombatOnly) ]
theorem okAngrathMinotaurPirateBolt : Action.check (DRS.ofList []) angrathMinotaurPirateBolt = [] := by decide
def whichOfYouBurnsBrightestBody : Action :=
  simultaneously
    [ dealDamage (letter .x) targetOpponentOrPlaneswalker .noncombatOnly,
      dealDamage (letter .x) eachCreatureThatSplitControls .noncombatOnly ]
theorem okWhichOfYouBurnsBrightestBody :
    Action.check (DRS.ofList []) whichOfYouBurnsBrightestBody = [] := by decide
/-- Chandra, Pyromaster's plus -/
def chandraPyromasterBolt : Action :=
  simultaneously
    [ instruct «this» (dealDamage (.lit 1) targetPlayerOrPlaneswalker .noncombatOnly),
      instruct «this» (dealDamage (.lit 1)
                         (targets (upTo 1)
                            (and
                               [isCreature, controlledBy splitOverPlaneswalker])) .noncombatOnly) ]
theorem okChandraPyromasterBolt : Action.check (DRS.ofList []) chandraPyromasterBolt = [] := by decide
/-- Ravager of the Fells -/
def ravagerOfTheFellsBolt : Action :=
  simultaneously
    [ instruct thisCreature (dealDamage (.lit 2) targetOpponentOrPlaneswalker .noncombatOnly),
      instruct thisCreature (dealDamage (.lit 2)
                               (targets (upTo 1)
                                  (and
                                     [isCreature, controlledBy
                                        splitOverPlaneswalker])) .noncombatOnly) ]
theorem okRavagerOfTheFellsBolt : Action.check (DRS.ofList []) ravagerOfTheFellsBolt = [] := by decide
/-- Soul of Shandalar's battlefield activation -/
def soulOfShandalarBolt : Action :=
  simultaneously
    [ instruct thisCreature (dealDamage (.lit 3) targetPlayerOrPlaneswalker .noncombatOnly),
      instruct thisCreature (dealDamage (.lit 3)
                               (targets (upTo 1)
                                  (and
                                     [isCreature, controlledBy
                                        splitOverPlaneswalker])) .noncombatOnly) ]
theorem okSoulOfShandalarBolt : Action.check (DRS.ofList []) soulOfShandalarBolt = [] := by decide
/-- Soul of Shandalar's graveyard activation -/
def soulOfShandalarGraveyardBolt : Action :=
  simultaneously
    [ instruct «this» (dealDamage (.lit 3) targetPlayerOrPlaneswalker .noncombatOnly),
      instruct «this» (dealDamage (.lit 3)
                         (targets (upTo 1)
                            (and
                               [isCreature, controlledBy splitOverPlaneswalker])) .noncombatOnly) ]
theorem okSoulOfShandalarGraveyardBolt :
    Action.check (DRS.ofList []) soulOfShandalarGraveyardBolt = [] := by decide
/-- Blightning -/
def blightning : Action :=
  sequentially
    [ dealDamage (.lit 3) targetPlayerOrPlaneswalker .noncombatOnly,
      discard (counted (exactly 2) (cardIn (handOf splitOverPlaneswalker))) ]
theorem okBlightning : Action.check (DRS.ofList []) blightning = [] := by decide
/-- Rakdos's Return -/
def rakdossReturn : Action :=
  sequentially
    [ dealDamage (letter .x) targetOpponentOrPlaneswalker .noncombatOnly,
      discard (counted (exactlyOf (letter .x)) (cardIn (handOf splitOverPlaneswalker))) ]
theorem okRakdossReturn : Action.check (DRS.ofList []) rakdossReturn = [] := by decide
/-- Nicol Bolas, Planeswalker's ultimate -/
def nicolBolasUltimate : Action :=
  sequentially
    [ instruct «this» (dealDamage (.lit 7) targetPlayerOrPlaneswalker .noncombatOnly),
      discard (counted (exactly 7) (cardIn (handOf splitOverPlaneswalker))),
      instruct splitOverPlaneswalker (sacrifice (counted (exactly 7) isPermanent)) ]
theorem okNicolBolasUltimate : Action.check (DRS.ofList []) nicolBolasUltimate = [] := by decide
/-- Pulse of the Forge -/
def pulseOfTheForge : Action :=
  sequentially
    [ dealDamage (.lit 4) targetPlayerOrPlaneswalker .noncombatOnly,
      doIf (compareAmt (lifeTotalOf splitOverPlaneswalker) .greater (lifeTotalOf you))
        (putInto «this» hand) ]
theorem okPulseOfTheForge : Action.check (DRS.ofList []) pulseOfTheForge = [] := by decide
/-- Goblin Lyre's losing arm -/
def goblinLyreLoseFlip : Action :=
  sequentially
    [ instruct thisArtifact (dealDamage (countOf (and [isCreature, youControl])) targetOpponentOrPlaneswalker .noncombatOnly),
      instruct thisArtifact (dealDamage
                               (countOf
                                  (and
                                     [isCreature, controlledBy
                                        splitOverPlaneswalker])) you .noncombatOnly) ]
theorem okGoblinLyreLoseFlip : Action.check (DRS.ofList []) goblinLyreLoseFlip = [] := by decide
/-- Quenchable Fire -/
def quenchableFire : Action :=
  sequentially
    [ instruct «this» (dealDamage (.lit 3) targetPlayerOrPlaneswalker .noncombatOnly),
      delay (beginningOf .upkeep (some you) nextTime)
        (instruct splitOverPlaneswalker (doUnless (instruct «this» (dealDamage (.lit 3) thatJoin .noncombatOnly)) (mana [pip .blue]))) ]
theorem okQuenchableFire : Action.check (DRS.ofList []) quenchableFire = [] := by decide
/-- Searing Blaze, both sentences -/
def searingBlaze : Ability :=
  abilityWord "landfall"
    (spell (replace
        (simultaneously
          [ dealDamage (.lit 1) targetPlayerOrPlaneswalker .noncombatOnly,
            dealDamage (.lit 1) (target
                                   (and
                                      [isCreature, controlledBy
                                         splitOverPlaneswalker])) .noncombatOnly ])
        (simultaneously
          [ dealDamage (.lit 3) thatJoin .noncombatOnly,
            dealDamage (.lit 3) (that creature) .noncombatOnly ])))
theorem okSearingBlaze : Ability.check (DRS.ofList []) searingBlaze = [] := by decide

/-- Heart of Bogardan -/
def heartOfBogardan : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Heart of Bogardan", cost := some [generic 2, pip .red, pip .red], types := [.enchantment],
      text :=
        [ cumulativeUpkeep (mana [generic 2]),
          when heartOfBogardanHeader
            (sequentially
              [ simultaneously
                  [ instruct «this» (dealDamage (letter .x) targetPlayerOrPlaneswalker .noncombatOnly),
                    instruct «this» (dealDamage (letter .x) eachCreatureThatSplitControls .noncombatOnly) ],
                define .x
                  (minus (times (.lit 2) (countersOn (.named "ageCounter") thisEnchantment))
                     (.lit 2)) ]) ] } }

/-- Burn at the Stake -/
def burnAtTheStake : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Burn at the Stake", cost := some [generic 2, pip .red, pip .red, pip .red],
      types := [.sorcery],
      text :=
        [ static (addedCost
            (perform (tap (counted anyNumber (and [isCreature, youControl, isUntapped]))))
            false),
          spell (dealDamage (times (.lit 3) groupSize) (target isAnyTarget) .noncombatOnly) ] } }

/-- Explosive Singularity -/
def explosiveSingularity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Explosive Singularity", cost := some [generic 8, pip .red, pip .red], types := [.sorcery],
      text :=
        [ static (addedCost
            (perform (tap (counted anyNumber (and [isCreature, youControl, isUntapped]))))
            true),
          static (costShift «this» (less (times (.lit 1) groupSize))),
          spell (dealDamage (.lit 10) (target isAnyTarget) .noncombatOnly) ] } }

/-- Tyrant of Valakut -/
def tyrantOfValakut : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tyrant of Valakut", cost := some [generic 5, pip .red, pip .red], types := [.creature],
      subtypes := [dragon],
      text :=
        [ keywordCosting (.named "surge") (mana [generic 3, pip .red, pip .red]),
          flying,
          triggeredIf (does thisCreature enter) (keywordCostPaid (.named "surge")
                                                           thisCreature)
            (instruct thisCreature (dealDamage (.lit 3) (target isAnyTarget) .noncombatOnly)) ],
      power := stat 5, toughness := stat 4 } }

/-- Fall of the Titans -/
def fallOfTheTitansCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fall of the Titans", cost := some [.variable, .variable, pip .red], types := [.instant],
      text :=
        [ keywordCosting (.named "surge") (mana [.variable, pip .red]),
          spell (dealDamage (letter .x) (eachOf (targets (upTo 2) isAnyTarget)) .noncombatOnly) ] } }

/-- Tribal Flames -/
def tribalFlames : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tribal Flames", cost := some [generic 1, pip .red], types := [.sorcery],
      text :=
        [ abilityWord "domain"
            (spell (sequentially
              [ dealDamage (letter .x) (target isAnyTarget) .noncombatOnly,
                define .x (distinctCount (.subtype .land .basicOnly)
                  (allOf (and [isLand, youControl]))) ])) ] } }

/-- Explosive Prodigy -/
def explosiveProdigyTrigger : Ability :=
  abilityWord "vivid"
    (when (does thisCreature enter)
      (sequentially
        [ instruct it
            (dealDamage (letter .x) (target (and [isCreature, controlledBy anOpponent]))
               .noncombatOnly),
          define .x (distinctCount .color (allOf (and [isPermanent, youControl]))) ]))
theorem okExplosiveProdigyTrigger : Ability.check (DRS.ofList []) explosiveProdigyTrigger = [] := by decide
/-- Niv-Mizzet, Guildpact -/
def nivMizzetGuildpactTrigger : Ability :=
  whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := .combatOnly)))
    (sequentially
      [ instruct thisCreature (dealDamage (letter .x) (target isAnyTarget) .noncombatOnly),
        instruct (target isPlayer) (draw (letter .x)),
        gainLife (letter .x),
        define .x (distinctCount .colorPair
          (allOf (and [isPermanent, youControl, colorCount .eq 2]))) ])
theorem okNivMizzetGuildpactTrigger : Ability.check (DRS.ofList []) nivMizzetGuildpactTrigger = [] := by decide

/-- Aurelia, the Law Above -/
def aureliaTheLawAbove : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aurelia, the Law Above", cost := some [generic 3, pip .red, pip .white],
      supertypes := [.legendary], types := [.creature], subtypes := [angel],
      text :=
        [ flying,
          vigilance,
          haste,
          whenever (does (a isPlayer) (attackWith (counted (atLeast 3) isCreature))) (draw (.lit 1)),
          whenever (does (a isPlayer) (attackWith (counted (atLeast 5) isCreature)))
            (sequentially [instruct «this» (dealDamage (.lit 3) eachOpponent .noncombatOnly), gainLife (.lit 3)]) ],
      power := stat 4, toughness := stat 4 } }

/-- Syr Konrad, the Grim -/
def syrKonradTheGrim : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Syr Konrad, the Grim", cost := some [generic 3, pip .black, pip .black],
      supertypes := [.legendary], types := [.creature],
      subtypes := [human, knight],
      text :=
        [ triggeredOr (does (a (and [isCreature, otherThan «this»])) die)
            [ does any (putInto (a (and [isCard, isCreature])) graveyard (from_ := some (not battlefield))),
              does (a (and [isCreature, in_ yourGraveyard])) (leave
                                                                        (from_ := yourGraveyard)) ]
            (instruct «this» (dealDamage (.lit 1) eachOpponent .noncombatOnly)),
          activated (mana [generic 1, pip .black]) (instruct eachPlayer (mill (.lit 1) eachPlayer)) ],
      power := stat 5, toughness := stat 4 } }

/-- Destructive Revelry -/
def destructiveRevelry : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Destructive Revelry", cost := some [pip .red, pip .green], types := [.instant],
      text :=
        [ spell (sequentially
            [ destroy (target (or [isArtifact, isEnchantment])),
              dealDamage (.lit 2) (controllerOf (that permanent)) .noncombatOnly ]) ] } }

/-- Fuming Effigy -/
def fumingEffigy : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fuming Effigy", cost := some [generic 3, pip .red], types := [.creature],
      subtypes := [spirit],
      text :=
        [ whenever
            (does (counted (atLeast 1) (cardIn yourGraveyard))
               (leave (from_ := yourGraveyard)))
            (instruct «this» (dealDamage (.lit 1) eachOpponent .noncombatOnly)) ],
      power := stat 4, toughness := stat 3 } }

/-- Breeches, Brazen Plunderer's slice -/
def eachOfThoseOpponentsTopCard : Action :=
  sequentially
    [ instruct «this» (dealDamage (.lit 1) eachOpponent .noncombatOnly),
      exile (slice .top (.lit 1) (libraryOf (eachOf (those player)))) ]
theorem okEachOfThoseOpponentsTopCard : Action.check (DRS.ofList []) eachOfThoseOpponentsTopCard = [] := by
  decide

/-- Inquisitor's Flail -/
def inquisitorsFlail : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Inquisitor's Flail", cost := some [generic 2], types := [.artifact],
      subtypes := [equipment],
      text :=
        [ static (damageRule .combatOnly (dealtBy (equipped creature)) everywhere
            (scale (multiplied .doubled)) .repeatedly),
          static (damageRule .combatOnly
            (dealtBy (a (and [isCreature, otherThan (equipped creature)])))
            (toRecipient (equipped creature)) (scale (multiplied .doubled))
            .repeatedly),
          keywordCosting (.named "equip") (mana [generic 2]) ] } }

/-- Oath of Kaya -/
def oathOfKaya : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Oath of Kaya", cost := some [generic 1, pip .white, pip .black],
      supertypes := [.legendary], types := [.enchantment],
      text :=
        [ when (does «this» enter)
            (sequentially [instruct «this» (dealDamage (.lit 3) (target isAnyTarget) .noncombatOnly), gainLife (.lit 3)]),
          whenever
            (does anOpponent
               (attackWith (counted (atLeast 1) isCreature)
                  (some (a (and [isPlaneswalker, youControl])))))
            (sequentially [instruct «this» (dealDamage (.lit 2) (that player) .noncombatOnly), gainLife (.lit 2)]) ] } }

/-- Frostwielder -/
def frostwielder : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Frostwielder", cost := some [generic 2, pip .red, pip .red], types := [.creature],
      subtypes := [human, shaman],
      text :=
        [ static (instead (would any (move (a (and [isCreature,
              happenedTo thisCreature (dealDamage anyAmount
                                                 (asMarker .permanent
                                                    (relative
                                                       .object))) soFarThisTurn])) battlefield (some graveyard) .departure)) (exile it)),
          activated tapSymbol (instruct thisCreature (dealDamage (.lit 1) (target isAnyTarget) .noncombatOnly)) ],
      power := stat 1, toughness := stat 2 } }

/-- Crackling Doom -/
def cracklingDoom : Action :=
  sequentially
    [ dealDamage (.lit 2) eachOpponent .noncombatOnly,
      instruct eachOpponent (sacrifice
        (a (and [isCreature,
                  superlative .max (.stat .power)
                    (and [isCreature, controlledBy (that player)])]))) ]
theorem okCracklingDoom : Action.check (DRS.ofList []) cracklingDoom = [] := by decide
def theFallen : Ability :=
  at_ (beginningOfYour .upkeep)
    (instruct thisCreature (dealDamage (.lit 1) (each (and [or [isOpponent, isPlaneswalker],
                   happenedTo thisCreature (dealDamage anyAmount                      (relative (.join .player .object))) thisGame])) .noncombatOnly))
theorem okTheFallen : Ability.check (DRS.ofList []) theFallen = [] := by decide
/-- Cavalcade of Calamity -/
def cavalcadeOfCalamity : Ability :=
  whenever
    (does (a (and [isCreature, youControl, hasPower .atMost (.lit 1)])) attack)
    (instruct thisEnchantment (dealDamage (.lit 1) (the (and [or [isPlayer, isPlaneswalker],
                  inCombat .attackedBy (some (that creature))])) .noncombatOnly))
theorem okCavalcadeOfCalamity : Ability.check (DRS.ofList []) cavalcadeOfCalamity = [] := by decide

/-- Blind Fury -/
def blindFury : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blind Fury", cost := some [generic 2, pip .red, pip .red], types := [.instant],
      text :=
        [ spell (sequentially
            [ establishThroughout (abilityLoss (allOf isCreature) [written trample]) untilEndOfTurn,
              establishThroughout (damageRule .combatOnly (dealtBy (a isCreature))
                                     (toRecipient (a isCreature))
                  (scale (multiplied .doubled)) .repeatedly) thisTurn ]) ] } }

/-- Chandra, Awakened Inferno's emblem -/
def chandraAwakenedInfernoEmblem : Action :=
  getEmblem
    [ at_ (beginningOfYour .upkeep)
        (instruct (asMarker .emblem «this») (dealDamage (.lit 1) you .noncombatOnly)) ]
theorem okChandraAwakenedInfernoEmblem :
    Action.check (DRS.ofList []) chandraAwakenedInfernoEmblem = [] := by decide

/-- Keeper of the Flame -/
def keeperOfTheFlame : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Keeper of the Flame", cost := some [pip .red, pip .red], types := [.creature],
      subtypes := [human, wizard],
      text :=
        [ activated (compound [mana [pip .red], tapSymbol])
            (sequentially
              [ chooseWhile
                  (target (and [isOpponent, compare .greater (lifeTotalOf you) (axes := [.playerStat .lifeTotal])]))
                  (whileDoing (does you (activate thisAbility))),
                instruct thisCreature (dealDamage (.lit 2) (that player) .noncombatOnly) ]) ],
      power := stat 1, toughness := stat 2 } }

def diabolicEdict : Action := instruct (target isPlayer) (sacrifice (aTheirChoice isCreature))
theorem okDiabolicEdict : Action.check (DRS.ofList []) diabolicEdict = [] := by decide
def innocentBlood : Action := instruct eachPlayer (sacrifice (aTheirChoice isCreature))
theorem okInnocentBlood : Action.check (DRS.ofList []) innocentBlood = [] := by decide
def cryOfContrition : Action := discard (a (cardIn (handOf (target isPlayer))))
theorem okCryOfContrition : Action.check (DRS.ofList []) cryOfContrition = [] := by decide
def cyclingCost : Action := discard «this»
theorem okCyclingCost : Action.check (DRS.ofList []) cyclingCost = [] := by decide
def raiseTheAlarm : Action :=
  createToken (creatureToken 1 1 [.white] [soldier]) (count := .lit 2)
theorem okRaiseTheAlarm : Action.check (DRS.ofList []) raiseTheAlarm = [] := by decide
def actOfTreason : Action := gainControl (target isCreature) (some untilEndOfTurn) (controller :=
    you)
theorem okActOfTreason : Action.check (DRS.ofList []) actOfTreason = [] := by decide
def wordsOfWorship : Action :=
  replaceNextEvent (would you (draw anyAmount)) (gainLife (.lit 5)) (some thisTurn)
theorem okWordsOfWorship : Action.check (DRS.ofList []) wordsOfWorship = [] := by decide
def theLastRoninII : Action :=
  triggerReflexively (mill (.lit 4) you)
    (putInto (target (and [isCreature, in_ yourGraveyard])) hand)
theorem okTheLastRoninII : Action.check (DRS.ofList []) theLastRoninII = [] := by decide

def moonlitWakeCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Moonlit Wake", cost := some [generic 2, pip .white], types := [.enchantment],
      text := [moonlitWake] } }

def laquatussDisdain : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Laquatus's Disdain", cost := some [generic 1, pip .blue], types := [.instant],
      text :=
        [ spell (sequentially
            [ counter (target (and [isSpell, castFrom graveyard])), draw (.lit 1) ]) ] } }

/-- Stifle -/
def stifle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stifle", cost := some [pip .blue], types := [.instant],
      text :=
        [ spell (counter (target (or [abilityHead .anyActivated, abilityHead .anyTriggered]))) ] } }

/-- Disallow -/
def disallow : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Disallow", cost := some [generic 1, pip .blue, pip .blue], types := [.instant],
      text :=
        [ spell (counter
              (target (or [isSpell, abilityHead .anyActivated, abilityHead .anyTriggered]))) ] } }

/-- Fry -/
def fry : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fry", cost := some [generic 1, pip .red], types := [.instant],
      text :=
        [ static (cant any (counter thisSpell)),
          spell (dealDamage (.lit 5)
                   (target
                      (and
                         [or [isCreature, isPlaneswalker], or
                            [hasColor .white, hasColor .blue]])) .noncombatOnly) ] } }

/-- Termination Facilitator -/
def terminationFacilitator : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Termination Facilitator", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [human, assassin],
      text :=
        [ activatedOnlyDuring tapSymbol
            (putCounters (.lit 1) (printedKind (.named "bountyCounter"))
              (target (or [isCreature, isPlaneswalker])) .many)
            asSorcery,
          whenever
            (does any
               (dealDamage (to := a (and [or [isCreature, isPlaneswalker], controlledBy anOpponent,
                        hasCounter (some (.named "bountyCounter"))]))))
            (destroy it) ],
      power := stat 1, toughness := stat 3 } }

end Semantics.Cards
