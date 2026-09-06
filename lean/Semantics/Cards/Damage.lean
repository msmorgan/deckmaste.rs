import Semantics
import Semantics.Macros
import Semantics.Check.Card
import Semantics.Cards.Description
import Semantics.Cards.Anaphora
import Semantics.Cards.Trigger

/-!
# Semantics.Cards.Damage

Port of `idris/src/Experimental/Cards/Damage.idr`: the printed cards of the Damage family and
the bench items beside them.
-/

open Semantics Semantics.Macros

namespace Semantics.Cards

/-- Lightning Bolt -/
def bolt : Instruction := .dealDamage .this (.lit 3) (target anyTarget)
theorem okBolt : Instruction.check [] bolt = [] := by decide
def barrageOfBoulders : Instruction := .dealDamage .this (.lit 1) (each creatureYouDontControl)
theorem okBarrageOfBoulders : Instruction.check [] barrageOfBoulders = [] := by decide
def rabidBite : Instruction :=
  .dealDamage (target creatureYouControl) (.statOf (.stat .power) it) (target creatureYouDontControl)
theorem okRabidBite : Instruction.check [] rabidBite = [] := by decide
def preyUpon : Instruction := .fight (target creatureYouControl) (target creatureYouDontControl)
theorem okPreyUpon : Instruction.check [] preyUpon = [] := by decide
def arcTrail : Instruction :=
  .sequence
    [ .dealDamage .this (.lit 2) (target anyTarget), .dealDamage .this (.lit 1) (target anyOtherTarget) ]
theorem okArcTrail : Instruction.check [] arcTrail = [] := by decide
def deadshot : Instruction :=
  .sequence
    [ .setStatus .tapped (target creature),
      .dealDamage it (.statOf (.stat .power) it) (target (.and [creature, .other])) ]
theorem okDeadshot : Instruction.check [] deadshot = [] := by decide
def immersturmSkullcairn : Ability :=
  activatedOnlyDuring
    (.compound [.mana [generic 1, pip .black, pip .red, pip .red], .tapSymbol,
      .perform (sacrifice thisLand (agent := .you))])
    (.sequence
      [ .dealDamage it (.lit 3) (target .anyPlayer), discard (a (.inZone hand)) (agent := (that
          .player)) ])
    .asSorcery
theorem okImmersturmSkullcairn : Ability.check [] immersturmSkullcairn = [] := by decide
def pyriteSpellbomb : Ability :=
  activated (.compound [.mana [pip .red], .perform (sacrifice thisArtifact (agent := .you))])
    (.dealDamage it (.lit 2) (target anyTarget))
theorem okPyriteSpellbomb : Ability.check [] pyriteSpellbomb = [] := by decide
def karplusanYeti : Instruction :=
  .sequence
    [ dealDamageOwnPower [] thisCreature (target creature),
      .dealDamage (that (.type .creature)) (.statOf (.stat .power) it) thisCreature ]
theorem okKarplusanYeti : Instruction.check [] karplusanYeti = [] := by decide
def suddenDemise : Instruction :=
  .sequence
    [ choose (a (quality .color)),
      .dealDamage .this (.letter .x) (each (.and [creature, ofChosen .color])) ]
theorem okSuddenDemise : Instruction.check [] suddenDemise = [] := by decide
def caseOfTheGatewayExpress : Instruction :=
  .sequence
    [ choose (target creatureYouDontControl),
      .dealDamage (each creatureYouControl) (.lit 1) (that (.type .creature)) ]
theorem okCaseOfTheGatewayExpress : Instruction.check [] caseOfTheGatewayExpress = [] := by decide
def arrowsOfJustice : Instruction :=
  .dealDamage .this (.lit 4) (target (.and [creature, .or [attacking, blocking]]))
theorem okArrowsOfJustice : Instruction.check [] arrowsOfJustice = [] := by decide
def flamesOfTheRazeBoar : Instruction :=
  .sequence
    [ .dealDamage .this (.lit 4) (target (.and [creature, .hasPossessor .controller anOpponent])),
      .doOnlyIf
        (.dealDamage .this (.lit 2)
          (each (.and [creature, .other, .hasPossessor .controller (that .player)])))
        (exists_ (.and [creature, .hasPossessor .controller .you, .compare [.stat .power] .atLeast (.lit 4)]))
        none ]
theorem okFlamesOfTheRazeBoar : Instruction.check [] flamesOfTheRazeBoar = [] := by decide
def yawgmothDemon : Instruction :=
  .offer (sacrifice (a artifact) (agent := .you)) none
    (some (.sequence [.setStatus .tapped thisCreature, .dealDamage .this (.lit 2) .you])) (agent :=
        .you)
theorem okYawgmothDemon : Instruction.check [] yawgmothDemon = [] := by decide
def arcBlade : Instruction :=
  .sequence
    [ .dealDamage .this (.lit 2) (target anyTarget), exileWithCounters .this (.lit 3) (.named "Time") ]
theorem okArcBlade : Instruction.check [] arcBlade = [] := by decide
def abrade : Instruction :=
  chooseModes (exactly 1) [.dealDamage .this (.lit 3) (target creature), destroy (target artifact)]
theorem okAbrade : Instruction.check [] abrade = [] := by decide
def nibelheimAflame : Instruction :=
  .sequence
    [ choose (target creatureYouControl),
      .dealDamage it (.statOf (.stat .power) it) (each (otherCreature it)) ]
theorem okNibelheimAflame : Instruction.check [] nibelheimAflame = [] := by decide
def brashTaunter : Instruction := .fight thisCreature (target (otherCreature thisCreature))
theorem okBrashTaunter : Instruction.check [] brashTaunter = [] := by decide
def ulvenwaldTracker : Instruction :=
  .fight (target creatureYouControl) (target (.and [creature, .other]))
theorem okUlvenwaldTracker : Instruction.check [] ulvenwaldTracker = [] := by decide

def botBashingTime : Instruction :=
  .sequence
    [ .dealDamage .this (.lit 6) (target creature),
      replaceEvent (.dies (that (.type .creature))) (exile it) (some .thisTurn) ]
theorem okBotBashingTime : Instruction.check [] botBashingTime = [] := by decide
def wordsOfWar : Instruction :=
  replaceNextEvent (.draws .you) (.dealDamage .this (.lit 2) (target anyTarget)) (some .thisTurn)
theorem okWordsOfWar : Instruction.check [] wordsOfWar = [] := by decide

/-- Thunderwave -/
def thunderwave : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thunderwave", cost := some [generic 2, pip .red, pip .red], types := [.sorcery],
      text :=
        [ .spell none (.sequence
            [ rollDice 1 20 (agent := .you),
              .applyResultsTable
                [ rollRow (fromTo 1 9) (.dealDamage .this (.lit 3) (each creature)),
                  rollRow (fromTo 10 19)
                    (.sequence
                      [ offer (choose (a creature) (agent := some .you)) (agent := .you),
                        .dealDamage .this (.lit 3) (each (.and [creature, .notChosen])) ]),
                  rollRow (fromTo 20 20)
                    (.dealDamage .this (.lit 6)
                      (each (.and [creature, .hasPossessor .controller (.playerGroup .yourOpponents)]))) ] ]) ] } }

/-- Fog, Holy Day, Darkness, Root Snare -/
def fog : Instruction := preventAll .combatOnly .everywhere (some .thisTurn)
theorem okFog : Instruction.check [] fog = [] := by decide
/-- Indestructible Aura, Shielded Passage -/
def indestructibleAura : Instruction :=
  preventAll .any (.toRecipient (target creature)) (some .thisTurn)
theorem okIndestructibleAura : Instruction.check [] indestructibleAura = [] := by decide
def shieldmatesBlessing : Instruction :=
  preventNext .any (.toRecipient (target anyTarget)) (.lit 3) (some .thisTurn)
theorem okShieldmatesBlessing : Instruction.check [] shieldmatesBlessing = [] := by decide
def moonlitWake : Ability := whenever (.dies (a creature)) (gainLife (.lit 1) (agent := .you))
theorem okMoonlitWake : Ability.check [] moonlitWake = [] := by decide
def eliteJavelineer : Ability :=
  whenever (blocks thisCreature none) (.dealDamage .this (.lit 1) (target (.and [creature, attacking])))
theorem okEliteJavelineer : Ability.check [] eliteJavelineer = [] := by decide
/-- Glacial Chasm -/
def glacialChasmShield : Ability :=
  .static (.damageRule .any .unattributed (.toRecipient .you) (.prevent .all none) .repeatedly)
theorem okGlacialChasmShield : Ability.check [] glacialChasmShield = [] := by decide
def corneredCrook : Ability :=
  when (.enters thisCreature none)
    (offerWhen (sacrifice (a artifact) (agent := .you)) (.dealDamage .this (.lit 3) (target
        anyTarget)) (agent := .you))
theorem okCorneredCrook : Ability.check [] corneredCrook = [] := by decide

def aladdinsRing : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aladdin's Ring", cost := some [generic 8], types := [.artifact],
      text :=
        [ activated (.compound [.mana [generic 8], .tapSymbol])
            (.dealDamage thisArtifact (.lit 4) (target anyTarget)) ] } }

def chandrasRevolution : Instruction :=
  .sequence
    [ .dealDamage .this (.lit 4) (target creature), .setStatus .tapped (target land),
      .skipUntap (that (.type .land)) (.lit 1) ]
theorem okChandrasRevolution : Instruction.check [] chandrasRevolution = [] := by decide
def arbalestElite : Ability :=
  activated (.compound [.mana [generic 2, pip .white], .tapSymbol])
    (.sequence
      [ .dealDamage thisCreature (.lit 3) (target (.and [creature, .or [attacking, blocking]])),
        .skipUntap thisCreature (.lit 1) ])
theorem okArbalestElite : Ability.check [] arbalestElite = [] := by decide
def sizzlingBarrage : Instruction :=
  .dealDamage .this (.lit 4) (target (.and [creature, happenedTo .blockDeclaration .thisTurn]))
theorem okSizzlingBarrage : Instruction.check [] sizzlingBarrage = [] := by decide
def goadedAttackTrigger : Ability :=
  whenever (attacks (a (.and [creature, .hasDesignation "goaded" none])))
    (.dealDamage it (.lit 1) (controllerOf it))
theorem okGoadedAttackTrigger : Ability.check [] goadedAttackTrigger = [] := by decide

def extraArms : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Extra Arms", cost := some [generic 4, pip .red], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          whenever (attacks (.attachHost .enchanted (.type .creature)))
            (.dealDamage it (.lit 2) (target anyTarget)) ] } }

def chainReaction : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chain Reaction", cost := some [generic 2, pip .red, pip .red], types := [.sorcery],
      text :=
        [ .spell none (.sequence
            [ .dealDamage .this (.letter .x) (each creature), .define .x (countOf creature) ]) ] } }

/-- Black Vise -/
def blackVise : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Black Vise", cost := some [generic 1], types := [.artifact],
      text :=
        [ .static (entersChoosingPlayer thisArtifact (some (.players .opponent))),
          at_ (beginningOfPossessed .the .upkeep (the chosenPlayer))
            (.sequence
              [ .dealDamage thisArtifact (.letter .x) (that .player),
                .define .x (.arith .minus (countOf (.inZone (handOf they))) (.lit 4)) ]) ] } }

def harshSustenance : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Harsh Sustenance", cost := some [generic 1, pip .white, pip .black], types := [.instant],
      text :=
        [ .spell none (.sequence
            [ .dealDamage .this (.letter .x) (target anyTarget),
              gainLife (.letter .x) (agent := .you),
              .define .x (countOf creatureYouControl) ]) ] } }

/-- Purging Scythe -/
def purgingScythe : Ability :=
  at_ (.beginningOf .the .upkeep (.byPlayer .you))
    (.sequence
      [ .dealDamage thisArtifact (.lit 2)
          (the (.and [creature, .superlative .min (.stat .toughness) creature])),
        .doIf
          (.compareAmt (countOf (.and [creature, .superlative .min (.stat .toughness) creature]))
            .atLeast (.lit 2))
          (choose (someOf (exactly 1) them) (agent := some .you)) none ])
theorem okPurgingScythe : Ability.check [] purgingScythe = [] := by decide

def lionHeart : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lion Heart", cost := some [generic 4], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ when (.enters thisEquipment none) (.dealDamage it (.lit 2) (target anyTarget)),
          .static (getsPt (.attachHost .equipped (.type .creature)) (.up (.lit 2)) (.up (.lit 1))),
          keywordCosting "Equip" (.mana [generic 2]) ] } }

def lightmineField : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lightmine Field", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ whenever (attacks (counted (atLeast 1) creature))
            (.dealDamage thisEnchantment (.countOf (those (.type .creature)))
              (.eachOf (those (.type .creature)))) ] } }

def ingeniousArtillerist : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ingenious Artillerist", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Artificer"],
      text :=
        [ whenever (.enters (counted (atLeast 1) (.and [artifact, .hasPossessor .controller .you])) none)
            (.dealDamage thisCreature .groupSize (each .opponent)) ],
      power := stat 3, toughness := stat 1 } }

/-- Inferno Elemental -/
def infernoElemental : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Inferno Elemental", cost := some [generic 4, pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ triggeredOr (blocks thisCreature (some (a creature)))
            [becomesBlocked thisCreature (some (a creature))]
            (.dealDamage thisCreature (.lit 3) (that (.type .creature))) ],
      power := stat 4, toughness := stat 4 } }

/-- Psychic Purge -/
def psychicPurge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Psychic Purge", cost := some [pip .blue], types := [.sorcery],
      text :=
        [ .spell none (.dealDamage .this (.lit 1) (target anyTarget)),
          when
            (.causes
              (.source (a (.and [.or [spell, .abilityHead .anyOnStack],
                                 .hasPossessor .controller (a .opponent)])))
              (.verbedEvent (some .you) (.action "Discard") (some .this) none))
            (loseLife (.lit 5) (agent := (that .player))) ] } }

/-- Chandra Nalaar -/
def chandraNalaarsX : Ability :=
  activated (.loyaltySymbol .downX) (.dealDamage .this (.letter .x) (target creature))
theorem okChandraNalaarsX : Ability.check [] chandraNalaarsX = [] := by decide

def etherealHaze : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ethereal Haze", cost := some [pip .white], types := [.instant],
      subtypes := [spellType "Arcane"],
      text := [.spell none (preventAllBy .any (allOf creature) .everywhere (some .thisTurn))] } }

def defang : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Defang", cost := some [generic 1, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (.damageRule .any (.dealtBy (.attachHost .enchanted (.type .creature))) .everywhere
            (.prevent .all none) .repeatedly) ] } }

def sphereOfPurity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sphere of Purity", cost := some [generic 3, pip .white], types := [.enchantment],
      text :=
        [ .static (.damageRule .any (.dealtBy (a artifact)) (.toRecipient .you)
            (.prevent (.some (.lit 1)) none) .repeatedly) ] } }

def dazzlingReflection : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dazzling Reflection", cost := some [generic 1, pip .white], types := [.instant],
      text :=
        [ .spell none (.sequence
            [ gainLife (.statOf (.stat .power) (target creature)) (agent := .you),
              .establish
                (.damageRule .any (.dealtBy (that (.type .creature))) .everywhere (.prevent .all none)
                  .nextTimeOnly)
                (some .thisTurn) ]) ] } }

def thunderstaff : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thunderstaff", cost := some [generic 3], types := [.artifact],
      text :=
        [ .static (onlyWhile
            (.damageRule .combatOnly (.dealtBy (a creature)) (.toRecipient .you)
              (.prevent (.some (.lit 1)) none) .repeatedly)
            (.matches thisArtifact untapped)),
          activated (.compound [.mana [generic 2], .tapSymbol])
            (get (allOf (.and [creature, attacking])) (.up (.lit 1)) (.up (.lit 0))
              (some untilEndOfTurn)) ] } }

def gideonAllyOfZendikar : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gideon, Ally of Zendikar", cost := some [generic 2, pip .white, pip .white],
      supertypes := [.legendary], types := [.planeswalker], subtypes := [planeswalkerType "Gideon"],
      text :=
        [ activated (.loyaltySymbol (.up 1))
            (.sequence
              [ .establish
                  (.qualityChange thisPlaneswalker .sets
                    (.bundle
                      { characteristics :=
                        { types := [.creature],
                          subtypes := [creatureType "Human", creatureType "Soldier", creatureType "Ally"],
                          text := [keyword "Indestructible"], power := stat 5, toughness := stat 5 } }
                      (some .planeswalker)))
                  (some untilEndOfTurn),
                preventAll .any (.toRecipient it) (some .thisTurn) ]),
          activated (.loyaltySymbol .zero)
            (create (.lit 1) (creatureToken 2 2 [.white] [creatureType "Knight", creatureType "Ally"])),
          activated (.loyaltySymbol (.down 4))
            (.getEmblem
              [.static (getsPt (allOf creatureYouControl) (.up (.lit 1)) (.up (.lit 1)))] (agent :=
                  .you)) ],
      loyalty := stat 4 } }

def turnTheTables : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Turn the Tables", cost := some [generic 3, pip .white, pip .white], types := [.instant],
      text :=
        [ .spell none (.establish
            (.damageRule .combatOnly .unattributed (.toRecipient .you)
              (.redirect .all (target (.and [creature, attacking]))) .repeatedly)
            (some .thisTurn)) ] } }

def pariah : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pariah", cost := some [generic 2, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (.damageRule .any .unattributed (.toRecipient .you)
            (.redirect .all (.attachHost .enchanted (.type .creature))) .repeatedly) ] } }

def martyrsOfKorlis : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Martyrs of Korlis", cost := some [generic 3, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Human"],
      text :=
        [ .static (onlyWhile
            (.damageRule .any (.dealtBy (allOf artifact)) (.toRecipient .you)
              (.redirect .all thisCreature) .repeatedly)
            (.matches thisCreature untapped)) ],
      power := stat 1, toughness := stat 6 } }

def wardOfPiety : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ward of Piety", cost := some [generic 1, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          activated (.mana [generic 1, pip .white])
            (.establish
              (.damageRule .any .unattributed (.toRecipient (.attachHost .enchanted (.type .creature)))
                (.redirect (.shield (.lit 1)) (target anyTarget)) .repeatedly)
              (some .thisTurn)) ] } }

def mirrorwoodTreefolk : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mirrorwood Treefolk", cost := some [generic 3, pip .green], types := [.creature],
      subtypes := [creatureType "Treefolk"],
      text :=
        [ activated (.mana [generic 2, pip .red, pip .white])
            (.establish
              (.damageRule .any .unattributed (.toRecipient thisCreature)
                (.redirect .all (target anyTarget)) .nextTimeOnly)
              (some .thisTurn)) ],
      power := stat 2, toughness := stat 4 } }

def carom : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Carom", cost := some [generic 1, pip .white], types := [.instant],
      text :=
        [ .spell none (.sequence
            [ .establish
                (.damageRule .any .unattributed (.toRecipient (target creature))
                  (.redirect (.shield (.lit 1)) (target (.and [creature, .other]))) .repeatedly)
                (some .thisTurn),
              .draw (.lit 1) (agent := .you) ]) ] } }

def daughterOfAutumn : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Daughter of Autumn", cost := some [generic 2, pip .green, pip .green],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Avatar"],
      text :=
        [ activated (.mana [pip .white])
            (.establish
              (.damageRule .any .unattributed (.toRecipient (target (.and [creature, .colorIs .white])))
                (.redirect (.shield (.lit 1)) thisCreature) .repeatedly)
              (some .thisTurn)) ],
      power := stat 2, toughness := stat 4 } }

def aegisOfHonor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aegis of Honor", cost := some [pip .white], types := [.enchantment],
      text :=
        [ activated (.mana [generic 1])
            (.establish
              (.damageRule .any (.dealtBy (a instantOrSorcery)) (.toRecipient .you)
                (.redirect .all (controllerOf it)) .nextTimeOnly)
              (some .thisTurn)) ] } }

def candlesGlow : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Candles' Glow", cost := some [generic 1, pip .white], types := [.instant],
      subtypes := [spellType "Arcane"],
      text :=
        [ .spell none (.establish
            (.damageRule .any .unattributed (.toRecipient (target anyTarget))
              (.prevent (.shield (.lit 3)) (some (gainLife preventedThisWay (agent := .you))))
                  .repeatedly)
            (some .thisTurn)) ] } }

/-- Inkshield -/
def inkshieldRider : Instruction :=
  .establish
    (.damageRule .any .unattributed (.toRecipient .you)
      (.prevent .all (some (create preventedThisWay (creatureToken 2 1 [.white, .black] []))))
      .repeatedly)
    (some .thisTurn)
theorem okInkshieldRider : Instruction.check [] inkshieldRider = [] := by decide

def urzasArmor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Urza's Armor", cost := some [generic 6], types := [.artifact],
      text :=
        [ .static (.damageRule .any (.dealtBy (a source)) (.toRecipient .you)
            (.prevent (.some (.lit 1)) none) .repeatedly) ] } }

def circleOfProtectionRed : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Circle of Protection: Red", cost := some [generic 1, pip .white], types := [.enchantment],
      text :=
        [ activated (.mana [generic 1])
            (.establish
              (.damageRule .any (.dealtBy (aYourChoice (.and [source, .colorIs .red]))) (.toRecipient .you)
                (.prevent .all none) .nextTimeOnly)
              (some .thisTurn)) ] } }

def healingGrace : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Healing Grace", cost := some [pip .white], types := [.instant],
      text :=
        [ .spell none (.sequence
            [ .establish
                (.damageRule .any (.dealtBy (aYourChoice source)) (.toRecipient (target anyTarget))
                  (.prevent (.shield (.lit 3)) none) .repeatedly)
                (some .thisTurn),
              gainLife (.lit 3) (agent := .you) ]) ] } }

def reverseDamage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Reverse Damage", cost := some [generic 1, pip .white, pip .white], types := [.instant],
      text :=
        [ .spell none (.establish
            (.damageRule .any (.dealtBy (aYourChoice source)) (.toRecipient .you)
              (.prevent .all (some (gainLife preventedThisWay (agent := .you)))) .nextTimeOnly)
            (some .thisTurn)) ] } }

def deflectingPalm : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Deflecting Palm", cost := some [pip .red, pip .white], types := [.instant],
      text :=
        [ .spell none (.establish
            (.damageRule .any (.dealtBy (aYourChoice source)) (.toRecipient .you)
              (.prevent .all (some (.dealDamage .this .thatMuch (controllerOf it)))) .nextTimeOnly)
            (some .thisTurn)) ] } }

def templeAltisaur : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Temple Altisaur", cost := some [generic 4, pip .white], types := [.creature],
      subtypes := [creatureType "Dinosaur"],
      text :=
        [ .static (.damageRule .any (.dealtBy (a source))
            (.toRecipient (a (.and [.hasSubtype (creatureType "Dinosaur"), .hasPossessor .controller .you])))
            (.prevent (.allBut (.lit 1)) none) .repeatedly) ],
      power := stat 3, toughness := stat 4 } }

def darkSphere : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dark Sphere", cost := some [], types := [.artifact],
      text :=
        [ activated (.compound [.tapSymbol, .perform (sacrifice thisArtifact (agent := .you))])
            (.establish
              (.damageRule .any (.dealtBy (aYourChoice source)) (.toRecipient .you)
                (.prevent (.half .down) none) .nextTimeOnly)
              (some .thisTurn)) ] } }

def shadowbane : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shadowbane", cost := some [generic 1, pip .white], types := [.instant],
      text :=
        [ .spell none (.establish
            (.damageRule .any (.dealtBy (aYourChoice source))
              (.toRecipient (youAnd (allOf creatureYouControl)))
              (.prevent .all
                (some (.doIf (.preventedFromSource (.and [source, .colorIs .black]))
                  (gainLife preventedThisWay (agent := .you)) none)))
              .nextTimeOnly)
            (some .thisTurn)) ] } }

def sphereOfLaw : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sphere of Law", cost := some [generic 3, pip .white], types := [.enchantment],
      text :=
        [ .static (.damageRule .any (.dealtBy (a (.and [source, .colorIs .red]))) (.toRecipient .you)
            (.prevent (.some (.lit 2)) none) .repeatedly) ] } }

def lavaAxe : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lava Axe", cost := some [generic 4, pip .red], types := [.sorcery],
      text :=
        [ .spell none (.dealDamage .this (.lit 5) (target (.or [.hasType .planeswalker, .anyPlayer]))) ] } }

def searingFlesh : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Searing Flesh", cost := some [generic 6, pip .red], types := [.sorcery],
      text :=
        [ .spell none (.dealDamage .this (.lit 7) (target (.or [.hasType .planeswalker, .opponent]))) ] } }

def onakkeJavelineerBolt : Ability :=
  activated .tapSymbol
    (.dealDamage thisCreature (.lit 2) (target (.or [.hasType .battle, .anyPlayer])))
theorem okOnakkeJavelineerBolt : Ability.check [] onakkeJavelineerBolt = [] := by decide

/-- Firesong and Sunspeaker -/
def firesongJoinEcho : Instruction :=
  .sequence
    [ .dealDamage .this (.lit 3) (target (.or [creature, .anyPlayer])),
      .dealDamage .this (.lit 1) thatJoin ]
theorem okFiresongJoinEcho : Instruction.check [] firesongJoinEcho = [] := by decide

/-- Forcefield -/
def forcefield : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Forcefield", cost := some [generic 3], types := [.artifact],
      text :=
        [ activated (.mana [generic 1])
            (.establish
              (.damageRule .combatOnly (.dealtBy (aYourChoice (.and [creature, unblocked])))
                (.toRecipient .you) (.prevent (.allBut (.lit 1)) none) .nextTimeOnly)
              (some .thisTurn)) ] } }

def endure : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Endure", cost := some [generic 3, pip .white, pip .white], types := [.instant],
      text :=
        [ .spell none
            (preventAll .any
              (.toRecipient (youAnd (allOf (.and [permanent, .hasPossessor .controller .you]))))
              (some .thisTurn)) ] } }

def harmsWay : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Harm's Way", cost := some [pip .white], types := [.instant],
      text :=
        [ .spell none (.establish
            (.damageRule .any (.dealtBy (aYourChoice source))
              (.toRecipient (youAnd (allOf (.and [permanent, .hasPossessor .controller .you]))))
              (.redirect (.shield (.lit 2)) (target anyTarget)) .repeatedly)
            (some .thisTurn)) ] } }

def divineDeflection : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Divine Deflection", cost := some [.variable, pip .white], types := [.instant],
      text :=
        [ .spell none (.establish
            (.damageRule .any .unattributed
              (.toRecipient (youAnd (allOf (.and [permanent, .hasPossessor .controller .you]))))
              (.prevent (.shield (.letter .x)) (some (.dealDamage .this .thatMuch (target anyTarget))))
              .repeatedly)
            (some .thisTurn)) ] } }

def glarecasterShield : Ability :=
  activated (.mana [generic 5, pip .white])
    (.establish
      (.damageRule .any .unattributed (.toRecipient (youAnd thisCreature))
        (.redirect .all (target anyTarget)) .nextTimeOnly)
      (some .thisTurn))
theorem okGlarecasterShield : Ability.check [] glarecasterShield = [] := by decide

def furnaceOfRath : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Furnace of Rath", cost := some [generic 1, pip .red, pip .red, pip .red],
      types := [.enchantment],
      text :=
        [ .static (.damageRule .any (.dealtBy (a source))
            (.toRecipient (a (.or [permanent, .anyPlayer]))) (.scale (.multiplied .doubled))
            .repeatedly) ] } }

def gratuitousViolence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gratuitous Violence", cost := some [generic 2, pip .red, pip .red, pip .red],
      types := [.enchantment],
      text :=
        [ .static (.damageRule .any (.dealtBy (a creatureYouControl))
            (.toRecipient (a (.or [permanent, .anyPlayer]))) (.scale (.multiplied .doubled))
            .repeatedly) ] } }

def fieryEmancipation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fiery Emancipation", cost := some [generic 3, pip .red, pip .red, pip .red],
      types := [.enchantment],
      text :=
        [ .static (.damageRule .any (.dealtBy (a (.and [source, .hasPossessor .controller .you])))
            (.toRecipient (a (.or [permanent, .anyPlayer]))) (.scale (.multiplied .tripled))
            .repeatedly) ] } }

def sulfuricVapors : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sulfuric Vapors", cost := some [generic 3, pip .red], types := [.enchantment],
      text :=
        [ .static (.damageRule .any (.dealtBy (a (.and [spell, .colorIs .red])))
            (.toRecipient (a (.or [permanent, .anyPlayer]))) (.scale (.shifted .up (.lit 1)))
            .repeatedly) ] } }

def lashknifeBarrier : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lashknife Barrier", cost := some [generic 2, pip .white], types := [.enchantment],
      text :=
        [ when (.enters thisEnchantment none) (.draw (.lit 1) (agent := .you)),
          .static (.damageRule .any (.dealtBy (a source)) (.toRecipient (a creatureYouControl))
            (.scale (.shifted .down (.lit 1))) .repeatedly) ] } }

def ghostsOfTheInnocent : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ghosts of the Innocent", cost := some [generic 5, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Spirit"],
      text :=
        [ .static (.damageRule .any (.dealtBy (a source))
            (.toRecipient (a (.or [permanent, .anyPlayer]))) (.scale (.halved .down)) .repeatedly) ],
      power := stat 4, toughness := stat 5 } }

def fireServant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fire Servant", cost := some [generic 3, pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ .static (.damageRule .any
            (.dealtBy (a (.and [instantOrSorcery, .colorIs .red, .hasPossessor .controller .you])))
            .everywhere (.scale (.multiplied .doubled)) .repeatedly) ],
      power := stat 4, toughness := stat 3 } }

def platedPegasus : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Plated Pegasus", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [creatureType "Pegasus"],
      text :=
        [ keyword "Flash",
          keyword "Flying",
          .static (.damageRule .any (.dealtBy (a spell)) (.toRecipient (a (.or [permanent, .anyPlayer])))
            (.prevent (.some (.lit 1)) none) .repeatedly) ],
      power := stat 1, toughness := stat 2 } }

def spitemare : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Spitemare", cost := some [generic 2, hybridPip .red .white, hybridPip .red .white],
      types := [.creature], subtypes := [creatureType "Elemental"],
      text :=
        [ whenever (.isDealtDamage .any thisCreature) (.dealDamage it .thatMuch (target anyTarget)) ],
      power := stat 3, toughness := stat 3 } }

def grollub : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grollub", cost := some [generic 2, pip .black], types := [.creature],
      subtypes := [creatureType "Beast"],
      text := [whenever (.isDealtDamage .any thisCreature) (gainLife .thatMuch (agent := (each
          .opponent)))],
      power := stat 3, toughness := stat 3 } }

def moggManiac : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mogg Maniac", cost := some [generic 1, pip .red], types := [.creature],
      subtypes := [creatureType "Goblin"],
      text :=
        [ whenever (.isDealtDamage .any thisCreature)
            (.dealDamage it .thatMuch (target (.or [.hasType .planeswalker, .opponent]))) ],
      power := stat 1, toughness := stat 1 } }

def repercussion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Repercussion", cost := some [generic 1, pip .red, pip .red], types := [.enchantment],
      text :=
        [ whenever (.isDealtDamage .any (a creature))
            (.dealDamage thisEnchantment .thatMuch (controllerOf (that (.type .creature)))) ] } }

def spitefulShadows : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Spiteful Shadows", cost := some [generic 1, pip .black], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          whenever (.isDealtDamage .any (.attachHost .enchanted (.type .creature)))
            (.dealDamage it .thatMuch (controllerOf it)) ] } }

def bindingAgony : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Binding Agony", cost := some [generic 1, pip .black], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          whenever (.isDealtDamage .any (.attachHost .enchanted (.type .creature)))
            (.dealDamage thisAura .thatMuch (controllerOf (that (.type .creature)))) ] } }

def darienKingOfKjeldor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Darien, King of Kjeldor", cost := some [generic 4, pip .white, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text :=
        [ whenever (.isDealtDamage .any .you)
            (offer (create .thatMuch (creatureToken 1 1 [.white] [creatureType "Soldier"])) (agent
                := .you)) ],
      power := stat 3, toughness := stat 3 } }

def screamingNemesis : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Screaming Nemesis", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ keyword "Haste",
          whenever (.isDealtDamage .any thisCreature)
            (.sequence
              [ .dealDamage it .thatMuch (target (.and [anyTarget, .otherThan .this])),
                .doIf (.dealtThisWay .anyPlayer)
                  (.establish (playerCant (.core .gainLife) they) (some .restOfGame)) none ]) ],
      power := stat 3, toughness := stat 3 } }

def sonicShrieker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sonic Shrieker", cost := some [generic 2, pip .red, pip .white, pip .black],
      types := [.creature], subtypes := [creatureType "Dragon"],
      text :=
        [ keyword "Flying",
          when (.enters thisCreature none)
            (.sequence
              [ .dealDamage it (.lit 2) (target anyTarget),
                gainLife (.lit 2) (agent := .you),
                .doIf (.dealtThisWay .anyPlayer) (discard (a (.inZone hand)) (agent := they)) none
                    ]) ],
      power := stat 4, toughness := stat 4 } }

def grievousWoundLifeLock : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grievous Wound", cost := some [generic 3, pip .black, pip .black], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" .anyPlayer,
          .static (playerCant (.core .gainLife) (.attachHost .enchanted .player)),
          whenever (.isDealtDamage .any (.attachHost .enchanted .player))
            (loseLife (.half .up (lifeTotalOf they)) (agent := they)) ] } }

def cursedScroll : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cursed Scroll", cost := some [generic 1], types := [.artifact],
      text :=
        [ activated (.compound [.mana [generic 3], .tapSymbol])
            (.sequence
              [ choose (a (quality .cardName)),
                revealCards (aAtRandom (.inZone (handOf .you))),
                .doIf (.matches (that .card) (.named .chosen))
                  (.dealDamage thisArtifact (.lit 2) (target anyTarget)) none ]) ] } }

def magusOfTheScroll : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Magus of the Scroll", cost := some [pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ activated (.compound [.mana [generic 3], .tapSymbol])
            (.sequence
              [ choose (a (quality .cardName)),
                revealCards (aAtRandom (.inZone (handOf .you))),
                .doIf (.matches (that .card) (.named .chosen))
                  (.dealDamage thisCreature (.lit 2) (target anyTarget)) none ]) ],
      power := stat 1, toughness := stat 1 } }

/-- Stuffy Doll -/
def stuffyDoll : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stuffy Doll", cost := some [generic 5], types := [.artifact, .creature],
      subtypes := [creatureType "Construct"],
      text :=
        [ keyword "Indestructible",
          .static (entersChoosingPlayer thisCreature none),
          whenever (.isDealtDamage .any thisCreature) (.dealDamage it .thatMuch (the chosenPlayer)),
          activated .tapSymbol (.dealDamage thisCreature (.lit 1) thisCreature) ],
      power := stat 0, toughness := stat 1 } }

def saheeliRaiPlusOne : Instruction :=
  .sequence [scry (.lit 1) (agent := .you), .dealDamage .this (.lit 1) (each .opponent)]
theorem okSaheeliRaiPlusOne : Instruction.check [] saheeliRaiPlusOne = [] := by decide
def sarkhansUnsealingLine : Ability :=
  whenever
    (.casts .you
      (a (.and [creature, spell,
                .or [ .compare [.stat .power] .eq (.lit 4), .compare [.stat .power] .eq (.lit 5),
                      .compare [.stat .power] .eq (.lit 6) ]]))
      none)
    (.dealDamage thisEnchantment (.lit 4) (target anyTarget))
theorem okSarkhansUnsealingLine : Ability.check [] sarkhansUnsealingLine = [] := by decide
/-- Savage Swipe, both sentences -/
def savageSwipeLine : Instruction :=
  .sequence
    [ .doOnlyIf (get (target creatureYouControl) (.up (.lit 2)) (.up (.lit 2)) (some
        untilEndOfTurn))
        (.compareAmt (.statOf (.stat .power) it) .eq (.lit 2)) none,
      .fight it (target creatureYouDontControl) ]
theorem okSavageSwipeLine : Instruction.check [] savageSwipeLine = [] := by decide

def infernoOfTheStarMounts : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Inferno of the Star Mounts", cost := some [generic 4, pip .red, pip .red],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Dragon"],
      text :=
        [ .static (objectCant (.action "Counter") .this),
          keyword "Flying",
          keyword "Haste",
          activated (.mana [pip .red])
            (.triggerThisWay (get thisCreature (.up (.lit 1)) (.up (.lit 0)) (some untilEndOfTurn))
              (.statBecomes it .power (.lit 20))
              (.dealDamage it (.lit 20) (target anyTarget))) ],
      power := stat 6, toughness := stat 6 } }

def incinerate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Incinerate", cost := some [generic 1, pip .red], types := [.instant],
      text :=
        [ .spell none (.sequence
            [ .dealDamage .this (.lit 3) (target anyTarget),
              .establish
                (objectCant (.action "Regenerate")
                  (a (.and [creature, .happenedTo (.mk .damageTaken .thisWay none)])))
                (some .thisTurn) ]) ] } }

def ashZealot : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ash Zealot", cost := some [pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Warrior"],
      text :=
        [ keyword "FirstStrike",
          keyword "Haste",
          whenever (.casts (a .anyPlayer) (a (.and [spell, .castFrom graveyard])) none)
            (.dealDamage thisCreature (.lit 3) (that .player)) ],
      power := stat 2, toughness := stat 2 } }

def galvanicBlastLine : Instruction :=
  .replace (.dealDamage .this (.lit 2) (target anyTarget))
    (.doOnlyIf (.dealDamage .this (.lit 4) thatJoin)
      (.compareAmt (countOf (.and [artifact, .hasPossessor .controller .you])) .atLeast (.lit 3)) none)
theorem okGalvanicBlastLine : Instruction.check [] galvanicBlastLine = [] := by decide
/-- Furious Reprisal -/
def furiousReprisal : Instruction :=
  .dealDamage .this (.lit 2) (.eachOf (.described (.target (exactly 2)) anyTarget))
theorem okFuriousReprisal : Instruction.check [] furiousReprisal = [] := by decide
/-- Bullseye, Death Dealer -/
def bullseyeModalCost : Ability :=
  activated
    (.compound [.mana [generic 3], .tapSymbol,
      .perform (chooseModes (exactly 1)
        [ sacrifice (a artifact) (agent := .you), discard (a (.and [.not land, .inZone hand]))
            (agent := .you) ])])
    (.dealDamage .this (.lit 2) (target anyTarget))
theorem okBullseyeModalCost : Ability.check [] bullseyeModalCost = [] := by decide

/-- Twinshot Sniper -/
def twinshotSniper : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Twinshot Sniper", cost := some [generic 3, pip .red], types := [.artifact, .creature],
      subtypes := [creatureType "Goblin", creatureType "Archer"],
      text :=
        [ keyword "Reach",
          when (.enters thisCreature none) (.dealDamage it (.lit 2) (target anyTarget)),
          abilityWord "channel"
            (activated (.compound [.mana [generic 1, pip .red], .perform (discard .this (agent :=
                .you))])
              (.dealDamage it (.lit 2) (target anyTarget))) ],
      power := stat 2, toughness := stat 3 } }

/-- Quakebringer -/
def quakebringerDamage : Ability :=
  triggeredIf (.beginningOf .the .upkeep (.byPlayer .you))
    (.or [ .matches .this (.inZone battlefield),
           .and [ .matches .this (.inZone (graveyardOf .you)),
                  exists_ (.and [creature, .hasSubtype (creatureType "Giant"),
                                 .hasPossessor .controller .you]) ] ])
    (.dealDamage .this (.lit 2) (each .opponent))
theorem okQuakebringerDamage : Ability.check [] quakebringerDamage = [] := by decide
/-- Sand Strangler -/
def sandStranglerDamage : Ability :=
  triggeredIf (.enters thisCreature none)
    (.or [ exists_ (.and [land, .hasSubtype (landType "Desert"), .hasPossessor .controller .you]),
           exists_ (.and [land, .hasSubtype (landType "Desert"), .inZone (graveyardOf .you)]) ])
    (offer (.dealDamage .this (.lit 3) (target creature)) (agent := .you))
theorem okSandStranglerDamage : Ability.check [] sandStranglerDamage = [] := by decide

def brazenDwarf : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Brazen Dwarf", cost := some [generic 1, pip .red], types := [.creature],
      subtypes := [creatureType "Dwarf", creatureType "Shaman"],
      text :=
        [ whenever (.rollsDice .you .many none .anyResult)
            (.dealDamage thisCreature (.lit 1) (each .opponent)) ],
      power := stat 1, toughness := stat 3 } }

/-- Lavalanche -/
def lavalanche : Instruction :=
  .performSimultaneously
    [ .dealDamage .this (.letter .x) targetPlayerOrPlaneswalker,
      .dealDamage .this (.letter .x) eachCreatureThatSplitControls ]
theorem okLavalanche : Instruction.check [] lavalanche = [] := by decide
/-- Flame Wave -/
def flameWave : Instruction :=
  .performSimultaneously
    [ .dealDamage .this (.lit 4) targetPlayerOrPlaneswalker,
      .dealDamage .this (.lit 4) eachCreatureThatSplitControls ]
theorem okFlameWave : Instruction.check [] flameWave = [] := by decide
/-- Chandra Nalaar's ultimate -/
def chandraNalaarUltimate : Instruction :=
  .performSimultaneously
    [ .dealDamage .this (.lit 10) targetPlayerOrPlaneswalker,
      .dealDamage .this (.lit 10) eachCreatureThatSplitControls ]
theorem okChandraNalaarUltimate : Instruction.check [] chandraNalaarUltimate = [] := by decide
/-- Chandra, Pyrogenius's ultimate -/
def chandraPyrogeniusUltimate : Instruction :=
  .performSimultaneously
    [ .dealDamage .this (.lit 6) targetPlayerOrPlaneswalker,
      .dealDamage .this (.lit 6) eachCreatureThatSplitControls ]
theorem okChandraPyrogeniusUltimate : Instruction.check [] chandraPyrogeniusUltimate = [] := by decide
/-- Bonfire of the Damned -/
def bonfireOfTheDamned : Instruction :=
  .performSimultaneously
    [ .dealDamage .this (.letter .x) targetPlayerOrPlaneswalker,
      .dealDamage .this (.letter .x) eachCreatureThatSplitControls ]
theorem okBonfireOfTheDamned : Instruction.check [] bonfireOfTheDamned = [] := by decide
/-- Chandra's Fury -/
def chandrasFury : Instruction :=
  .performSimultaneously
    [ .dealDamage .this (.lit 4) targetPlayerOrPlaneswalker,
      .dealDamage .this (.lit 1) eachCreatureThatSplitControls ]
theorem okChandrasFury : Instruction.check [] chandrasFury = [] := by decide
/-- Angrath, Minotaur Pirate's plus -/
def angrathMinotaurPirateBolt : Instruction :=
  .performSimultaneously
    [ .dealDamage .this (.lit 1) targetOpponentOrPlaneswalker,
      .dealDamage .this (.lit 1) eachCreatureThatSplitControls ]
theorem okAngrathMinotaurPirateBolt : Instruction.check [] angrathMinotaurPirateBolt = [] := by decide
def whichOfYouBurnsBrightestBody : Instruction :=
  .performSimultaneously
    [ .dealDamage .this (.letter .x) targetOpponentOrPlaneswalker,
      .dealDamage .this (.letter .x) eachCreatureThatSplitControls ]
theorem okWhichOfYouBurnsBrightestBody :
    Instruction.check [] whichOfYouBurnsBrightestBody = [] := by decide
/-- Chandra, Pyromaster's plus -/
def chandraPyromasterBolt : Instruction :=
  .performSimultaneously
    [ .dealDamage .this (.lit 1) targetPlayerOrPlaneswalker,
      .dealDamage .this (.lit 1)
        (.described (.target (upTo 1))
          (.and [creature, .hasPossessor .controller splitOverPlaneswalker])) ]
theorem okChandraPyromasterBolt : Instruction.check [] chandraPyromasterBolt = [] := by decide
/-- Ravager of the Fells -/
def ravagerOfTheFellsBolt : Instruction :=
  .performSimultaneously
    [ .dealDamage thisCreature (.lit 2) targetOpponentOrPlaneswalker,
      .dealDamage thisCreature (.lit 2)
        (.described (.target (upTo 1))
          (.and [creature, .hasPossessor .controller splitOverPlaneswalker])) ]
theorem okRavagerOfTheFellsBolt : Instruction.check [] ravagerOfTheFellsBolt = [] := by decide
/-- Soul of Shandalar's battlefield activation -/
def soulOfShandalarBolt : Instruction :=
  .performSimultaneously
    [ .dealDamage thisCreature (.lit 3) targetPlayerOrPlaneswalker,
      .dealDamage thisCreature (.lit 3)
        (.described (.target (upTo 1))
          (.and [creature, .hasPossessor .controller splitOverPlaneswalker])) ]
theorem okSoulOfShandalarBolt : Instruction.check [] soulOfShandalarBolt = [] := by decide
/-- Soul of Shandalar's graveyard activation -/
def soulOfShandalarGraveyardBolt : Instruction :=
  .performSimultaneously
    [ .dealDamage .this (.lit 3) targetPlayerOrPlaneswalker,
      .dealDamage .this (.lit 3)
        (.described (.target (upTo 1))
          (.and [creature, .hasPossessor .controller splitOverPlaneswalker])) ]
theorem okSoulOfShandalarGraveyardBolt :
    Instruction.check [] soulOfShandalarGraveyardBolt = [] := by decide
/-- Blightning -/
def blightning : Instruction :=
  .sequence
    [ .dealDamage .this (.lit 3) targetPlayerOrPlaneswalker,
      discard (counted (exactly 2) (.inZone hand)) (agent := splitOverPlaneswalker) ]
theorem okBlightning : Instruction.check [] blightning = [] := by decide
/-- Rakdos's Return -/
def rakdossReturn : Instruction :=
  .sequence
    [ .dealDamage .this (.letter .x) targetOpponentOrPlaneswalker,
      discard (counted (.exactlyOf (.letter .x)) (.inZone hand)) (agent := splitOverPlaneswalker) ]
theorem okRakdossReturn : Instruction.check [] rakdossReturn = [] := by decide
/-- Nicol Bolas, Planeswalker's ultimate -/
def nicolBolasUltimate : Instruction :=
  .sequence
    [ .dealDamage .this (.lit 7) targetPlayerOrPlaneswalker,
      discard (counted (exactly 7) (.inZone hand)) (agent := splitOverPlaneswalker),
      sacrifice (counted (exactly 7) permanent) (agent := splitOverPlaneswalker) ]
theorem okNicolBolasUltimate : Instruction.check [] nicolBolasUltimate = [] := by decide
/-- Pulse of the Forge -/
def pulseOfTheForge : Instruction :=
  .sequence
    [ .dealDamage .this (.lit 4) targetPlayerOrPlaneswalker,
      .doIf (.compareAmt (lifeTotalOf splitOverPlaneswalker) .greater (lifeTotalOf .you))
        (move .this hand) none ]
theorem okPulseOfTheForge : Instruction.check [] pulseOfTheForge = [] := by decide
/-- Goblin Lyre's losing arm -/
def goblinLyreLoseFlip : Instruction :=
  .sequence
    [ .dealDamage thisArtifact (countOf creatureYouControl) targetOpponentOrPlaneswalker,
      .dealDamage thisArtifact
        (countOf (.and [creature, .hasPossessor .controller splitOverPlaneswalker])) .you ]
theorem okGoblinLyreLoseFlip : Instruction.check [] goblinLyreLoseFlip = [] := by decide
/-- Quenchable Fire -/
def quenchableFire : Instruction :=
  .sequence
    [ .dealDamage .this (.lit 3) targetPlayerOrPlaneswalker,
      delay (.beginningOf .the .upkeep (.byPlayer .you))
        (doUnless (.dealDamage .this (.lit 3) thatJoin) (.mana [pip .blue]) (agent :=
            splitOverPlaneswalker)) ]
theorem okQuenchableFire : Instruction.check [] quenchableFire = [] := by decide
/-- Searing Blaze, both sentences -/
def searingBlaze : Ability :=
  abilityWord "landfall"
    (.spell none
      (.replace
        (.performSimultaneously
          [ .dealDamage .this (.lit 1) targetPlayerOrPlaneswalker,
            .dealDamage .this (.lit 1)
              (target (.and [creature, .hasPossessor .controller splitOverPlaneswalker])) ])
        (.performSimultaneously
          [ .dealDamage .this (.lit 3) thatJoin,
            .dealDamage .this (.lit 3) (that (.type .creature)) ])))
theorem okSearingBlaze : Ability.check [] searingBlaze = [] := by decide

/-- Heart of Bogardan -/
def heartOfBogardan : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Heart of Bogardan", cost := some [generic 2, pip .red, pip .red], types := [.enchantment],
      text :=
        [ cumulativeUpkeep (.mana [generic 2]),
          when heartOfBogardanHeader
            (.sequence
              [ .performSimultaneously
                  [ .dealDamage .this (.letter .x) targetPlayerOrPlaneswalker,
                    .dealDamage .this (.letter .x) eachCreatureThatSplitControls ],
                .define .x
                  (.arith .minus (times (.lit 2) (countersOn (.named "Age") thisEnchantment)) (.lit 2)) ]) ] } }

/-- Burn at the Stake -/
def burnAtTheStake : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Burn at the Stake", cost := some [generic 2, pip .red, pip .red, pip .red],
      types := [.sorcery],
      text :=
        [ .static (.addedCost
            (.perform (tap (counted anyNumber (.and [creature, .hasPossessor .controller .you, untapped]))))
            false),
          .spell none (.dealDamage .this (times (.lit 3) .groupSize) (target anyTarget)) ] } }

/-- Explosive Singularity -/
def explosiveSingularity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Explosive Singularity", cost := some [generic 8, pip .red, pip .red], types := [.sorcery],
      text :=
        [ .static (.addedCost
            (.perform (tap (counted anyNumber (.and [creature, .hasPossessor .controller .you, untapped]))))
            true),
          .static (.costShift .this (.less (times (.lit 1) .groupSize) none)),
          .spell none (.dealDamage .this (.lit 10) (target anyTarget)) ] } }

/-- Tyrant of Valakut -/
def tyrantOfValakut : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tyrant of Valakut", cost := some [generic 5, pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Dragon"],
      text :=
        [ keywordCosting "Surge" (.mana [generic 3, pip .red, pip .red]),
          keyword "Flying",
          triggeredIf (.enters thisCreature none) (costWasPaid (.byKeyword "Surge") none thisCreature)
            (.dealDamage thisCreature (.lit 3) (target anyTarget)) ],
      power := stat 5, toughness := stat 4 } }

/-- Fall of the Titans -/
def fallOfTheTitansCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fall of the Titans", cost := some [.variable, .variable, pip .red], types := [.instant],
      text :=
        [ keywordCosting "Surge" (.mana [.variable, pip .red]),
          .spell none
            (.dealDamage .this (.letter .x) (.eachOf (.described (.target (upTo 2)) anyTarget))) ] } }

/-- Tribal Flames -/
def tribalFlames : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tribal Flames", cost := some [generic 1, pip .red], types := [.sorcery],
      text :=
        [ abilityWord "domain"
            (.spell none (.sequence
              [ .dealDamage .this (.letter .x) (target anyTarget),
                .define .x (.distinctCount (.subtype .land .basicOnly)
                  (allOf (.and [land, .hasPossessor .controller .you]))) ])) ] } }

/-- Explosive Prodigy -/
def explosiveProdigyTrigger : Ability :=
  abilityWord "vivid"
    (when (.enters thisCreature none)
      (.sequence
        [ .dealDamage it (.letter .x) (target (.and [creature, .hasPossessor .controller anOpponent])),
          .define .x (.distinctCount .color (allOf (.and [permanent, .hasPossessor .controller .you]))) ]))
theorem okExplosiveProdigyTrigger : Ability.check [] explosiveProdigyTrigger = [] := by decide
/-- Niv-Mizzet, Guildpact -/
def nivMizzetGuildpactTrigger : Ability :=
  whenever (dealsCombatDamage thisCreature (a .anyPlayer))
    (.sequence
      [ .dealDamage thisCreature (.letter .x) (target anyTarget),
        .draw (.letter .x) (agent := (target .anyPlayer)),
        gainLife (.letter .x) (agent := .you),
        .define .x (.distinctCount .colorPair
          (allOf (.and [permanent, .hasPossessor .controller .you, .colorCount .eq 2]))) ])
theorem okNivMizzetGuildpactTrigger : Ability.check [] nivMizzetGuildpactTrigger = [] := by decide

/-- Aurelia, the Law Above -/
def aureliaTheLawAbove : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aurelia, the Law Above", cost := some [generic 3, pip .red, pip .white],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Angel"],
      text :=
        [ keyword "Flying",
          keyword "Vigilance",
          keyword "Haste",
          whenever (.attacksWith (a .anyPlayer) none (counted (atLeast 3) creature)) (.draw (.lit 1)
              (agent := .you)),
          whenever (.attacksWith (a .anyPlayer) none (counted (atLeast 5) creature))
            (.sequence [.dealDamage .this (.lit 3) (each .opponent), gainLife (.lit 3) (agent :=
                .you)]) ],
      power := stat 4, toughness := stat 4 } }

/-- Syr Konrad, the Grim -/
def syrKonradTheGrim : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Syr Konrad, the Grim", cost := some [generic 3, pip .black, pip .black],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Knight"],
      text :=
        [ triggeredOr (.dies (a (.and [creature, .otherThan .this])))
            [ putIntoFrom (a creature) graveyard (.anywhereBut [battlefield]),
              leavesZone (a (.and [creature, .inZone (graveyardOf .you)])) (graveyardOf .you) ]
            (.dealDamage .this (.lit 1) (each .opponent)),
          activated (.mana [generic 1, pip .black]) (mill (.lit 1) (each .anyPlayer) (agent := (each
              .anyPlayer))) ],
      power := stat 5, toughness := stat 4 } }

/-- Destructive Revelry -/
def destructiveRevelry : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Destructive Revelry", cost := some [pip .red, pip .green], types := [.instant],
      text :=
        [ .spell none (.sequence
            [ destroy (target (.or [artifact, enchantment])),
              .dealDamage .this (.lit 2) (controllerOf (that .permanent)) ]) ] } }

/-- Fuming Effigy -/
def fumingEffigy : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fuming Effigy", cost := some [generic 3, pip .red], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ whenever (leavesZone (counted (atLeast 1) (.inZone (graveyardOf .you))) (graveyardOf .you))
            (.dealDamage .this (.lit 1) (each .opponent)) ],
      power := stat 4, toughness := stat 3 } }

/-- Breeches, Brazen Plunderer's slice -/
def eachOfThoseOpponentsTopCard : Instruction :=
  .sequence
    [ .dealDamage .this (.lit 1) (each .opponent),
      exile (.librarySlice .top (.lit 1) (.eachOf (those .player))) ]
theorem okEachOfThoseOpponentsTopCard : Instruction.check [] eachOfThoseOpponentsTopCard = [] := by
  decide

/-- Inquisitor's Flail -/
def inquisitorsFlail : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Inquisitor's Flail", cost := some [generic 2], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ .static (.damageRule .combatOnly (.dealtBy (.attachHost .equipped (.type .creature))) .everywhere
            (.scale (.multiplied .doubled)) .repeatedly),
          .static (.damageRule .combatOnly
            (.dealtBy (a (otherCreature (.attachHost .equipped (.type .creature)))))
            (.toRecipient (.attachHost .equipped (.type .creature))) (.scale (.multiplied .doubled))
            .repeatedly),
          keywordCosting "Equip" (.mana [generic 2]) ] } }

/-- Oath of Kaya -/
def oathOfKaya : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Oath of Kaya", cost := some [generic 1, pip .white, pip .black],
      supertypes := [.legendary], types := [.enchantment],
      text :=
        [ when (.enters .this none)
            (.sequence [.dealDamage .this (.lit 3) (target anyTarget), gainLife (.lit 3) (agent :=
                .you)]),
          whenever
            (.attacksWith anOpponent
              (some (a (.and [.hasType .planeswalker, .hasPossessor .controller .you])))
              (counted (atLeast 1) creature))
            (.sequence [.dealDamage .this (.lit 2) (that .player), gainLife (.lit 2) (agent :=
                .you)]) ] } }

/-- Frostwielder -/
def frostwielder : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Frostwielder", cost := some [generic 2, pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Shaman"],
      text :=
        [ .static (.replacement
            (.dies (a (.and [creature,
              .happenedTo (.mk .damageTaken .thisTurn (some (.involving thisCreature)))])))
            [] none (exile it) .repeatedly none),
          activated .tapSymbol (.dealDamage thisCreature (.lit 1) (target anyTarget)) ],
      power := stat 1, toughness := stat 2 } }

/-- Crackling Doom -/
def cracklingDoom : Instruction :=
  .sequence
    [ .dealDamage .this (.lit 2) (each .opponent),
      sacrifice
        (a (.and [creature,
                  .superlative .max (.stat .power)
                    (.and [creature, .hasPossessor .controller (that .player)])])) (agent := (each
                        .opponent)) ]
theorem okCracklingDoom : Instruction.check [] cracklingDoom = [] := by decide
def theFallen : Ability :=
  at_ (.beginningOf .the .upkeep (.byPlayer .you))
    (.dealDamage thisCreature (.lit 1)
      (each (.and [.or [.opponent, .hasType .planeswalker],
                   happenedToInvolving .damageTaken .thisGame thisCreature])))
theorem okTheFallen : Ability.check [] theFallen = [] := by decide
/-- Cavalcade of Calamity -/
def cavalcadeOfCalamity : Ability :=
  whenever
    (attacks (a (.and [creature, .hasPossessor .controller .you, .compare [.stat .power] .atMost (.lit 1)])))
    (.dealDamage thisEnchantment (.lit 1)
      (the (.and [.or [.anyPlayer, .hasType .planeswalker],
                  .inCombat .attackedBy (some (that (.type .creature)))])))
theorem okCavalcadeOfCalamity : Ability.check [] cavalcadeOfCalamity = [] := by decide

/-- Blind Fury -/
def blindFury : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blind Fury", cost := some [generic 2, pip .red, pip .red], types := [.instant],
      text :=
        [ .spell none (.sequence
            [ .establish (.abilityLoss (allOf creature) [.written (keyword "Trample")])
                (some untilEndOfTurn),
              .establish
                (.damageRule .combatOnly (.dealtBy (a creature)) (.toRecipient (a creature))
                  (.scale (.multiplied .doubled)) .repeatedly)
                (some .thisTurn) ]) ] } }

/-- Chandra, Awakened Inferno's emblem -/
def chandraAwakenedInfernoEmblem : Instruction :=
  .getEmblem
    [ at_ (.beginningOf .the .upkeep (.byPlayer .you))
        (.dealDamage (.asMarker .emblem .this) (.lit 1) .you) ] (agent := .you)
theorem okChandraAwakenedInfernoEmblem :
    Instruction.check [] chandraAwakenedInfernoEmblem = [] := by decide

/-- Keeper of the Flame -/
def keeperOfTheFlame : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Keeper of the Flame", cost := some [pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ activated (.compound [.mana [pip .red], .tapSymbol])
            (.sequence
              [ chooseWhile
                  (target (.and [.opponent, .compare [.playerStat .lifeTotal] .greater (lifeTotalOf .you)]))
                  (.whileDoing (.activates .you thisAbility)),
                .dealDamage thisCreature (.lit 2) (that .player) ]) ],
      power := stat 1, toughness := stat 2 } }

def diabolicEdict : Instruction := sacrifice (aTheirChoice creature) (agent := (target .anyPlayer))
theorem okDiabolicEdict : Instruction.check [] diabolicEdict = [] := by decide
def innocentBlood : Instruction := sacrifice (aTheirChoice creature) (agent := (each .anyPlayer))
theorem okInnocentBlood : Instruction.check [] innocentBlood = [] := by decide
def cryOfContrition : Instruction := discard (a (.inZone hand)) (agent := (target .anyPlayer))
theorem okCryOfContrition : Instruction.check [] cryOfContrition = [] := by decide
def cyclingCost : Instruction := discard .this (agent := .you)
theorem okCyclingCost : Instruction.check [] cyclingCost = [] := by decide
def raiseTheAlarm : Instruction :=
  create (.lit 2) (creatureToken 1 1 [.white] [creatureType "Soldier"])
theorem okRaiseTheAlarm : Instruction.check [] raiseTheAlarm = [] := by decide
def actOfTreason : Instruction := gainControl (target creature) (some untilEndOfTurn) (agent :=
    .you)
theorem okActOfTreason : Instruction.check [] actOfTreason = [] := by decide
def wordsOfWorship : Instruction :=
  replaceNextEvent (.draws .you) (gainLife (.lit 5) (agent := .you)) (some .thisTurn)
theorem okWordsOfWorship : Instruction.check [] wordsOfWorship = [] := by decide
def theLastRoninII : Instruction :=
  .triggerReflexively (mill (.lit 4) .you (agent := .you))
    (move (target (.and [creature, .inZone (graveyardOf .you)])) hand)
theorem okTheLastRoninII : Instruction.check [] theLastRoninII = [] := by decide

def moonlitWakeCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Moonlit Wake", cost := some [generic 2, pip .white], types := [.enchantment],
      text := [moonlitWake] } }

def laquatussDisdain : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Laquatus's Disdain", cost := some [generic 1, pip .blue], types := [.instant],
      text :=
        [ .spell none (.sequence
            [ .counterSpell (target (.and [spell, .castFrom graveyard])), .draw (.lit 1) (agent :=
                .you) ]) ] } }

/-- Stifle -/
def stifle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stifle", cost := some [pip .blue], types := [.instant],
      text :=
        [ .spell none
            (.counterSpell (target (.or [.abilityHead .anyActivated, .abilityHead .anyTriggered]))) ] } }

/-- Disallow -/
def disallow : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Disallow", cost := some [generic 1, pip .blue, pip .blue], types := [.instant],
      text :=
        [ .spell none
            (.counterSpell
              (target (.or [spell, .abilityHead .anyActivated, .abilityHead .anyTriggered]))) ] } }

/-- Fry -/
def fry : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fry", cost := some [generic 1, pip .red], types := [.instant],
      text :=
        [ .static (objectCant (.action "Counter") .this),
          .spell none
            (.dealDamage .this (.lit 5)
              (target (.and [.or [creature, .hasType .planeswalker], .or [.colorIs .white, .colorIs .blue]]))) ] } }

/-- Termination Facilitator -/
def terminationFacilitator : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Termination Facilitator", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Assassin"],
      text :=
        [ activatedOnlyDuring .tapSymbol
            (.putCounters (.lit 1) (.printed (.named "Bounty"))
              (target (.or [creature, .hasType .planeswalker])))
            .asSorcery,
          whenever
            (.isDealtDamage .any
              (a (.and [.or [creature, .hasType .planeswalker], .hasPossessor .controller anOpponent,
                        .hasCounters (some (.named "Bounty"))])))
            (destroy it) ],
      power := stat 1, toughness := stat 3 } }

end Semantics.Cards
