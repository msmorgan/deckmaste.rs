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
def bolt : Instruction := Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 3) (target anyTarget)
theorem okBolt : Instruction.check [] bolt = [] := by decide
def barrageOfBoulders : Instruction := Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 1) (each creatureYouDontControl)
theorem okBarrageOfBoulders : Instruction.check [] barrageOfBoulders = [] := by decide
def rabidBite : Instruction :=
  Primitives.Instruction.dealDamage (target creatureYouControl) (Primitives.Amount.statOf (.stat .power) it) (target creatureYouDontControl)
theorem okRabidBite : Instruction.check [] rabidBite = [] := by decide
def preyUpon : Instruction := Primitives.Instruction.fight (target creatureYouControl) (target creatureYouDontControl)
theorem okPreyUpon : Instruction.check [] preyUpon = [] := by decide
def arcTrail : Instruction :=
  Primitives.Instruction.sequence
    [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 2) (target anyTarget), Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 1) (target anyOtherTarget) ]
theorem okArcTrail : Instruction.check [] arcTrail = [] := by decide
def deadshot : Instruction :=
  Primitives.Instruction.sequence
    [ Primitives.Instruction.setStatus .tapped (target creature),
      Primitives.Instruction.dealDamage it (Primitives.Amount.statOf (.stat .power) it) (target (Primitives.Predicate.and [creature, Primitives.Predicate.other])) ]
theorem okDeadshot : Instruction.check [] deadshot = [] := by decide
def immersturmSkullcairn : Ability :=
  activatedOnlyDuring
    (Primitives.Cost.compound [Primitives.Cost.mana [generic 1, pip .black, pip .red, pip .red], Primitives.Cost.tapSymbol,
      Primitives.Cost.perform (sacrifice thisLand (agent := Primitives.NounPhrase.you))])
    (Primitives.Instruction.sequence
      [ Primitives.Instruction.dealDamage it (.lit 3) (target Primitives.Predicate.anyPlayer), discard (a (Primitives.Predicate.inZone hand)) (agent := (that
          .player)) ])
    Primitives.Timing.asSorcery
theorem okImmersturmSkullcairn : Ability.check [] immersturmSkullcairn = [] := by decide
def pyriteSpellbomb : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [pip .red], Primitives.Cost.perform (sacrifice thisArtifact (agent := Primitives.NounPhrase.you))])
    (Primitives.Instruction.dealDamage it (.lit 2) (target anyTarget))
theorem okPyriteSpellbomb : Ability.check [] pyriteSpellbomb = [] := by decide
def karplusanYeti : Instruction :=
  Primitives.Instruction.sequence
    [ dealDamageOwnPower thisCreature (target creature),
      Primitives.Instruction.dealDamage (that (.type .creature)) (Primitives.Amount.statOf (.stat .power) it) thisCreature ]
theorem okKarplusanYeti : Instruction.check [] karplusanYeti = [] := by decide
def suddenDemise : Instruction :=
  Primitives.Instruction.sequence
    [ choose (a (quality .color)),
      Primitives.Instruction.dealDamage Primitives.NounPhrase.this (Primitives.Amount.letter .x) (each (Primitives.Predicate.and [creature, ofChosen .color])) ]
theorem okSuddenDemise : Instruction.check [] suddenDemise = [] := by decide
def caseOfTheGatewayExpress : Instruction :=
  Primitives.Instruction.sequence
    [ choose (target creatureYouDontControl),
      Primitives.Instruction.dealDamage (each creatureYouControl) (.lit 1) (that (.type .creature)) ]
theorem okCaseOfTheGatewayExpress : Instruction.check [] caseOfTheGatewayExpress = [] := by decide
def arrowsOfJustice : Instruction :=
  Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 4) (target (Primitives.Predicate.and [creature, Primitives.Predicate.or [attacking, blocking]]))
theorem okArrowsOfJustice : Instruction.check [] arrowsOfJustice = [] := by decide
def flamesOfTheRazeBoar : Instruction :=
  Primitives.Instruction.sequence
    [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 4) (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller anOpponent])),
      Primitives.Instruction.doOnlyIf
        (Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 2)
          (each (Primitives.Predicate.and [creature, Primitives.Predicate.other, Primitives.Predicate.hasPossessor .controller (that .player)])))
        (exists_ (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.compare [.stat .power] .atLeast (.lit 4)]))
        none ]
theorem okFlamesOfTheRazeBoar : Instruction.check [] flamesOfTheRazeBoar = [] := by decide
def yawgmothDemon : Instruction :=
  Primitives.Instruction.offer (sacrifice (a artifact) (agent := Primitives.NounPhrase.you)) none
    (some (Primitives.Instruction.sequence [Primitives.Instruction.setStatus .tapped thisCreature, Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 2) Primitives.NounPhrase.you])) (agent :=
        Primitives.NounPhrase.you)
theorem okYawgmothDemon : Instruction.check [] yawgmothDemon = [] := by decide
def arcBlade : Instruction :=
  Primitives.Instruction.sequence
    [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 2) (target anyTarget), exileWithCounters Primitives.NounPhrase.this (.lit 3) (.named "Time") ]
theorem okArcBlade : Instruction.check [] arcBlade = [] := by decide
def abrade : Instruction :=
  chooseModes (exactly 1) [Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 3) (target creature), destroy (target artifact)]
theorem okAbrade : Instruction.check [] abrade = [] := by decide
def nibelheimAflame : Instruction :=
  Primitives.Instruction.sequence
    [ choose (target creatureYouControl),
      Primitives.Instruction.dealDamage it (Primitives.Amount.statOf (.stat .power) it) (each (otherCreature it)) ]
theorem okNibelheimAflame : Instruction.check [] nibelheimAflame = [] := by decide
def brashTaunter : Instruction := Primitives.Instruction.fight thisCreature (target (otherCreature thisCreature))
theorem okBrashTaunter : Instruction.check [] brashTaunter = [] := by decide
def ulvenwaldTracker : Instruction :=
  Primitives.Instruction.fight (target creatureYouControl) (target (Primitives.Predicate.and [creature, Primitives.Predicate.other]))
theorem okUlvenwaldTracker : Instruction.check [] ulvenwaldTracker = [] := by decide

def botBashingTime : Instruction :=
  Primitives.Instruction.sequence
    [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 6) (target creature),
      replaceEvent (Primitives.GameEvent.dies (that (.type .creature))) (exile it) (some Primitives.Duration.thisTurn) ]
theorem okBotBashingTime : Instruction.check [] botBashingTime = [] := by decide
def wordsOfWar : Instruction :=
  replaceNextEvent (Primitives.GameEvent.draws Primitives.NounPhrase.you) (Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 2) (target anyTarget)) (some Primitives.Duration.thisTurn)
theorem okWordsOfWar : Instruction.check [] wordsOfWar = [] := by decide

/-- Thunderwave -/
def thunderwave : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thunderwave", cost := some [generic 2, pip .red, pip .red], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequence
            [ rollDice 1 20 (agent := Primitives.NounPhrase.you),
              Primitives.Instruction.applyResultsTable
                [ rollRow (fromTo 1 9) (Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 3) (each creature)),
                  rollRow (fromTo 10 19)
                    (Primitives.Instruction.sequence
                      [ offer (choose (a creature) (agent := some Primitives.NounPhrase.you)) (agent := Primitives.NounPhrase.you),
                        Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 3) (each (Primitives.Predicate.and [creature, Primitives.Predicate.notChosen])) ]),
                  rollRow (fromTo 20 20)
                    (Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 6)
                      (each (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller (Primitives.NounPhrase.playerGroup .yourOpponents)]))) ] ]) ] } }

/-- Fog, Holy Day, Darkness, Root Snare -/
def fog : Instruction := preventAll .combatOnly Primitives.DamageScope.everywhere (some Primitives.Duration.thisTurn)
theorem okFog : Instruction.check [] fog = [] := by decide
/-- Indestructible Aura, Shielded Passage -/
def indestructibleAura : Instruction :=
  preventAll .any (Primitives.DamageScope.toRecipient (target creature)) (some Primitives.Duration.thisTurn)
theorem okIndestructibleAura : Instruction.check [] indestructibleAura = [] := by decide
def shieldmatesBlessing : Instruction :=
  preventNext .any (Primitives.DamageScope.toRecipient (target anyTarget)) (.lit 3) (some Primitives.Duration.thisTurn)
theorem okShieldmatesBlessing : Instruction.check [] shieldmatesBlessing = [] := by decide
def moonlitWake : Ability := whenever (Primitives.GameEvent.dies (a creature)) (gainLife (.lit 1) (agent := Primitives.NounPhrase.you))
theorem okMoonlitWake : Ability.check [] moonlitWake = [] := by decide
def eliteJavelineer : Ability :=
  whenever (blocks thisCreature none) (Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 1) (target (Primitives.Predicate.and [creature, attacking])))
theorem okEliteJavelineer : Ability.check [] eliteJavelineer = [] := by decide
/-- Glacial Chasm -/
def glacialChasmShield : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.damageRule .any Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you) (Primitives.DamageOp.prevent Primitives.PreventCut.all none) .repeatedly)
theorem okGlacialChasmShield : Ability.check [] glacialChasmShield = [] := by decide
def corneredCrook : Ability :=
  when (Primitives.GameEvent.enters thisCreature none)
    (offerWhen (sacrifice (a artifact) (agent := Primitives.NounPhrase.you)) (Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 3) (target
        anyTarget)) (agent := Primitives.NounPhrase.you))
theorem okCorneredCrook : Ability.check [] corneredCrook = [] := by decide

def aladdinsRing : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aladdin's Ring", cost := some [generic 8], types := [.artifact],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 8], Primitives.Cost.tapSymbol])
            (Primitives.Instruction.dealDamage thisArtifact (.lit 4) (target anyTarget)) ] } }

def chandrasRevolution : Instruction :=
  Primitives.Instruction.sequence
    [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 4) (target creature), Primitives.Instruction.setStatus .tapped (target land),
      Primitives.Instruction.skipUntap (that (.type .land)) (.lit 1) ]
theorem okChandrasRevolution : Instruction.check [] chandrasRevolution = [] := by decide
def arbalestElite : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2, pip .white], Primitives.Cost.tapSymbol])
    (Primitives.Instruction.sequence
      [ Primitives.Instruction.dealDamage thisCreature (.lit 3) (target (Primitives.Predicate.and [creature, Primitives.Predicate.or [attacking, blocking]])),
        Primitives.Instruction.skipUntap thisCreature (.lit 1) ])
theorem okArbalestElite : Ability.check [] arbalestElite = [] := by decide
def sizzlingBarrage : Instruction :=
  Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 4) (target (Primitives.Predicate.and [creature, happenedTo .blockDeclaration .thisTurn]))
theorem okSizzlingBarrage : Instruction.check [] sizzlingBarrage = [] := by decide
def goadedAttackTrigger : Ability :=
  whenever (attacks (a (Primitives.Predicate.and [creature, Primitives.Predicate.hasDesignation "goaded" none])))
    (Primitives.Instruction.dealDamage it (.lit 1) (controllerOf it))
theorem okGoadedAttackTrigger : Ability.check [] goadedAttackTrigger = [] := by decide

def extraArms : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Extra Arms", cost := some [generic 4, pip .red], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          whenever (attacks (Primitives.NounPhrase.attachHost .enchanted (.type .creature)))
            (Primitives.Instruction.dealDamage it (.lit 2) (target anyTarget)) ] } }

def chainReaction : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chain Reaction", cost := some [generic 2, pip .red, pip .red], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequence
            [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (Primitives.Amount.letter .x) (each creature), Primitives.Instruction.define .x (countOf creature) ]) ] } }

/-- Black Vise -/
def blackVise : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Black Vise", cost := some [generic 1], types := [.artifact],
      text :=
        [ Primitives.Ability.static (entersChoosingPlayer thisArtifact (some (Primitives.ChoiceDomain.players Primitives.Predicate.opponent))),
          at_ (beginningOfPossessed .the .upkeep (the chosenPlayer))
            (Primitives.Instruction.sequence
              [ Primitives.Instruction.dealDamage thisArtifact (Primitives.Amount.letter .x) (that .player),
                Primitives.Instruction.define .x (Primitives.Amount.arith .minus (countOf (Primitives.Predicate.inZone (handOf they))) (.lit 4)) ]) ] } }

def harshSustenance : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Harsh Sustenance", cost := some [generic 1, pip .white, pip .black], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequence
            [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (Primitives.Amount.letter .x) (target anyTarget),
              gainLife (Primitives.Amount.letter .x) (agent := Primitives.NounPhrase.you),
              Primitives.Instruction.define .x (countOf creatureYouControl) ]) ] } }

/-- Purging Scythe -/
def purgingScythe : Ability :=
  at_ (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
    (Primitives.Instruction.sequence
      [ Primitives.Instruction.dealDamage thisArtifact (.lit 2)
          (the (Primitives.Predicate.and [creature, Primitives.Predicate.superlative .min (.stat .toughness) creature])),
        Primitives.Instruction.doIf
          (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [creature, Primitives.Predicate.superlative .min (.stat .toughness) creature]))
            .atLeast (.lit 2))
          (choose (someOf (exactly 1) them) (agent := some Primitives.NounPhrase.you)) none ])
theorem okPurgingScythe : Ability.check [] purgingScythe = [] := by decide

def lionHeart : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lion Heart", cost := some [generic 4], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ when (Primitives.GameEvent.enters thisEquipment none) (Primitives.Instruction.dealDamage it (.lit 2) (target anyTarget)),
          Primitives.Ability.static (getsPt (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 1))),
          keywordCosting "Equip" (Primitives.Cost.mana [generic 2]) ] } }

def lightmineField : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lightmine Field", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ whenever (attacks (counted (atLeast 1) creature))
            (Primitives.Instruction.dealDamage thisEnchantment (Primitives.Amount.countOf (those (.type .creature)))
              (Primitives.NounPhrase.eachOf (those (.type .creature)))) ] } }

def ingeniousArtillerist : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ingenious Artillerist", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Artificer"],
      text :=
        [ whenever (Primitives.GameEvent.enters (counted (atLeast 1) (Primitives.Predicate.and [artifact, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) none)
            (Primitives.Instruction.dealDamage thisCreature Primitives.Amount.groupSize (each Primitives.Predicate.opponent)) ],
      power := stat 3, toughness := stat 1 } }

/-- Inferno Elemental -/
def infernoElemental : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Inferno Elemental", cost := some [generic 4, pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ triggeredOr (blocks thisCreature (some (a creature)))
            [becomesBlocked thisCreature (some (a creature))]
            (Primitives.Instruction.dealDamage thisCreature (.lit 3) (that (.type .creature))) ],
      power := stat 4, toughness := stat 4 } }

/-- Psychic Purge -/
def psychicPurge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Psychic Purge", cost := some [pip .blue], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 1) (target anyTarget)),
          when
            (Primitives.GameEvent.causes
              (Primitives.Causing.source (a (Primitives.Predicate.and [Primitives.Predicate.or [spell, Primitives.Predicate.abilityHead .anyOnStack],
                                 Primitives.Predicate.hasPossessor .controller (a Primitives.Predicate.opponent)])))
              (Primitives.GameEvent.verbedEvent (some Primitives.NounPhrase.you) (.action "Discard") (some Primitives.NounPhrase.this) none))
            (loseLife (.lit 5) (agent := (that .player))) ] } }

/-- Chandra Nalaar -/
def chandraNalaarsX : Ability :=
  activated (Primitives.Cost.loyaltySymbol .downX) (Primitives.Instruction.dealDamage Primitives.NounPhrase.this (Primitives.Amount.letter .x) (target creature))
theorem okChandraNalaarsX : Ability.check [] chandraNalaarsX = [] := by decide

def etherealHaze : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ethereal Haze", cost := some [pip .white], types := [.instant],
      subtypes := [spellType "Arcane"],
      text := [Primitives.Ability.spell none (preventAllBy .any (allOf creature) Primitives.DamageScope.everywhere (some Primitives.Duration.thisTurn))] } }

def defang : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Defang", cost := some [generic 1, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (Primitives.NounPhrase.attachHost .enchanted (.type .creature))) Primitives.DamageScope.everywhere
            (Primitives.DamageOp.prevent Primitives.PreventCut.all none) .repeatedly) ] } }

def sphereOfPurity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sphere of Purity", cost := some [generic 3, pip .white], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (a artifact)) (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you)
            (Primitives.DamageOp.prevent (Primitives.PreventCut.some (.lit 1)) none) .repeatedly) ] } }

def dazzlingReflection : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dazzling Reflection", cost := some [generic 1, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequence
            [ gainLife (Primitives.Amount.statOf (.stat .power) (target creature)) (agent := Primitives.NounPhrase.you),
              Primitives.Instruction.establish
                (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (that (.type .creature))) Primitives.DamageScope.everywhere (Primitives.DamageOp.prevent Primitives.PreventCut.all none)
                  .nextTimeOnly)
                (some Primitives.Duration.thisTurn) ]) ] } }

def thunderstaff : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thunderstaff", cost := some [generic 3], types := [.artifact],
      text :=
        [ Primitives.Ability.static (onlyWhile
            (Primitives.StaticSpec.damageRule .combatOnly (Primitives.DamageAgent.dealtBy (a creature)) (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you)
              (Primitives.DamageOp.prevent (Primitives.PreventCut.some (.lit 1)) none) .repeatedly)
            (Primitives.Condition.matches thisArtifact untapped)),
          activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2], Primitives.Cost.tapSymbol])
            (get (allOf (Primitives.Predicate.and [creature, attacking])) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 0))
              (some untilEndOfTurn)) ] } }

def gideonAllyOfZendikar : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gideon, Ally of Zendikar", cost := some [generic 2, pip .white, pip .white],
      supertypes := [.legendary], types := [.planeswalker], subtypes := [planeswalkerType "Gideon"],
      text :=
        [ activated (Primitives.Cost.loyaltySymbol (.up 1))
            (Primitives.Instruction.sequence
              [ Primitives.Instruction.establish
                  (Primitives.StaticSpec.qualityChange thisPlaneswalker .sets
                    (Primitives.QualityPayload.bundle
                      { characteristics :=
                        { types := [.creature],
                          subtypes := [creatureType "Human", creatureType "Soldier", creatureType "Ally"],
                          text := [keyword "Indestructible"], power := stat 5, toughness := stat 5 } }
                      (some .planeswalker)))
                  (some untilEndOfTurn),
                preventAll .any (Primitives.DamageScope.toRecipient it) (some Primitives.Duration.thisTurn) ]),
          activated (Primitives.Cost.loyaltySymbol .zero)
            (create (.lit 1) (creatureToken 2 2 [.white] [creatureType "Knight", creatureType "Ally"])),
          activated (Primitives.Cost.loyaltySymbol (.down 4))
            (Primitives.Instruction.getEmblem
              [Primitives.Ability.static (getsPt (allOf creatureYouControl) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)))] (agent :=
                  Primitives.NounPhrase.you)) ],
      loyalty := stat 4 } }

def turnTheTables : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Turn the Tables", cost := some [generic 3, pip .white, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.establish
            (Primitives.StaticSpec.damageRule .combatOnly Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you)
              (Primitives.DamageOp.redirect Primitives.PreventCut.all (target (Primitives.Predicate.and [creature, attacking]))) .repeatedly)
            (some Primitives.Duration.thisTurn)) ] } }

def pariah : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pariah", cost := some [generic 2, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (Primitives.StaticSpec.damageRule .any Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you)
            (Primitives.DamageOp.redirect Primitives.PreventCut.all (Primitives.NounPhrase.attachHost .enchanted (.type .creature))) .repeatedly) ] } }

def martyrsOfKorlis : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Martyrs of Korlis", cost := some [generic 3, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Human"],
      text :=
        [ Primitives.Ability.static (onlyWhile
            (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (allOf artifact)) (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you)
              (Primitives.DamageOp.redirect Primitives.PreventCut.all thisCreature) .repeatedly)
            (Primitives.Condition.matches thisCreature untapped)) ],
      power := stat 1, toughness := stat 6 } }

def wardOfPiety : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ward of Piety", cost := some [generic 1, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          activated (Primitives.Cost.mana [generic 1, pip .white])
            (Primitives.Instruction.establish
              (Primitives.StaticSpec.damageRule .any Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient (Primitives.NounPhrase.attachHost .enchanted (.type .creature)))
                (Primitives.DamageOp.redirect (Primitives.PreventCut.shield (.lit 1)) (target anyTarget)) .repeatedly)
              (some Primitives.Duration.thisTurn)) ] } }

def mirrorwoodTreefolk : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mirrorwood Treefolk", cost := some [generic 3, pip .green], types := [.creature],
      subtypes := [creatureType "Treefolk"],
      text :=
        [ activated (Primitives.Cost.mana [generic 2, pip .red, pip .white])
            (Primitives.Instruction.establish
              (Primitives.StaticSpec.damageRule .any Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient thisCreature)
                (Primitives.DamageOp.redirect Primitives.PreventCut.all (target anyTarget)) .nextTimeOnly)
              (some Primitives.Duration.thisTurn)) ],
      power := stat 2, toughness := stat 4 } }

def carom : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Carom", cost := some [generic 1, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequence
            [ Primitives.Instruction.establish
                (Primitives.StaticSpec.damageRule .any Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient (target creature))
                  (Primitives.DamageOp.redirect (Primitives.PreventCut.shield (.lit 1)) (target (Primitives.Predicate.and [creature, Primitives.Predicate.other]))) .repeatedly)
                (some Primitives.Duration.thisTurn),
              Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you) ]) ] } }

def daughterOfAutumn : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Daughter of Autumn", cost := some [generic 2, pip .green, pip .green],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Avatar"],
      text :=
        [ activated (Primitives.Cost.mana [pip .white])
            (Primitives.Instruction.establish
              (Primitives.StaticSpec.damageRule .any Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient (target (Primitives.Predicate.and [creature, Primitives.Predicate.colorIs .white])))
                (Primitives.DamageOp.redirect (Primitives.PreventCut.shield (.lit 1)) thisCreature) .repeatedly)
              (some Primitives.Duration.thisTurn)) ],
      power := stat 2, toughness := stat 4 } }

def aegisOfHonor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aegis of Honor", cost := some [pip .white], types := [.enchantment],
      text :=
        [ activated (Primitives.Cost.mana [generic 1])
            (Primitives.Instruction.establish
              (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (a instantOrSorcery)) (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you)
                (Primitives.DamageOp.redirect Primitives.PreventCut.all (controllerOf it)) .nextTimeOnly)
              (some Primitives.Duration.thisTurn)) ] } }

def candlesGlow : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Candles' Glow", cost := some [generic 1, pip .white], types := [.instant],
      subtypes := [spellType "Arcane"],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.establish
            (Primitives.StaticSpec.damageRule .any Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient (target anyTarget))
              (Primitives.DamageOp.prevent (Primitives.PreventCut.shield (.lit 3)) (some (gainLife preventedThisWay (agent := Primitives.NounPhrase.you))))
                  .repeatedly)
            (some Primitives.Duration.thisTurn)) ] } }

/-- Inkshield -/
def inkshieldRider : Instruction :=
  Primitives.Instruction.establish
    (Primitives.StaticSpec.damageRule .any Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you)
      (Primitives.DamageOp.prevent Primitives.PreventCut.all (some (create preventedThisWay (creatureToken 2 1 [.white, .black] []))))
      .repeatedly)
    (some Primitives.Duration.thisTurn)
theorem okInkshieldRider : Instruction.check [] inkshieldRider = [] := by decide

def urzasArmor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Urza's Armor", cost := some [generic 6], types := [.artifact],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (a source)) (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you)
            (Primitives.DamageOp.prevent (Primitives.PreventCut.some (.lit 1)) none) .repeatedly) ] } }

def circleOfProtectionRed : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Circle of Protection: Red", cost := some [generic 1, pip .white], types := [.enchantment],
      text :=
        [ activated (Primitives.Cost.mana [generic 1])
            (Primitives.Instruction.establish
              (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (aYourChoice (Primitives.Predicate.and [source, Primitives.Predicate.colorIs .red]))) (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you)
                (Primitives.DamageOp.prevent Primitives.PreventCut.all none) .nextTimeOnly)
              (some Primitives.Duration.thisTurn)) ] } }

def healingGrace : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Healing Grace", cost := some [pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequence
            [ Primitives.Instruction.establish
                (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (aYourChoice source)) (Primitives.DamageScope.toRecipient (target anyTarget))
                  (Primitives.DamageOp.prevent (Primitives.PreventCut.shield (.lit 3)) none) .repeatedly)
                (some Primitives.Duration.thisTurn),
              gainLife (.lit 3) (agent := Primitives.NounPhrase.you) ]) ] } }

def reverseDamage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Reverse Damage", cost := some [generic 1, pip .white, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.establish
            (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (aYourChoice source)) (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you)
              (Primitives.DamageOp.prevent Primitives.PreventCut.all (some (gainLife preventedThisWay (agent := Primitives.NounPhrase.you)))) .nextTimeOnly)
            (some Primitives.Duration.thisTurn)) ] } }

def deflectingPalm : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Deflecting Palm", cost := some [pip .red, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.establish
            (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (aYourChoice source)) (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you)
              (Primitives.DamageOp.prevent Primitives.PreventCut.all (some (Primitives.Instruction.dealDamage Primitives.NounPhrase.this Primitives.Amount.thatMuch (controllerOf it)))) .nextTimeOnly)
            (some Primitives.Duration.thisTurn)) ] } }

def templeAltisaur : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Temple Altisaur", cost := some [generic 4, pip .white], types := [.creature],
      subtypes := [creatureType "Dinosaur"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (a source))
            (Primitives.DamageScope.toRecipient (a (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Dinosaur"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
            (Primitives.DamageOp.prevent (Primitives.PreventCut.allBut (.lit 1)) none) .repeatedly) ],
      power := stat 3, toughness := stat 4 } }

def darkSphere : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dark Sphere", cost := some [], types := [.artifact],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.tapSymbol, Primitives.Cost.perform (sacrifice thisArtifact (agent := Primitives.NounPhrase.you))])
            (Primitives.Instruction.establish
              (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (aYourChoice source)) (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you)
                (Primitives.DamageOp.prevent (Primitives.PreventCut.half .down) none) .nextTimeOnly)
              (some Primitives.Duration.thisTurn)) ] } }

def shadowbane : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shadowbane", cost := some [generic 1, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.establish
            (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (aYourChoice source))
              (Primitives.DamageScope.toRecipient (youAnd (allOf creatureYouControl)))
              (Primitives.DamageOp.prevent Primitives.PreventCut.all
                (some (Primitives.Instruction.doIf (Primitives.Condition.preventedFromSource (Primitives.Predicate.and [source, Primitives.Predicate.colorIs .black]))
                  (gainLife preventedThisWay (agent := Primitives.NounPhrase.you)) none)))
              .nextTimeOnly)
            (some Primitives.Duration.thisTurn)) ] } }

def sphereOfLaw : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sphere of Law", cost := some [generic 3, pip .white], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (a (Primitives.Predicate.and [source, Primitives.Predicate.colorIs .red]))) (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you)
            (Primitives.DamageOp.prevent (Primitives.PreventCut.some (.lit 2)) none) .repeatedly) ] } }

def lavaAxe : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lava Axe", cost := some [generic 4, pip .red], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 5) (target (Primitives.Predicate.or [Primitives.Predicate.hasType .planeswalker, Primitives.Predicate.anyPlayer]))) ] } }

def searingFlesh : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Searing Flesh", cost := some [generic 6, pip .red], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 7) (target (Primitives.Predicate.or [Primitives.Predicate.hasType .planeswalker, Primitives.Predicate.opponent]))) ] } }

def onakkeJavelineerBolt : Ability :=
  activated Primitives.Cost.tapSymbol
    (Primitives.Instruction.dealDamage thisCreature (.lit 2) (target (Primitives.Predicate.or [Primitives.Predicate.hasType .battle, Primitives.Predicate.anyPlayer])))
theorem okOnakkeJavelineerBolt : Ability.check [] onakkeJavelineerBolt = [] := by decide

/-- Firesong and Sunspeaker -/
def firesongJoinEcho : Instruction :=
  Primitives.Instruction.sequence
    [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 3) (target (Primitives.Predicate.or [creature, Primitives.Predicate.anyPlayer])),
      Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 1) thatJoin ]
theorem okFiresongJoinEcho : Instruction.check [] firesongJoinEcho = [] := by decide

/-- Forcefield -/
def forcefield : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Forcefield", cost := some [generic 3], types := [.artifact],
      text :=
        [ activated (Primitives.Cost.mana [generic 1])
            (Primitives.Instruction.establish
              (Primitives.StaticSpec.damageRule .combatOnly (Primitives.DamageAgent.dealtBy (aYourChoice (Primitives.Predicate.and [creature, unblocked])))
                (Primitives.DamageScope.toRecipient Primitives.NounPhrase.you) (Primitives.DamageOp.prevent (Primitives.PreventCut.allBut (.lit 1)) none) .nextTimeOnly)
              (some Primitives.Duration.thisTurn)) ] } }

def endure : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Endure", cost := some [generic 3, pip .white, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none
            (preventAll .any
              (Primitives.DamageScope.toRecipient (youAnd (allOf (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))))
              (some Primitives.Duration.thisTurn)) ] } }

def harmsWay : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Harm's Way", cost := some [pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.establish
            (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (aYourChoice source))
              (Primitives.DamageScope.toRecipient (youAnd (allOf (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))))
              (Primitives.DamageOp.redirect (Primitives.PreventCut.shield (.lit 2)) (target anyTarget)) .repeatedly)
            (some Primitives.Duration.thisTurn)) ] } }

def divineDeflection : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Divine Deflection", cost := some [.variable, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.establish
            (Primitives.StaticSpec.damageRule .any Primitives.DamageAgent.unattributed
              (Primitives.DamageScope.toRecipient (youAnd (allOf (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))))
              (Primitives.DamageOp.prevent (Primitives.PreventCut.shield (Primitives.Amount.letter .x)) (some (Primitives.Instruction.dealDamage Primitives.NounPhrase.this Primitives.Amount.thatMuch (target anyTarget))))
              .repeatedly)
            (some Primitives.Duration.thisTurn)) ] } }

def glarecasterShield : Ability :=
  activated (Primitives.Cost.mana [generic 5, pip .white])
    (Primitives.Instruction.establish
      (Primitives.StaticSpec.damageRule .any Primitives.DamageAgent.unattributed (Primitives.DamageScope.toRecipient (youAnd thisCreature))
        (Primitives.DamageOp.redirect Primitives.PreventCut.all (target anyTarget)) .nextTimeOnly)
      (some Primitives.Duration.thisTurn))
theorem okGlarecasterShield : Ability.check [] glarecasterShield = [] := by decide

def furnaceOfRath : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Furnace of Rath", cost := some [generic 1, pip .red, pip .red, pip .red],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (a source))
            (Primitives.DamageScope.toRecipient (a (Primitives.Predicate.or [permanent, Primitives.Predicate.anyPlayer]))) (Primitives.DamageOp.scale (Primitives.DamageScale.multiplied .doubled))
            .repeatedly) ] } }

def gratuitousViolence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gratuitous Violence", cost := some [generic 2, pip .red, pip .red, pip .red],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (a creatureYouControl))
            (Primitives.DamageScope.toRecipient (a (Primitives.Predicate.or [permanent, Primitives.Predicate.anyPlayer]))) (Primitives.DamageOp.scale (Primitives.DamageScale.multiplied .doubled))
            .repeatedly) ] } }

def fieryEmancipation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fiery Emancipation", cost := some [generic 3, pip .red, pip .red, pip .red],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (a (Primitives.Predicate.and [source, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
            (Primitives.DamageScope.toRecipient (a (Primitives.Predicate.or [permanent, Primitives.Predicate.anyPlayer]))) (Primitives.DamageOp.scale (Primitives.DamageScale.multiplied .tripled))
            .repeatedly) ] } }

def sulfuricVapors : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sulfuric Vapors", cost := some [generic 3, pip .red], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (a (Primitives.Predicate.and [spell, Primitives.Predicate.colorIs .red])))
            (Primitives.DamageScope.toRecipient (a (Primitives.Predicate.or [permanent, Primitives.Predicate.anyPlayer]))) (Primitives.DamageOp.scale (Primitives.DamageScale.shifted .up (.lit 1)))
            .repeatedly) ] } }

def lashknifeBarrier : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lashknife Barrier", cost := some [generic 2, pip .white], types := [.enchantment],
      text :=
        [ when (Primitives.GameEvent.enters thisEnchantment none) (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)),
          Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (a source)) (Primitives.DamageScope.toRecipient (a creatureYouControl))
            (Primitives.DamageOp.scale (Primitives.DamageScale.shifted .down (.lit 1))) .repeatedly) ] } }

def ghostsOfTheInnocent : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ghosts of the Innocent", cost := some [generic 5, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Spirit"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (a source))
            (Primitives.DamageScope.toRecipient (a (Primitives.Predicate.or [permanent, Primitives.Predicate.anyPlayer]))) (Primitives.DamageOp.scale (Primitives.DamageScale.halved .down)) .repeatedly) ],
      power := stat 4, toughness := stat 5 } }

def fireServant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fire Servant", cost := some [generic 3, pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.damageRule .any
            (Primitives.DamageAgent.dealtBy (a (Primitives.Predicate.and [instantOrSorcery, Primitives.Predicate.colorIs .red, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
            Primitives.DamageScope.everywhere (Primitives.DamageOp.scale (Primitives.DamageScale.multiplied .doubled)) .repeatedly) ],
      power := stat 4, toughness := stat 3 } }

def platedPegasus : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Plated Pegasus", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [creatureType "Pegasus"],
      text :=
        [ keyword "Flash",
          keyword "Flying",
          Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (a spell)) (Primitives.DamageScope.toRecipient (a (Primitives.Predicate.or [permanent, Primitives.Predicate.anyPlayer])))
            (Primitives.DamageOp.prevent (Primitives.PreventCut.some (.lit 1)) none) .repeatedly) ],
      power := stat 1, toughness := stat 2 } }

def spitemare : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Spitemare", cost := some [generic 2, hybridPip .red .white, hybridPip .red .white],
      types := [.creature], subtypes := [creatureType "Elemental"],
      text :=
        [ whenever (Primitives.GameEvent.isDealtDamage .any thisCreature) (Primitives.Instruction.dealDamage it Primitives.Amount.thatMuch (target anyTarget)) ],
      power := stat 3, toughness := stat 3 } }

def grollub : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grollub", cost := some [generic 2, pip .black], types := [.creature],
      subtypes := [creatureType "Beast"],
      text := [whenever (Primitives.GameEvent.isDealtDamage .any thisCreature) (gainLife Primitives.Amount.thatMuch (agent := (each
          Primitives.Predicate.opponent)))],
      power := stat 3, toughness := stat 3 } }

def moggManiac : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mogg Maniac", cost := some [generic 1, pip .red], types := [.creature],
      subtypes := [creatureType "Goblin"],
      text :=
        [ whenever (Primitives.GameEvent.isDealtDamage .any thisCreature)
            (Primitives.Instruction.dealDamage it Primitives.Amount.thatMuch (target (Primitives.Predicate.or [Primitives.Predicate.hasType .planeswalker, Primitives.Predicate.opponent]))) ],
      power := stat 1, toughness := stat 1 } }

def repercussion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Repercussion", cost := some [generic 1, pip .red, pip .red], types := [.enchantment],
      text :=
        [ whenever (Primitives.GameEvent.isDealtDamage .any (a creature))
            (Primitives.Instruction.dealDamage thisEnchantment Primitives.Amount.thatMuch (controllerOf (that (.type .creature)))) ] } }

def spitefulShadows : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Spiteful Shadows", cost := some [generic 1, pip .black], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          whenever (Primitives.GameEvent.isDealtDamage .any (Primitives.NounPhrase.attachHost .enchanted (.type .creature)))
            (Primitives.Instruction.dealDamage it Primitives.Amount.thatMuch (controllerOf it)) ] } }

def bindingAgony : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Binding Agony", cost := some [generic 1, pip .black], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          whenever (Primitives.GameEvent.isDealtDamage .any (Primitives.NounPhrase.attachHost .enchanted (.type .creature)))
            (Primitives.Instruction.dealDamage thisAura Primitives.Amount.thatMuch (controllerOf (that (.type .creature)))) ] } }

def darienKingOfKjeldor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Darien, King of Kjeldor", cost := some [generic 4, pip .white, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text :=
        [ whenever (Primitives.GameEvent.isDealtDamage .any Primitives.NounPhrase.you)
            (offer (create Primitives.Amount.thatMuch (creatureToken 1 1 [.white] [creatureType "Soldier"])) (agent
                := Primitives.NounPhrase.you)) ],
      power := stat 3, toughness := stat 3 } }

def screamingNemesis : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Screaming Nemesis", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ keyword "Haste",
          whenever (Primitives.GameEvent.isDealtDamage .any thisCreature)
            (Primitives.Instruction.sequence
              [ Primitives.Instruction.dealDamage it Primitives.Amount.thatMuch (target (Primitives.Predicate.and [anyTarget, Primitives.Predicate.otherThan Primitives.NounPhrase.this])),
                Primitives.Instruction.doIf (Primitives.Condition.dealtThisWay Primitives.Predicate.anyPlayer)
                  (Primitives.Instruction.establish (playerCant (.core .gainLife) they) (some Primitives.Duration.restOfGame)) none ]) ],
      power := stat 3, toughness := stat 3 } }

def sonicShrieker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sonic Shrieker", cost := some [generic 2, pip .red, pip .white, pip .black],
      types := [.creature], subtypes := [creatureType "Dragon"],
      text :=
        [ keyword "Flying",
          when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Instruction.sequence
              [ Primitives.Instruction.dealDamage it (.lit 2) (target anyTarget),
                gainLife (.lit 2) (agent := Primitives.NounPhrase.you),
                Primitives.Instruction.doIf (Primitives.Condition.dealtThisWay Primitives.Predicate.anyPlayer) (discard (a (Primitives.Predicate.inZone hand)) (agent := they)) none
                    ]) ],
      power := stat 4, toughness := stat 4 } }

def grievousWoundLifeLock : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grievous Wound", cost := some [generic 3, pip .black, pip .black], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" Primitives.Predicate.anyPlayer,
          Primitives.Ability.static (playerCant (.core .gainLife) (Primitives.NounPhrase.attachHost .enchanted .player)),
          whenever (Primitives.GameEvent.isDealtDamage .any (Primitives.NounPhrase.attachHost .enchanted .player))
            (loseLife (Primitives.Amount.half .up (lifeTotalOf they)) (agent := they)) ] } }

def cursedScroll : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cursed Scroll", cost := some [generic 1], types := [.artifact],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 3], Primitives.Cost.tapSymbol])
            (Primitives.Instruction.sequence
              [ choose (a (quality .cardName)),
                revealCards (aAtRandom (Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you))),
                Primitives.Instruction.doIf (Primitives.Condition.matches (that .card) (Primitives.Predicate.named Primitives.NameSource.chosen))
                  (Primitives.Instruction.dealDamage thisArtifact (.lit 2) (target anyTarget)) none ]) ] } }

def magusOfTheScroll : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Magus of the Scroll", cost := some [pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 3], Primitives.Cost.tapSymbol])
            (Primitives.Instruction.sequence
              [ choose (a (quality .cardName)),
                revealCards (aAtRandom (Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you))),
                Primitives.Instruction.doIf (Primitives.Condition.matches (that .card) (Primitives.Predicate.named Primitives.NameSource.chosen))
                  (Primitives.Instruction.dealDamage thisCreature (.lit 2) (target anyTarget)) none ]) ],
      power := stat 1, toughness := stat 1 } }

/-- Stuffy Doll -/
def stuffyDoll : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stuffy Doll", cost := some [generic 5], types := [.artifact, .creature],
      subtypes := [creatureType "Construct"],
      text :=
        [ keyword "Indestructible",
          Primitives.Ability.static (entersChoosingPlayer thisCreature none),
          whenever (Primitives.GameEvent.isDealtDamage .any thisCreature) (Primitives.Instruction.dealDamage it Primitives.Amount.thatMuch (the chosenPlayer)),
          activated Primitives.Cost.tapSymbol (Primitives.Instruction.dealDamage thisCreature (.lit 1) thisCreature) ],
      power := stat 0, toughness := stat 1 } }

def saheeliRaiPlusOne : Instruction :=
  Primitives.Instruction.sequence [scry (.lit 1) (agent := Primitives.NounPhrase.you), Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 1) (each Primitives.Predicate.opponent)]
theorem okSaheeliRaiPlusOne : Instruction.check [] saheeliRaiPlusOne = [] := by decide
def sarkhansUnsealingLine : Ability :=
  whenever
    (Primitives.GameEvent.casts Primitives.NounPhrase.you
      (a (Primitives.Predicate.and [creature, spell,
                Primitives.Predicate.or [ Primitives.Predicate.compare [.stat .power] .eq (.lit 4), Primitives.Predicate.compare [.stat .power] .eq (.lit 5),
                      Primitives.Predicate.compare [.stat .power] .eq (.lit 6) ]]))
      none)
    (Primitives.Instruction.dealDamage thisEnchantment (.lit 4) (target anyTarget))
theorem okSarkhansUnsealingLine : Ability.check [] sarkhansUnsealingLine = [] := by decide
/-- Savage Swipe, both sentences -/
def savageSwipeLine : Instruction :=
  Primitives.Instruction.sequence
    [ Primitives.Instruction.doOnlyIf (get (target creatureYouControl) (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2)) (some
        untilEndOfTurn))
        (Primitives.Condition.compareAmt (Primitives.Amount.statOf (.stat .power) it) .eq (.lit 2)) none,
      Primitives.Instruction.fight it (target creatureYouDontControl) ]
theorem okSavageSwipeLine : Instruction.check [] savageSwipeLine = [] := by decide

def infernoOfTheStarMounts : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Inferno of the Star Mounts", cost := some [generic 4, pip .red, pip .red],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Dragon"],
      text :=
        [ Primitives.Ability.static (objectCant (.action "Counter") Primitives.NounPhrase.this),
          keyword "Flying",
          keyword "Haste",
          activated (Primitives.Cost.mana [pip .red])
            (Primitives.Instruction.triggerThisWay (get thisCreature (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn))
              (Primitives.GameEvent.statBecomes it .power (.lit 20))
              (Primitives.Instruction.dealDamage it (.lit 20) (target anyTarget))) ],
      power := stat 6, toughness := stat 6 } }

def incinerate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Incinerate", cost := some [generic 1, pip .red], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequence
            [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 3) (target anyTarget),
              Primitives.Instruction.establish
                (objectCant (.action "Regenerate")
                  (a (Primitives.Predicate.and [creature, Primitives.Predicate.happenedTo (Primitives.LookbackClause.mk .damageTaken .thisWay none)])))
                (some Primitives.Duration.thisTurn) ]) ] } }

def ashZealot : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ash Zealot", cost := some [pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Warrior"],
      text :=
        [ keyword "FirstStrike",
          keyword "Haste",
          whenever (Primitives.GameEvent.casts (a Primitives.Predicate.anyPlayer) (a (Primitives.Predicate.and [spell, Primitives.Predicate.castFrom graveyard])) none)
            (Primitives.Instruction.dealDamage thisCreature (.lit 3) (that .player)) ],
      power := stat 2, toughness := stat 2 } }

def galvanicBlastLine : Instruction :=
  Primitives.Instruction.replace (Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 2) (target anyTarget))
    (Primitives.Instruction.doOnlyIf (Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 4) thatJoin)
      (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [artifact, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .atLeast (.lit 3)) none)
theorem okGalvanicBlastLine : Instruction.check [] galvanicBlastLine = [] := by decide
/-- Furious Reprisal -/
def furiousReprisal : Instruction :=
  Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 2) (Primitives.NounPhrase.eachOf (Primitives.NounPhrase.described (Primitives.DetPhrase.target (exactly 2)) anyTarget))
theorem okFuriousReprisal : Instruction.check [] furiousReprisal = [] := by decide
/-- Bullseye, Death Dealer -/
def bullseyeModalCost : Ability :=
  activated
    (Primitives.Cost.compound [Primitives.Cost.mana [generic 3], Primitives.Cost.tapSymbol,
      Primitives.Cost.perform (chooseModes (exactly 1)
        [ sacrifice (a artifact) (agent := Primitives.NounPhrase.you), discard (a (Primitives.Predicate.and [Primitives.Predicate.not land, Primitives.Predicate.inZone hand]))
            (agent := Primitives.NounPhrase.you) ])])
    (Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 2) (target anyTarget))
theorem okBullseyeModalCost : Ability.check [] bullseyeModalCost = [] := by decide

/-- Twinshot Sniper -/
def twinshotSniper : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Twinshot Sniper", cost := some [generic 3, pip .red], types := [.artifact, .creature],
      subtypes := [creatureType "Goblin", creatureType "Archer"],
      text :=
        [ keyword "Reach",
          when (Primitives.GameEvent.enters thisCreature none) (Primitives.Instruction.dealDamage it (.lit 2) (target anyTarget)),
          abilityWord "channel"
            (activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 1, pip .red], Primitives.Cost.perform (discard Primitives.NounPhrase.this (agent :=
                Primitives.NounPhrase.you))])
              (Primitives.Instruction.dealDamage it (.lit 2) (target anyTarget))) ],
      power := stat 2, toughness := stat 3 } }

/-- Quakebringer -/
def quakebringerDamage : Ability :=
  triggeredIf (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
    (Primitives.Condition.or [ Primitives.Condition.matches Primitives.NounPhrase.this (Primitives.Predicate.inZone battlefield),
           Primitives.Condition.and [ Primitives.Condition.matches Primitives.NounPhrase.this (Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)),
                  exists_ (Primitives.Predicate.and [creature, Primitives.Predicate.hasSubtype (creatureType "Giant"),
                                 Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]) ] ])
    (Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 2) (each Primitives.Predicate.opponent))
theorem okQuakebringerDamage : Ability.check [] quakebringerDamage = [] := by decide
/-- Sand Strangler -/
def sandStranglerDamage : Ability :=
  triggeredIf (Primitives.GameEvent.enters thisCreature none)
    (Primitives.Condition.or [ exists_ (Primitives.Predicate.and [land, Primitives.Predicate.hasSubtype (landType "Desert"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]),
           exists_ (Primitives.Predicate.and [land, Primitives.Predicate.hasSubtype (landType "Desert"), Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)]) ])
    (offer (Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 3) (target creature)) (agent := Primitives.NounPhrase.you))
theorem okSandStranglerDamage : Ability.check [] sandStranglerDamage = [] := by decide

def brazenDwarf : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Brazen Dwarf", cost := some [generic 1, pip .red], types := [.creature],
      subtypes := [creatureType "Dwarf", creatureType "Shaman"],
      text :=
        [ whenever (Primitives.GameEvent.rollsDice Primitives.NounPhrase.you .many none Primitives.RollWatch.anyResult)
            (Primitives.Instruction.dealDamage thisCreature (.lit 1) (each Primitives.Predicate.opponent)) ],
      power := stat 1, toughness := stat 3 } }

/-- Lavalanche -/
def lavalanche : Instruction :=
  Primitives.Instruction.performSimultaneously
    [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (Primitives.Amount.letter .x) targetPlayerOrPlaneswalker,
      Primitives.Instruction.dealDamage Primitives.NounPhrase.this (Primitives.Amount.letter .x) eachCreatureThatSplitControls ]
theorem okLavalanche : Instruction.check [] lavalanche = [] := by decide
/-- Flame Wave -/
def flameWave : Instruction :=
  Primitives.Instruction.performSimultaneously
    [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 4) targetPlayerOrPlaneswalker,
      Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 4) eachCreatureThatSplitControls ]
theorem okFlameWave : Instruction.check [] flameWave = [] := by decide
/-- Chandra Nalaar's ultimate -/
def chandraNalaarUltimate : Instruction :=
  Primitives.Instruction.performSimultaneously
    [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 10) targetPlayerOrPlaneswalker,
      Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 10) eachCreatureThatSplitControls ]
theorem okChandraNalaarUltimate : Instruction.check [] chandraNalaarUltimate = [] := by decide
/-- Chandra, Pyrogenius's ultimate -/
def chandraPyrogeniusUltimate : Instruction :=
  Primitives.Instruction.performSimultaneously
    [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 6) targetPlayerOrPlaneswalker,
      Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 6) eachCreatureThatSplitControls ]
theorem okChandraPyrogeniusUltimate : Instruction.check [] chandraPyrogeniusUltimate = [] := by decide
/-- Bonfire of the Damned -/
def bonfireOfTheDamned : Instruction :=
  Primitives.Instruction.performSimultaneously
    [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (Primitives.Amount.letter .x) targetPlayerOrPlaneswalker,
      Primitives.Instruction.dealDamage Primitives.NounPhrase.this (Primitives.Amount.letter .x) eachCreatureThatSplitControls ]
theorem okBonfireOfTheDamned : Instruction.check [] bonfireOfTheDamned = [] := by decide
/-- Chandra's Fury -/
def chandrasFury : Instruction :=
  Primitives.Instruction.performSimultaneously
    [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 4) targetPlayerOrPlaneswalker,
      Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 1) eachCreatureThatSplitControls ]
theorem okChandrasFury : Instruction.check [] chandrasFury = [] := by decide
/-- Angrath, Minotaur Pirate's plus -/
def angrathMinotaurPirateBolt : Instruction :=
  Primitives.Instruction.performSimultaneously
    [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 1) targetOpponentOrPlaneswalker,
      Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 1) eachCreatureThatSplitControls ]
theorem okAngrathMinotaurPirateBolt : Instruction.check [] angrathMinotaurPirateBolt = [] := by decide
def whichOfYouBurnsBrightestBody : Instruction :=
  Primitives.Instruction.performSimultaneously
    [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (Primitives.Amount.letter .x) targetOpponentOrPlaneswalker,
      Primitives.Instruction.dealDamage Primitives.NounPhrase.this (Primitives.Amount.letter .x) eachCreatureThatSplitControls ]
theorem okWhichOfYouBurnsBrightestBody :
    Instruction.check [] whichOfYouBurnsBrightestBody = [] := by decide
/-- Chandra, Pyromaster's plus -/
def chandraPyromasterBolt : Instruction :=
  Primitives.Instruction.performSimultaneously
    [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 1) targetPlayerOrPlaneswalker,
      Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 1)
        (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1))
          (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller splitOverPlaneswalker])) ]
theorem okChandraPyromasterBolt : Instruction.check [] chandraPyromasterBolt = [] := by decide
/-- Ravager of the Fells -/
def ravagerOfTheFellsBolt : Instruction :=
  Primitives.Instruction.performSimultaneously
    [ Primitives.Instruction.dealDamage thisCreature (.lit 2) targetOpponentOrPlaneswalker,
      Primitives.Instruction.dealDamage thisCreature (.lit 2)
        (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1))
          (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller splitOverPlaneswalker])) ]
theorem okRavagerOfTheFellsBolt : Instruction.check [] ravagerOfTheFellsBolt = [] := by decide
/-- Soul of Shandalar's battlefield activation -/
def soulOfShandalarBolt : Instruction :=
  Primitives.Instruction.performSimultaneously
    [ Primitives.Instruction.dealDamage thisCreature (.lit 3) targetPlayerOrPlaneswalker,
      Primitives.Instruction.dealDamage thisCreature (.lit 3)
        (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1))
          (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller splitOverPlaneswalker])) ]
theorem okSoulOfShandalarBolt : Instruction.check [] soulOfShandalarBolt = [] := by decide
/-- Soul of Shandalar's graveyard activation -/
def soulOfShandalarGraveyardBolt : Instruction :=
  Primitives.Instruction.performSimultaneously
    [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 3) targetPlayerOrPlaneswalker,
      Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 3)
        (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1))
          (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller splitOverPlaneswalker])) ]
theorem okSoulOfShandalarGraveyardBolt :
    Instruction.check [] soulOfShandalarGraveyardBolt = [] := by decide
/-- Blightning -/
def blightning : Instruction :=
  Primitives.Instruction.sequence
    [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 3) targetPlayerOrPlaneswalker,
      discard (counted (exactly 2) (Primitives.Predicate.inZone hand)) (agent := splitOverPlaneswalker) ]
theorem okBlightning : Instruction.check [] blightning = [] := by decide
/-- Rakdos's Return -/
def rakdossReturn : Instruction :=
  Primitives.Instruction.sequence
    [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (Primitives.Amount.letter .x) targetOpponentOrPlaneswalker,
      discard (counted (Primitives.Quantity.exactlyOf (Primitives.Amount.letter .x)) (Primitives.Predicate.inZone hand)) (agent := splitOverPlaneswalker) ]
theorem okRakdossReturn : Instruction.check [] rakdossReturn = [] := by decide
/-- Nicol Bolas, Planeswalker's ultimate -/
def nicolBolasUltimate : Instruction :=
  Primitives.Instruction.sequence
    [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 7) targetPlayerOrPlaneswalker,
      discard (counted (exactly 7) (Primitives.Predicate.inZone hand)) (agent := splitOverPlaneswalker),
      sacrifice (counted (exactly 7) permanent) (agent := splitOverPlaneswalker) ]
theorem okNicolBolasUltimate : Instruction.check [] nicolBolasUltimate = [] := by decide
/-- Pulse of the Forge -/
def pulseOfTheForge : Instruction :=
  Primitives.Instruction.sequence
    [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 4) targetPlayerOrPlaneswalker,
      Primitives.Instruction.doIf (Primitives.Condition.compareAmt (lifeTotalOf splitOverPlaneswalker) .greater (lifeTotalOf Primitives.NounPhrase.you))
        (move Primitives.NounPhrase.this hand) none ]
theorem okPulseOfTheForge : Instruction.check [] pulseOfTheForge = [] := by decide
/-- Goblin Lyre's losing arm -/
def goblinLyreLoseFlip : Instruction :=
  Primitives.Instruction.sequence
    [ Primitives.Instruction.dealDamage thisArtifact (countOf creatureYouControl) targetOpponentOrPlaneswalker,
      Primitives.Instruction.dealDamage thisArtifact
        (countOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller splitOverPlaneswalker])) Primitives.NounPhrase.you ]
theorem okGoblinLyreLoseFlip : Instruction.check [] goblinLyreLoseFlip = [] := by decide
/-- Quenchable Fire -/
def quenchableFire : Instruction :=
  Primitives.Instruction.sequence
    [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 3) targetPlayerOrPlaneswalker,
      delay (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
        (doUnless (Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 3) thatJoin) (Primitives.Cost.mana [pip .blue]) (agent :=
            splitOverPlaneswalker)) ]
theorem okQuenchableFire : Instruction.check [] quenchableFire = [] := by decide
/-- Searing Blaze, both sentences -/
def searingBlaze : Ability :=
  abilityWord "landfall"
    (Primitives.Ability.spell none
      (Primitives.Instruction.replace
        (Primitives.Instruction.performSimultaneously
          [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 1) targetPlayerOrPlaneswalker,
            Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 1)
              (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller splitOverPlaneswalker])) ])
        (Primitives.Instruction.performSimultaneously
          [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 3) thatJoin,
            Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 3) (that (.type .creature)) ])))
theorem okSearingBlaze : Ability.check [] searingBlaze = [] := by decide

/-- Heart of Bogardan -/
def heartOfBogardan : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Heart of Bogardan", cost := some [generic 2, pip .red, pip .red], types := [.enchantment],
      text :=
        [ cumulativeUpkeep (Primitives.Cost.mana [generic 2]),
          when heartOfBogardanHeader
            (Primitives.Instruction.sequence
              [ Primitives.Instruction.performSimultaneously
                  [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (Primitives.Amount.letter .x) targetPlayerOrPlaneswalker,
                    Primitives.Instruction.dealDamage Primitives.NounPhrase.this (Primitives.Amount.letter .x) eachCreatureThatSplitControls ],
                Primitives.Instruction.define .x
                  (Primitives.Amount.arith .minus (times (.lit 2) (countersOn (.named "Age") thisEnchantment)) (.lit 2)) ]) ] } }

/-- Burn at the Stake -/
def burnAtTheStake : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Burn at the Stake", cost := some [generic 2, pip .red, pip .red, pip .red],
      types := [.sorcery],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.addedCost
            (Primitives.Cost.perform (tap (counted anyNumber (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, untapped]))))
            false),
          Primitives.Ability.spell none (Primitives.Instruction.dealDamage Primitives.NounPhrase.this (times (.lit 3) Primitives.Amount.groupSize) (target anyTarget)) ] } }

/-- Explosive Singularity -/
def explosiveSingularity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Explosive Singularity", cost := some [generic 8, pip .red, pip .red], types := [.sorcery],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.addedCost
            (Primitives.Cost.perform (tap (counted anyNumber (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, untapped]))))
            true),
          Primitives.Ability.static (Primitives.StaticSpec.costShift Primitives.NounPhrase.this (Primitives.CostShift.less (times (.lit 1) Primitives.Amount.groupSize) none)),
          Primitives.Ability.spell none (Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 10) (target anyTarget)) ] } }

/-- Tyrant of Valakut -/
def tyrantOfValakut : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tyrant of Valakut", cost := some [generic 5, pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Dragon"],
      text :=
        [ keywordCosting "Surge" (Primitives.Cost.mana [generic 3, pip .red, pip .red]),
          keyword "Flying",
          triggeredIf (Primitives.GameEvent.enters thisCreature none) (costWasPaid (.byKeyword "Surge") none thisCreature)
            (Primitives.Instruction.dealDamage thisCreature (.lit 3) (target anyTarget)) ],
      power := stat 5, toughness := stat 4 } }

/-- Fall of the Titans -/
def fallOfTheTitansCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fall of the Titans", cost := some [.variable, .variable, pip .red], types := [.instant],
      text :=
        [ keywordCosting "Surge" (Primitives.Cost.mana [.variable, pip .red]),
          Primitives.Ability.spell none
            (Primitives.Instruction.dealDamage Primitives.NounPhrase.this (Primitives.Amount.letter .x) (Primitives.NounPhrase.eachOf (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 2)) anyTarget))) ] } }

/-- Tribal Flames -/
def tribalFlames : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tribal Flames", cost := some [generic 1, pip .red], types := [.sorcery],
      text :=
        [ abilityWord "domain"
            (Primitives.Ability.spell none (Primitives.Instruction.sequence
              [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (Primitives.Amount.letter .x) (target anyTarget),
                Primitives.Instruction.define .x (Primitives.Amount.distinctCount (.subtype .land .basicOnly)
                  (allOf (Primitives.Predicate.and [land, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))) ])) ] } }

/-- Explosive Prodigy -/
def explosiveProdigyTrigger : Ability :=
  abilityWord "vivid"
    (when (Primitives.GameEvent.enters thisCreature none)
      (Primitives.Instruction.sequence
        [ Primitives.Instruction.dealDamage it (Primitives.Amount.letter .x) (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller anOpponent])),
          Primitives.Instruction.define .x (Primitives.Amount.distinctCount .color (allOf (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))) ]))
theorem okExplosiveProdigyTrigger : Ability.check [] explosiveProdigyTrigger = [] := by decide
/-- Niv-Mizzet, Guildpact -/
def nivMizzetGuildpactTrigger : Ability :=
  whenever (dealsCombatDamage thisCreature (a Primitives.Predicate.anyPlayer))
    (Primitives.Instruction.sequence
      [ Primitives.Instruction.dealDamage thisCreature (Primitives.Amount.letter .x) (target anyTarget),
        Primitives.Instruction.draw (Primitives.Amount.letter .x) (agent := (target Primitives.Predicate.anyPlayer)),
        gainLife (Primitives.Amount.letter .x) (agent := Primitives.NounPhrase.you),
        Primitives.Instruction.define .x (Primitives.Amount.distinctCount .colorPair
          (allOf (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.colorCount .eq 2]))) ])
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
          whenever (Primitives.GameEvent.attacksWith (a Primitives.Predicate.anyPlayer) none (counted (atLeast 3) creature)) (Primitives.Instruction.draw (.lit 1)
              (agent := Primitives.NounPhrase.you)),
          whenever (Primitives.GameEvent.attacksWith (a Primitives.Predicate.anyPlayer) none (counted (atLeast 5) creature))
            (Primitives.Instruction.sequence [Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 3) (each Primitives.Predicate.opponent), gainLife (.lit 3) (agent :=
                Primitives.NounPhrase.you)]) ],
      power := stat 4, toughness := stat 4 } }

/-- Syr Konrad, the Grim -/
def syrKonradTheGrim : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Syr Konrad, the Grim", cost := some [generic 3, pip .black, pip .black],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Knight"],
      text :=
        [ triggeredOr (Primitives.GameEvent.dies (a (Primitives.Predicate.and [creature, Primitives.Predicate.otherThan Primitives.NounPhrase.this])))
            [ putIntoFrom (a creature) graveyard (Primitives.EventSource.anywhereBut [battlefield]),
              leavesZone (a (Primitives.Predicate.and [creature, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)])) (graveyardOf Primitives.NounPhrase.you) ]
            (Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 1) (each Primitives.Predicate.opponent)),
          activated (Primitives.Cost.mana [generic 1, pip .black]) (mill (.lit 1) (each Primitives.Predicate.anyPlayer) (agent := (each
              Primitives.Predicate.anyPlayer))) ],
      power := stat 5, toughness := stat 4 } }

/-- Destructive Revelry -/
def destructiveRevelry : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Destructive Revelry", cost := some [pip .red, pip .green], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequence
            [ destroy (target (Primitives.Predicate.or [artifact, enchantment])),
              Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 2) (controllerOf (that .permanent)) ]) ] } }

/-- Fuming Effigy -/
def fumingEffigy : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fuming Effigy", cost := some [generic 3, pip .red], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ whenever (leavesZone (counted (atLeast 1) (Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you))) (graveyardOf Primitives.NounPhrase.you))
            (Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 1) (each Primitives.Predicate.opponent)) ],
      power := stat 4, toughness := stat 3 } }

/-- Breeches, Brazen Plunderer's slice -/
def eachOfThoseOpponentsTopCard : Instruction :=
  Primitives.Instruction.sequence
    [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 1) (each Primitives.Predicate.opponent),
      exile (Primitives.NounPhrase.librarySlice .top (.lit 1) (Primitives.NounPhrase.eachOf (those .player))) ]
theorem okEachOfThoseOpponentsTopCard : Instruction.check [] eachOfThoseOpponentsTopCard = [] := by
  decide

/-- Inquisitor's Flail -/
def inquisitorsFlail : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Inquisitor's Flail", cost := some [generic 2], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.damageRule .combatOnly (Primitives.DamageAgent.dealtBy (Primitives.NounPhrase.attachHost .equipped (.type .creature))) Primitives.DamageScope.everywhere
            (Primitives.DamageOp.scale (Primitives.DamageScale.multiplied .doubled)) .repeatedly),
          Primitives.Ability.static (Primitives.StaticSpec.damageRule .combatOnly
            (Primitives.DamageAgent.dealtBy (a (otherCreature (Primitives.NounPhrase.attachHost .equipped (.type .creature)))))
            (Primitives.DamageScope.toRecipient (Primitives.NounPhrase.attachHost .equipped (.type .creature))) (Primitives.DamageOp.scale (Primitives.DamageScale.multiplied .doubled))
            .repeatedly),
          keywordCosting "Equip" (Primitives.Cost.mana [generic 2]) ] } }

/-- Oath of Kaya -/
def oathOfKaya : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Oath of Kaya", cost := some [generic 1, pip .white, pip .black],
      supertypes := [.legendary], types := [.enchantment],
      text :=
        [ when (Primitives.GameEvent.enters Primitives.NounPhrase.this none)
            (Primitives.Instruction.sequence [Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 3) (target anyTarget), gainLife (.lit 3) (agent :=
                Primitives.NounPhrase.you)]),
          whenever
            (Primitives.GameEvent.attacksWith anOpponent
              (some (a (Primitives.Predicate.and [Primitives.Predicate.hasType .planeswalker, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
              (counted (atLeast 1) creature))
            (Primitives.Instruction.sequence [Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 2) (that .player), gainLife (.lit 2) (agent :=
                Primitives.NounPhrase.you)]) ] } }

/-- Frostwielder -/
def frostwielder : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Frostwielder", cost := some [generic 2, pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Shaman"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.replacement
            (Primitives.GameEvent.dies (a (Primitives.Predicate.and [creature,
              Primitives.Predicate.happenedTo (Primitives.LookbackClause.mk .damageTaken .thisTurn (some (Primitives.EventComplement.involving thisCreature)))])))
            [] none (exile it) .repeatedly none),
          activated Primitives.Cost.tapSymbol (Primitives.Instruction.dealDamage thisCreature (.lit 1) (target anyTarget)) ],
      power := stat 1, toughness := stat 2 } }

/-- Crackling Doom -/
def cracklingDoom : Instruction :=
  Primitives.Instruction.sequence
    [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 2) (each Primitives.Predicate.opponent),
      sacrifice
        (a (Primitives.Predicate.and [creature,
                  Primitives.Predicate.superlative .max (.stat .power)
                    (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller (that .player)])])) (agent := (each
                        Primitives.Predicate.opponent)) ]
theorem okCracklingDoom : Instruction.check [] cracklingDoom = [] := by decide
def theFallen : Ability :=
  at_ (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
    (Primitives.Instruction.dealDamage thisCreature (.lit 1)
      (each (Primitives.Predicate.and [Primitives.Predicate.or [Primitives.Predicate.opponent, Primitives.Predicate.hasType .planeswalker],
                   happenedToInvolving .damageTaken .thisGame thisCreature])))
theorem okTheFallen : Ability.check [] theFallen = [] := by decide
/-- Cavalcade of Calamity -/
def cavalcadeOfCalamity : Ability :=
  whenever
    (attacks (a (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.compare [.stat .power] .atMost (.lit 1)])))
    (Primitives.Instruction.dealDamage thisEnchantment (.lit 1)
      (the (Primitives.Predicate.and [Primitives.Predicate.or [Primitives.Predicate.anyPlayer, Primitives.Predicate.hasType .planeswalker],
                  Primitives.Predicate.inCombat .attackedBy (some (that (.type .creature)))])))
theorem okCavalcadeOfCalamity : Ability.check [] cavalcadeOfCalamity = [] := by decide

/-- Blind Fury -/
def blindFury : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blind Fury", cost := some [generic 2, pip .red, pip .red], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequence
            [ Primitives.Instruction.establish (Primitives.StaticSpec.abilityLoss (allOf creature) [Primitives.AbilityLost.written (keyword "Trample")])
                (some untilEndOfTurn),
              Primitives.Instruction.establish
                (Primitives.StaticSpec.damageRule .combatOnly (Primitives.DamageAgent.dealtBy (a creature)) (Primitives.DamageScope.toRecipient (a creature))
                  (Primitives.DamageOp.scale (Primitives.DamageScale.multiplied .doubled)) .repeatedly)
                (some Primitives.Duration.thisTurn) ]) ] } }

/-- Chandra, Awakened Inferno's emblem -/
def chandraAwakenedInfernoEmblem : Instruction :=
  Primitives.Instruction.getEmblem
    [ at_ (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
        (Primitives.Instruction.dealDamage (Primitives.NounPhrase.asMarker .emblem Primitives.NounPhrase.this) (.lit 1) Primitives.NounPhrase.you) ] (agent := Primitives.NounPhrase.you)
theorem okChandraAwakenedInfernoEmblem :
    Instruction.check [] chandraAwakenedInfernoEmblem = [] := by decide

/-- Keeper of the Flame -/
def keeperOfTheFlame : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Keeper of the Flame", cost := some [pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [pip .red], Primitives.Cost.tapSymbol])
            (Primitives.Instruction.sequence
              [ chooseWhile
                  (target (Primitives.Predicate.and [Primitives.Predicate.opponent, Primitives.Predicate.compare [.playerStat .lifeTotal] .greater (lifeTotalOf Primitives.NounPhrase.you)]))
                  (Primitives.Concurrent.whileDoing (Primitives.GameEvent.activates Primitives.NounPhrase.you thisAbility)),
                Primitives.Instruction.dealDamage thisCreature (.lit 2) (that .player) ]) ],
      power := stat 1, toughness := stat 2 } }

def diabolicEdict : Instruction := sacrifice (aTheirChoice creature) (agent := (target Primitives.Predicate.anyPlayer))
theorem okDiabolicEdict : Instruction.check [] diabolicEdict = [] := by decide
def innocentBlood : Instruction := sacrifice (aTheirChoice creature) (agent := (each Primitives.Predicate.anyPlayer))
theorem okInnocentBlood : Instruction.check [] innocentBlood = [] := by decide
def cryOfContrition : Instruction := discard (a (Primitives.Predicate.inZone hand)) (agent := (target Primitives.Predicate.anyPlayer))
theorem okCryOfContrition : Instruction.check [] cryOfContrition = [] := by decide
def cyclingCost : Instruction := discard Primitives.NounPhrase.this (agent := Primitives.NounPhrase.you)
theorem okCyclingCost : Instruction.check [] cyclingCost = [] := by decide
def raiseTheAlarm : Instruction :=
  create (.lit 2) (creatureToken 1 1 [.white] [creatureType "Soldier"])
theorem okRaiseTheAlarm : Instruction.check [] raiseTheAlarm = [] := by decide
def actOfTreason : Instruction := gainControl (target creature) (some untilEndOfTurn) (agent :=
    Primitives.NounPhrase.you)
theorem okActOfTreason : Instruction.check [] actOfTreason = [] := by decide
def wordsOfWorship : Instruction :=
  replaceNextEvent (Primitives.GameEvent.draws Primitives.NounPhrase.you) (gainLife (.lit 5) (agent := Primitives.NounPhrase.you)) (some Primitives.Duration.thisTurn)
theorem okWordsOfWorship : Instruction.check [] wordsOfWorship = [] := by decide
def theLastRoninII : Instruction :=
  Primitives.Instruction.triggerReflexively (mill (.lit 4) Primitives.NounPhrase.you (agent := Primitives.NounPhrase.you))
    (move (target (Primitives.Predicate.and [creature, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)])) hand)
theorem okTheLastRoninII : Instruction.check [] theLastRoninII = [] := by decide

def moonlitWakeCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Moonlit Wake", cost := some [generic 2, pip .white], types := [.enchantment],
      text := [moonlitWake] } }

def laquatussDisdain : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Laquatus's Disdain", cost := some [generic 1, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequence
            [ Primitives.Instruction.counterSpell (target (Primitives.Predicate.and [spell, Primitives.Predicate.castFrom graveyard])), Primitives.Instruction.draw (.lit 1) (agent :=
                Primitives.NounPhrase.you) ]) ] } }

/-- Stifle -/
def stifle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stifle", cost := some [pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none
            (Primitives.Instruction.counterSpell (target (Primitives.Predicate.or [Primitives.Predicate.abilityHead .anyActivated, Primitives.Predicate.abilityHead .anyTriggered]))) ] } }

/-- Disallow -/
def disallow : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Disallow", cost := some [generic 1, pip .blue, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none
            (Primitives.Instruction.counterSpell
              (target (Primitives.Predicate.or [spell, Primitives.Predicate.abilityHead .anyActivated, Primitives.Predicate.abilityHead .anyTriggered]))) ] } }

/-- Fry -/
def fry : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fry", cost := some [generic 1, pip .red], types := [.instant],
      text :=
        [ Primitives.Ability.static (objectCant (.action "Counter") Primitives.NounPhrase.this),
          Primitives.Ability.spell none
            (Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 5)
              (target (Primitives.Predicate.and [Primitives.Predicate.or [creature, Primitives.Predicate.hasType .planeswalker], Primitives.Predicate.or [Primitives.Predicate.colorIs .white, Primitives.Predicate.colorIs .blue]]))) ] } }

/-- Termination Facilitator -/
def terminationFacilitator : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Termination Facilitator", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Assassin"],
      text :=
        [ activatedOnlyDuring Primitives.Cost.tapSymbol
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "Bounty"))
              (target (Primitives.Predicate.or [creature, Primitives.Predicate.hasType .planeswalker])))
            Primitives.Timing.asSorcery,
          whenever
            (Primitives.GameEvent.isDealtDamage .any
              (a (Primitives.Predicate.and [Primitives.Predicate.or [creature, Primitives.Predicate.hasType .planeswalker], Primitives.Predicate.hasPossessor .controller anOpponent,
                        Primitives.Predicate.hasCounters (some (.named "Bounty"))])))
            (destroy it) ],
      power := stat 1, toughness := stat 3 } }

end Semantics.Cards
